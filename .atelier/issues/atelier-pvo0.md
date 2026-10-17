---
created_at: "2026-10-17T09:14:09.736875227+00:00"
id: "atelier-pvo0"
issue_type: "bug"
labels:
- "cli-output"
priority: "P3"
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
title: "Name the command for the remaining silent or bare truncations"
updated_at: "2026-10-17T09:14:09.736875227+00:00"
---

## Goal

atelier-ytxg made the work-view lists name the command that shows the rest. Its review found more cuts outside that item:
- `issue show`'s Recently Updated rollup stops at 8 rows with no count or command (`crates/atelier-cli/src/commands/issue.rs`, about line 521; finding atelier-hrfh).
- Two `land` errors list 5 paths then "and N more" with no command (`session.rs`, about lines 363 and 391; finding atelier-wslw); `git status` or `git diff --name-only <base> -- .atelier` would show the rest.
- "Did you mean a, b, c (and N more)?" names no command (`crates/atelier-app/src/record_ref.rs`, about line 90; finding atelier-378y).
- Also noted: `status` on an owner's branch silently cuts scoped work at 8 (undocumented), and `evidence list`'s `Showing: N of M` names no command.

## Done when

- Each of these names the command that shows the rest, or is documented as a bounded window.
- Rendering tests cover each.
