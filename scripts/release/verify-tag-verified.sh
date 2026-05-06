#!/usr/bin/env bash
set -euo pipefail

TAG="${1:?tag is required}"
REPO="${2:-${GH_REPO:-HiroyukiFuruno/katana-ast-lint}}"
ATTEMPTS="${KAL_TAG_VERIFICATION_ATTEMPTS:-12}"
SLEEP_SECONDS="${KAL_TAG_VERIFICATION_SLEEP_SECONDS:-5}"

ensure_local_tag() {
  git rev-parse -q --verify "refs/tags/${TAG}" >/dev/null && return
  if git remote get-url origin >/dev/null 2>&1; then
    git fetch --quiet origin "refs/tags/${TAG}:refs/tags/${TAG}" || true
  fi
  git rev-parse -q --verify "refs/tags/${TAG}" >/dev/null && return
  echo "${TAG} is not available locally and could not be fetched from origin." >&2
  exit 1
}

ensure_local_tag

if [[ "$(git cat-file -t "${TAG}" 2>/dev/null || true)" != "tag" ]]; then
  echo "${TAG} must be an annotated signed tag." >&2
  exit 1
fi

if ! git tag -v "${TAG}" >/dev/null 2>&1; then
  echo "Local git tag -v could not verify ${TAG}; continuing with GitHub verification." >&2
fi

if ! command -v gh >/dev/null 2>&1; then
  echo "gh CLI is required to verify GitHub tag signature state." >&2
  exit 1
fi

last_reason="unavailable"
last_tagger="unknown"

for ((attempt = 1; attempt <= ATTEMPTS; attempt++)); do
  if ! ref_type="$(gh api "repos/${REPO}/git/ref/tags/${TAG}" --jq '.object.type' 2>/dev/null)"; then
    last_reason="tag_ref_unavailable"
  elif [[ "${ref_type}" != "tag" ]]; then
    echo "${TAG} must resolve to an annotated tag object on GitHub, got ${ref_type}." >&2
    exit 1
  else
    tag_sha="$(gh api "repos/${REPO}/git/ref/tags/${TAG}" --jq '.object.sha')"
    verification="$(
      gh api "repos/${REPO}/git/tags/${tag_sha}" \
        --jq '[.verification.verified, .verification.reason, .tagger.name, .tagger.email] | @tsv'
    )"
    IFS=$'\t' read -r verified reason tagger_name tagger_email <<< "${verification}"

    last_reason="${reason:-unavailable}"
    last_tagger="${tagger_name:-unknown} <${tagger_email:-unknown}>"
    if [[ "${verified}" == "true" ]]; then
      echo "${TAG} is GitHub Verified for ${REPO}."
      exit 0
    fi
  fi

  if ((attempt < ATTEMPTS)); then
    sleep "${SLEEP_SECONDS}"
  fi
done

echo "${TAG} is not GitHub Verified for ${REPO}." >&2
echo "reason: ${last_reason}" >&2
echo "tagger: ${last_tagger}" >&2
echo "Use a tagger identity that GitHub can associate with the signing key." >&2
exit 1
