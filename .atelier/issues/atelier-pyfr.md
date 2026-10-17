---
created_at: "2026-10-17T09:36:28.307299116+00:00"
id: "atelier-pyfr"
issue_type: "bug"
labels:
- "cli-output"
workflow_branch:
  branch_base: master
  work_branch: bug/atelier-pyfr
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-o3lm"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Give every invalid-value error the shared form"
updated_at: "2026-10-17T09:36:37.248771510+00:00"
---

## Goal

Mission validation (evidence atelier-pnha) found invalid-value errors in several shapes despite the Errors rule in `docs/product/human-cli-output.md`:
- `issue link A B --role nope` lists `validates` and `evidenced_by` (which it rejects), omits `blocks` (which it accepts), and is not in the `Invalid <field> '<value>'. Valid values: …` form.
- `issue list --issue-type nope` uses the shared form, but `issue create`/`issue update --issue-type nope` give a workflow-policy sentence.
- An unknown transition says `Unknown transition 'nope' … available from 'todo' are: …`; `issue list --limit 0` says `issue inventory limit must be a positive integer`; `issue create --status done` lists `backlog, todo` while help also lists `draft`.
- Values clap checks exit 2 with clap wording; values checked later exit 1.

## Done when

- Every invalid value for a command argument uses one form, names only the values that command accepts, and the doc states which exit status it uses.
- Tests cover `issue link --role`, `--issue-type` on create, update and list, an unknown transition, `--limit 0` and `issue create --status`.
