# Human CLI Output

Atelier human-readable CLI output is an operator interface, not a scripting
contract. It is now the primary supported command-result surface. Human output
may change to become more scannable, but it must keep enough text context for
non-interactive logs, narrow terminals, and colorless environments.

This document defines the visual and structural rules for default non-JSON
output. It is also the migration guide for scripts and agents that previously
expected command-result JSON from ordinary Atelier commands. It is the
implementation guide for improving mission detail, issue detail,
list/ready/search queues, and compact hierarchy views.

## Goals

- Let an operator identify the record, state, blockers, progress, and next
  useful command without opening several follow-up views.
- Use the same output grammar across mission, issue, queue, and hierarchy
  surfaces.
- Keep command output focused on the small set of fields needed for the current
  workflow, with explicit next commands for drill-down state.
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
- Footer: intent-labeled next actions when a follow-up action is obvious.
  The command supplies the actions; shared formatting may align labels and
  commands but must not invent generic advice.

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
- canonical Markdown file path when a `.atelier/issues/<id>.md` record exists;
- hierarchy parent;
- description and acceptance criteria when present;
- close reason when present;
- blockers and dependents;
- subissue summary and bounded child rows;
- recent activity;
- next actions for editing the Markdown record, validating the issue, adding a
  note, starting work, reopening, or closing as appropriate.

Required sections for mission detail views:

- identity and status;
- body, constraints, risks, and validation expectations when present;
- planning/checkpoint references from mission prose and evidence summaries;
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
  atelier issue note atelier-1234 "..."
  atelier issue transition atelier-1234 close --reason "..."
```

## Setup And Health Views

Setup and health commands should name only the next command that can succeed in
the current state. A fresh `atelier init` checkout creates tracker directories,
runtime state, and starter workflow policy. Its default next steps point to
`atelier lint` before issue creation. Health commands may name low-level repair
commands only when the checked state is actually stale, invalid, missing, or
otherwise degraded; `doctor --fix` is the admin explicit local repair path.

## Queue Views

Use a queue view when the command returns many independent records, such as
`atelier issue list`, `atelier issue list --ready`, and `atelier search`.

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
example, `issue list --ready` may include the blocked count, while
`issue search` should echo the search query.

Quiet mode remains the terse path for strict composition values only. Quiet
output may contain IDs, counts, paths, status tokens, and pass/fail tokens. It
must omit headings, footers, explanatory prose, and partial human detail views.
Commands that cannot provide meaningful terse output should prefer the normal
human view instead of pretending that a truncated human report is script-safe.
Automation should use durable tracker state and explicit command workflows
rather than command-result JSON.

Example shape:

```text
Ready high
----------
  atelier-1234 task  Rebuild markdown index
  atelier-1235 bug   Fix export check regression

Blocked high
------------
  atelier-1236 epic  Markdown-first cutover - blocked by atelier-0009
```

## Script And Agent Migration

Atelier no longer treats `atelier <command> --json` command-result envelopes as
the stable automation API for normal tracker workflows. Existing scripts and
agents should migrate by choosing the smallest supported surface that matches
their need:

- Use quiet acknowledgements for simple command composition. Mutating commands
  should print only the affected ID, changed fields, stable paths, and recovery
  guidance when quiet output is requested. Scripts may branch on exit status
  and the stable IDs or paths in quiet output, but should not parse full detail
  views.
- Read canonical records under `.atelier/` when durable tracker state is
  needed. These Markdown records are the reviewable, mergeable source of truth
  for issues, missions, evidence, and activity sidecars. Planning and
  checkpoint intent lives in the accountable record prose or referenced
  repository Markdown artifacts until first-class records are reintroduced.
- Use committed-state commands for handoff gates. `atelier lint` is the
  supported noninteractive check for invalid tracker state. Local runtime repair
  commands are admin repair tools, not normal script workflow.
- Preserve blocked-command and record context in stale projection or invalid
  canonical-record errors, then give one ordered recovery path through lint,
  record repair, health check or fix, and rerunning the blocked command.
- Use focused drill-down commands for targeted state. Prefer commands such as
  `atelier issue show <id>`, `atelier mission show <id>`, `atelier mission status <id>`,
  `atelier issue list --ready`, `atelier issue list --blocked`,
  `atelier graph tree --compact`, and issue blocker commands over scraping
  broad human reports.
- Use documented authored JSON inputs and derived projection files only where a
  specific document defines that contract, such as bundle input JSON or a
  future Mission Control projection.

Full human output is allowed to change for readability, grouping, width, color,
and next-command guidance. Treat it as an operator interface, not a stable
machine API. Scripts that scrape section headings, table spacing, prose, or
complete detail output must be migrated to one of the supported surfaces above.

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
- A future `--color=auto|always|never` flag is a separate CLI-surface artifact
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
for presentation. Canonical state projection, authored JSON inputs, and
diagnostic logging are separate from command-result rendering.

## Testing Expectations

Human-output changes need focused tests at the behavior boundary they affect:

- detail views with blockers, subissues, recent activity, and empty sections;
- mission detail with linked work, blockers, evidence, and evidence gaps;
- queues with mixed priority, type, status, blockers, and parent context;
- compact hierarchy with deep trees, wide sibling sets, and closed/open mixes;
- narrow-width or bounded-output behavior when wrapping or omission is involved;
- migration checks proving scripts can use quiet acknowledgements, canonical
  records, projections, or focused drill-down commands without parsing full
  human output when a command previously had command-result JSON consumers.

Tests should assert durable signals and structure, not incidental whitespace
unless the test is specifically for a formatter primitive.

## Operator Output Audit

The `atelier-rgd1` audit sampled the common operator surfaces named by the CLI
stabilization mission: `status`, `mission status`, `mission show`,
`mission list`, `issue show`, `issue list --ready`, `evidence record`,
`evidence show/list`, dependency and link list output, `graph impact`,
`worktree status`, `lint`, and admin repair commands when local state is
degraded.

Classification:

- Healthy orientation, mission, issue, evidence, relationship, worktree, and
  health-check views have concise default answers and explicit drill-down
  commands in existing focused tests.
- Degraded orientation and mission status output keeps ordinary reads usable
  while routing repair to `atelier lint` or admin repair commands only when
  committed records or local state are degraded.
- Fresh `atelier init` previously suggested `atelier issue create "Task"` before
  workflow setup, which produced an immediate workflow-policy error. The default
  setup output now creates workflow policy and routes through `atelier lint`
  before issue creation.
- No additional failed output classifications were found in the sampled common
  operator workflows; future failures should become follow-up implementation
  issues linked to the owning CLI-surface epic.

## Downstream Work

The implementation epics under the CLI output mission should use this order:

1. Upgrade mission and issue detail views around the shared detail grammar.
2. Replace flat issue queues with grouped queue views.
3. Add compact hierarchy output as a bounded scan view separate from full tree.
4. Validate the surfaces together and document any remaining exceptions.

If a downstream slice needs a new rule not covered here, update this document
before implementing divergent output.
