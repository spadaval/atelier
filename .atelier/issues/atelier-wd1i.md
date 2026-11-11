---
created_at: "2026-11-10T00:56:09.915045297+00:00"
id: "atelier-wd1i"
issue_type: "task"
labels:
- "redesign"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-5zre"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-oci2"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-qr5s"
    type: "advances"
  - kind: "issue"
    id: "atelier-scx2"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-t747"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-u1ko"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Run the cutover on this host"
updated_at: "2026-11-11T04:46:58.576919592+00:00"
---

## Goal

Make the change of tool once, on this host, by running the reviewed runbook. Nothing in this item is decided on the day: the judgment was reviewed in atelier-u1ko, and this item is the two commands, what must be true before them, and what shows they worked.

## Done when

- [ ] **Sumeet named the time**, and it is recorded as a dated note on mission atelier-qr5s before anything below starts.
- [ ] **Quiet first.** At that time no item is in progress and no other thread is running in atelier, astromech, demiurge, life-on-rails or cosplay-studio, and each checkout is clean with its base at its remote's tip. The runbook's preflight checks this and stops otherwise; a stop here is not worked around.
- [ ] **The switch is merged from its own checkout**, with `atelier merge atelier-u1ko --push --wait --timeout 9` under the installed old binary, run again if CI is still pending. If master gained code since the switch's CI ran, bringing it in (`atelier merge --sync`) and its CI run come first, and a sync that conflicts stops the cutover before it starts.
- [ ] **`scripts/cutover/cutover.sh` ran to its end on this host**, straight after that merge, from the checkout that has `master`. Its closing report is kept in the mission's Result: for each of the five repositories the conversion commit, the count of items and missions, `--verify` passing, and the count of relative links that still resolve.
- [ ] **Afterwards, checked by hand and not only by the script:** `atelier --version` is the new tool; `atelier check` passes in each of the five real checkouts; each base is at its remote's tip; `atelier status` in atelier names the hooks as installed; the skill every agent loads is the new one (`skills/agent-factory/SKILL.md` in the skill checkout is the new text); `atelier-serve` is stopped and disabled; and CI on atelier's `master` passed on the conversion commit with the tree's binary.
- [ ] **This item was closed with `atelier close`**, the first act the new tool recorded, by the runbook's last step.
- [ ] **If any step failed:** `scripts/cutover/rollback.sh` was run, the checks the rollback trials used pass (each base and hooks directory as recorded, the fallback release installed and passing `check`, the skill checkout back, the old site answering), and the mission gets a dated note saying which step failed and what it showed. The cutover is then not retried in the same sitting: the failure is diagnosed first.

## Scope

This item changes no file by hand. The runbook's conversion commits are the only direct pushes, and the mission's Authority allows them. It is never started: do not run `atelier start` on it, since the runbook stops while any item is in progress, and `atelier close` ends an item that was never started. It runs on the base.

Not this item's: fixing what first use breaks (atelier-2mlh), removing the fallback branch of CI's `check` step (atelier-2mlh), the docs (atelier-6w8d), and the work host, which Sumeet converts later with the same runbook and which must not fetch a converted repository before then.

## Context

Mission atelier-qr5s, step G of `docs/strategies/redesign-3-cutover.md` (Shape: "the window is short, and Sumeet opens it"; Authority; Risks). Protocol: `docs/strategies/redesign-architecture.md`, Transition, part 4, with its Rollback. The page to read on the day is `scripts/cutover/README.md`, which atelier-u1ko writes: what must be quiet, the commands, how long each step took in the sandbox, and when to roll back.

Between the switch's merge and the end of the runbook, the tree's binary cannot read this repository's records and a reinstall from the tree would break every repository on the host. Nothing else is started in that window, and it is one thread's work from the merge to the last check.

From the cutover until the Desk's mission page is live, Sumeet reads a mission with `atelier show <mission>` and its files; there is no brief or debrief page.

## Unknowns

- When no work is in flight in any of the five repositories and no other thread is running in them. Sumeet's naming the time settles it.
