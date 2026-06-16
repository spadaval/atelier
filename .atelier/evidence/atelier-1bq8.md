---
created_at: "2026-06-15T18:57:07.354922029+00:00"
id: "atelier-1bq8"
evidence_type: "validation"
captured_at: "2026-06-15T18:57:07.124166609+00:00"
command: "bash -lc 'set -e; matches=$(rg \"atelier::|crate::db|crate::projection_index|crate::record_store|crate::activity|removed_command_guidance\" crates fuzz -n || true); printf \"%s\\n\" \"$matches\"; test \"$matches\" = \"crates/atelier-cli/src/main.rs:2:use atelier::{commands, telemetry};\"; test ! -d src; ! rg \"was removed\" crates/atelier-cli/src docs/product/cli-surface.md -n'"
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
title: "Only atelier:: code import is the CLI binary importing its own library crate; no root package source tree exists"
updated_at: "2026-06-15T18:57:10.772835052+00:00"
---

## Summary

Only atelier:: code import is the CLI binary importing its own library crate; no root package source tree exists

## Command

```console
bash -lc 'set -e; matches=$(rg "atelier::|crate::db|crate::projection_index|crate::record_store|crate::activity|removed_command_guidance" crates fuzz -n || true); printf "%s\n" "$matches"; test "$matches" = "crates/atelier-cli/src/main.rs:2:use atelier::{commands, telemetry};"; test ! -d src; ! rg "was removed" crates/atelier-cli/src docs/product/cli-surface.md -n'
```

Exit status: 0

## Stdout

Bytes: 69
Truncated: no

```text
crates/atelier-cli/src/main.rs:2:use atelier::{commands, telemetry};
```

## Stderr

Bytes: 0
Truncated: no

```text
```
