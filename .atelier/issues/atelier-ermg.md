---
created_at: "2026-10-16T01:33:10.546027704+00:00"
id: "atelier-ermg"
issue_type: "decision"
labels:
- "adoption"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-b7hb"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T01:33:10.660596125+00:00"
status: "done"
title: "Represent size, unverified assumptions, won't-do decisions and already-built work"
updated_at: "2026-10-16T01:33:10.660596125+00:00"
---

## Context

Decided by the implementing agent under atelier-b7hb, which asked for this decision record; Sumeet may revise it. astromech's backlog had S/M/L/XL sizes, unverified markers, a nine-row won't-do table and built rows.

## Question

Backlogs carry four things Atelier had no home for: a size estimate, a marker that an assumption is unverified, a won't-do list with reasons, and a note of what already exists. Which deserve first-class support, and how is each represented?

## Options

- Size: free labels; a reserved `size:` label checked by `check`; or a new front-matter field.
- Unverified assumptions: a new marker or field; or the Unknowns section every issue now has.
- Won't-do: superseded issues; a won't-do table in a strategy; or decision records.
- Already built: done records with a done claim and evidence; or prose in the parent.

## Decision

- **Size** is the reserved label `size:s`, `size:m`, `size:l` or `size:xl`. `check` rejects other values and more than one size per record, `issue show` displays it, and `issue update -l size:<x>` replaces the old size. A label needs no new field, works with `issue list --label`, and is optional.
- **Unverified assumptions** go in the Unknowns section, which every work item now has (atelier-9vq3). Write each as a statement to verify; no separate marker.
- **Won't-do** items are decision records: the Question names the idea, the Decision says no and why. Prune keeps decision records (atelier-r7cd), so the list survives, and `issue list --issue-type decision` shows it.
- **Already-built** work is not recorded as done issues, which would need a done claim and proof for work that never went through Atelier. Describe it in the parent's Background or Context ("Already built: ...") so children can build on it.

## Close Reason

Size is a checked size: label; assumptions use Unknowns; won't-do items are decision records; already-built work is prose in the parent.
