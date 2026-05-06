#!/usr/bin/env bash
set -euo pipefail

INPUT_VERSION="${1:-}"
CARGO_VERSION="$(awk -F '"' '/^version = / { print $2; exit }' Cargo.toml)"

if [[ -z "$INPUT_VERSION" ]]; then
  INPUT_VERSION="$CARGO_VERSION"
fi

VERSION_BARE="${INPUT_VERSION#v}"
VERSION="v${VERSION_BARE}"

if [[ ! "$VERSION_BARE" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.-]+)?(\+[0-9A-Za-z.-]+)?$ ]]; then
  echo "Invalid release version: ${INPUT_VERSION}" >&2
  exit 1
fi

# Allow the release version to have a suffix (e.g. from branch names) as long as the base version matches Cargo.toml
if [[ "$VERSION_BARE" != "$CARGO_VERSION" && "$VERSION_BARE" != "$CARGO_VERSION-"* ]]; then
  echo "Cargo.toml version (${CARGO_VERSION}) does not match release version (${VERSION_BARE})" >&2
  exit 1
fi

echo "Release version: ${VERSION}"

if [[ -n "${GITHUB_OUTPUT:-}" ]]; then
  {
    echo "version=${VERSION}"
    echo "version_bare=${VERSION_BARE}"
    echo "cargo_version=${CARGO_VERSION}"
  } >> "$GITHUB_OUTPUT"
fi
