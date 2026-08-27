#!/usr/bin/env bash
set -euo pipefail

DEFAULT_BRANCH="${KATANA_DEFAULT_BRANCH:-master}"
REMOTE_NAME="${KATANA_REMOTE:-origin}"
TARGET_VERSION="${KATANA_RELEASE_VERSION:-}"
apply_changes=0
skip_pull=0

usage() {
  cat <<'EOF'
Usage: scripts/release/cleanup-after-release.sh [options]

Options:
  --default-branch <name>   Branch name to switch to before cleanup (default: master).
  --remote <name>           Git remote to inspect and clean (default: origin).
  --version <vX.Y.Z>        Verify the GitHub Release exists before cleanup.
  --apply                   Execute deletion (omit for dry-run).
  --skip-pull               Skip pull --ff-only (for offline/manual testing).
  --help                    Show this help.
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --default-branch)
      DEFAULT_BRANCH="$2"
      shift 2
      ;;
    --remote)
      REMOTE_NAME="$2"
      shift 2
      ;;
    --version)
      TARGET_VERSION="$2"
      shift 2
      ;;
    --apply)
      apply_changes=1
      shift
      ;;
    --skip-pull)
      skip_pull=1
      shift
      ;;
    --help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown option: $1" >&2
      usage
      exit 1
      ;;
  esac
done

require_release() {
  if [[ -z "${TARGET_VERSION}" ]]; then
    return 0
  fi
  if ! command -v gh >/dev/null 2>&1; then
    echo "gh is required to verify release existence when --version is set." >&2
    exit 1
  fi
  if ! gh release view "${TARGET_VERSION}" --repo "${GITHUB_REPOSITORY:-HiroyukiFuruno/katana-ast-lint}" >/dev/null 2>&1; then
    echo "Release ${TARGET_VERSION} does not exist yet; aborting cleanup." >&2
    exit 1
  fi
}

is_merged_into_default() {
  local candidate="$1"
  if git merge-base --is-ancestor "${candidate}" "${DEFAULT_BRANCH}" 2>/dev/null; then
    return 0
  fi
  return 1
}

current_ref="$(git symbolic-ref --short HEAD 2>/dev/null || true)"
if [[ -z "${current_ref}" ]]; then
  echo "Detached HEAD; release cleanup requires a branch checkout." >&2
  exit 1
fi

require_release

if ! git switch "$DEFAULT_BRANCH" >/dev/null 2>&1; then
  echo "Failed to switch to ${DEFAULT_BRANCH} before cleanup." >&2
  exit 1
fi

if (( skip_pull == 0 )); then
  if ! git pull --ff-only "${REMOTE_NAME}" "${DEFAULT_BRANCH}"; then
    echo "Failed to refresh ${DEFAULT_BRANCH} from ${REMOTE_NAME} with --ff-only." >&2
    exit 1
  fi
else
  echo "Skipping pull --ff-only because --skip-pull was requested."
fi

if [[ -n "$(git status --porcelain)" ]]; then
  echo "Refusing cleanup with dirty working tree on ${DEFAULT_BRANCH}." >&2
  exit 1
fi

default_repo_branch="$(git symbolic-ref --short "refs/remotes/${REMOTE_NAME}/HEAD" 2>/dev/null || true)"
if [[ -n "$default_repo_branch" ]]; then
  default_repo_branch="${default_repo_branch#refs/remotes/$REMOTE_NAME/}"
  if [[ "$default_repo_branch" != "$DEFAULT_BRANCH" ]]; then
    echo "Remote ${REMOTE_NAME} default branch is ${default_repo_branch}, expected ${DEFAULT_BRANCH}." >&2
    echo "Release cleanup will continue with ${DEFAULT_BRANCH}."
  fi
fi

declare -A release_worktree_paths=()
declare -A release_worktree_targets=()
current_worktree="$(git rev-parse --show-toplevel)"
worktree_path=""
worktree_branch=""
while IFS= read -r line; do
  case "$line" in
    worktree\ *)
      if [[ -n "$worktree_path" && "$worktree_branch" == refs/heads/release/* ]]; then
        branch_name="${worktree_branch#refs/heads/}"
        release_worktree_paths["$branch_name"]="${worktree_path}"
      fi
      worktree_path="${line#worktree }"
      worktree_branch=""
      ;;
    branch\ refs/heads/*)
      worktree_branch="${line#branch }"
      ;;
  esac
done < <(git worktree list --porcelain)
if [[ -n "$worktree_path" && "$worktree_branch" == refs/heads/release/* ]]; then
  branch_name="${worktree_branch#refs/heads/}"
  release_worktree_paths["$branch_name"]="${worktree_path}"
fi

for release_branch in "${!release_worktree_paths[@]}"; do
  if [[ "${release_worktree_paths[$release_branch]}" == "${current_worktree}" ]]; then
    continue
  fi
  if ! is_merged_into_default "$release_branch"; then
    continue
  fi
  release_worktree_targets["$release_branch"]=1
done

delete_remote_branches=()
while IFS= read -r branch_ref; do
  branch_name="${branch_ref#refs/remotes/${REMOTE_NAME}/}"
  [[ -z "$branch_name" ]] && continue
  [[ "$branch_name" == "$DEFAULT_BRANCH" ]] && continue
  if ! git merge-base --is-ancestor "${branch_ref}" "refs/remotes/${REMOTE_NAME}/${DEFAULT_BRANCH}" 2>/dev/null; then
    continue
  fi
  if git show-ref --verify --quiet "refs/heads/${branch_name}"; then
    continue
  fi
  delete_remote_branches+=("$branch_name")
done < <(git for-each-ref --format='%(refname)' "refs/remotes/${REMOTE_NAME}/release/*" || true)

if (( ${#delete_remote_branches[@]} == 0 )); then
  echo "No remote release branches to clean."
else
  for branch_name in "${delete_remote_branches[@]}"; do
    if (( apply_changes == 1 )); then
      git push "${REMOTE_NAME}" --delete "${branch_name}"
      echo "Deleted remote branch: ${REMOTE_NAME}/${branch_name}"
    else
      echo "[dry-run] would delete remote branch: ${REMOTE_NAME}/${branch_name}"
    fi
  done
fi

for branch_name in "${!release_worktree_targets[@]}"; do
  path="${release_worktree_paths["$branch_name"]}"
  if [[ "$path" == "$current_worktree" ]]; then
    continue
  fi
  if (( apply_changes == 1 )); then
    git worktree remove "$path"
    echo "Deleted worktree: ${path}"
  else
    echo "[dry-run] would delete worktree: ${path}"
  fi
done

delete_local_branches=()
while IFS= read -r branch; do
  if [[ -z "$branch" ]]; then
    continue
  fi
  if [[ "$branch" == "$DEFAULT_BRANCH" || "$branch" == "$current_ref" ]]; then
    continue
  fi
  if ! is_merged_into_default "$branch"; then
    continue
  fi
  delete_local_branches+=("$branch")
done < <(git branch --list --format='%(refname:short)' 'release/*' 2>/dev/null || true)

if (( ${#delete_local_branches[@]} == 0 )); then
  echo "No local release branches to clean."
else
  for branch in "${delete_local_branches[@]}"; do
    if (( apply_changes == 1 )); then
      git branch -d "$branch"
      echo "Deleted local branch: ${branch}"
    else
      echo "[dry-run] would delete local branch: ${branch}"
    fi
  done
fi

if (( apply_changes == 1 )); then
  git worktree prune
  echo "Pruned stale worktrees."
fi

echo "Release cleanup completed for ${DEFAULT_BRANCH}."
