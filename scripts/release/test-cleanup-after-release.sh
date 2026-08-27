#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel)"
CLEANUP_SCRIPT="$REPO_ROOT/scripts/release/cleanup-after-release.sh"

failed=0

report_failed() {
  local message="$1"
  echo "FAIL: ${message}" >&2
  failed=1
}

assert_clean() {
  local branch_name="$1"
  if ! git -C "$REPO" show-ref --verify --quiet "refs/heads/${branch_name}"; then
    return 0
  fi
  report_failed "Expected branch ${branch_name} to be removed but it still exists"
}

assert_exists() {
  local branch_name="$1"
  if git -C "$REPO" show-ref --verify --quiet "refs/heads/${branch_name}"; then
    return 0
  fi
  report_failed "Expected branch ${branch_name} to remain but it is missing"
}

assert_path_not_exists() {
  local path="$1"
  if [[ -d "$path" ]]; then
    report_failed "Expected worktree path ${path} to be removed"
  fi
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
name = "release-hook-test"
version = "0.1.0"
EOF
  cat > "$seed/Cargo.lock" <<'EOF'
# placeholder lock
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

main_case() {
  local tmp_root="$1"
  REPO="$(mk_repo "$tmp_root")"
  local worktree_merged="${tmp_root}/wt-release-v0.1.0"

  # Create a merged release branch.
  git -C "$REPO" switch -c release/v0.1.0
  printf '%s\n' "release branch payload" > "$REPO/merged.txt"
  git -C "$REPO" add merged.txt
  git -C "$REPO" commit -m "chore: release worktree scenario #1"

  git -C "$REPO" switch master
  git -C "$REPO" merge --no-edit release/v0.1.0
  git -C "$REPO" push >/dev/null

  git -C "$REPO" worktree add "$worktree_merged" release/v0.1.0

  # Create an unmerged release branch that must be retained.
  git -C "$REPO" switch master
  git -C "$REPO" switch -c release/v0.1.1
  printf '%s\n' "unmerged payload" > "$REPO/unmerged.txt"
  git -C "$REPO" add unmerged.txt
  git -C "$REPO" commit -m "chore: unmerged branch #1"

  # Build a stale feature branch and advance remote master once.
  local stale_clone="${tmp_root}/stale"
  git clone "$REPO/.git" "$stale_clone" >/dev/null
  git -C "$stale_clone" checkout -b master origin/master >/dev/null
  printf '%s\n' "upstream update" > "$stale_clone/upstream-update.txt"
  git -C "$stale_clone" add upstream-update.txt
  git -C "$stale_clone" commit -m "chore: upstream change #1"
  git -C "$stale_clone" push >/dev/null
  local expected_head
  expected_head="$(git -C "$stale_clone" rev-parse HEAD)"

  git -C "$REPO" switch -c feature/contract-cleanup-test
  printf '%s\n' "feature payload" > "$REPO/feature.txt"
  git -C "$REPO" add feature.txt
  git -C "$REPO" commit -m "chore: feature branch #1"

  # Execute cleanup in apply mode. This should switch to master, pull --ff-only,
  # delete merged local branch/worktree, and keep the unmerged ones.
  if ! (cd "$REPO" && "$CLEANUP_SCRIPT" --default-branch master --apply); then
    report_failed "cleanup script exited with failure"
  fi

  local local_master_head="$(git -C "$REPO" rev-parse master)"
  if [[ "$local_master_head" != "$expected_head" ]]; then
    report_failed "master was not pulled to remote tip"
  fi

  if ! git -C "$REPO" symbolic-ref --short HEAD | grep -q '^master$'; then
    report_failed "cleanup did not leave repository on default branch"
  fi

  assert_clean release/v0.1.0
  assert_exists release/v0.1.1

  assert_path_not_exists "$worktree_merged"

  # Return to a branch that keeps the unmerged release branch in place.
  git -C "$REPO" switch release/v0.1.1
  git -C "$REPO" symbolic-ref --short HEAD | grep -q '^release/v0.1.1$' || report_failed "cleanup did not preserve unmerged release branch"

  # End the case on the unmerged branch to avoid moving state unexpectedly.
  git -C "$REPO" checkout -q -b temp-endpoint
}

REPO=""

tmp_root="$(mktemp -d)"
trap 'rm -rf "$tmp_root"' EXIT

main_case "$tmp_root"

if (( failed == 0 )); then
  echo "Release cleanup tests passed"
  exit 0
fi

exit 1
