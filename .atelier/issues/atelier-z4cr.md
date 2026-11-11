---
created_at: "2026-11-11T02:17:25.300286627+00:00"
id: "atelier-z4cr"
issue_type: "mission"
labels: []
strategy:
  path: docs/strategies/desk.md
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Build the Desk: the human's place to plan missions and read results, on the redesigned records"
updated_at: "2026-11-11T08:11:02.426298751+00:00"
---

## Outcome

On this host, a service reached over the tailnet shows Atelier repositories on the redesigned records: converted scratch clones of all five until this host's cutover, and the live repositories after it. A mission has one page with what it is for, its documents, its items and what happened to them, and the agents working on it. An inbox lists what waits on Sumeet. From a page he can say go on a mission, answer an open question, and start a planning session with an agent. Each act is made by the core's own function, the one the `atelier` command calls, under his name: saying go ends in one commit that `ready` makes itself, and an answer is written by `decide` in the item's checkout and committed by the implementer's next `submit`. `atelier` itself has no site.

## Background

Planned on 2026-11-11 from Sumeet's direction in the design discussion over the dashboard spike (branch `spike/mission-control`). ADR 0032 holds the decisions and `docs/product/desk.md` the product intent. It runs beside the redesign's cutover (atelier-qr5s) and neither waits on the other: the Desk is built, installed and validated against converted scratch clones. Pointing it at the live repositories is a loose end outside the mission, atelier-qxmr, which waits on this host's cutover (atelier-wd1i). The mission's area was swept by hand in place of a groom run: atelier-oci2 now deletes the old site, atelier-wpj2 is superseded by this mission, and a probe noted on atelier-aab1 on 2026-11-10 answers what this mission needs from T3 Code; that item stays open for its own Report and for whether thread links open on the phone. Health, the feed and the database are the next mission.

## Notes

- 2026-11-11 [decision] 2026-11-11 orchestrator, after atelier-1ie1 merged (pull request #129): docs/architecture/desk.md finds the core can feed every page and the Design stands. (1) The server item is split: atelier-l5ai adds six small views to atelier-app (M1, M2, M3, M4, M6, M7), and atelier-uwxy then builds the crate. (2) atelier-l5ai is blocked by atelier-psjw, which is approved with CI running on pull request #128. (3) The history of acts (M5, 'atelier log', specified and not built, sized high) is left out of this mission; the Activity tab and the plan-against-actual part of the old debrief wait for it. (4) atelier-desk serves the built app from a directory given at start and does not embed it.
  Reason: (1) Two short branches cross the switch more safely than one long one. (2) On master the code that builds each item's record across branches is in atelier-cli, and atelier-psjw moves it, so the reading pages cannot avoid it. (3) Neither success criterion needs the history, the strategy puts the feed in the next mission, and the mission page shows what happened to an item from its Report, reviews and findings. (4) Building the next/ workspace must need no Node, which atelier-oci2 established.
- 2026-11-11 [note] 2026-11-11 orchestrator: atelier-psjw merged (pull request #128). Planning review by GPT-6 Astra approved atelier-l5ai (atelier-ecc9), atelier-uwxy (atelier-agar), the mission's decision note (atelier-6y75) and, after one fix (finding atelier-byht), atelier-94pv (atelier-u83k). The redesign's planner answered on the tracker side: (a) merges into next/, the root Cargo.toml and the CI and release workflows are allowed until atelier-u1ko is started, then frozen until atelier-wd1i has run, with work continuing on branches; (b) atelier-uwxy must not change what the release ships or give atelier-app the server's dependencies, now a Done when line there; (c) 'atelier log' belongs to mission atelier-295u, item atelier-8b0a, after the cutover.
- 2026-11-11 [decision] 2026-11-11 orchestrator, after atelier-l5ai (pull request #130) and atelier-uwxy (pull request #131) merged: atelier-e05n and atelier-h3hl are planned as the server's halves of threads and acts, and a new item atelier-l2aj carries their pages. atelier-h3hl no longer waits on atelier-94pv and now follows atelier-e05n. The browser's end-to-end proof of saying go moves to the validation item atelier-va99.
  Reason: The Rust half can merge into next/ before the switch freezes it, while the app (atelier-94pv) is built in parallel; atelier-e05n and atelier-h3hl edit the same server files, so they run in order. The desk CI job builds no Rust, so a browser test against a real server belongs with the installed service.
- 2026-11-11 [note] 2026-11-11 orchestrator: planning review by GPT-6 Astra approved atelier-h3hl (atelier-88vm), atelier-l2aj (atelier-63fq) and, after two fixes (findings atelier-69ua and atelier-1p2u), atelier-e05n (atelier-kvnj) and the mission's note (atelier-k1s7). atelier-94pv is being implemented; atelier-e05n starts now beside it.
- 2026-11-11 [note] 2026-11-11 orchestrator: merged atelier-e05n (pull request #133: the server reads T3 Code's threads, proved against the running T3 Code) and atelier-94pv (pull request #134: the app in desk/ with the repositories, mission, item and inbox pages, and a desk CI job). On the converted clone the mission page of atelier-z4cr shows the title, status and counts 'atelier show' prints, and the inbox the five items of 'atelier list --needs-human'. atelier-h3hl (acts and sessions in the server) is being implemented. One deferred finding from atelier-e05n, atelier-mg2j (SIGTERM does not revoke the T3 token), will be taken into atelier-va99, since a service is stopped with SIGTERM.
- 2026-11-11 [note] 2026-11-11 orchestrator: merged atelier-h3hl (pull request #136: POST /api/act performs ready and decide through atelier-app's functions, and POST /api/sessions starts a planning thread, proved once against the running T3 Code in a scratch project that was then removed) and atelier-l2aj (pull request #138: agents on the mission and item pages, a live-threads page, Say go, the answer form and Start a planning session). CI caught one real defect before the second merge: a poll that arrived during a read could show stale data for 30 seconds; fixed and re-reviewed. Observations from using the acts, for Sumeet and the tracker's planner, not acted on here: a refusal's text and next step speak to an agent at a terminal ('git switch ... here'); 'rules' often repeats 'text'; and 'ready' accepts a mission whose plan has no review, saying so only in its text. atelier-va99 (install and the two criterion commands) starts now.
