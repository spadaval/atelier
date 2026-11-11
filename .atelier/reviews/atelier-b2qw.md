---
schema: atelier.review
schema_version: 2
id: atelier-b2qw
title: finding on atelier-u1ko @d41d3bcd
target: atelier-u1ko
identity: GPT-6 Astra independent planning reviewer
commit: d41d3bcd1be9bacd36781ba370621136144f6a60
created_at: 2026-11-11T04:50:09.005118830Z
event:
  event: finding
  recommendation: fix_now
---

high: .atelier/issues/atelier-u1ko.md:72 Step 5 moves only the real clone's base ref while expressly preserving its checked-out feature branch, then installs the new shared Git hooks there. The rehearsal Report establishes that astromech is on such a branch. Its checkout still lacks .atelier/config.toml and holds old harness settings; next/crates/atelier-cli/src/repo.rs:36 opens config from the checkout, not the updated base. status/start therefore fail after the cutover, and the new pre-push hook cannot read its config. A successful run on fresh base clones misses this. Settle how existing checkouts and worktrees return to usable converted records before resuming agents, preserving feature work, and rehearse a real-clone surrogate left on a feature branch plus its shared hooks. Update Transition and G's end checks accordingly.
