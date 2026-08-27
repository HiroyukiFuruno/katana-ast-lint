#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel)"
CONTRACT_SCRIPT="$REPO_ROOT/scripts/hooks/validate-issue-and-dependency-contract.sh"
LEFTHOOK_FILE="$REPO_ROOT/lefthook.yml"

failed=0

report_failed() {
  local message="$1"
  echo "FAIL: ${message}" >&2
  failed=1
}

mk_repo() {
  local tmp_root="$1"
  local origin="${tmp_root}/origin.git"
  local seed="${tmp_root}/seed"
  local repo="${tmp_root}/repo"

  git init --bare "${origin}" >/dev/null

  git init -b master "$seed" >/dev/null
  git -C "$seed" config user.name "Hook Test"
  git -C "$seed" config user.email "hook-test@example.com"
  cat > "$seed/Cargo.toml" <<'EOF'
[package]
name = "hook-test"
version = "0.1.0"
EOF
  cat > "$seed/Cargo.lock" <<'EOF'
# lock placeholder
EOF
  git -C "$seed" add Cargo.toml Cargo.lock
  git -C "$seed" commit -m "seed #1" >/dev/null
  git -C "$seed" remote add origin "${origin}"
  git -C "$seed" push -u origin master >/dev/null

  git clone "$origin" "$repo" >/dev/null
  git -C "$repo" config user.name "Hook Test"
  git -C "$repo" config user.email "hook-test@example.com"

  printf '%s\n' "$repo"
}

run_case() {
  local title="$1"
  local commit_message="$2"
  local expected_status="$3"
  local prepare_fn="$4"

  local tmp_root
  tmp_root="$(mktemp -d)"
  local repo_dir
  repo_dir="$(mk_repo "$tmp_root")"

  git -C "$repo_dir" switch -c "$title" "origin/master"

  "$prepare_fn" "$repo_dir"
  git -C "$repo_dir" commit -m "$commit_message"

  set +e
  (cd "$repo_dir" && "$CONTRACT_SCRIPT")
  local status=$?
  set -e

  if (( status != expected_status )); then
    report_failed "${title}: expected status ${expected_status} but got ${status}"
  fi

  rm -rf "$tmp_root"
}

prepare_pass_normal() {
  local repo_dir="$1"
  cat > "$repo_dir/Cargo.toml" <<'EOF'
[package]
name = "hook-test"
version = "0.2.0"
EOF
  cat > "$repo_dir/Cargo.lock" <<'EOF'
# lock placeholder changed
EOF
  mkdir -p "$repo_dir/docs"
  printf '%s\n' "dependency evidence" > "$repo_dir/docs/migration.md"
  git -C "$repo_dir" add Cargo.toml Cargo.lock docs/migration.md
}

prepare_fail_no_issue() {
  local repo_dir="$1"
  printf '%s\n' "plain change" > "$repo_dir/README.md"
  git -C "$repo_dir" add README.md
}

prepare_fail_manifest_missing_lock() {
  local repo_dir="$1"
  cat > "$repo_dir/Cargo.toml" <<'EOF'
[package]
name = "hook-test"
version = "0.2.1"
EOF
  mkdir -p "$repo_dir/release-notes"
  printf '%s\n' "release note" > "$repo_dir/release-notes/v0.2.1.md"
  git -C "$repo_dir" add Cargo.toml release-notes/v0.2.1.md
}

check_lefthook_order() {
  local prepush_line check_line dep_line
  prepush_line=$(rg -n "^pre-push:" "$LEFTHOOK_FILE" | awk -F: '{print $1}' | head -n1)
  check_line=$(rg -n "^[[:space:]]+check-strict:" "$LEFTHOOK_FILE" | awk -F: '{print $1}' | head -n1)
  dep_line=$(rg -n "dependency-and-issue-contract:" "$LEFTHOOK_FILE" | awk -F: '{print $1}' | head -n1)

  if [[ -z "$prepush_line" ]]; then
    report_failed "pre-push block missing in lefthook.yml"
    return
  fi
  if [[ -z "$check_line" ]]; then
    report_failed "check-strict command missing in lefthook.yml"
    return
  fi
  if [[ -z "$dep_line" ]]; then
    report_failed "dependency-and-issue-contract command missing in lefthook.yml"
    return
  fi

  if (( prepush_line >= check_line || check_line >= dep_line )); then
    report_failed "Existing hook delegation order is wrong: check-strict should run before dependency-and-issue-contract under pre-push"
  fi
}

run_case "issue-contract-pass" "feat: update dependency contracts #12" 0 prepare_pass_normal
run_case "issue-contract-fail-no-issue" "chore: update docs without issue" 1 prepare_fail_no_issue
run_case "issue-contract-fail-manifest-lock" "chore: dependency manifest bump #12" 1 prepare_fail_manifest_missing_lock
check_lefthook_order

if (( failed == 0 )); then
  echo "Hook contract tests passed"
  exit 0
fi

exit 1
