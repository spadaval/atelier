---
created_at: "2026-07-06T20:37:08.402928123+00:00"
id: "atelier-hfjg"
evidence_type: "test"
captured_at: "2026-07-06T20:37:08.372393843+00:00"
command: "sh -c 'set -eu; if rg -n \"\\b(projection_index|refresh_projection|RebuildProjection|ProjectionLoader|ProjectionRebuildLock|IssueRelationshipProjection|mission_projection_worktree|remove_projection_state|rebuilt_projection|projection_issue)\\b\" crates; then exit 1; fi; if rg --files crates | rg -i \"projection\"; then exit 1; fi; remaining=$(rg -n -i \"projection\" crates); printf \"%s\\n\" \"$remaining\"; test \"$(printf \"%s\\n\" \"$remaining\" | wc -l)\" -eq 5'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-xa9s"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-xa9s"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "sh -c 'set -eu; if rg -n \"\\b(projection_index|refresh_projection|RebuildProjection|ProjectionLoader|ProjectionRebuildLock|IssueRelationshipProjection|mission_projection_worktree|remove_projection_state|rebuilt_projection|projection_issue)\\b\" crates; then exit 1; fi; if rg --files crates | rg -i \"projection\"; then exit 1; fi; remaining=$(rg -n -i \"projection\" crates); printf \"%s\\n\" \"$remaining\"; test \"$(printf \"%s\\n\" \"$remaining\" | wc -l)\" -eq 5'"
updated_at: "2026-07-06T20:37:08.405098837+00:00"
---

## Summary

sh -c 'set -eu; if rg -n "\b(projection_index|refresh_projection|RebuildProjection|ProjectionLoader|ProjectionRebuildLock|IssueRelationshipProjection|mission_projection_worktree|remove_projection_state|rebuilt_projection|projection_issue)\b" crates; then exit 1; fi; if rg --files crates | rg -i "projection"; then exit 1; fi; remaining=$(rg -n -i "projection" crates); printf "%s\n" "$remaining"; test "$(printf "%s\n" "$remaining" | wc -l)" -eq 5'

## Command

```console
sh -c 'set -eu; if rg -n "\b(projection_index|refresh_projection|RebuildProjection|ProjectionLoader|ProjectionRebuildLock|IssueRelationshipProjection|mission_projection_worktree|remove_projection_state|rebuilt_projection|projection_issue)\b" crates; then exit 1; fi; if rg --files crates | rg -i "projection"; then exit 1; fi; remaining=$(rg -n -i "projection" crates); printf "%s\n" "$remaining"; test "$(printf "%s\n" "$remaining" | wc -l)" -eq 5'
```
Exit status: 0

## Stdout

Bytes: 533
Truncated: no

```text
crates/atelier-workflow/src/lib.rs:2733:    fn starter_policy_does_not_expose_projection_freshness_validator() {
crates/atelier-workflow/src/lib.rs:2873:        for field in ["workflow_projection_tables", "projection_tables"] {
crates/atelier-sqlite/src/cache.rs:1111:            "projection_sources",
crates/atelier-app/src/workflow_validation.rs:1007:    fn baseline_default_checks_do_not_report_projection_freshness() {
crates/atelier-app/src/workflow_validation.rs:1024:        assert!(!reason.contains("projection freshness"));
```

## Stderr

Bytes: 0
Truncated: no

```text
```
