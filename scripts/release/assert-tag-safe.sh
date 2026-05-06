#!/usr/bin/env bash
set -euo pipefail

TAG="${1:?tag is required}"
REMOTE="${2:-origin}"

local_exists=false
local_target=""
if git rev-parse -q --verify "refs/tags/${TAG}" >/dev/null; then
  local_exists=true
  local_target="$(git rev-parse "${TAG}^{}")"
fi

remote_target="$(
  git ls-remote --tags "${REMOTE}" "refs/tags/${TAG}^{}" |
    awk 'NR == 1 { print $1 }'
)"
if [[ -z "${remote_target}" ]]; then
  remote_target="$(
    git ls-remote --tags "${REMOTE}" "refs/tags/${TAG}" |
      awk 'NR == 1 { print $1 }'
  )"
fi

if [[ -z "${remote_target}" ]]; then
  echo "${TAG} does not exist on ${REMOTE}; creating a new tag is safe."
  exit 0
fi

if [[ "${local_exists}" != "true" ]]; then
  echo "${TAG} already exists on ${REMOTE}; fetch it before retrying release-tag." >&2
  exit 1
fi

if [[ "${local_target}" != "${remote_target}" ]]; then
  echo "${TAG} target differs from ${REMOTE}; refusing to overwrite a released tag." >&2
  echo "local:  ${local_target}" >&2
  echo "remote: ${remote_target}" >&2
  exit 1
fi

echo "${TAG} target matches ${REMOTE}; no tag overwrite is required."
