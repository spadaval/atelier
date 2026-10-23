---
created_at: "2026-10-23T15:47:42.382342880+00:00"
id: "atelier-53cl"
evidence_type: "test"
commit: "e8805f7963b7c597f8009ce572eeaa1f5537b9ae"
result: "pass"
captured_at: "2026-10-23T15:47:39.815537320+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(land_with_an_id_points_to_the_owners_branch) | test(unowned_branch_lands_unless_it_changes_records)'"
exit_status: "0"
agent_identity: "claude-implementer-0uaj"
target:
  kind: "issue"
  id: "atelier-ns04"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ns04"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "land <id> off the owner's branch says git switch <branch> && atelier land; missions and unstarted items get their own fix; unknown IDs are named"
updated_at: "2026-10-23T15:47:42.382769317+00:00"
---

## Summary

land <id> off the owner's branch says git switch <branch> && atelier land; missions and unstarted items get their own fix; unknown IDs are named

## Command

```console
cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(land_with_an_id_points_to_the_owners_branch) | test(unowned_branch_lands_unless_it_changes_records)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 645
SHA-256: 08a8626955f284c424d24966f19a226ed4cf0fd775e5791cb4c8076a0627f7d7
Test summary: Summary [   0.459s] 2 tests run: 2 passed, 714 skipped
