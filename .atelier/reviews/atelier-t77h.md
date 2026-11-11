---
schema: atelier.review
schema_version: 2
id: atelier-t77h
title: finding on atelier-u1ko @d41d3bcd
target: atelier-u1ko
identity: GPT-6.1 Sol independent planning reviewer
commit: d41d3bcd1be9bacd36781ba370621136144f6a60
created_at: 2026-11-11T04:51:31.633163064Z
event:
  event: finding
  recommendation: fix_now
---

high: .atelier/issues/atelier-u1ko.md:72 Leaving the real checkout on its feature branch preserves its old records and old harness settings while replacing the host-wide binary and installing new Git hooks there. The rehearsal's For the cutover section identifies astromech as such a checkout. Advancing its base ref alone does not put config.toml, converted records, or reviewed AGENTS.md into its working tree: Repo::open reads the checkout's config first (next/crates/atelier-cli/src/repo.rs:36-40), check validates that checkout and reports a missing/invalid new config, and pre-push likewise reads its config (check.rs:220-224), even for a feature push. Thus the real checkout cannot satisfy G's final check, and its next session and pushes break after quiet ends. Settle a safe checkout/worktree policy before installation, including idle linked worktrees, that preserves feature work but leaves every checkout agents resume on converted records, and record/test its rollback. Exercise the known feature-branch case in the full run rather than cloning every sandbox on its base; correct Transition accordingly.
