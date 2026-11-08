---
created_at: "2026-11-08T05:52:14.422031833+00:00"
id: "atelier-pmib"
issue_type: "task"
labels:
- "agent-factory"
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
status: "todo"
title: "Record in an ADR why Agent Factory's one copy moves here, superseding ADR 0023"
updated_at: "2026-11-08T05:52:14.422031833+00:00"
---

## Goal

Record why the skill moves here, so later readers do not read ADR 0023's "not vendored" as forbidding it. ADR 0023 rejected a second copy; this moves the single copy.

## Done when

- [ ] A new ADR in `docs/adr/` (next free number) states: the skill's only copy is `skills/agent-factory` in this repository, with its evals under `evals/agent-factory`; it is installed with `npx skills add spadaval/atelier --skill agent-factory -g`, unpinned; the binary carries no copy and makes no version promise, and `atelier doctor` only warns when either is stale; `atelier setup --link` links the installed skill to a checkout for editing; the skill steps aside outside Atelier repositories.
- [ ] It supersedes ADR 0023 in full, says why ADR 0023's reasoning (one copy) still holds, and lists the alternatives considered: keep spadaval/.agents as the source, ship the skill inside the binary or with `atelier setup` (ADR 0020's deferred option, now closed: rejected, since it locks versions together), and a custom installer (rejected for the `skills` CLI).
- [ ] It retires ADR 0020 Decision 3's rule that a command change updates the upstream skill in the same piece of work (they now share one commit), and keeps its boundary that the skill maps concepts while Atelier's help owns syntax. It replaces Decision 1's plan-file fallback for repositories without Atelier with "the skill steps aside". ADR 0020's and 0023's Status sections name the new ADR.
- [ ] `docs/index.md` lists it.

## Scope

`docs/adr/` and `docs/index.md`. No link or path changes elsewhere; the import item moves those with the skill.

## Context

The mission strategy, docs/strategies/agent-factory-in-atelier.md, has the reasons. Sumeet directed the move on 2026-11-08.
