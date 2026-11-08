---
created_at: "2026-11-08T05:35:31.731292457+00:00"
id: "atelier-edaj"
issue_type: "feature"
labels:
- "review"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Have atelier check catch plan records with no Done when lines or broken links"
updated_at: "2026-11-08T05:35:31.731292457+00:00"
---

## Goal

Plan review spends reviewer time on mechanical problems a script can find. Have `atelier check` report them so reviewers can spend their time on vagueness and sequencing.

## Done when

- [ ] `atelier check` reports an open work item with no Done when lines, and a Done when line with no text.
- [ ] `atelier check` reports a repository-relative Markdown link in an issue record or a strategy file that a mission links when the target file does not exist (anchors need not be checked).
- [ ] Each report names the record, the line and what to do instead.
- [ ] The new problems print as warnings and do not change `atelier check`'s exit code, so on the current base of each local Atelier repository (atelier, astromech, life-on-rails, cosplay-studio, demiurge, worldcrafter) it exits as it does with the installed binary. The Report lists what the new checks find in each repository today.
- [ ] Tests cover each new report and the existing-records case.

## Scope

Atelier's code, `docs/product` and this item's records. Fixing the problems the check finds in other repositories is out of scope; list them.

## Context

The 2026-11-08 review audit found that plan review mostly caught criteria that could not pass, false facts, ordering and ownership problems, which need judgment; broken links and missing criteria are the mechanical share. Lauren Tan's pstack checks plan structure with a script (check-plan.mjs) for the same reason.
