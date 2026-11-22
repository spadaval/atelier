---
schema: atelier.item
schema_version: 3
id: atelier-fxwl
title: Silence the unused-variable warning that every release build of atelier prints
status: approved
mission: null
labels: []
priority: medium
blocked_by: []
implementers:
- mcp:launch:b3b5ffd62a4c227e4ec8ce23f1e32b83538dd28e5e6c91e0a2d3d91ead863ef6
ready: null
trivial: null
caused_by: null
created_at: 2026-11-22T08:16:56.815304990Z
---

## Goal

`cargo install --path crates/atelier-cli --locked` prints `warning: unused
variable: variable` for `commands::hold`, whose body exists only in debug
builds. CI's clippy runs a debug build and never sees it. Found while
installing `master` during `atelier-ww7w`.

## Done when

- [x] `cargo build --release -p atelier-cli --locked` prints no warning.
- [x] Debug builds are unchanged: the allowance applies only where `debug_assertions` is off.

## Scope

In: one attribute on `hold` in `crates/atelier-cli/src/commands.rs`.

## Report

One line: `#[cfg_attr(not(debug_assertions), allow(unused_variables))]` on
`commands::hold`. `cargo build --release -p atelier-cli --locked` then
finishes with no warning, where it printed one before, and `cargo fmt --
--check` passes. The attribute changes no behaviour in either build, which
the diff shows whole. The lead of `atelier-ww7w` made
it outside that mission's plan, as upkeep found on the way.
