---
created_at: "2026-11-09T04:33:01.921084844+00:00"
id: "atelier-bg7w"
issue_type: "task"
labels:
- "agent-factory"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-8ep4"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Cap each role's instructions in words and flag long sentences, then rewrite the skill to pass"
updated_at: "2026-11-09T04:33:01.921084844+00:00"
---

## Goal

The skill's instructions should be as clear and concise as they can be. The 300-line budget per role (atelier-qj9d) is meant to force that, but it counts lines, so when a file reaches the limit agents pack clauses into longer sentences instead of cutting rules (atelier-mt5r: "Fit atelier-fyay's rule into plan.md without growing it"). The result is sentences like plan.md's 93-word definition of a ready mission. Replace the line budget with a word budget and a per-sentence readability check, then rewrite the skill until it passes.

## Done when

- [ ] A script in `scripts/` prints, for the agent-facing skill: each role's read-path word count (SKILL.md, the procedure and every file it links, whole), each file's Flesch-Kincaid grade, and every sentence over 30 words or over grade 14 with its file and line. It ignores code blocks, code spans and link targets. Tests on small fixtures show it flags a long sentence, ignores code, and counts a linked file once.
- [ ] `maintainers/skill-evaluation.md` replaces the line budget with: at most 2,500 words per role's read path and 3,500 for the orchestrator, and no flagged sentence. The script reports and does not fail CI; reviewers of skill changes read its output.
- [ ] Every role's read path is within its word budget, and the script flags no sentence in an agent-facing file, or the Report lists each remaining one with why it cannot be split.
- [ ] Rules that answer an edge case no transcript or eval shows an agent meeting are removed. The Report lists each removed rule and why, such as orchestrate.md's "a deferred finding fixed in passing stays deferred" and "a new finding on the fix of a low-severity finding is deferred unless it is trivial".
- [ ] Sets of three or more items are written as lists, not as one sentence joined by semicolons.
- [ ] The four existing scenarios (refused-land-push, vacuous-test-review, done-when-rewrite, trivial-typo), orchestrator-arranges-review (atelier-8ep4), and the saying-no scenarios atelier-l58v added, run 3 times each with Opus at the skill before this item and after, every run read by hand: the new skill passes at least as many runs per scenario. Results go in `evals/agent-factory/results/`.

## Scope

`skills/agent-factory` agent-facing files, `maintainers/skill-evaluation.md`, the new script and its tests, and eval results. Not `constitution.md`. Moving guidance between files is atelier-8ep4, which comes first.

## Context

On master 470e59ec, file averages read at grade 8 to 13, which looks fine, but about 70 sentences run over 30 words; the longest are plan.md's ready-mission definition (93 words), atelier.md's Words section (89) and garden.md's digest rule (62). File averages hide them, so the check scores each sentence, as the Hemingway Editor does. Readability formulas use sentence length and syllables; they do not see stacked conditions directly, but those sentences are always long.
