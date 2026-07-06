---
created_at: "2026-06-25T00:56:09.621977563+00:00"
id: "atelier-272k"
evidence_type: "validation"
captured_at: "2026-06-25T00:56:09.443505181+00:00"
command: "bash -lc 'if rg \"StylePolicy::plain\\\\(\" crates/atelier-cli/src/commands; then exit 1; else echo \"no StylePolicy::plain() in crates/atelier-cli/src/commands\"; fi'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-3llx"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-3llx"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc 'if rg \"StylePolicy::plain\\\\(\" crates/atelier-cli/src/commands; then exit 1; else echo \"no StylePolicy::plain() in crates/atelier-cli/src/commands\"; fi'"
updated_at: "2026-06-25T00:56:13.061184840+00:00"
---

## Summary

bash -lc 'if rg "StylePolicy::plain\\(" crates/atelier-cli/src/commands; then exit 1; else echo "no StylePolicy::plain() in crates/atelier-cli/src/commands"; fi'

## Command

```console
bash -lc 'if rg "StylePolicy::plain\\(" crates/atelier-cli/src/commands; then exit 1; else echo "no StylePolicy::plain() in crates/atelier-cli/src/commands"; fi'
```

Exit status: 0

## Stdout

Bytes: 59
Truncated: no

```text
no StylePolicy::plain() in crates/atelier-cli/src/commands
```

## Stderr

Bytes: 0
Truncated: no

```text
```

