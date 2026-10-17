---
created_at: "2026-10-17T01:44:17.972310338+00:00"
id: "atelier-2mky"
issue_type: "mission"
labels:
- "cli-output"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-0cal"
    type: "advances"
  - kind: "issue"
    id: "atelier-0xcr"
    type: "advances"
  - kind: "issue"
    id: "atelier-1oez"
    type: "advances"
  - kind: "issue"
    id: "atelier-4t5l"
    type: "advances"
  - kind: "issue"
    id: "atelier-6esg"
    type: "advances"
  - kind: "issue"
    id: "atelier-fb1i"
    type: "advances"
  - kind: "issue"
    id: "atelier-guew"
    type: "advances"
  - kind: "issue"
    id: "atelier-h0nt"
    type: "advances"
  - kind: "issue"
    id: "atelier-kset"
    type: "advances"
  - kind: "issue"
    id: "atelier-sfd9"
    type: "advances"
  - kind: "issue"
    id: "atelier-w4j2"
    type: "advances"
  - kind: "issue"
    id: "atelier-woul"
    type: "advances"
  - kind: "issue"
    id: "atelier-x3ru"
    type: "advances"
  - kind: "issue"
    id: "atelier-z6vj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "draft"
title: "Make bundle and check validate the same way and name what they found"
updated_at: "2026-10-17T01:44:17.972310338+00:00"
---

## Outcome

`bundle preview` refuses exactly what `bundle apply` refuses. It draws the graph it would create as a tree, with type, status and field-level changes, and its errors name the entry at fault. `atelier check` confirms what it checked and prints each finding once, against the record that owns it. It reports graph errors even when a record fails to parse, and gives a repair route for links to records that never existed.

## Success criteria

- S1: `bundle preview` runs the same staged-graph validation as `apply` (atelier-guew). Shown by a test per `apply` refusal.
- S2: The preview renders as a tree with new and updated marks and field-level changes, and fixture tests compare it against real output (atelier-h0nt, atelier-woul, atelier-1oez). Shown by fixture tests.
- S3: Bundle errors name entries by `client_ref` and drop the garbage tail. A bundle whose evidence commit is missing gets a warning. `apply` points at the new graph and the commit step. `bundle --help` carries the bundle contract (atelier-x3ru, atelier-z6vj, atelier-6esg, atelier-0cal, atelier-0xcr). Shown by CLI tests.
- S4: `check` confirms what it checked and prints one line per finding against the owning record. It shows graph errors despite a parse failure, does not call other record kinds corrupt, and gives a repair route for links to records that never existed (atelier-4t5l, atelier-kset, atelier-fb1i, atelier-w4j2, atelier-sfd9). Shown by CLI tests.

## Background

Split out of atelier-o3lm on 2026-10-16 at the human's direction. Its items come from that mission's six-surface CLI audit; see atelier-o3lm's Background for the method and the transcript scripts. The output contract (atelier-o3lm S1, atelier-i2qq) and ADR 0022 (atelier-bw0b) govern how output looks. atelier-o3lm's Notes leave the human an open question: a Markdown input format for bundles. atelier-rz2l (record import) may answer part of it.
