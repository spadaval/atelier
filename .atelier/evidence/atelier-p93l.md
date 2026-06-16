---
created_at: "2026-06-11T03:52:16.655425603+00:00"
id: "atelier-p93l"
evidence_type: "test"
captured_at: "2026-06-11T03:52:16.655312282+00:00"
agent_identity: "codex"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "mission"
    id: "atelier-8bky"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "JSON removal targeted validation: cargo fmt -- --check, cargo test --test cli_integration -- --nocapture, git diff --check, atelier export --check, atelier lint, and atelier doctor passed. cargo nextest run had one unrelated/reproducible smoke concurrency failure: smoke::adversarial::test_concurrent_creates_5 reports Projection index is stale after concurrent creates."
updated_at: "2026-06-11T03:52:16.655425603+00:00"
---

JSON removal targeted validation: cargo fmt -- --check, cargo test --test cli_integration -- --nocapture, git diff --check, atelier export --check, atelier lint, and atelier doctor passed. cargo nextest run had one unrelated/reproducible smoke concurrency failure: smoke::adversarial::test_concurrent_creates_5 reports Projection index is stale after concurrent creates.
