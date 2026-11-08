---
created_at: "2026-11-08T05:53:04.970886621+00:00"
id: "atelier-7iq0"
issue_type: "task"
labels:
- "agent-factory"
- "review"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-9mdj"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-dkul"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-l58v"
    type: "supersedes"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-08T05:57:29.816555052+00:00"
status: "superseded"
title: "Tell reviewers in the skill to record their verdict with the new command"
updated_at: "2026-11-08T05:57:29.816555052+00:00"
---

## Goal

Once the verdict command exists and the skill lives here, make the skill's review guidance use it, so a shrink or reject actually blocks merge instead of living only in prose.

## Done when

- [ ] `skills/agent-factory/procedures/review.md` tells the reviewer to record its verdict and sizes with the command `atelier-9mdj` added, and `orchestrate.md` tells the accountable agent how to decide an open shrink or reject, both as `atelier review --help` describes them.
- [ ] `references/atelier.md` names the command and the merge refusal in one or two lines, deferring flags to the help.
- [ ] The skill stays within `atelier-qj9d`'s line limits.

## Scope

`skills/agent-factory`.

## Context

`atelier-l58v` writes the verdict into review.md as prose; this item connects it to the record. It waits for the command and for the skill to move here, so the guidance lands in this repository.

## Report

Superseded by atelier-l58v. Sumeet chose on 2026-11-08 to record shrink and reject as fix_now findings whose text opens with the verdict, as l58v plans, with no new record or merge gate. The sizes live in the finding's text.
