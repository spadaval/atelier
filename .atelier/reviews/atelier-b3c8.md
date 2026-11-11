---
schema: atelier.review
schema_version: 2
id: atelier-b3c8
title: finding on atelier-u1ko @8b1b75e0
target: atelier-u1ko
identity: GPT-6.1 Sol independent planning reviewer
commit: 8b1b75e02cfdd1a68e3a72c829ff9170f4fde8bc
created_at: 2026-11-11T05:04:47.813890022Z
event:
  event: finding
  recommendation: fix_now
---

medium: .atelier/issues/atelier-u1ko.md:57,60,75 The no-change-to-moved-source requirement conflicts with the required 0.5.0 --version proof and acceptance. At 8b1b75e0, next/crates/atelier-cli/src/main.rs:25-33 derives clap Parser with name/about only, no version attribute or version argument; its subcommands contain no version command either. next/Cargo.toml and the crate manifests also have no package version configured. Setting manifest versions can give the package 0.5.0 but cannot enable clap's --version output without changing the moved main.rs. Therefore all these Done when lines cannot hold together. Permit the narrowly named CLI source edit that exposes the Cargo package version, plus the manifest/lock edits, as exceptions to the mechanical move; retain the Report's changed-file inventory. No broader CLI redesign is needed.
