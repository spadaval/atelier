---
created_at: "2026-11-08T22:25:20.461573649+00:00"
id: "atelier-fnql"
issue_type: "task"
labels:
- "review"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "review"
    id: "atelier-3qze"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T03:55:51.255673469+00:00"
status: "superseded"
title: "Let a review disposition record a finding as fixed, so a deferred finding fixed in passing needs no follow-up"
updated_at: "2026-11-11T03:55:51.255673469+00:00"
---

## Goal

Sumeet directed on 2026-11-08 that a deferred finding fixed in passing "stays deferred, or is recorded as fixed", and that `fix_now` never marks a fix. Today `review disposition --decision` offers only `fix_now` (an older form), `defer`, `no_action` and `wont_fix`, so the skill records such a finding as deferred and moves its follow-up under the item so the merge marks it done. A `fixed` decision naming the fixing commit would say what happened in one record.

## Done when

- [ ] `review disposition --decision fixed --resolution-commit <sha>` settles a finding recommended defer, and `merge --check` accepts it.
- [ ] `review disposition --help` describes it, and the skill's orchestrate.md uses it in place of moving the follow-up.

## Report

Dropped in the cutover sort (atelier-5zre), reason (a). It adds a decision to `review disposition`, and the What is removed table removes the nine `review` subcommands; the new `drop` settles a deferred finding with a reason.
