---
schema: atelier.item
schema_version: 3
id: atelier-bg7w
title: Cap each role's instructions in words and flag long sentences, then rewrite the skill to pass
status: dropped
mission: null
labels:
- agent-factory
priority: medium
blocked_by:
- atelier-8ep4
implementers: []
ready:
  by: conversion
  at: 2026-11-09T04:33:01.921084844Z
trivial: null
caused_by: null
created_at: 2026-11-09T04:33:01.921084844Z
---

## Goal

The skill's instructions should be as clear and concise as they can be. The 300-line budget per role (atelier-qj9d) is meant to force that, but it counts lines, so when a file reaches the limit agents pack clauses into longer sentences instead of cutting rules (atelier-mt5r: "Fit atelier-fyay's rule into plan.md without growing it"). The result is sentences like plan.md's 93-word definition of a ready mission. Replace the line budget with a word budget and a per-sentence readability check, then rewrite the skill until it passes.

## Done when

- [ ] A script in `scripts/` prints, for the agent-facing skill: each role's read-path word count (SKILL.md, the procedure and every file it links, whole), each file's Flesch-Kincaid grade, and every sentence over 30 words or over grade 14 with its file and line. The word count includes everything an agent reads, code spans, code blocks and templates included; only link targets are left out. Readability scoring leaves out code blocks and code spans. Tests on small fixtures show it flags a long sentence, keeps code out of the readability score but in the word count, and counts a linked file once.
- [ ] `maintainers/skill-evaluation.md` gains a budget section, which replaces atelier-qj9d's 300-line rule: at most 2,500 words per role's read path and 3,500 for the orchestrator, counted by the script, and no flagged sentence. The script reports and does not fail CI; reviewers of skill changes read its output.
- [ ] Every role's read path is within its word budget, and the script flags no sentence in an agent-facing file, or the Report lists each remaining one with why it cannot be split.
- [ ] The Report has an inventory of each conditional or exception rule on the agent-facing skill: the rule, and the transcript, eval, review finding or owner direction behind it, or none found. Rules with none found are removed. A rule with evidence or owner direction keeps its meaning, though it may be restated more simply. The Report lists each removed rule.
- [ ] Sets of three or more items are written as lists, not as one sentence joined by semicolons.
- [ ] refused-land-push, done-when-rewrite, trivial-typo, orchestrator-arranges-review (atelier-8ep4) and atelier-l58v's costly-hardening and plan-defect run 3 times each with Opus at the skill before this item and after, every run read by hand, and the new skill passes at least as many runs per scenario. Each counted run used the skill (its transcript shows `atelier status` succeeding and a skill file read). Baseline runs from atelier-8ep4 at the same skill commit may be reused. vacuous-test-review and cross-repo-regression run only if atelier-wl6u has given them Atelier fixtures, and rare-edge-chain only if atelier-pju0 has rebuilt it; the Report names any left out and why. Results go in `evals/agent-factory/results/`.

## Scope

`skills/agent-factory` agent-facing files, `maintainers/skill-evaluation.md`, the new script and its tests, and eval results. Not `constitution.md`. Moving guidance between files is atelier-8ep4, which comes first.

## Context

On master 470e59ec, file averages read at grade 8 to 13, which looks fine, but about 70 sentences run over 30 words; the longest are plan.md's ready-mission definition (93 words), atelier.md's Words section (89) and garden.md's digest rule (62). File averages hide them, so the check scores each sentence, as the Hemingway Editor does. Readability formulas use sentence length and syllables; they do not see stacked conditions directly, but those sentences are always long.

## Report

Dropped: The files it would measure are gone. The rewritten skill is 319 lines, cut by eval to the passages a scenario supports (evals/agent-factory/results/2026-11-11-next-skill.md).
