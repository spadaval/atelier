# Issue Inventory And Mission Overview

`atelier issue list` and `atelier work missions` are separate read surfaces.
They use the same canonical issue and relationship facts, but they answer
different operator questions and must not share a hierarchy-shaped read model.

## Operator Questions

| Surface | Operator question | It does not own |
| --- | --- | --- |
| `atelier issue list` | Which issue records match these simple metadata filters, and which ID should I inspect? | Mission membership, hierarchy, blocker-aware selection, progress rollups, or operational queue grouping. |
| `atelier work missions` | Which current missions and directly linked epics define the mission backlog, and what work sits outside the visible mission structure? | Leaf-task selection, record mutation, exhaustive issue inventory, or one mission's full execution detail. |
| `atelier work mission <mission-id>` | What is happening inside this one mission, including its actionable leaf work? | Cross-mission comparison or generic inventory. |

`work missions` is the **Mission Overview** in headings, help, and operator
guidance. The plural form is the cross-mission overview; the singular
`work mission <mission-id>` remains the scoped drill-down.

## Flat Issue Inventory

The default `atelier issue list` includes every canonical issue record,
including done-category records. A record appears once even when it has a
parent, advances more than one mission, or has blockers. Each human row contains
the issue ID, issue type, exact workflow status, priority, and title. Rows have
no parent or mission headings, indentation, progress counts, blocker-derived
state, or per-row commands.

Inventory filters are exact metadata filters:

- `--status <status>` selects an exact configured workflow status; `all` means
  no status restriction and is the default;
- `--category <category>` selects a configured derived status category;
- `--issue-type <type>`, `--label <label>`, and `--priority <priority>` select
  the corresponding canonical metadata;
- supplied filters combine with AND semantics.

Ready and blocked are operational judgments, not metadata. `issue list
--ready` and `issue list --blocked` are removed without aliases. Use `atelier
work ready` and `atelier work blocked` for those questions.

Inventory ordering is issue ID ascending after filtering. It does not use
blocker order, priority rank, recency, parentage, or mission membership. The
default human and quiet views show at most 50 rows. `--limit <positive-integer>`
changes that budget; it does not change selection or ordering. When rows are
omitted, human output states `Showing <shown> of <matching> issues` and suggests
narrowing the metadata filters or raising `--limit`. Quiet output prints only
the selected issue IDs, one per line, in the same order and under the same
limit. An empty human result says that no issues match the selected filters;
an empty quiet result writes nothing.

Representative default output:

```text
Issue Inventory
===============
ID            Type        Status       Priority  Title
atelier-4fip  epic        todo         high      Build the formatted Mission Overview
atelier-c0mp  mission     in_progress  high      Separate issue inventory from the mission overview
atelier-vgqe  task        in_progress  high      Define the issue inventory and Mission Overview contract

Showing 3 of 3 issues

Drill Down
----------
  Inspect a record: atelier issue show <issue-id>
```

Representative filtered composition:

```text
$ atelier issue list --issue-type mission --category active --quiet
atelier-c0mp
```

## Mission Overview Membership

A mission's scope remains its direct outgoing `advances` links plus the
descendants of those linked roots. Parentage alone never assigns an epic or
issue to a mission.

The Mission Overview classifies direct roots as follows:

- a directly advanced epic is an epic row under that mission;
- a directly advanced non-epic is included in that mission's `Direct work`
  summary and is not presented as an epic or indented beneath one;
- descendants contribute to their root's collapsed state and progress counts,
  but leaf rows do not appear in the cross-mission default;
- each issue ID contributes once to a mission's total counts even if redundant
  roots make it reachable more than once. Individual root summaries may
  overlap, so they are not presented as additive mission totals.

If an epic or other root directly advances more than one mission, it is shown
or summarized under every mission that owns an `advances` link. This is shared
membership, not a conflict to resolve in the renderer. Repository-wide counts
deduplicate issue IDs; per-mission counts intentionally describe each
mission's scope independently.

