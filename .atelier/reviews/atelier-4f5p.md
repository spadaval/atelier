---
schema: atelier.review
schema_version: 2
id: atelier-4f5p
title: finding on atelier-fcb7 @4b32d02b
target: atelier-fcb7
identity: gpt-6-astra-amend3-r3
commit: 4b32d02b42395edcd048eeb0c3d3374882295a56
created_at: 2026-11-10T04:58:28.394758443Z
event:
  event: finding
  recommendation: fix_now
---

medium docs/strategies/redesign-architecture.md:93-103 and atelier-mal9:38: atelier-fbin is only partly resolved. Registry packages are now allowed, but the three required checks still permit old Rust source through Cargo target paths or Rust module path attributes. For example, next/crates/atelier-core/Cargo.toml can set [lib].path to ../../../crates/atelier-core/src/lib.rs: its manifest remains under next/, with no symlink, include macro or build-script read. A #[path] module can likewise reach old source. Thus all three stated checks can pass while the new workspace builds on the old one, contrary to the amendment. Extend the required script to check resolved source paths of local Cargo targets (including custom-build targets) and Rust module path attributes, and require failing fixtures for these routes. Keep registry packages, internal next/crates/ paths, copied web sources and provenance text allowed. This is a planning-contract gap; no build or test was run. No other must-fix found in the full amendment, and its paths are confined to the permitted strategy, issue and review files.
