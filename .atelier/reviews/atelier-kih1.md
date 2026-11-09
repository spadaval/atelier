---
schema: atelier.review
schema_version: 3
id: atelier-kih1
title: approval on branch t3/drop-macos-ci @00833324
branch: t3/drop-macos-ci
identity: sol
commit: 008333241d4fd2c24ba1c4fa32c623c39b94c0fe
created_at: 2026-11-09T21:15:52.214195490Z
event:
  event: approval
---

Verdict: merge. Complexity low: one CI runner change and matching documentation. Value medium: removes the macOS job from the current merge wait. Risk low: the workflow change is visible and reversible; untested macOS behavior is the human-approved support policy. Correctness: pass. Fitness: pass. No findings or open questions. Independently reviewed the full branch diff. CI retains the Linux build, fmt, atelier check and both nextest suites, with the existing job-selection conditions. Searched workflows, scripts, crates, docs and the skill: no dependency on the old test (ubuntu-latest) or test (macos-latest) names or matrix.os remains. Read ci_passed.rs and forge.rs: the gate judges the trusted workflow conclusion rather than job names. release.yml uses its own dist matrix; dist-workspace.toml retains both macOS archives. Current product, architecture, skill, README and AGENTS guidance does not promise macOS CI or full macOS support. The release strategy's dated best-effort exception accurately describes the retained archives; older macOS CI requirements belong to closed missions. Changed prose uses plain sentences and no em dashes. Fresh proof passed: git diff --check HEAD^ HEAD -- . ':!.atelier'; cargo fmt -- --check; cargo metadata --locked --format-version 1 > /dev/null; scripts/ci-jobs.test.sh (18 cases); scripts/ci-jobs.sh 1858e8a6 00833324 (rust=true, web=true). Residual limits: no hosted CI or release build run in this read-only review, no macOS execution, and the claimed five-minute CI duration was not measured.
