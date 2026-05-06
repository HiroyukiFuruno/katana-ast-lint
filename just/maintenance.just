# ============================================================
# Maintenance
# ============================================================

# Update dependency crates safely (respects Cargo.toml SemVer)
update-safe:
    {{CARGO}} update

# Upgrade ALL dependencies to absolute latest versions (including breaking changes)
update:
    {{CARGO}} upgrade -i
    {{CARGO}} update
