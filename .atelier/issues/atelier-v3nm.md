---
created_at: "2026-11-09T03:59:33.124261341+00:00"
id: "atelier-v3nm"
issue_type: "validation"
labels:
- "agent-factory"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-cfq9"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-fl12"
    type: "advances"
  - kind: "issue"
    id: "atelier-n7c4"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-ss4y"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-xb18"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Validate planner-end-to-end: a fresh planner replans the site mission under the new skill"
updated_at: "2026-11-09T03:59:33.124261341+00:00"
---

## Goal

Show mission atelier-fl12's criterion planner-end-to-end holds: the planner role, given only the new skill, produces a plan that meets its directive on a real mission, the site mission atelier-yfqu, whose replan is the next planning job anyway.

## Done when

- [ ] A fresh planner agent that did not write atelier-xb18, atelier-n7c4 or atelier-cfq9, given only the installed skill, the repository and atelier-yfqu's 2026-11-08 note of Sumeet's direction, replans atelier-yfqu on the base, and the agent running this item reads the result by hand.
- [ ] The replan's strategy has: a Design section that settles where the site's code lives, how it ships and which libraries it uses, with reasons (the note leaves these open, so any well-argued answer passes); a Sizing section with a reason per scale and a worth-it sentence; a Derisking section naming the riskiest assumption, with the item that tests it ordered first; and no `needs-human` item inside the mission's scope.
- [ ] Two planning review passes are recorded on the replan, from two model families other than the planner's and different from each other, run without seeing each other's findings, and every fix they asked for is re-reviewed.
- [ ] `atelier brief atelier-yfqu` shows the Sizing and Derisking sections under the Outcome.
- [ ] A receipt for planner-end-to-end records the commits, the reviewers' model families and anything the replan lacked; Sumeet's approval of the site plan is the site mission's own gate and is not required here.

## Context

Run on the base once atelier-xb18, atelier-n7c4, atelier-cfq9 and atelier-ss4y have merged, so the replan uses the skill as it ships, and the installed binary and skill include them (`cargo install --path crates/atelier-cli --locked`; move `~/projects/atelier-skill` forward).