## Default Inclusion And Exceptional Work

By default, `atelier work missions` excludes mission records whose configured
status category is `done`. `atelier work missions --all` includes them after
the non-done missions; `--all` changes done-mission inclusion but does not
disable output budgets. A done mission with nonterminal scoped work stays
hidden by default, but its nonterminal work is counted in the `Outside visible
missions` summary so it cannot silently disappear.

The overview ends with `Outside visible missions` when either count is nonzero:

- `Unassigned` counts non-mission, nonterminal issue records that are not
  reachable from any mission through `advances` roots and descendants;
- `Linked only to done missions` counts nonterminal records that are reachable
  from missions, but from no mission visible in the default view.

These are facts, not synthetic missions or epic children. The overview does not
invent an `Unassigned` mission, change relationships, or imply that a record is
ready. Operators use `atelier issue list` to browse the underlying records,
`atelier work ready` or `atelier work blocked` for operational selection, and
`atelier issue show <id>` for a selected record.

## Mission Overview Ordering And Budgets

Mission rows and epic rows use the same deterministic sort keys:

1. configured status category: active, blocked, todo, done, then unknown;
2. priority: critical, high, medium, low, then unknown;
3. issue ID ascending.

The human overview shows at most 20 missions and at most 10 directly linked
epics per visible mission. Selection and counts are computed before truncation.
An omitted mission or epic count appears next to the affected section. The
focused drill-down is `atelier work mission <mission-id>` for a mission and
`atelier work epic <epic-id>` for an epic. Operators can use `atelier issue
list --issue-type mission` to browse mission records when the cross-mission
budget omits a mission.

Quiet Mission Overview output prints only visible mission IDs, one per line, in
the same order and under the same 20-mission budget. It never prints epic IDs,
rollups, headings, omission prose, or ANSI escapes. `--all --quiet` includes
done mission IDs under the same ordering and budget. An empty quiet result
writes nothing.

Representative default output:

```text
Mission Overview
================

atelier-c0mp  active  high  Separate issue inventory from the mission overview
  Progress: 4 active · 6 todo · 0 done · 0 blocked
  atelier-4fip  epic  todo  high  Build the formatted Mission Overview
      3 child issues · 0 done · 0 blocked
  atelier-nzu9  epic  todo  high  Rework issue list as simple inventory
      2 child issues · 0 done · 0 blocked
  Direct work: 2 roots · 1 active · 1 todo
  Drill down: atelier work mission atelier-c0mp

Outside visible missions
------------------------
  Unassigned: 3 nonterminal issues
  Linked only to done missions: 1 nonterminal issue
  Browse records: atelier issue list
```

`atelier work missions --all` uses the same shape and adds done missions. The
scoped commands expand the collapsed information:

```text
atelier work mission atelier-c0mp
atelier work epic atelier-4fip
```

## Color And Noninteractive Output

Both surfaces use shared page, panel, row, and semantic style helpers. Human
color is enabled only when standard output is an interactive terminal and the
`NO_COLOR` environment variable is absent. The presence of `NO_COLOR`, even
with an empty value, disables color. Noninteractive output and all quiet output
contain no ANSI escapes.

Color reinforces headings, exact status, priority, blockers, and secondary
metadata. Text, indentation, counts, and omission messages carry the complete
meaning, so TTY color, `NO_COLOR`, and captured noninteractive output have the
same records, ordering, status, priority, blocker, progress, and omission
information.

## Legacy Queue Boundary

The legacy `atelier work queue` is not the Mission Overview and is not the
generic inventory owner. Current guidance routes flat browsing to `issue list`,
cross-mission coordination to `work missions`, scoped coordination to `work
mission` or `work epic`, and selection or triage to `work ready` or `work
blocked`. The legacy queue must not gain a compatibility alias or fallback to
either new read model; it can be removed once remaining callers are migrated.
