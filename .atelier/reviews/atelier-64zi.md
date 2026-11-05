---
schema: atelier.review
schema_version: 1
id: atelier-64zi
title: approval on atelier-esu1 @285718ce
target: atelier-esu1
identity: claude-reviewer-esu1
commit: 285718cec3655473ca81c8c3452a05a79394a845
created_at: 2026-11-04T23:40:38.002187167Z
event:
  event: approval
---

Contract met: Authority in docs/strategies/release-distribution.md now says a human's request for a specific release authorizes pushing its tag once master CI passes on the release commit, and that any other release tag, the first release's version, platforms, channels and secrets still come back to the human. Read the whole strategy and AGENTS.md merge/push policy: the wording fits, is scoped to the requested release (no blanket tag authority), and ties the push to green master CI on that commit, which closes the release.yml gap (it publishes on any matching tag push). Checks: git diff --check, cargo fmt --check, cargo metadata --locked pass; PR #19 CI passed on 285718ce (Linux and macOS tests, dist builds). scripts/merge-ci-passed.sh runs only inside atelier merge (exit 2 here); note master gained code (crates/atelier-app/src/path_log.rs) since the fork, so that check will ask for atelier merge --sync, a push and a fresh CI pass before merging.
