---
schema: atelier.item
schema_version: 3
id: atelier-8ep4
title: Give each Agent Factory role only its own instructions, with shared guidance in standards files
status: ready
mission: null
labels:
- agent-factory
priority: high
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-09T04:33:01.597472444Z
trivial: null
caused_by: null
created_at: 2026-11-09T04:33:01.597472444Z
---

## Goal

Each Agent Factory role should read only the instructions for its own work. Instructions for another role bias an agent toward doing that work: an orchestrator that reads all of `procedures/review.md` tends to review instead of arranging an independent review. PR #63 adds this principle to `constitution.md` ("Each role reads only what it needs"). Today the skill breaks it in five places, which this item fixes by moving shared guidance into standards files, each linked only by the roles that use it.

1. **Review standard.** The orchestrator reads all of review.md (Stance, Verdict, What to Look For) to use two sections: Recommendations and Output.
2. **SKILL.md's Independent Review** (367 words) is read by every role, including diagnose, decide, audit and validate workers, though only orchestrators and lone authors arrange review. Who decides findings, and what `fix_now` means, is written in three files: SKILL.md, review.md and orchestrate.md.
3. **`references/atelier.md`** is read by every role, but most of it (start, merge, push without asking, stale approvals, merging the base) is for orchestrators and implementers. A reviewer or diagnose worker is told how to merge and push.
4. **Shaping.** groom and garden apply "plan's shaping tests" and write items "as `plan` does", so they read all of plan.md, including the strategy file format and strategic revision.
5. **Digest.** groom.md and garden.md each spell out the same digest rules (about 250 words), and they already differ: an empty run files nothing in groom but notes its window in garden.

## Done when

- [ ] A review standard holds review's Recommendations, finding severity and Output format, and the sizing scales. `orchestrate` links it and not `procedures/review.md`; `procedures/review.md` keeps only how to perform a review.
- [ ] SKILL.md states the review rule in at most three sentences: non-trivial changes are reviewed independently before merge, and no agent approves or decides findings on its own work. Arranging review, briefing a reviewer, deciding findings and the no-reviewer case each have one home, linked only by the roles that act on them. `grep` finds who decides findings stated in exactly one file.
- [ ] `references/atelier.md` is split by who acts on each part, such as the issue lifecycle, records (the decision-record test, `needs-human`), and loose ends and validation items. The read paths of review, diagnose, decide, audit and validate contain no instruction to merge, land or push: `grep -n 'atelier merge\|land\|push'` over each path's files finds none, or each hit is listed in the Report with why that role needs it.
- [ ] A shaping standard holds plan's mission shaping tests and what makes an issue ready. plan, groom, garden and orchestrate link it; groom and garden no longer link or name `plan` for these.
- [ ] A digest standard holds the rules groom and garden share, with one rule for an empty run. The digest sections of groom.md and garden.md keep only what differs: their entry fields, label and inputs. Their other sections (groom's What to Propose and Authority, garden's Fixing at the Right Level) keep their rules.
- [ ] Every file on a role's read path applies to that role in full. The Report has a table of each role's read path (files, and word count) before and after, counted as atelier-qj9d counted lines: SKILL.md, the procedure, and every file it links, whole.
- [ ] A new eval scenario, `evals/agent-factory/scenarios/orchestrator-arranges-review`, set in an Atelier repository, gives an orchestrator a finished change awaiting review. It passes when the orchestrator dispatches an independent reviewer and fails when it reviews or approves the change itself. It runs 3 times with Opus at the old skill and the new one, every run read by hand. The new skill passes at least 2 of 3 and no fewer than the old. If any run is inconclusive, or the two arms differ by exactly one pass, both arms run 6 times instead, once: the new skill then passes at least 4 of 6 and no fewer than the old, with no further rerun.
- [ ] refused-land-push, done-when-rewrite and trivial-typo run 3 times each with Opus at the old skill and the new one, read by hand, and the new skill passes at least as many runs per scenario. Each counted run used the skill: its transcript shows `atelier status` succeeding and a skill file read. vacuous-test-review runs too if atelier-wl6u has given it an Atelier fixture by then; otherwise the Report says it was left out because the skill steps aside in its fixture. Results go in `evals/agent-factory/results/`.
- [ ] A fresh agent given only the new skill, this repository's AGENTS.md and the CLI help answers atelier-qj9d's seven questions (lifecycle order, what makes an approval stale, when to file a loose end, who arranges review, what the reviewer is and is not given, who decides findings on an author's own work, who records the approval) with no contradiction found.

## Scope

`skills/agent-factory` (agent-facing files), its eval scenarios and results, and links in this repository's docs that point at moved anchors. `constitution.md` and `maintainers/` change only to fix links. Moving text may tighten it, but rewriting the skill for readability and setting a word budget is atelier-bg7w.

## Context

Analysis that found these, 2026-11-08: every role's read path on master 470e59ec, by atelier-qj9d's rule and a whitespace word count, runs 2,128 to 2,893 words, and the orchestrator's 4,071, of which orchestrate.md itself is about 1,250. SKILL.md's Independent Review is 367 words. atelier-qj9d (2026-11-07) removed every reference but atelier.md and folded each rule into "the one file whose role needs it"; this partly reverses that with a stricter test: a shared file is fine when every role that reads it uses all of it.
