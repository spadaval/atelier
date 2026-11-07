---
created_at: "2026-11-07T15:21:12.591218395+00:00"
id: "atelier-2bho"
issue_type: "validation"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-mmpv"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Validate the simpler-lifecycle mission's success criteria"
updated_at: "2026-11-07T15:21:12.591218395+00:00"
---

## Goal

Check atelier-mmpv's three success criteria independently (docs/strategies/simpler-lifecycle.md: lifecycle, legible, every-repo) against what is merged, before the mission closes.

## Done when

- [x] Each criterion has an explicit result (pass, fail or partial) and a linked evidence receipt with its criterion slug; the Report states failures, blocked checks and proof limits.

## Report

One of the three criteria passes and two hold only in part. lifecycle passes: `lifecycle_cost` runs every step in the criterion's order on `main` and `master` fixtures with the unedited starter policy (2 of 2 passed, none skipped), asserts no refusal, one approval and done in the merge commit, and prints 12 author and 3 reviewer commands. legible is partial: a fresh cold read of the skill, astromech's AGENTS.md and policy, and the CLI help got the lifecycle order, staleness and loose ends right, but found contradictions. One is on topic: where an in-flight item's loose end is filed. The other three are word meanings (close, done, ADR). every-repo is partial: all five base policies have the new shape and `check` and `doctor` pass everywhere, but astromech's main still has four open After merge lines on items in flight on other branches, and its checkout runs the old policy on feature/astromech-v6r3. Following the assignment, I filed no follow-up bugs; the findings go to the mission's orchestrator.

### Open items

- lifecycle: pass, receipt atelier-oh52. Limit: the command count is printed, not pinned.
- legible: partial (recorded fail), receipt atelier-pznw. references/atelier.md:144-147 files an in-flight item's loose end "on the work branch", while astromech AGENTS.md:36-39 commits filing "straight to `main`". "Nothing else closes" (atelier.md:27-31) clashes with `close_reason` on done, decide and supersede in the policy and `issue transition --help`. done as a status (atelier.md:24-26) clashes with superseded in category done (workflow.yaml:32-35,41-43). The decision record "is the ADR" (tracker-records.md:216-217) clashes with `issue update --adr`. tracker-records.md:237-238 names `issue note --one-way`, a flag the CLI lacks. The judgement rests on one cold reader.
- every-repo: partial (recorded fail), receipt atelier-ulrl. Open After merge lines remain on astromech origin/main: astromech-1hh9:67, astromech-v3u4:85 (non-standard form, which `atelier check` misses), astromech-v6r3:40 and astromech-y6d6:55. More remain on in-flight branches: astromech-dl53:65 on astromech-y6d6, and astromech-7f11:34, astromech-7yb5:29 and astromech-icde:33 on feature/astromech-v6r3. astromech-dl53:67 on main still tells the author to add an After merge line. The astromech checkout (feature/astromech-v6r3, 163 commits behind main) passes `check` and `doctor` under the old policy. atelier, cosplay-studio, demiurge and life-on-rails have none open.
