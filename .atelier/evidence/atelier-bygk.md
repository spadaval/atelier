---
created_at: "2026-06-15T18:56:46.776703569+00:00"
id: "atelier-bygk"
evidence_type: "validation"
captured_at: "2026-06-15T18:56:46.567509418+00:00"
command: "bash -lc 'set -e; ! rg \"atelier::|crate::db|crate::projection_index|crate::record_store|crate::activity|removed_command_guidance\" crates fuzz -n; ! rg \"was removed\" crates/atelier-cli/src docs/product/cli-surface.md -n; set +e; target/debug/atelier finish >/tmp/vu2b-finish2.out 2>/tmp/vu2b-finish2.err; status=$?; set -e; test $status -ne 0; rg \"unrecognized subcommand .finish.\" /tmp/vu2b-finish2.err; ! rg \"was removed|atelier issue close\" /tmp/vu2b-finish2.err'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-vu2b"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-vu2b"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Scoped code search confirms no root compatibility module paths or removed-command guidance implementation remains"
updated_at: "2026-06-15T18:56:50.273904829+00:00"
---

## Summary

Scoped code search confirms no root compatibility module paths or removed-command guidance implementation remains

## Command

```console
bash -lc 'set -e; ! rg "atelier::|crate::db|crate::projection_index|crate::record_store|crate::activity|removed_command_guidance" crates fuzz -n; ! rg "was removed" crates/atelier-cli/src docs/product/cli-surface.md -n; set +e; target/debug/atelier finish >/tmp/vu2b-finish2.out 2>/tmp/vu2b-finish2.err; status=$?; set -e; test $status -ne 0; rg "unrecognized subcommand .finish." /tmp/vu2b-finish2.err; ! rg "was removed|atelier issue close" /tmp/vu2b-finish2.err'
```

Exit status: 0

## Stdout

Bytes: 109
Truncated: no

```text
crates/atelier-cli/src/main.rs:2:use atelier::{commands, telemetry};
error: unrecognized subcommand 'finish'
```

## Stderr

Bytes: 0
Truncated: no

```text
```
