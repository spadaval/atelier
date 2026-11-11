---
schema: atelier.item
schema_version: 3
id: atelier-yzuv
title: Stop mission issue show restating scope three times
status: draft
mission: atelier-4pzj
labels:
- cli-output
- issue-commands
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T01:10:57.964882526Z
---

## Goal

`atelier issue show <mission> --full` lists its scope three times: `advances …` rows under Linked Issues, a Mission Rollup, and Needs a Human and Done Work panels (`render_objective_rollup_section`, crates/atelier-cli/src/commands/issue.rs). Other problems in the same view, checked on atelier-mmpv on 2026-11-07:
- `Relationships : 13 advances links, 15 other links`: "other" is never explained, and the count depends on how links are stored, counting a context link stored on both records more than one stored once (atelier-t18i, finding atelier-9839).
- `Evidence Gates: 4 validating evidence records linked, 13 scoped issues without evidence`, though receipts gate nothing since atelier-md0v.
- Subissues and Impact, which can never apply to a mission, still print.
- An item can show one status word in one panel and another elsewhere.
- On parents, the Subissues summary uses a different format: `1 total | status: done=1 | priority: critical=1`.

## Done when

- [ ] Mission show prints Outcome, Success criteria, strategy path and one scope line, e.g. `Scope: 7 issues: 2 ready, 3 blocked, 2 backlog. More: atelier work mission atelier-1w8c`. Nothing else repeats `work mission`.
- [ ] The Relationships and Evidence Gates lines are gone from mission and parent show.
- [ ] Sections that cannot apply to missions are omitted, and each item uses one status word everywhere in the view.
- [ ] The parent Subissues summary matches the work views' count wording, and rendering tests are updated.

## Context

Extends atelier-7u4o (brief `issue show`) and atelier-deyy (dense work rows). atelier-t18i's link count goes with the Relationships line; its cycle message moved to atelier-x3ru (decision atelier-mzpn).
