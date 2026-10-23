---
created_at: "2026-10-23T17:36:20.729005014+00:00"
id: "atelier-yulf"
evidence_type: "test"
commit: "2957f128bf09b4ff608a939d3a8d3fe80f407ced"
result: "pass"
captured_at: "2026-10-23T17:36:18.304409907+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(a_pending_review_on_the_owners_branch_blocks_no_turn) | test(a_resolved_fix_now_awaiting_approval_blocks_no_turn)'"
exit_status: "0"
agent_identity: "claude-implementer-buqu"
target:
  kind: "issue"
  id: "atelier-i80r"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-i80r"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Pending review (review.current alone, or with findings.disposed only awaiting fresh approval of resolved fix_now) reports without blocking; with another failure, an open finding or an undisposed finding it blocks"
updated_at: "2026-10-23T17:36:20.729486353+00:00"
---

## Summary

Pending review (review.current alone, or with findings.disposed only awaiting fresh approval of resolved fix_now) reports without blocking; with another failure, an open finding or an undisposed finding it blocks

## Command

```console
cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(a_pending_review_on_the_owners_branch_blocks_no_turn) | test(a_resolved_fix_now_awaiting_approval_blocks_no_turn)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 659
SHA-256: 03958e677747acdc47ea2a38a4da889ca399787ce74c272925881db387358be3
Test summary: Summary [   0.447s] 2 tests run: 2 passed, 743 skipped
