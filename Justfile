set shell := ["bash", "-uc"]

REPO_ROOT := justfile_directory()
RTK := env_var_or_default("RTK", `command -v rtk 2> /dev/null || true`)
RTK_CMD := if RTK == "" { "" } else { RTK + " " }
CARGO := env_var_or_default("CARGO", RTK_CMD + "cargo")
JOBS := env_var_or_default("JOBS", "2")
VERSION := env_var_or_default("VERSION", `awk -F '"' '/^version = / { print $2; exit }' Cargo.toml`)
VERSION_BARE := replace(VERSION, "v", "")
TAG := "v" + VERSION_BARE
RELEASE_REPO := env_var_or_default("RELEASE_REPO", "HiroyukiFuruno/katana-ast-lint")
RELEASE_TAGGER_NAME := env_var_or_default("RELEASE_TAGGER_NAME", "HiroyukiFuruno")
RELEASE_TAGGER_EMAIL := env_var_or_default("RELEASE_TAGGER_EMAIL", "hfuruno0114@gmail.com")

export RUSTFLAGS := env_var_or_default("RUSTFLAGS", "-D warnings")

[private]
default: help

# Show available recipes
help:
    @just --list --unsorted

import 'just/quality.just'
import 'just/release.just'
import 'just/maintenance.just'
