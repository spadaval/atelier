---
schema: atelier.review
schema_version: 2
id: atelier-d65y
title: approval on atelier-l5ai @36bbd029
target: atelier-l5ai
identity: sol-l5ai-r1
commit: 36bbd0293e6b47538a817b7d03d898e600f2cfc8
created_at: 2026-11-11T05:38:49.242110913Z
event:
  event: approval
---

Verdict: merge. The six shared views let the Desk read committed records and reuse the behavior of show without copying the CLI. Sizes: complexity medium (a second repository read mode and two data views across app, records and CLI), value high (all six views are required for the Desk), risk low (read APIs and moved code with unchanged CLI regression tests). Contract: pass. Inspected the four branch-owned commits in 1592c02a..36bbd029 excluding master, all six M1/M2/M3/M4/M6/M7 implementations, CLI call sites, new tests, Report and desk.md. Config is read at HEAD; locate/everything/current/missions/links and the shared views use commits in committed mode, including suppression of the done-item checkout settlement overlay. Located selection and read_from share one decision. Document listing excludes mission.md and reviews and traverses nested regular files. plan_changed_at compares commits and shares the existing mission membership filter with plan_changed. Annotation and approved_at each have one crate owner. No existing test was edited; the records test change only appends a test. Fitness: pass. The constructor flag, view types, ReadFrom selection, recursive document walker and shared plan filter each trace to a Done when requirement. No new crate or dependency, and no unrelated moving part found. Findings: none; no FIX NOW. Fresh proof in next/: cargo build -j 4 passed; TMPDIR=$HOME/.cache/atelier-desk/tmp cargo nextest run -j 4 passed all 253 tests, 0 skipped, with full output read; cargo fmt -- --check and cargo metadata --locked --format-version 1 passed. Root git diff --check HEAD^ HEAD -- . excluding .atelier passed; the implementation commit also passes diff --check. Open questions: none. Residual risk: committed mode is a documented read convention on Repo, not a type-level ban on store/planning/act methods; Desk callers must use the documented commit-backed methods. Detached HEAD intentionally supplies its own committed config. Tests not run: historical root workspace and cross-repository smoke, since this review targets next/ and installs no binary. Code stayed read-only; this approval is the only record created.
