---
created_at: "2026-06-10T20:58:39.654760506+00:00"
id: "atelier-g28t"
data: "{\"constraints\":[\"JSON output remains the automation contract and must stay stable unless separately approved.\",\"Color must improve scanning but cannot be the only carrier of meaning.\",\"Human output must work in narrow terminals and non-interactive logs.\"],\"evidence\":[],\"milestones\":[],\"plans\":[],\"risks\":[\"Human-output changes may break users scraping text output; mitigate by documenting JSON as the stable contract and keeping text output explicitly human-oriented.\",\"Adding color and hierarchy inconsistently could make output noisier; mitigate with shared formatter primitives and snapshot tests.\"],\"validation\":[\"Representative golden/snapshot coverage exists for mission show, issue show, issue list/ready/search, and compact hierarchy views across terminal width and color settings.\",\"JSON snapshots or compatibility tests prove machine-readable output was not accidentally changed.\",\"Docs describe the human-output patterns and when a command should use a table, sectioned detail view, grouped queue, or compact tree.\"],\"work\":[]}"
links:
- target_id: "atelier-o54s"
  target_kind: "issue"
  type: "advances"
- target_id: "atelier-o54s"
  target_kind: "issue"
  type: "blocked_by"
- target_id: "atelier-o78q"
  target_kind: "issue"
  type: "advances"
- target_id: "atelier-omnw"
  target_kind: "issue"
  type: "advances"
- target_id: "atelier-u8xc"
  target_kind: "issue"
  type: "advances"
- target_id: "atelier-ugeo"
  target_kind: "issue"
  type: "advances"
schema: "atelier.mission"
schema_version: 1
status: "open"
title: "Improve CLI output"
updated_at: "2026-06-10T20:58:39.654760506+00:00"
---

Atelier's default human-readable CLI output should become scannable, hierarchical, and useful for operators and agents. JSON output is mostly acceptable and remains the automation contract; this mission targets non-JSON views that currently under-communicate, dump tables, or overwhelm users.

## Problem

Current default views make it difficult to understand project state quickly:

- `atelier mission show` exposes too little of the mission graph and evidence state.
- `atelier issue list` is a flat table dump instead of a prioritized, grouped work view.
- `atelier issue tree` is the only hierarchy view, but it is too massive for routine scanning.
- There is no consistent use of color, visual hierarchy, indentation, sectioning, or compact status/progress cues.
- Formatter behavior is not yet an established pattern that future CLI surfaces can reuse.

Use the Jira CLI issue view (`jira view`) as a reference point for dense but readable issue presentation: strong title/status identity, grouped metadata, readable sections, related work context, and obvious next actions. The result should be Atelier-native, not a clone.

## End State

All relevant default human CLI surfaces present useful information at the right density, with clear hierarchy and consistent formatting conventions. Operators can inspect a mission, issue, work queue, or compact issue hierarchy without switching to JSON or a TUI. Future contributors have reusable formatter helpers, docs, and tests that make good output the default path.

## Scope

- Improve human output for mission, issue, list, ready/search where relevant, and compact hierarchy views.
- Preserve machine-readable `--json` contracts unless a separate migration explicitly changes them.
- Introduce or consolidate reusable human-output formatting primitives for color, labels, sections, indentation, wrapping, terminal width, empty states, and command footers.
- Add validation fixtures/golden coverage for representative terminal widths and color modes.

## Out of Scope

- Replacing CLI output with the Mission Control TUI.
- Changing durable tracker semantics solely to make rendering easier.
- Breaking existing Agent Factory commands that rely on JSON or documented core command names.

Recommended subskill: agent-factory orchestrate.
