---
created_at: "2026-11-11T02:20:42.636381587+00:00"
id: "atelier-t6o6"
issue_type: "task"
labels:
- "needs-human"
- "redesign"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-fcb7"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T02:51:32.929529308+00:00"
status: "superseded"
title: "Decide six points the core loop rewrite left to Sumeet: Review rules the binary does not enforce, Done when with no checkbox, and thread identity"
updated_at: "2026-11-11T02:51:32.929529308+00:00"
---

## Goal

Decide six points the core loop rewrite (mission atelier-fcb7) could not settle, because each sits in a section of the product specification that only Sumeet changes, or shows a specification to be wrong. The new binary in `next/` behaves as described below until he says otherwise. Mission 3 writes the skill and the cutover against these answers.

1. **Three rules of the Review section are stated as required and the binary does not enforce them.** A kept rebuttal can be rebutted again (the section says a rebuttal gets one try). `submit` opens a fourth round with no escalation (it says escalation is required after three rounds without an approval). The same thread can review every round (it says a rebuttal goes to a fresh reviewer). The mission's plan left all three to the skill, as the first review item's Scope did for the fresh reviewer. `status` shows the one-try rule; nothing shows the other two. Options: leave them to the skill and reword the section to say so; or make them rules of `submit` and `review`, which `ATELIER_SKIP` could pass. Recommended: leave them to the skill, and have `status` show the round number and the three-round rule.
2. **The mission's lead is offered as the decider, and any independent thread can decide.** The section says the lead decides when the item's mission has one. `decide` refuses only the item's implementers and the finding's reviewers. Recommended: keep it; a lead that died should not strand a question.
3. **The review brief holds no merge check results.** The section promises "the latest merge check results", and nothing records a check's result, by the specification's own rule that nobody records a receipt. Since `submit` now starts CI, the brief could name the pull request instead. Recommended: drop the promise and print the pull request.
4. **An item with no checkbox under Done when counts as ticked.** `submit` sends it to review with nothing ticked. 93 ready items in four of the five converted repositories are in that state, because their Done when is prose. A planning reviewer blocked adding a gate without his direction, since it changes Lifecycle. Options: leave it; refuse `submit` when Done when has no checkbox line; or have the converter turn prose into one unticked line. Recommended: the converter writes one unticked line holding the prose, which changes no rule.
5. **A harness's own subagent is its parent's thread.** Under Claude Code, and under T3 Code, a child started with the native subagent tool has its parent's identity, so `review` refuses it as the author; only a `delegate_task` thread can review. Codex's `spawn_agent` child has its own `CODEX_THREAD_ID`, which the identity rule does not read. Inside Codex's session-start hook none of the variables the rule reads is set, so `status` calls the thread `human`. Recommended: keep the rule, have the skill say to start reviewers and deciders as threads, and decide whether to read `CODEX_THREAD_ID`.
6. **`init` is in the product specification's command list and no mission's steps build it.** The converter writes `config.toml` for the five existing repositories. Recommended: build it when a sixth repository needs it.

## Done when

- [x] Each of the six has Sumeet's answer in this item's Report.
- [x] Each answer that changes a specification or the binary is filed as an item.

## Report

Answered by Sumeet on 2026-11-11: the six answers are in this item's Report, and the work they need is atelier-1oel (points 1, 3 and 5) and atelier-yudj (point 4).

Sumeet answered all six on 2026-11-11, in conversation with the mission's planner, who relayed them to the orchestrator of atelier-qr5s.

1. **Review rules the binary does not enforce:** as recommended. They stay with the skill, the Review section is reworded to say so, and `status` shows the round number and the three-round rule. He authorised that rewording of the product specification's Review section. Filed as atelier-1oel.
2. **Any independent thread may decide:** as recommended. Kept; nothing changes.
3. **Merge check results in the review brief:** as recommended. The promise is dropped and the brief names the pull request. He authorised that rewording of the Review section. Filed as atelier-1oel.
4. **Done when with no checkbox:** as recommended. The converter writes the prose as one unticked line. Filed as atelier-yudj.
5. **Thread identity:** changed from the recommendation. A Codex subagent gets an identity of its own: when `CODEX_THREAD_ID` is set and differs from `CODEX_SESSION_ID`, the identity is the one today's rule gives, then `/`, then `CODEX_THREAD_ID`. His work host runs Codex only, and T3's guidance there is to start a same-provider child with the native tool, so the old rule would have refused every reviewer. Claude Code is unchanged: its subagent is still its parent. Measured on this host on 2026-11-11 with Codex 0.167 (`/home/supadava/.cache/atelier-identity-probe/`). Filed as atelier-1oel. Identity inside Codex's session-start hook is not part of it; atelier-1oel reports what that would take.
6. **`init`:** as recommended. Not built until a sixth repository needs it.

Nothing else in the specifications' Words, Lifecycle or Review sections changes. The new skill (atelier-orb2) is written against these answers.
