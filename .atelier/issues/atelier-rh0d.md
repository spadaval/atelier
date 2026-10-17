---
created_at: "2026-10-17T01:10:57.963639867+00:00"
id: "atelier-rh0d"
issue_type: "bug"
labels:
- "cli-output"
- "issue-commands"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-o3lm"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Fix link role defaults and role errors in issue link and unlink"
updated_at: "2026-10-17T01:10:57.963639867+00:00"
---

## Goal

`--role` defaults to `advances` for both `link` and `unlink`:
- `atelier issue link xmtr f0rr` between two tasks fails with `atelier-xmtr (task) cannot advance a task`, although the user never wrote "advance".
- `atelier issue unlink xmtr f0rr` with no role, when a `blocked_by` link exists, prints `No link atelier-xmtr -> atelier-f0rr (advances) exists` and exits 0.

The invalid-role error (`validate_issue_link_role`, `crates/atelier-cli/src/commands/relate.rs:67-96`) prints `Built-in workflow roles: advances, assumption, blocked_by, …, evidenced_by, …, validates`. That list includes `evidenced_by` and `validates`, which link refuses as proof roles, and omits `blocks`, which it accepts.

## Done when

- `link` without `--role` either requires it or, when the default fails, says `no --role given; the default is advances (work → mission). For sequencing use --role blocked_by; for context --role related.`
- `unlink` without `--role` removes the only link between the two records, or lists the roles that exist and exits non-zero. Unlinking a link that doesn't exist exits non-zero.
- The role error groups roles: `readiness: blocked_by, blocks · mission: advances · context: related, … · custom: …`, with no proof roles.
- CLI tests cover each case.
