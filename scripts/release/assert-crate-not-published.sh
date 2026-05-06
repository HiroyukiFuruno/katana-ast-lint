#!/usr/bin/env bash
set -euo pipefail

VERSION_BARE="${1:?version is required}"
PACKAGE="${2:-katana-ast-lint}"

if cargo info "${PACKAGE}@${VERSION_BARE}" --registry crates-io >/dev/null 2>&1; then
  echo "${PACKAGE} ${VERSION_BARE} is already published on crates.io." >&2
  echo "Bump Cargo.toml before dispatching a crates.io publish release." >&2
  exit 1
fi

echo "${PACKAGE} ${VERSION_BARE} is not published on crates.io."
