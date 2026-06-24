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

## Workflow Vocabulary

Human output must distinguish four concepts that are easy to flatten in code but
different to operators:

- Workflow state is the durable issue status, such as `todo`, `in_progress`,
  `review`, `validation`, `blocked`, or `done`.
- Blocker state explains whether the row can move now, is blocked directly, or
  is blocked because an owning parent is blocked.
- Display role explains why the row is visible in this command output.
- Next action names the command that can usefully change or inspect the state.

Use these display roles consistently:

| Role | Meaning | Row behavior |
| --- | --- | --- |
| `executable` | The row is the current work or an action can be taken immediately. | Put it in the primary section and include the next lifecycle command nearby. |
| `selectable` | The row can be started or selected by an operator, but it is not yet active. | Put it in ready/selectable sections and show blocker count as zero. |
| `blocked` | The row itself has open blockers. | Put it in blocked sections and summarize the most important blocker titles or counts. |
| `blocked-through-parent` | The row may be otherwise ready, but a parent or owning objective blocks it. | Keep it out of selectable work; show the parent blocker once and route drill-down to the parent. |
| `context-only` | The row explains surrounding state but is not next work. | Use quieter row styling and clear text such as `shown for context`. |
| `omitted` | Matching rows exist but are hidden by list budgets. | Print the omitted count and the focused command that reveals them. |

Rows may use shorter text than the role token, but the meaning must remain
visible in colorless output. Do not use implementation labels such as
`context; parent blocked`, `projection`, or `derived` as normal operator
language.

## Summaries, Budgets, And Footers

Default human output should be bounded. Commands that may return many records
must choose a budget, state when it was applied, and provide one focused
drill-down command. Repeated per-row commands are not allowed in the normal
view.

Preferred list budget behavior:

- show the most actionable rows first, then the most severe blockers, then
  context rows;
- keep short fields such as ID, type, status, priority, and blocker count in
  columns or row prefixes;
- make title and free-form text the final field so it can wrap;
- state omitted counts with the reason, for example `12 more blocked issues
  omitted`;
- put the command for the full list in `Next Commands` or `Drill-downs`, not on
  every row.

Footers are intent-labeled. Use labels such as `Start ready work`,
`Inspect blocker`, `Repair tracker state`, `Show full history`, or `Validate
record`, followed by the exact command. Command code chooses the available
actions; shared formatters only render and deduplicate them.

## Responsibility Boundary

The shared formatter owns presentation mechanics:

- section headings, identity lines, aligned metadata, row prefixes, indentation,
  bounded-list rendering, omitted-count messages, footer rendering, style tokens,
  color enablement, and colorless fallbacks;
- reusable labels for workflow state, blocker state, display role, evidence
  state, and public recovery callouts;
- dirty-path summaries and path samples once command/app code supplies the
  classified state.

Command and app logic own state correctness:

- deciding whether an issue is executable, selectable, blocked,
  blocked-through-parent, context-only, or omitted;
- refreshing or rejecting stale projections before a status-like view claims
  current state;
- checking Git state, workflow validators, configured policy, permissions, and
  provider review state;
- choosing the exact next commands and public recovery path.

Command-specific renderers may still own domain-specific body text, prose order,
and specialized sections. They must not create one-off color policy, duplicate
footer ranking rules, or private workflow vocabulary.

### Domain Facts Versus Rendered Explanations

Domain and workflow services should not return UI annotations, section plans,
or command strings. They return typed facts and rule evaluations: current
workflow status, available transitions, unsatisfied requirements, observed
evidence counts, open blockers, review state, projection health, and checkout
facts.

Application/read-model code assembles those facts for a particular command.
For example, the status read model may combine checkout changes, active work,
transition evaluations, review state, and health checks. It should preserve the
typed facts rather than flattening them into prewritten display messages.

Renderers translate typed facts into human output. If a transition evaluation
says `close` is blocked by an unsatisfied evidence requirement with zero
matching validation records, the renderer may print:

```text
  in progress  atelier-1234  Simplify status output
      -> close blocked: needs linked validation evidence
```

That wording belongs to the renderer. The fact that the `close` transition is
blocked by an unsatisfied evidence requirement belongs to the domain/app
services. Renderers must not rediscover that fact by directly scanning evidence
records or duplicating workflow validators.

The same split applies to footer commands. Domain/app services may return an
actionable state such as an unsatisfied evidence requirement for an issue
target. The CLI adapter owns the exact command spelling, such as
`atelier evidence record --target issue/<id> --kind validation "..."`.

## Detail Views

Use a detail view when the command focuses on one record, such as
`atelier issue show <id>` or `atelier issue show <objective-id>`.

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
- configured validator failures, including evidence validators when workflow
  policy requires them;
- next commands for likely coordination steps.

Mission-shaped reports belong on the read-only mission report surface once it
exists. Issue/workflow commands continue to own creation, linking, mutation,
transitions, and closeout. Root `atelier status` remains checkout/work
orientation and may signpost a mission report, but it should not become the full
mission health report.

Do not hide empty sections when their absence is operationally meaningful.
For example, `Mission blockers: 0` and failed configured validator messages are
useful during closeout. Long free-form bodies may be printed as text blocks, but lists of
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

Queue and objective-status views must not imply that all visible rows are next
work. Ready/selectable rows, blocked rows, blocked-through-parent rows,
context-only parent rows, and omitted rows need distinct text. Parent rows shown
only to explain child work are context-only unless the parent itself is the
action target.

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
  `atelier issue show <id>`, `atelier issue show <objective-id>`,
  `atelier issue status <objective-id>`, `atelier issue list --ready`,
  `atelier issue list --blocked`, and issue blocker commands over scraping
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
field. The compact hierarchy view is distinct from exhaustive relationship
tree output
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

## Zen Alignment

This output contract exists to make the product principles visible:

- The repository remains the source of truth: human output summarizes committed
  record state and points to the canonical record or recovery command when state
  is stale.
- Proof stands on its own: validation and evidence sections name attached proof
  and the command that inspects it instead of relying on chat context.
- Output models the domain instead of flattening it: missions, epics, issues,
  blockers, evidence, and review artifacts keep distinct labels and sections.
- Coordination is visible: blockers, owners, parent context, and next commands
  are first-class output, not implied by row order or color.
- Every formatting feature must justify its cost: add shared helpers only when
  they remove duplicated policy or make repeated operator decisions clearer.
- Obsolete output paths are removed once replaced: retired, hidden, admin, and
  replacement commands are described as such and are not polished back into
  normal workflow.

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
- mission detail with linked work, blockers, evidence records, and configured
  validator failures;
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
stabilization mission: `status`, `issue status <objective-id>`, `issue show`,
`issue list --ready`, `evidence record`, `evidence show/list`, dependency and
link list output, issue impact rendered by `issue show`, `lint`, and admin
repair commands when local state is degraded.

Classification:

- Healthy orientation, objective, issue, evidence, relationship, checkout, and
  health-check views have concise default answers and explicit drill-down
  commands in existing focused tests.
- Degraded orientation and objective status output keeps ordinary reads usable
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
