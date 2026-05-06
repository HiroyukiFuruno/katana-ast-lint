#!/usr/bin/env bash
set -euo pipefail

VERSION_BARE="${1:?version is required}"
PACKAGE_VERSION="$(awk -F '"' '/^version = / { print $2; exit }' Cargo.toml)"
PACKAGE="katana-ast-lint-${PACKAGE_VERSION}.crate"
PACKAGE_PATH="target/package/${PACKAGE}"
CHECKSUM_PATH="${PACKAGE_PATH}.sha256"

cargo package --locked --allow-dirty

if [[ ! -f "$PACKAGE_PATH" ]]; then
  echo "Expected package was not created: ${PACKAGE_PATH}" >&2
  exit 1
fi

if command -v sha256sum >/dev/null 2>&1; then
  sha256sum "$PACKAGE_PATH" > "$CHECKSUM_PATH"
else
  shasum -a 256 "$PACKAGE_PATH" > "$CHECKSUM_PATH"
fi

if [[ -n "${GITHUB_OUTPUT:-}" ]]; then
  {
    echo "package_path=${PACKAGE_PATH}"
    echo "checksum_path=${CHECKSUM_PATH}"
  } >> "$GITHUB_OUTPUT"
fi

echo "Packaged ${PACKAGE_PATH}"
echo "Checksum ${CHECKSUM_PATH}"
