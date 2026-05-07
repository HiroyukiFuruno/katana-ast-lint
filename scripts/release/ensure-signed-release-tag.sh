#!/usr/bin/env bash
set -euo pipefail

TAG="${1:?tag is required}"
REMOTE="${2:-origin}"
TAGGER_NAME="${RELEASE_TAGGER_NAME:-Katana Release}"
TAGGER_EMAIL="${RELEASE_TAGGER_EMAIL:-hfuruno0114@gmail.com}"

validate_local_tag() {
  if [[ "$(git cat-file -t "${TAG}" 2>/dev/null || true)" != "tag" ]]; then
    echo "${TAG} exists but is not an annotated signed tag." >&2
    exit 1
  fi
}

if git rev-parse -q --verify "refs/tags/${TAG}" >/dev/null; then
  validate_local_tag
  scripts/release/assert-tag-safe.sh "${TAG}" "${REMOTE}"
  git push "${REMOTE}" "refs/tags/${TAG}"
  echo "Tag ${TAG} already exists locally; pushed or confirmed remote tag."
  exit 0
fi

if git remote get-url "${REMOTE}" >/dev/null 2>&1; then
  git fetch --quiet "${REMOTE}" "refs/tags/${TAG}:refs/tags/${TAG}" || true
fi

if git rev-parse -q --verify "refs/tags/${TAG}" >/dev/null; then
  validate_local_tag
  echo "Tag ${TAG} already exists on ${REMOTE}; skipping creation."
  exit 0
fi

scripts/release/assert-tag-safe.sh "${TAG}" "${REMOTE}"

if [[ -z "${RELEASE_GPG_KEY:-}" ]]; then
  echo "RELEASE_GPG_KEY is required to create release tags automatically." >&2
  exit 1
fi

export GNUPGHOME="${GNUPGHOME:-${RUNNER_TEMP:-$(pwd)/target}/gnupg-release}"
mkdir -p "${GNUPGHOME}"
chmod 700 "${GNUPGHOME}"
umask 077
printf '%s' "${RELEASE_GPG_KEY}" | gpg --batch --import -

key_id="$(gpg --list-secret-keys --keyid-format=LONG --with-colons | awk -F: '/^sec/ { print $5; exit }')"
if [[ -z "${key_id}" ]]; then
  echo "No secret key available after importing RELEASE_GPG_KEY." >&2
  exit 1
fi

if [[ -n "${RELEASE_GPG_PASSPHRASE:-}" ]]; then
  gpg_wrapper="${GNUPGHOME}/gpg-wrapper.sh"
  cat > "${gpg_wrapper}" <<'WRAPPER'
#!/usr/bin/env bash
set -euo pipefail
printf '%s' "${RELEASE_GPG_PASSPHRASE}" | gpg --batch --pinentry-mode loopback --passphrase-fd 0 "$@"
WRAPPER
  chmod 700 "${gpg_wrapper}"
  git config gpg.program "${gpg_wrapper}"
fi

git config user.name "${TAGGER_NAME}"
git config user.email "${TAGGER_EMAIL}"
git -c user.signingkey="${key_id}" tag -s "${TAG}" -m "katana-ast-lint ${TAG}"
git push "${REMOTE}" "refs/tags/${TAG}"
