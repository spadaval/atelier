---
created_at: "2026-11-08T21:02:40.598319646+00:00"
id: "atelier-3sk9"
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
    id: "atelier-9s9i"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Add a gardener: agents fix routine mess, and bring the human the judgment calls"
updated_at: "2026-11-08T21:06:32.850458660+00:00"
---

## Goal

Give Agent Factory a gardener (`procedures/garden.md`) that tends the conditions agents work in, not the product. It finds where agents trip, from their transcripts and the code, and fixes the ground with guardrails, weeding and better signage. The human gardens too: the agent does the bulk reading and the routine upkeep, and brings the human anything that takes taste as a recommendation to triage.

## Done when

- [ ] Boundary: garden.md says the gardener changes how agents work here (code structure without behaviour change, checks, docs, error and help text, skills, tooling), never what the product does, and sends a product question it meets to the plan's owner as an issue.
- [ ] Finding trips: a run reads agent transcripts and the code, plus review records, reverts and failed checks. It looks at what is new since its last run (the last 7 days on a first run). A trip in the transcripts earns a new rule, check or gate only once it has happened at least twice in the last 30 days; mess found by reading the code (dead code, residue, a broken link) needs no repeat. Its transcript method works for any harness: start from raw transcripts, not summaries or memory; check the transcript around each claimed failure and handoff; follow subagent and thread lineage to the root.
- [ ] Fix level: each trip is fixed at the highest level that works: architecture (make it impossible), then a check, then a sign where the agent trips (an error message or help text before a rule in a distant doc). Guardrails are nudges: a better default or message comes before a new gate, and removing a rule whose mistake can no longer happen counts as much as adding one.
- [ ] Triage: garden.md splits the gardener's changes into routine upkeep and recommendations, with examples of each. Routine upkeep is a change whose correctness is evident from the diff and that adds or removes no rule, check, gate, interface or default, at most about 100 changed lines (dead code, residue, a doc line the code contradicts, a broken link): the gardener makes it, has it reviewed by an agent like any change, and merges it. Everything else, and anything the gardener is unsure of, is a recommendation.
- [ ] Recommendations reach the human as one digest per run and repository: an item labelled `needs-human` whose entries each give the trip with its transcript or code evidence, the proposed fix and its level, its size and the alternatives. The human answers each entry (do it, do it differently, no) in a note on that item. The next run reads the answers: accepted entries become issues, it adds to an unanswered digest instead of filing another, and it does not propose a declined recommendation again without new evidence.
- [ ] A run covers one repository, never touches a branch in flight, and reports the window, the trips found, the routine fixes merged, the recommendations made, and the rules and checks added and removed.
- [ ] `references/verify-skill.md` sets the verify-skill standard, starting from atelier-l58v's draft, which the Design section below copies. The gardener keeps a repository's verify skill working when agents trip on it; authors keep its feature map current with their features.
- [ ] SKILL.md lists `garden`, and garden's read path stays within atelier-qj9d's 300 lines, counted by its rule; the Report gives the count.
- [ ] Trial: a fresh agent given only the skill runs the gardener on this repository, and its output is read by hand. Each trip it reports is checked against its transcript, no change it calls routine should have been a recommendation, and its digest is in the Report. The trial proposes removing at least one rule or check, or says why it found none. The trial merges and files nothing.

## Scope

`skills/agent-factory` in this repository, after atelier-dkul moves it here, and this item's records. Not the daily schedule (atelier-jy8k).

## Context

Agreed with Sumeet in conversation on 2026-11-08. atelier-l58v's draft gardener (agents-skills 9f775bb, `procedures/garden.md`) mixed in product work: sweeping the backlog and judging whether merged changes were worth their cost. Those went to plan.md's backlog sweep and review's verdict, and the gardener moved here. Gardening is also Sumeet's job, and much of it takes taste, so the agent reads and does routine upkeep and brings the rest as recommendations. atelier-9s9i folds codex-reflect's transcript method into audit.md first; this item moves or links it, whichever fits.

## Unknowns

- Where each harness on this host (Claude Code, Codex, T3 Code) keeps transcripts, and how much a run can read in one session.

## Design

atelier-l58v's draft verify-skill standard, copied from spadaval/.agents `git show 9f775bb:skills/agent-factory/references/verify-skill.md` (an unpushed branch whose next commit deletes it):

> A repository's verify skill lets any agent prove that the app works, not only
> that its tests pass. It is a skill in the repository (for example
> `.agents/skills/verify/SKILL.md`) built on two things:
>
> - **A control CLI** the agent drives the app with: commands that start it, act
>   on it as a user would (open a page, send a request, tap a button) and read its
>   state as text. Output an agent must interpret by eye, such as a screenshot,
>   is a last resort.
> - **A feature map:** each thing the app does, and the command or scenario that
>   proves it works. A change names the features it touches, and its proof runs
>   their entries.
>
> Its sections:
>
> - **Launch:** start the app from a clean checkout with its dependencies in one
>   command, and how to tell it is ready.
> - **Doctor:** check the environment first (tools, ports, credentials, devices)
>   and say how to fix each failure, so a broken setup is not taken for a broken
>   change.
> - **Drive:** each control CLI command, with an example.
> - **Evidence:** what to capture as proof (command output, screenshots,
>   recordings) and where, so a reviewer can check it.
> - **Cleanup:** stop what Launch started and remove what the run made.
>
> Nobody relies on it until an agent has run it end to end from a clean checkout:
> Doctor, Launch, Drive of one feature, Evidence and Cleanup.
