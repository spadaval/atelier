---
created_at: "2026-11-10T00:56:05.152036368+00:00"
id: "atelier-z5bp"
issue_type: "spike"
labels:
- "redesign"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-jah8"
    type: "advances"
  - kind: "issue"
    id: "atelier-scx2"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Trace 30 astromech bug fixes to the merge that caused each, and choose how defects after merge are counted"
updated_at: "2026-11-10T01:29:37.529296747+00:00"
---

## Goal

Find out whether a defect found after merge can be traced to the item that introduced it, and fix the method the baseline uses. This is the baseline's riskiest assumption, so it is tested first and on a sample.

## Done when

- [ ] `docs/architecture/quality/scorecard-baseline/defect-method.md` states the rule for what counts as a bug fix in a repository's history, written so the same rule can be applied to the other four repositories.
- [ ] It lists 30 bug fixes merged in astromech between 2026-10-15 and astromech's commit in the baseline manifest, drawn by that rule with a stated seed, and for each one the merge that introduced the defect or "not traceable" with the reason.
- [ ] It states the chosen measure: tracing, if at least 20 of the 30 were traced; otherwise bug items filed per merged item, by repository and week.
- [ ] It names what the method cannot see, such as defects nobody has found yet.

## Scope

Read-only on astromech. A sample of 30, not a full count. No reusable script is required here; the full count is a later item.

## Context

Strategy: `docs/strategies/redesign-1-baseline.md`, Derisking. The manifest is `docs/architecture/quality/scorecard-baseline/manifest.json`; read no history after the commit it gives. Astromech is at `~/projects/astromech` and adopted Atelier on 2026-10-15. Trace by `git blame` on the lines a fix changed, then read both changes. Astromech is a private repository: report process facts and paraphrase anything about its content, and send nothing from it to an outside service.
