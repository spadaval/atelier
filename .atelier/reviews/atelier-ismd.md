---
schema: atelier.review
schema_version: 3
id: atelier-ismd
title: approval on branch release-0.4.0-linux-only @d1d9e1a0
branch: release-0.4.0-linux-only
identity: gpt-6.1-sol
commit: d1d9e1a089e5585d46fb62ef91e0d1cb7200fbd1
created_at: 2026-11-09T21:45:29.096640936Z
event:
  event: approval
---

Verdict: merge. The change removes both Darwin release targets and accurately documents the human's decision to support Linux only.
Sizes: complexity low (configuration and prose only); value high (removes the unsupported macOS runner from the release path); risk medium (changes published platform availability, explicitly authorized and documented).
Contract: pass. Reviewed git diff HEAD^ HEAD at d1d9e1a089e5585d46fb62ef91e0d1cb7200fbd1, .github/workflows/release.yml, .github/build-setup.yml and dist-workspace.toml. The workflow consumes dist's computed build matrix and uploaded artifacts; neither setup nor publication requires Darwin. cargo-dist 0.33.1 dist generate --check and dist plan both passed. The JSON plan contains exactly four executable archives: aarch64 and x86_64 Linux, each gnu and musl. All four matrix entries use Ubuntu runners. The generated shell installer's target_triples contain Linux only. Source archive, installer and checksums remain platform-independent assets.
Fitness: pass. Ran the requested case-insensitive macOS git grep and examined each hit. README, development setup, CI comment and current release strategy Shape explicitly state macOS is unsupported. README's possible source build is not a support promise. The 0.3.0 changelog and original strategy criteria/environment/risks describe earlier scope; the release strategy's dated current Shape explicitly supersedes its original platform targets. Other hits are implementation branches/comments, test fixtures, or documentation of an existing cache path. No current installation documentation promises macOS binaries or best-effort macOS support. The 0.4.0 changelog accurately names Linux-only x86_64/aarch64 glibc/musl binaries and Linux CI.
Proof: git diff --check HEAD^ HEAD -- . ':!.atelier'; cargo fmt -- --check; cargo metadata --locked --format-version 1 > /dev/null; dist generate --check; dist plan; JSON plan inspection. All passed on the pinned code.
Findings: none. Open questions: none. Residual risk: this review validates the release plan and workflow, not a completed build or publication. The already-pushed old v0.4.0 tag does not acquire this change automatically; release/tag handling is outside this read-only review.
