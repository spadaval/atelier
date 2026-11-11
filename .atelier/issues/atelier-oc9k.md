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
status: "backlog"
title: "Validate the Desk's two success criteria on this host, independently"
updated_at: "2026-11-11T07:06:02.255706073+00:00"
---

## Goal

Show, by someone who built none of it, that the mission's two success criteria hold on this host: desk-matches-cli and act-is-a-record, as `docs/strategies/desk.md` states them.

## Done when

- [ ] A validator that implemented and reviewed none of the Desk's items runs the desk-matches-cli command of atelier-va99 against the installed service, over the tailnet hostname, on converted clones of all five repositories, and it passes. The validator also checks by hand, for one mission in each of two repositories, that the page and the new `atelier show <mission>` agree, without relying on the command, and that the inbox agrees with `atelier list --needs-human`.
- [ ] The validator runs the act-is-a-record command and it passes; and by hand, on a scratch repository, says go from a mission's page and reads the commit it left beside the one `atelier ready <mission> --by <name>` leaves in a second clone.
- [ ] The validator opens the mission page and the inbox at a phone width and a desktop width and says whether anything is unreadable, overflows or errors, and whether a thread's link from a page opens T3 Code.
- [ ] A receipt for each criterion records the commit the service runs, the commands and their results. A criterion that does not hold files a linked bug and is reported to the orchestrator; it is not fixed by the validator.

## Context

Run on the base once atelier-va99 has merged and the orchestrator has updated the service from `master` by the document's steps. First check that the commit the service reports is `master`'s merge of atelier-va99 or later, and record it; if it is not, stop and tell the orchestrator. Use the `validate` procedure. The commands and how to run them are in `desk/README.md` and `docs/architecture/desk.md`. Act only on scratch repositories under `~/.cache/atelier-desk/scratch/`, never on a live repository or the served clones, and start no T3 thread.
