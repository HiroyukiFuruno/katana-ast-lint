# Sweep old build artifacts locally (older than 7 days)
sweep:
    @{{CARGO}} sweep --time 7 || true

# Remove build artifacts
clean: sweep
    {{CARGO}} clean

# Safely update Cargo.lock within the current Cargo.toml ranges
update-safe:
    {{CARGO}} update

# Update Cargo.toml dependency versions interactively, then refresh Cargo.lock
update:
    @if ! command -v cargo-upgrade >/dev/null 2>&1; then \
        echo "cargo-upgrade is required. Install with: cargo install cargo-edit" >&2; \
        exit 1; \
    fi
    {{CARGO}} upgrade -i
    {{CARGO}} update
