# Verify VERSION matches Cargo.toml
version-check:
    scripts/release/verify-version.sh "{{VERSION}}"

# Verify crate version is not already published
publish-target-check:
    scripts/release/assert-crate-not-published.sh "{{VERSION_BARE}}" katana-ast-lint

# Build package artifact and checksum
package:
    scripts/release/package-crate.sh "{{VERSION_BARE}}"

# Dry-run crates.io publication
publish-dry-run:
    {{CARGO}} publish --dry-run --locked

# Release preflight without requiring CARGO_REGISTRY_TOKEN
release-check: version-check publish-target-check check package publish-dry-run
    @echo "Release check passed for {{TAG}}"

# Create and push a signed annotated tag
release-tag:
    scripts/release/verify-version.sh "{{VERSION}}"
    scripts/release/assert-tag-safe.sh "{{TAG}}"
    @if git rev-parse -q --verify "refs/tags/{{TAG}}" >/dev/null; then \
        if [ "$(git cat-file -t "{{TAG}}")" != "tag" ]; then \
            echo "{{TAG}} exists but is not an annotated signed tag" >&2; \
            exit 1; \
        fi; \
        git tag -v "{{TAG}}"; \
    else \
        GIT_COMMITTER_NAME="{{RELEASE_TAGGER_NAME}}" \
        GIT_COMMITTER_EMAIL="{{RELEASE_TAGGER_EMAIL}}" \
        git -c user.name="{{RELEASE_TAGGER_NAME}}" -c user.email="{{RELEASE_TAGGER_EMAIL}}" tag -s "{{TAG}}" -m "katana-ast-lint {{TAG}}"; \
    fi
    git push origin "refs/tags/{{TAG}}"
    scripts/release/verify-tag-verified.sh "{{TAG}}" "{{RELEASE_REPO}}"

# Dispatch GitHub release workflow without crates.io publication
release-github: release-tag
    gh workflow run release.yml --repo {{RELEASE_REPO}} --ref master -f version="{{TAG}}" -f publish_crate=false

# Dispatch GitHub release workflow with crates.io publication
release-publish: release-tag
    scripts/release/assert-crate-not-published.sh "{{VERSION_BARE}}" katana-ast-lint
    gh secret list --repo {{RELEASE_REPO}} | grep -q '^CARGO_REGISTRY_TOKEN[[:space:]]' || { echo "CARGO_REGISTRY_TOKEN secret is required for crates.io publication" >&2; exit 1; }
    gh workflow run release.yml --repo {{RELEASE_REPO}} --ref master -f version="{{TAG}}" -f publish_crate=true

# Full release entrypoint
release: release-publish
