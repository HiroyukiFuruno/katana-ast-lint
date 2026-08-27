#!/usr/bin/env bash
set -euo pipefail

if ! command -v git >/dev/null 2>&1; then
  echo "git is required for issue/dependency contract checks." >&2
  exit 1
fi

DEFAULT_BRANCH="${KATANA_DEFAULT_BRANCH:-master}"
ORIGIN_REF="$(git symbolic-ref refs/remotes/origin/HEAD 2>/dev/null || true)"
if [[ -n "$ORIGIN_REF" ]]; then
  DEFAULT_BRANCH="${ORIGIN_REF#refs/remotes/origin/}"
fi

current_branch="$(git rev-parse --abbrev-ref HEAD)"
if [[ "$current_branch" == "$DEFAULT_BRANCH" || "$current_branch" == "HEAD" || -z "$current_branch" ]]; then
  exit 0
fi

base_ref="$(git rev-parse "${current_branch}@{upstream}" 2>/dev/null || true)"
if [[ -z "$base_ref" ]]; then
  base_ref="origin/${DEFAULT_BRANCH}"
fi
if ! git rev-parse --verify "$base_ref" >/dev/null 2>&1; then
  base_ref="$(git merge-base "${current_branch}" "origin/${DEFAULT_BRANCH}" 2>/dev/null || true)"
fi
if [[ -z "$base_ref" ]]; then
  if git rev-parse "${current_branch}^" >/dev/null 2>&1; then
    base_ref="$(git rev-parse "${current_branch}^")"
  else
    echo "No comparison base found for branch ${current_branch}."
    echo "Skipping issue/dependency contract checks."
    exit 0
  fi
fi

commits_without_issue=()
while IFS= read -r commit; do
  message="$(git log -n 1 --format=%B "$commit")"
  if ! grep -Eq "#[0-9]+" <<< "$message"; then
    commits_without_issue+=("$(git rev-parse --short "$commit")")
  fi
done < <(git rev-list --no-merges "${base_ref}..${current_branch}")

if (( ${#commits_without_issue[@]} > 0 )); then
  echo "The following commits do not contain an issue reference (e.g. #12):" >&2
  printf '  - %s\n' "${commits_without_issue[@]}" >&2
  exit 1
fi

mapfile -t changed_files < <(git diff --name-only "${base_ref}..${current_branch}")

has_manifest_update=false
has_lock_update=false
has_evidence_update=false

for file in "${changed_files[@]}"; do
  case "$file" in
    Cargo.toml|*/Cargo.toml)
      has_manifest_update=true
      ;;
    Cargo.lock|*/Cargo.lock)
      has_lock_update=true
      ;;
    CHANGELOG.md|release-notes/*|release-readiness/*|docs/*|openspec/*)
      has_evidence_update=true
      ;;
  esac
done

if [[ "$has_manifest_update" == true ]]; then
  if [[ "$has_lock_update" != true ]]; then
    echo "Dependency manifest changed but Cargo.lock was not updated in the same change set." >&2
    echo "Manifest/lockfile pair is required by downstream dependency-update contract." >&2
    exit 1
  fi

  if [[ "$has_evidence_update" != true ]]; then
    echo "Dependency manifest/lockfile updates require migration/verification evidence files in this change set." >&2
    echo "Update docs, openspec, or release notes to record upstream release version, migration note, and validation evidence." >&2
    exit 1
  fi
fi

echo "Issue and dependency contract checks passed for ${current_branch}."
