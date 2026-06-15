---
created_at: "2026-06-15T18:57:07.354922029+00:00"
id: "atelier-1bq8"
evidence_type: "validation"
captured_at: "2026-06-15T18:57:07.124166609+00:00"
command: "bash -lc 'set -e; matches=$(rg \"atelier::|crate::db|crate::projection_index|crate::record_store|crate::activity|removed_command_guidance\" crates fuzz -n || true); printf \"%s\\n\" \"$matches\"; test \"$matches\" = \"crates/atelier-cli/src/main.rs:2:use atelier::{commands, telemetry};\"; test ! -d src; ! rg \"was removed\" crates/atelier-cli/src docs/product/cli-surface.md -n'"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-vu2b"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 69
    summary: "crates/atelier-cli/src/main.rs:2:use atelier::{commands, telemetry};\n"
    truncated: false
  stderr:
    bytes: 0
    summary: ""
    truncated: false
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
status: "pass"
title: "Only atelier:: code import is the CLI binary importing its own library crate; no root package source tree exists"
updated_at: "2026-06-15T18:57:10.772835052+00:00"
---

Only atelier:: code import is the CLI binary importing its own library crate; no root package source tree exists

Command: bash -lc 'set -e; matches=$(rg "atelier::|crate::db|crate::projection_index|crate::record_store|crate::activity|removed_command_guidance" crates fuzz -n || true); printf "%s\n" "$matches"; test "$matches" = "crates/atelier-cli/src/main.rs:2:use atelier::{commands, telemetry};"; test ! -d src; ! rg "was removed" crates/atelier-cli/src docs/product/cli-surface.md -n'
Exit status: 0

Stdout summary:
crates/atelier-cli/src/main.rs:2:use atelier::{commands, telemetry};

Stderr summary:
(none)

