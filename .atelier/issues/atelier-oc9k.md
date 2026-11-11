---
created_at: "2026-11-11T07:02:47.170648626+00:00"
id: "atelier-oc9k"
issue_type: "validation"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-va99"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-z4cr"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T08:54:53.321098629+00:00"
status: "done"
title: "Validate the Desk's two success criteria on this host, independently"
updated_at: "2026-11-11T08:54:57.841379242+00:00"
---

## Goal

Show, by someone who built none of it, that the mission's two success criteria hold on this host: desk-matches-cli and act-is-a-record, as `docs/strategies/desk.md` states them.

## Done when

- [x] A validator that implemented and reviewed none of the Desk's items runs the desk-matches-cli command of atelier-va99 against the installed service, over the tailnet hostname, on converted clones of all five repositories, and it passes. The validator also checks by hand, for one mission in each of two repositories, that the page and the new `atelier show <mission>` agree, without relying on the command, and that the inbox agrees with `atelier list --needs-human`.
- [x] The validator runs the act-is-a-record command and it passes; and by hand, on a scratch repository, says go from a mission's page and reads the commit it left beside the one `atelier ready <mission> --by <name>` leaves in a second clone.
- [x] The validator opens the mission page and the inbox at a phone width and a desktop width and says whether anything is unreadable, overflows or errors, and whether a thread's link from a page opens T3 Code.
- [x] A receipt for each criterion records the commit the service runs, the commands and their results. A criterion that does not hold files a linked bug and is reported to the orchestrator; it is not fixed by the validator.

## Context

Run on the base once atelier-va99 has merged and the orchestrator has updated the service from `master` by the document's steps. First check that the commit the service reports is `master`'s merge of atelier-va99 or later, and record it; if it is not, stop and tell the orchestrator. Use the `validate` procedure. The commands and how to run them are in `desk/README.md` and `docs/architecture/desk.md`. Act only on scratch repositories under `~/.cache/atelier-desk/scratch/`, never on a live repository or the served clones, and start no T3 thread.

## Notes

- 2026-11-11 [handoff] Independent validation complete. Both criteria hold on service 5df3ca74. Proof, by-hand comparisons, layout notes and what the commands do not check are in the Report. Receipts: atelier-kluy (desk-matches-cli), atelier-92jd (act-is-a-record).

## Report

Both success criteria hold on the installed Desk. Service built_from is 5df3ca74cbfcb303121211add4dc7d0533f73ec6, master's merge of atelier-va99. Independent validator; none of the Desk's items were implemented or reviewed here.

desk-matches-cli: pass. GET /api/repos listed astromech, atelier, cosplay-studio, demiurge, life-on-rails with no errors. From desk/: TMPDIR=~/.cache/atelier-desk/tmp pnpm desk-matches-cli http://buildbot.tail47515a.ts.net:3780 ~/.cache/atelier-desk/clones --atelier next/target/release/atelier exited 0, printed same for all 23 open missions and 5 inboxes, then "All 23 open missions and 5 inboxes match the CLI." By hand over the tailnet hostname: atelier-z4cr page title "Build the Desk: the human's place to plan missions and read results, on the redesigned records", status Ready, counts 1 ready, matching `atelier show atelier-z4cr` in the clone (Status: ready; Items Ready: atelier-oc9k). astromech-610m page title "Ship v1.0", status Ready, counts 3 draft, 10 ready, matching `atelier show astromech-610m`. Inbox needs-human IDs equal `atelier list --needs-human` in each clone: atelier 5 (atelier-3km4 atelier-7w9s atelier-8i86 atelier-j86z atelier-pywh); astromech 47, same set. Receipt atelier-kluy. The command checks title, status, in-progress, per-status counts, page/console errors, and inbox IDs. It does not check documents, item titles, questions, agents, phone width, or thread links (outside the criterion). It defaults to next/target/debug/atelier; this run passed --atelier to the release binary.

act-is-a-record: pass. From desk/: TMPDIR=~/.cache/atelier-desk/tmp pnpm act-is-a-record ~/.cache/atelier-desk/clones/atelier --atelier next/target/release/atelier --desk next/target/release/atelier-desk exited 0 on draft atelier-j529: one new commit each, message "Ready atelier-j529", path M .atelier/missions/atelier-j529/mission.md, records equal with ready time set aside, page Ready, no Say go button, no errors. --differ exited 1: DIFF ... Desk "  by: sumeet", CLI "  by: sumeet-differs". By hand on copies under ~/.cache/atelier-desk/scratch/oc9k/ served by atelier-desk --config on 127.0.0.1:3791 (not 3773/3780/7341), caller sumeet, draft atelier-fd9o cloned twice at 74eb5b5e: Desk commit 02ead25faa8f134f6e2b71c8fbf5dfba5b1fe034 message "Ready atelier-fd9o" path M .atelier/missions/atelier-fd9o/mission.md ready.by sumeet ready.at 2026-11-11T08:52:01.597045811Z; CLI `atelier ready atelier-fd9o --by sumeet` commit 1f39e1adee99773893d3ed8466ae65a46c85237b, same message and path, ready.by sumeet, ready.at 2026-11-11T08:52:09.955524432Z; records equal once ready.at is set aside; reloaded page status Ready, zero Say go buttons. Receipt atelier-92jd. The command uses its own Desk, not the installed service, which matches the criterion. It does not compare commit author; --differ only changes ready.by. Scratch server stopped; ~/.cache/atelier-desk/scratch/oc9k/ removed. Installed service left running at 5df3ca74.

Layout and thread links. Mission pages (atelier-z4cr, astromech-610m) and /inbox at 1440 and 390 px: no console or page errors, no horizontal overflow. Titles that do not fit are ellipsized; nothing is unreadable. Screenshots under ~/.cache/atelier-desk/shots/oc9k/. Converted clone mission pages and the inbox have no thread links. /agents lists six live T3 threads "Holding nothing in the records"; each title links to https://app.t3.codes/threads/f69ed1d3-873d-498a-9515-8ab96392223c/... Opening the z4cr orchestrator thread returned HTTP 200, title T3 Code, heading "Open this thread". No planning session was started.
