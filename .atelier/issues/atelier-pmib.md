---
created_at: "2026-11-08T05:52:14.422031833+00:00"
id: "atelier-pmib"
issue_type: "task"
labels:
- "agent-factory"
workflow_branch:
  branch_base: master
  work_branch: atelier-pmib
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-g6rp"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-08T07:58:23.405675023+00:00"
status: "done"
title: "Record in an ADR why Agent Factory's one copy moves here, superseding ADR 0023"
updated_at: "2026-11-08T07:58:23.405675023+00:00"
---

## Goal

Record why the skill moves here, so later readers do not read ADR 0023's "not vendored" as forbidding it. ADR 0023 rejected a second copy; this moves the single copy.

## Done when

- [x] A new ADR in `docs/adr/` (next free number) states: the skill's only copy is `skills/agent-factory` in this repository, with its evals under `evals/agent-factory`; it is installed with `npx skills add spadaval/atelier --skill agent-factory -g`, unpinned; the binary carries no copy and makes no version promise, and `atelier doctor` only warns when either is stale; `atelier setup --link` links the installed skill to a checkout for editing; the skill steps aside outside Atelier repositories.
- [x] It supersedes ADR 0023 in full, says why ADR 0023's reasoning (one copy) still holds, and lists the alternatives considered: keep spadaval/.agents as the source, ship the skill inside the binary or with `atelier setup` (ADR 0020's deferred option, now closed: rejected, since it locks versions together), and a custom installer (rejected for the `skills` CLI).
- [x] It retires ADR 0020 Decision 3's rule that a command change updates the upstream skill in the same piece of work (they now share one commit), and keeps its boundary that the skill maps concepts while Atelier's help owns syntax. It replaces Decision 1's plan-file fallback for repositories without Atelier with "the skill steps aside". ADR 0020's and 0023's Status sections name the new ADR.
- [x] `docs/index.md` lists it.

## Scope

`docs/adr/` and `docs/index.md`. No link or path changes elsewhere; the import item moves those with the skill.

## Context

The mission strategy, docs/strategies/agent-factory-in-atelier.md, has the reasons. Sumeet directed the move on 2026-11-08.

## Report

**Outcome:** ADR 0029 records the move of Agent Factory's one copy into this repository and supersedes ADR 0023.

**Changes:** `docs/adr/0029-agent-factory-lives-in-atelier.md` (new). Its six decisions cover the location, install with `npx skills add spadaval/atelier --skill agent-factory -g`, no version promise plus doctor's freshness warning, `setup --link`, stepping aside outside Atelier, and the process/records boundary. It supersedes ADR 0023 in full (the one-copy rule stands), ADR 0020 Decision 3's sync rule (its concepts-versus-syntax boundary stays) and Decision 1's plan-file fallback. Alternatives considered: keep spadaval/.agents, ship in the binary or with `setup` (ADR 0020's deferral, closed as rejected), a custom installer, and vendoring. ADR 0020's and 0023's Status sections name ADR 0029, and `docs/index.md`'s Agent Factory entry cites it. The entry's link still points at spadaval/.agents, because the skill is still there; atelier-dkul moves links.

**Proof:** `git diff --check` clean. `grep -rn 'adr/0029\|(0029' docs` finds the three references, and the target file exists. ADR 0029 was confirmed unclaimed on every local branch before writing.

**Review:** pending.

**Open items:** none.
