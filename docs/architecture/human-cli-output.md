# Human CLI Output

Atelier human-readable CLI output is an operator interface, not a scripting
contract. JSON output remains the stable automation boundary. Human output may
change to become more scannable, but it must keep enough text context for
non-interactive logs, narrow terminals, and colorless environments.

This document defines the visual and structural rules for default non-JSON
output. It is the implementation guide for improving mission detail, issue
detail, list/ready/search queues, and compact hierarchy views.

## Goals

- Let an operator identify the record, state, blockers, progress, and next
  useful command without opening several follow-up views.
- Use the same output grammar across mission, issue, queue, and hierarchy
  surfaces.
- Preserve exact JSON schemas unless a separate migration explicitly changes
  them.
- Keep output useful when copied from logs, pasted into handoffs, or viewed in a
  terminal without color.

## Shared Grammar

Every human view should be built from the same primitives:

- Identity line: canonical ID, kind or issue type, status, and title.
- Metadata block: short aligned key/value rows for priority, owner, assignee,
  parent, timestamps, labels, and close state.
- Section heading: a plain text heading followed by a same-width underline.
- Count summary: compact `name=count` groups for progress, status, priority, or
  evidence state.
- Row prefix: two spaces for child rows; deeper hierarchy uses two additional
  spaces per level.
- Empty state: an explicit `(none)` or a short sentence naming what is absent.
- Footer: next useful commands when a follow-up action is obvious.

The current `atelier issue show` output already uses much of this grammar:
identity line, metadata rows, section headings, hierarchy, blockers, subissues,
recent activity, and next commands. New surfaces should reuse that shape instead
of inventing command-specific tables.

## Detail Views

Use a detail view when the command focuses on one record, such as
`atelier issue show` or `atelier mission show`.

Detail views should answer these questions in order:

1. What record is this?
2. Is it open, blocked, closed, or otherwise actionable?
3. Why does it exist?
4. What owns, blocks, validates, or advances it?
5. What changed recently?
6. What should the operator run next?

Required sections for issue detail views:

- identity and metadata;
- hierarchy parent;
- description and acceptance criteria when present;
- close reason when present;
- blockers and dependents;
- subissue summary and bounded child rows;
- recent activity;
- next commands.

Required sections for mission detail views:

- identity and status;
- body, constraints, risks, and validation expectations when present;
- plans, milestones, and evidence summaries;
- mission blockers with open blockers visibly marked;
- linked work grouped by ready, blocked, done, and backlog;
- evidence gaps;
- next commands for likely coordination steps.

Do not hide empty sections when their absence is operationally meaningful.
For example, `Mission blockers: 0` and an evidence gap message are useful during
closeout. Long free-form bodies may be printed as text blocks, but lists of
related records should be bounded or grouped before they become noisy.

Example shape:

```text
atelier-1234 [task] open - Rebuild markdown index
================================================
Priority: high
Parent:   atelier-0010
Labels:   tracker, migration

Why
---
Keep committed markdown and rebuilt runtime state aligned.

Blockers
--------
  atelier-0009 [open] high - Decide migration ordering

Next Commands
-------------
  atelier issue comment atelier-1234 "..."
  atelier issue close atelier-1234 --reason "..."
```

## Queue Views

Use a queue view when the command returns many independent records, such as
`atelier issue list`, `atelier issue ready`, and `atelier issue search`.

Queue views should be grouped before they are tabulated. Preferred grouping
order is:

1. readiness: ready, blocked, in progress, backlog, done;
2. priority: critical, high, medium, low;
3. issue type;
4. parent epic or mission context when available.

Rows should include at least ID, priority, type, status when not implied by the
group, title, and compact blocker or parent cues when available. Use fixed-width
columns only for short fields. Titles and other free text should be the final
column so they can wrap or truncate consistently.

Empty queue output should say what was searched and what to try next. For
example, `issue ready` may include the blocked count, while `issue search`
should echo the search query.

Quiet mode remains the terse path. For read commands, quiet output should omit
headings, footers, explanatory prose, and color while preserving the minimum
record identifiers needed to act on the result. JSON mode remains the automation
path and must not be built by parsing human output.

Example shape:

```text
Ready high
----------
  atelier-1234 task  Rebuild markdown index
  atelier-1235 bug   Fix export check regression

Blocked high
------------
  atelier-1236 epic  Markdown-first cutover  blocked_by=atelier-0009
```

## Compact Hierarchy Views

Use a compact hierarchy view when the operator needs shape rather than every
field. The compact hierarchy view is distinct from the exhaustive `issue tree`
view.

Compact hierarchy output should:

- show parent/child shape with two-space indentation;
- show status and priority in the row prefix;
- include child progress summaries for collapsed nodes;
- apply deterministic limits for depth and sibling count;
- state when rows are omitted, including the hidden count;
- preserve the existing full tree behavior unless a separate migration changes
  it.

Deep trees should remain readable in narrow terminals. Prefer collapsed
summaries over horizontal expansion.

## Color And Symbols

Color is optional hierarchy, never the only carrier of meaning.

- A colorless log must still show status, priority, blockers, and omitted-row
  counts as text.
- Symbols may be used only when paired with text or when they are already a
  familiar status marker in the same line.
- Initial color policy is automatic only: enable color for interactive terminals
  when `NO_COLOR` is not set, and disable it for non-interactive output.
- Avoid one-off per-command color decisions; use shared style helpers once
  color is introduced.
- A future `--color=auto|always|never` flag is a separate CLI-surface decision
  and should be captured as its own issue before implementation depends on it.

Until shared color helpers exist, output changes should focus on layout and text
structure. Downstream implementation may add color behind a single formatter
boundary.

## Width And Wrapping

Human output must work in narrow terminals and logs.

- Short metadata keys may align in a fixed column.
- Long free text should wrap or remain in the final column.
- Tables must not rely on unlimited terminal width.
- Tests for changed output should include at least one narrow-width fixture or a
  deterministic formatter unit that proves wrapping/omission behavior.
- Do not make layout depend on terminal width unless tests can force the width.

If terminal width is unknown, prefer conservative output: fewer columns, more
vertical grouping, and explicit omitted-row summaries.

## Formatter Boundary

New output work should introduce reusable formatter helpers only when they serve
more than one surface or make a testable policy explicit.

Recommended helper boundaries:

- section headings and empty-state rendering;
- label/status/priority text normalization;
- bounded related-record lists;
- grouped queue rendering;
- hierarchy row rendering and omission summaries;
- color/style decisions, once color is added.

Command handlers should keep collecting domain data and call formatter helpers
for presentation. JSON object construction should remain separate from human
formatting so changing display text does not accidentally alter automation
contracts.

## Testing Expectations

Human-output changes need focused tests at the behavior boundary they affect:

- detail views with blockers, subissues, recent activity, and empty sections;
- mission detail with linked work, blockers, evidence, and evidence gaps;
- queues with mixed priority, type, status, blockers, and parent context;
- compact hierarchy with deep trees, wide sibling sets, and closed/open mixes;
- narrow-width or bounded-output behavior when wrapping or omission is involved;
- JSON compatibility tests for every command whose human output changes.

Tests should assert durable signals and structure, not incidental whitespace
unless the test is specifically for a formatter primitive.

## Downstream Work

The implementation epics under the CLI output mission should use this order:

1. Upgrade mission and issue detail views around the shared detail grammar.
2. Replace flat issue queues with grouped queue views.
3. Add compact hierarchy output as a bounded scan view separate from full tree.
4. Validate the surfaces together and document any remaining exceptions.

If a downstream slice needs a new rule not covered here, update this document
before implementing divergent output.
