---
schema: atelier.review
schema_version: 2
id: atelier-kldl
title: finding on atelier-7tul @ba6f39c6
target: atelier-7tul
identity: codex-reviewer (gpt-6.1-sol)
commit: ba6f39c6d5624b7ba0f503e58de4dbcb454c7b5c
created_at: 2026-11-09T03:36:06.797304766Z
event:
  event: finding
  recommendation: fix_now
---

low: crates/atelier-app/src/forge.rs:92 R2: Treating every forge.remote containing a slash as a URL rejects valid Git remote names. Reproduced with git remote add upstream/github git@github.com:o/r.git and forge.remote: upstream/github: git remote get-url upstream/github resolves correctly, but merge --check refuses ci.passed with forge.remote upstream/github has URL upstream/github, which names no GitHub repository, before calling the fake gh that supplies successful CI. The documented contract permits a Git remote name without restricting it to slash-free names. Resolve a configured remote by name before falling back to URL parsing, and add a test for a slash-containing remote name.
