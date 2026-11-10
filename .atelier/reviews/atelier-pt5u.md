---
schema: atelier.review
schema_version: 2
id: atelier-pt5u
title: finding on atelier-fcb7 @9394bb27
target: atelier-fcb7
identity: grok-4.6-amend3-r1
commit: 9394bb27f77fc5da005bab5e28496dbf8e6a0a27
created_at: 2026-11-10T04:53:28.201666739Z
event:
  event: finding
  recommendation: fix_now
---

high redesign-architecture.md:77-92 and atelier-mal9: next/ is specified as a second Cargo workspace that CI builds beside the old one, with isolation "no file under next/ names a path under crates/", while the switch claims cargo install --path crates/atelier-cli after moving next/ to the root. Those cannot all hold: members at next/crates/<name>/ make next/Cargo.toml name crates/, so a grep check fails; a flat next/<name>/ layout makes the switch install path miss; and scripts/ci-jobs.sh only prints rust=/web= while .github/workflows/ci.yml always cargo builds the root workspace (Cargo.toml members are crates/atelier-* only, dist-workspace.toml is cargo:., the web job is crates/atelier-site/web). Consequence: mal9 either bans the layout the switch needs or ships one that breaks the install path, and CI never compiles or tests next/. Smallest fix: pin members at next/crates/<name>/; isolation is cargo metadata --manifest-path next/Cargo.toml with every package manifest under next/; name ci.yml in mal9 so a selected next/ change runs fmt, build and nextest in next/.
