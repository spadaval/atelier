---
created_at: "2026-06-15T05:11:19.634599426+00:00"
id: "atelier-qsbe"
issue_type: "epic"
labels:
- "architecture"
- "contract"
- "rewrite"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-0fhv"
  - kind: "issue"
    id: "atelier-14z2"
  - kind: "issue"
    id: "atelier-2q5s"
  - kind: "issue"
    id: "atelier-3kap"
  - kind: "issue"
    id: "atelier-4ren"
  - kind: "issue"
    id: "atelier-4wor"
  - kind: "issue"
    id: "atelier-5dgb"
  - kind: "issue"
    id: "atelier-7vfj"
  - kind: "issue"
    id: "atelier-8wvr"
  - kind: "issue"
    id: "atelier-fjmw"
  - kind: "issue"
    id: "atelier-kjj1"
  - kind: "issue"
    id: "atelier-nbni"
  - kind: "issue"
    id: "atelier-nyn0"
  - kind: "issue"
    id: "atelier-rjua"
  - kind: "issue"
    id: "atelier-rxgn"
  - kind: "issue"
    id: "atelier-sclf"
  - kind: "issue"
    id: "atelier-uz8g"
  - kind: "issue"
    id: "atelier-v64l"
  - kind: "issue"
    id: "atelier-vv2i"
  - kind: "issue"
    id: "atelier-wng0"
  - kind: "issue"
    id: "atelier-wz3t"
  - kind: "issue"
    id: "atelier-xmvz"
  - kind: "issue"
    id: "atelier-y3ur"
  - kind: "issue"
    id: "atelier-ycmp"
  - kind: "issue"
    id: "atelier-yo9i"
  - kind: "issue"
    id: "atelier-zwna"
  children:
  - kind: "issue"
    id: "atelier-0rdo"
  - kind: "issue"
    id: "atelier-4ra1"
  - kind: "issue"
    id: "atelier-f74g"
  - kind: "issue"
    id: "atelier-qsib"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Architecture contracts and ADRs for crate rewrite"
updated_at: "2026-06-15T15:16:25.104361056+00:00"
---

## Description

Define the architecture contract that every implementation epic in the crate rewrite depends on. This epic owns the durable decision artifacts for workspace layering, crate dependency direction, SQLite projection/runtime ownership, and the active-work source-of-truth model.

## Outcome

- Architecture docs define the target Cargo workspace crates, crate responsibilities, and allowed temporary adapter policy.
- ADR 0009 records the virtual workspace root and CLI-owned executable decision.
- `docs/architecture/source-layout.md` and `CONTEXT.md` describe the workspace split as target state, not a transitional scaffold.
- ADR 0004 is amended or superseded so current work is status-derived rather than owned by runtime active-work or claim state.
- Implementation epics remain blocked until the architecture contract is explicit enough for agents to execute without private chat context.

## Evidence

- Documentation diff shows the target crate map, dependency rules, virtual root decision, and CLI-owned binary ownership.
- ADR diff records ADR 0009 and the active-work source-of-truth update.
- Tracker graph shows implementation epics blocked by this contract epic.
- `atelier lint atelier-qsbe` and `atelier export --check` pass.
