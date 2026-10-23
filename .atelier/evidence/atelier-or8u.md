---
created_at: "2026-10-23T15:53:51.777696649+00:00"
id: "atelier-or8u"
evidence_type: "test"
commit: "fd8ad78e1bfc812396803e7a1f2e6a40784cc95a"
result: "pass"
captured_at: "2026-10-23T15:53:49.127547762+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(land_with_an_id_points_to_the_owners_branch) | test(unowned_branch_lands_unless_it_changes_records)'"
exit_status: "0"
agent_identity: "atelier merge --sync"
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
title: "land <id> off the owner's branch says git switch <branch> && atelier land; missions and unstarted items get their own fix; unknown IDs are named (rerun after merging master)"
updated_at: "2026-10-23T15:53:51.778350720+00:00"
---

## Summary

land <id> off the owner's branch says git switch <branch> && atelier land; missions and unstarted items get their own fix; unknown IDs are named (rerun after merging master)

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
SHA-256: 3ee311cdb18f18523b22625312d84da10eca15e187604119e974ca8cdd613884
Test summary: Summary [   0.484s] 2 tests run: 2 passed, 720 skipped
