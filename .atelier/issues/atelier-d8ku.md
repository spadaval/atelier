---
created_at: "2026-10-17T10:00:37.372029871+00:00"
id: "atelier-d8ku"
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
    id: "atelier-rc8j"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Give --commit and leftover parser errors the shared invalid-value form"
updated_at: "2026-10-23T14:32:24.333450912+00:00"
---

## Goal

Mission atelier-o3lm validation (evidence atelier-ba8q) found invalid values still outside the shared form in `docs/product/human-cli-output.md`:
- `review record <id> --identity r --commit zzz x` prints `review commit must be a full Git commit ID` and exits 1; a well-formed unknown commit prints a raw `git merge-base --is-ancestor … failed: fatal: …` with no recovery step.
- `prune --retention-days abc` and `issue list --limit -3` print clap's own wording (exit 2).

## Done when

- These use `Invalid <field> '<value>'. Valid values: …` and exit 2; an unknown commit names the commit and one recovery step.
- Tests cover each.

## Notes

- 2026-10-23 [note] 2026-10-23 regroom against master ff7cc896: still needed. review --commit zzz: 'review commit must be a full Git commit ID'; unknown full SHA leaks raw git merge-base error; prune --retention-days abc and issue list --limit -3 print clap wording (exit 2). parse_limit at main.rs:465 is the model.
- 2026-10-23 [decision] A well-formed commit the branch does not hold exits 1 with 'commit <sha> is not on this branch ... Next: git log --format=%H'; only malformed values exit 2.
  Reason: A missing commit is like an unknown record reference, which exits 1; the invalid-value form is for values outside the accepted form.
