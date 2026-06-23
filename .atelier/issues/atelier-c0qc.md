---
created_at: "2026-06-23T15:17:38.279396802+00:00"
id: "atelier-c0qc"
issue_type: "mission"
labels:
- "cli"
- "human-output"
- "ux"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-kx2y"
    type: "advances"
  - kind: "issue"
    id: "atelier-vhxi"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "ready"
title: "Refresh human CLI output ergonomics"
updated_at: "2026-06-23T20:49:29.751789580+00:00"
---

## Description

Refresh Atelier's default human CLI output so operators can scan state, blockers, proof, and next actions without fighting repeated inline help, raw telemetry formatting, unbounded lists, or monochrome walls of text. The mission follows the Zen of Atelier: the repository remains the source of truth; proof must stand on its own in command transcripts and docs; output should model the domain instead of flattening everything into key/value blobs; coordination and blockers must be visible without being noisy; every formatting feature must justify its cost; and obsolete output patterns should be deleted once the shared grammar is clear.

Constraints:
- Mission boundary: `atelier-vhxi` owns product semantics and workflow
  correctness for the fixed mission/epic model, hierarchy validation, branch
  action visibility, and context-only links. `atelier-kx2y` owns presentation:
  output grammar, formatter/color policy, bounded lists, command wording,
  footers, and refreshed human views.
- Coordination rule: if a change alters what records are valid, what a
  transition does, what lint rejects, or what workflow YAML means, it belongs in
  `atelier-vhxi` before affected UX surfaces are polished. If a change only
  changes how correct state is rendered for humans, it belongs in
  `atelier-kx2y`.
- The storage/cache rewrite mission remains separate. This mission may reflect
  its final terminology in command output, but it should not absorb cache schema,
  record-file storage, or lazy cache access work.
- Keep quiet output and durable tracker records stable for composition; refresh default human output as an operator interface, not a machine contract.
- Use a shared formatter/style boundary before broad command rewrites so colors, headings, footers, wrapping, and omitted-row summaries are consistent.
- Enable color only for interactive terminals by default, keep colorless output complete, and honor NO_COLOR; add a --color flag only through an explicit follow-up decision if needed.
- Move repeated drill-down commands to ranked footers; rows should carry human meaning first and IDs/commands second.
- Bound broad lists by default, especially evidence and activity views, and state omitted counts with focused drill-down commands.
- Treat status/Git disagreement, stale status-like views, and config/parser failures as correctness bugs or public recovery UX problems, not as formatting concepts.
- Separate workflow state, blocker state, display role, and next command so visible rows are not confused with next work.
- Preserve one obvious lifecycle path through issue workflow transitions; retired or hidden command names should not be polished back into normal workflow.
- Do not preserve obsolete formatting shims once the new grammar is implemented and validated.

Risks:
- A broad formatting pass can create snapshot churn without improving decisions; sequence by operator workflow and require before/after transcript evidence.
- Color can harm logs and accessibility if it carries meaning alone; all status and blocker meaning must remain textual.
- Formatter abstractions can become ornamental; introduce helpers only where they enforce shared policy across multiple surfaces.
- Changing dense outputs may hide useful proof or recovery information; each command needs validation that the next action remains visible.

Validation:
- Command audit and human output docs name the shared output grammar, color policy, list budget, footer/drill-down rules, and affected command surfaces.
- Command audit docs incorporate actual agent complaint evidence from `docs/product/command-audit/agent-complaints.md`, including status correctness bugs, hidden ready work, parent-blocker ambiguity, command-language, and stale-help failures.
- Before/after transcripts cover status, issue list/search, issue show, issue status, issue transition --options, issue blocked, history, evidence list/show, man, and at least one admin/review surface.
- Interactive-color behavior is proven with forced terminal/color tests, and non-interactive or NO_COLOR output remains colorless and semantically complete.
- Focused CLI tests cover wrapping or bounded output for long dirty path lists, long titles, large blocker sets, evidence list limits, and history rows.
- Workflow documentation and admin guidance explain the fixed mission/epic
  domain model, workflow configuration ownership, built-in validators/actions,
  transition execution order, custom context-only links, and the difference
  between canonical docs, `atelier man <role>`, `issue transition --options`,
  and `atelier lint`.
- target/debug/atelier lint, git diff --check, cargo fmt -- --check, and focused CLI integration tests pass before mission closeout.

## Outcome

- Default human output uses a shared, documented grammar for command titles,
  summaries, rows, grouped sections, bounded lists, footers, and color.
- Status and workflow-decision surfaces are correct before they are polished:
  status output agrees with Git, status-like views use current record state, and
  config/parser failures give public recovery guidance rather than internal
  terminology.
- Queue and search surfaces are easier to scan: repeated inline help is moved
  to ranked footers, blocker/context language is explicit, and broad summaries
  use human-readable counts instead of telemetry-style `key=value` blobs.
- Work-selection output distinguishes executable, selectable, context-only,
  blocked, and blocked-through-parent rows so agents do not mistake visibility
  for actionability.
- Detail and transition surfaces summarize dirty checkout state, render recent
  activity as human sentences, and keep raw paths, raw event fields, and long
  transcripts behind focused drill-downs or verbose output.
- Evidence and history surfaces have useful default information budgets,
  omitted-count reporting, and focused drill-down commands for full detail.
- Interactive terminals use shared color styling to reinforce status, blockers,
  warnings, and secondary metadata, while non-interactive and `NO_COLOR` output
  remain complete and colorless.
- Before/after transcripts and focused tests prove that the refreshed output
  improves human scanability without removing the workflow guidance needed by
  agents and reviewers.
- Help and admin/recovery output stop teaching stale flags, implementation
  nouns, duplicate lifecycle paths, or hidden mechanics as normal workflow.
- Admin-type agents get a useful `atelier man admin` path for understanding
  workflow/config ownership, live transition inspection, policy validation,
  repair, branch recovery, pruning, and maintenance without treating man pages
  as the full workflow reference.
