---
created_at: "2026-11-10T00:56:04.626625678+00:00"
id: "atelier-qr5s"
issue_type: "mission"
labels:
- "redesign"
strategy:
  path: docs/strategies/redesign-3-cutover.md
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Move all five repositories and Agent Factory onto the new loop, one cutover per host"
updated_at: "2026-11-11T04:59:18.908424841+00:00"
---

## Outcome

On this host, atelier, astromech, demiurge, life-on-rails and cosplay-studio run on the new records and the new binary. Agent Factory is rewritten against the new tool, and each procedure it keeps has an eval scenario it changed. The old workspace is deleted, and the site with it: the new binary has no `serve`, and the Desk (mission atelier-z4cr) replaces it outside this mission. The old product docs and the ADRs are replaced by the design record. Sumeet has a runbook to convert his work host.

## Background

One of four missions that redesign Atelier, in order: baseline, core loop, cutover, gardener. Sumeet set the direction in conversation on 2026-11-09, after an assessment of the project. The target design is in the [product specification](docs/strategies/redesign-product.md) and the [architecture specification](docs/strategies/redesign-architecture.md); this mission's own design is in its strategy. Other Atelier work is paused while the four run.

## Notes

- 2026-11-10 [note] 2026-11-10: orchestration started, unattended, on Sumeet's go-ahead of 2026-11-10. This run prepares the cutover and does not perform it: steps A to E go to their merges, steps F and G are planned, and the switch (atelier-u1ko) stops at an approval on its branch until Sumeet names the time. Steps A (atelier-orb2), C (atelier-5zre), D (atelier-mlwx) and E (atelier-oci2) are planned and go to their planning reviews. Found while planning E: mission 2 never copied the site into next/, so the new binary already has no serve, and step E only proves that and corrects the two specifications. Implementers are Claude Opus 5.5, reviewers GPT-6.1 Sol; each item has its own worktree under ~/.t3/worktrees/atelier/<id>.
- 2026-11-10 [note] 2026-11-10: Sumeet answered atelier-t6o6's six points on 2026-11-11 (as recommended, except that a Codex subagent gets a thread identity of its own); they are in that item's Report, and atelier-1oel (identity, the round in status, the pull request in the review brief) and atelier-yudj (the converter writes a prose Done when as one unticked line) carry them. atelier-psjw, the Desk's tracker change (ready, decide and repository access as functions in atelier-app; ready, drop, close and lead commit what they write), joined this mission's queue before the switch, after atelier-1oel. All seven plans passed planning review, in one to three rounds each. Merged: atelier-yudj (pull request 123, one round; draft and ready items with no checkbox went from 197 and 93 to 0 across the five repositories) and atelier-mlwx (pull request 124, one round). Step D's answer: 0.4.0 is the fallback release, and no 0.4.1 is needed; it matches the installed binary on every command and site route compared, except the review brief wording 74848f4c added. Running: atelier-1oel, atelier-orb2, atelier-5zre and atelier-oci2.
- 2026-11-10 [note] 2026-11-10: steps A, D and E are merged, with atelier-1oel. Step A, the new skill (atelier-orb2, pull request 125, one review round): skills/agent-factory-next/ is 387 agent-facing lines against the live skill's 1,223, 206 of them new and 181 the carried garden, groom and audit procedures. Step D (atelier-mlwx, pull request 124): 0.4.0 is the fallback. Step E (atelier-oci2, pull request 127): the new binary has no site, proved, and the specifications say so. atelier-1oel (pull request 126, four rounds): a Codex spawn_agent child reviewed its parent's item in a real session, recorded as <parent>/<child>; spawn_agent forks its parent's conversation unless given fork_turns none, which the skill now says. Step B is two items: atelier-a4rs ports six eval scenarios to the new binary and adds two, with the harness holding two agents at most by construction (plan approved on its fourth round, in progress), and atelier-t747 runs no skill first and cuts every passage that does not beat it by two runs of three. Steps F and G are planned and in their second planning round with two reviewers; round 1 found that the protocol's rollback did not cover the closing step, that a checkout left on a feature branch would be unusable after the cutover, and that the work host cannot run the same conversion, and the plans now settle each. Running: atelier-psjw, atelier-5zre, atelier-a4rs.
