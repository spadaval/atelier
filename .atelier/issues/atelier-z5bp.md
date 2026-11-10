---
created_at: "2026-11-10T00:56:05.152036368+00:00"
id: "atelier-z5bp"
issue_type: "spike"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-z5bp
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
status: "in_progress"
title: "Trace 30 astromech bug fixes to the merge that caused each, and choose how defects after merge are counted"
updated_at: "2026-11-10T15:00:23.489269240+00:00"
---

## Goal

Find out whether a defect found after merge can be traced to the item that introduced it, and fix the method the baseline uses. This is the baseline's riskiest assumption, so it is tested first and on a sample.

## Done when

- [x] `docs/architecture/quality/scorecard-baseline/defect-method.md` states the rule for what counts as a bug fix in a repository's history, written so the same rule can be applied to the other four repositories.
- [x] It lists 30 bug fixes merged in astromech between 2026-10-15 and astromech's commit in the baseline manifest, drawn by that rule with a stated seed, and for each one the merge that introduced the defect or "not traceable" with the reason.
- [x] It states the chosen measure: tracing, if at least 20 of the 30 were traced; otherwise bug items filed per merged item, by repository and week.
- [x] It names what the method cannot see, such as defects nobody has found yet.

## Scope

Read-only on astromech. A sample of 30, not a full count. No reusable script is required here; the full count is a later item.

## Context

Strategy: `docs/strategies/redesign-1-baseline.md`, Derisking. The manifest is `docs/architecture/quality/scorecard-baseline/manifest.json`; read no history after the commit it gives. Astromech is at `~/projects/astromech` and adopted Atelier on 2026-10-15. Trace by `git blame` on the lines a fix changed, then read both changes. Astromech is a private repository: report process facts and paraphrase anything about its content, and send nothing from it to an outside service.

## Notes

- 2026-11-10 [note] Handoff: approved by sol-reviewer-r4 (atelier-zmbk) at 8f692e75 after four review rounds. Three fix_now findings fixed (atelier-k10m, atelier-0p3n, atelier-yz6k); none deferred or left undisposed. 27 of 30 traced, so tracing is the measure; see the Report. Merge waits only on CI (ci.passed).

## Report

All four Done when lines hold in `docs/architecture/quality/scorecard-baseline/defect-method.md`. The rule (done `bug` items closed in the window) and the seeded draw (`sha256("z5bp:<id>")`, lowest 30) were rerun from the document's own command block and gave the same 52 candidates and the same 30. 27 of the 30 were traced to the merge that introduced the defect (11 after merge, 5 before merge, 11 before adoption), so the chosen measure is tracing; 3 are not traceable, with reasons. The document's last section names what the method cannot see.

### Decisions

- A bug fix is a done `bug` item, read from tracker records only, so the fallback measure counts the same population and anyone gets the same list.
- Fix commits are those whose subject names the item; failing that, the item's branch line back from the commit that closed it. Message bodies mention items they only filed, so they are not used.
- "Traced" means the introducing merge was found, including pull requests merged before adoption. Only 16 of the 30 trace to an item; read as "traced to an item", the sample would fall short of 20. The orchestrator may want to confirm this reading.
- A bug filed at or before its cause's merge is "before merge" and is not a defect after merge.
- When a later change meets older code, the later change is the cause.
- A defect in a test, such as one that fails intermittently on the base, counts like any other (row 24); step 2 skips test files by path, and tests inside a source file are blamed with it (review finding atelier-k10m).

### Surprises

- Blame alone named the cause in 21 of 27 traces; it often names the last edit, not the cause.
- Three of the 12 items that caused a sampled bug were themselves bug fixes (astromech-0d7l, astromech-36g1, astromech-nrhv), all in journal handling, each causing the next bug in a chain.
- Astromech's early branches were fast-forwarded into main, so many items have no merge commit; the item is found from commit subjects instead.

### Open items

- Rows 7, 11 and 13 rest on the attribution conventions; a second tracer could pick a different cause.
