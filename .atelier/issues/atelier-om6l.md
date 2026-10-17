---
created_at: "2026-10-17T01:10:57.988066400+00:00"
id: "atelier-om6l"
issue_type: "feature"
labels:
- "cli-output"
- "work-views"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-4pzj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Show what holds a mission up in a Waiting on section of work mission"
updated_at: "2026-10-17T01:10:57.988066400+00:00"
---

## Goal

`work mission` prints blocker IDs on rows but never says what they are, so "what is holding this mission up" takes one `issue show` per ID. On master, `work mission 1w8c` repeats `blockers: atelier-bw0b` on three rows without its title, status (ready, high, in progress on another branch) or that it advances no mission. In the fixture, 7 blocked or backlog items in atelier-j76w wait on only 5 items at the far end of their chains: spvd (decision, needs a human, holds 4), 97yk (needs a human, holds 1), ubti (ready, low, no mission, end of a 4-deep chain, and cut from `work ready` by its 20-row limit), 9vdd (ready, its mission superseded), s47h (active in this mission). The dashboard's only Ready row (1l9e) unblocks nothing. `issue show atelier-5hp9` already computes `transitive blocker path atelier-5hp9 -> atelier-fhep -> atelier-il31 -> atelier-ubti` and `Next: atelier issue show atelier-ubti` (workflow.rs transition readiness), so the facts exist.

## Done when

- `work mission <id>` opens with a section listing each open item at the end of a blocker chain (no open blockers of its own) that holds mission work, with its type, state, priority, title, mission (or "no mission", or "mission X superseded"), a ⚠ when it needs a human, and what it holds. Chains deeper than 1 print as a path. For the fixture:
  ```
  Waiting on (5 hold 7 items)
    ⚠ atelier-spvd  decision · needs a human  medium  Pick widget wire format
        holds atelier-qhnh (+3 children), atelier-ly7a
    ⚠ atelier-97yk  needs a human  critical  Approve widget vendor account — holds atelier-1jl2
    atelier-ubti  ready · no mission  low  Audit legacy widget names
        holds atelier-il31 → atelier-fhep → atelier-5hp9 → atelier-g0q4
    atelier-9vdd  ready · mission atelier-ba3h superseded  Define gadget schema — holds atelier-1gc5
    atelier-s47h  active  Tokenise widget input — holds atelier-9t9q
  ```
- Each blocker appears once, however many items it holds. Done or superseded blockers drop out and what they held moves to Ready.
- Rendering tests cover a cross-mission blocker, an unassigned blocker, a needs-human decision, a blocker inherited from a parent, a 4-deep chain, and a done or superseded blocker.

## Scope

Read side and rendering in `crates/atelier-cli/src/commands/work.rs` (ScopedWork::panels, around lines 497-551) using `atelier_app::objective_graph::dependency_closure`. No classification change.

## Context

Extends atelier-fdxj (backlog: ranking ready work by unblock count), which this does not replace. Astromech transcripts show `atelier work mission astromech-7q0z | head -60` followed by `issue show` on blockers (2 sessions). Fixture: scratch clone with mission atelier-j76w; parent atelier-qhnh (children s47h active, 9t9q blocked_by s47h, 1gc5 blocked_by 9vdd); qhnh blocked_by decision atelier-spvd (needs a human); chain 5hp9 <- fhep <- il31 <- ubti (ubti ready, low, no mission); 9vdd advances superseded mission atelier-ba3h.

## Unknowns

The human asked whether a dedicated "what do I do to get this mission moving" view would be better; the auditor recommends making it this section of `work mission` (the view people already open, so no extra command, with per-chain detail under `--blocked`) rather than a new command or a per-item tree that repeats shared blockers, but the human decides.
