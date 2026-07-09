---
created_at: "2026-07-06T17:20:25.687157663+00:00"
id: "atelier-c0mp"
issue_type: "mission"
labels:
- "cli"
- "human-output"
- "inventory"
- "mission-dashboard"
fields:
  workflow_branch:
    branch_base: master
    integration_target: master
    merge_strategy: squash
    owner_issue_id: atelier-c0mp
    owner_kind: mission
    review_target: master
    work_branch: mission/atelier-c0mp
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-4fip"
    type: "advances"
  - kind: "issue"
    id: "atelier-g5fl"
    type: "advances"
  - kind: "issue"
    id: "atelier-nzu9"
    type: "advances"
  - kind: "issue"
    id: "atelier-vgqe"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Mission: Separate issue inventory from the mission overview"
updated_at: "2026-07-06T17:44:36.666721847+00:00"
---

## Description

Restore a clear read-side boundary between generic issue inventory and mission coordination. `atelier issue list` becomes a simple flat record list. `atelier work missions` becomes the formatted high-level overview for mission scope and linked epics. Constraints: preserve canonical mission `advances` and epic-child semantics; keep child tasks collapsed in the cross-mission default; use shared presentation infrastructure rather than command-local styling; do not introduce a new dashboard noun, TUI, compatibility alias, or machine-readable command-result contract. Risks to resolve in the public contract are deterministic ordering and limits, direct mission-linked work, unassigned work, and work reachable from more than one mission.

## Outcome

- `atelier issue list` renders a flat, deterministic inventory with one row per matching issue and simple metadata filters; it does not infer hierarchy, mission rollups, blocker-aware work selection, or operational queue grouping.
- `atelier work missions` renders a formatted Mission Overview that hides done missions by default, shows epics linked to each visible mission through `advances`, represents collapsed child work with truthful progress and state summaries, and provides explicit drill-down commands.
- Done missions remain inspectable through an explicit option, direct mission-linked work and unassigned nonterminal work are accounted for without pretending they are epic children, and no issue silently disappears from the overview's stated scope.
- Mission overview formatting uses the shared human-output components and semantic style roles: interactive terminals receive consistent color, while `NO_COLOR` and noninteractive output retain the same status, priority, blocker, progress, and omission meaning without ANSI escapes.
- Help, product contracts, role guidance, and regression coverage agree on the inventory-versus-dashboard boundary, and independent validation checks the delivered commands rather than accepting prior closed tracker claims.

## Evidence

- Attached child evidence must include focused command transcripts for the flat `atelier issue list` inventory and formatted `atelier work missions` overview, including deterministic ordering, omission accounting, and `NO_COLOR` output.
- Independent validation issue `atelier-g5fl` must exercise the delivered commands in a temporary repository and attach its validation evidence before this mission requests publication.
- `cargo nextest run`, `cargo fmt -- --check`, `git diff --check`, and `atelier check atelier-c0mp` must pass on the integrated mission branch.
- The mission review artifact must account for the public contract, both epic diffs, their independent review evidence, and any residual behavior or evidence gaps.
- Evidence record `atelier-1q1r`: PASS — post-master-integration regression at exact candidate `a9763d40` preserved the prior `atelier-g5fl` classification: four mission-critical temporary-repository/help scenarios passed; the full suite passed 704 tests with none skipped; inventory/overview help, durable docs, manager `work missions` plus `man work-model` guidance, formatting, diff, mission lint, and repository lint all passed.
- Evidence record `atelier-4k8q`: superseded tooling-input failure — an over-specific unverified full-hash assertion stopped the first capture before any validation scenario ran. The corrected short-hash assertion and every requested check passed in `atelier-1q1r`; this failed capture is not a product or publication-candidate failure.
