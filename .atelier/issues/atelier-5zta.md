---
created_at: "2026-10-17T01:10:57.987773924+00:00"
id: "atelier-5zta"
issue_type: "task"
labels:
- "cli-output"
- "work-views"
priority: "P2"
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
title: "Put each work section's count in its heading"
updated_at: "2026-10-17T01:10:57.987773924+00:00"
---

## Goal

`atelier list` heads each mission and label group with no count, so a reader counts rows to learn how much work each holds; an agent ran `atelier work ready 2>&1 | grep -cE '^\s+ready '` to count under the old tool (checked 2026-11-11 with the new binary in a converted scratch clone).

## Done when

- [ ] Each mission and label heading in `list` carries its count, e.g. `Labelled cli-output (4):`.
- [ ] `list --ready`'s count agrees with what `status` counts on the same records; a test asserts it.

## Context

Reworks the same rendering as atelier-deyy.

Rewritten for the new tool in the cutover sort (atelier-5zre): the `work` views became `list`; `work mission`'s summary line went with that view.
