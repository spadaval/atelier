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
updated_at: "2026-11-08T21:02:40.598319646+00:00"
---

## Goal

Give Agent Factory a gardener (`procedures/garden.md`) that tends the conditions agents work in, not the product. It finds where agents trip, from their transcripts and the code, and fixes the ground with guardrails, weeding and better signage. The human gardens too: the agent does the bulk reading and the routine upkeep, and brings the human anything that takes taste as a recommendation to triage.

## Done when

- [ ] Boundary: garden.md says the gardener changes how agents work here (code structure without behaviour change, checks, docs, error and help text, skills, tooling), never what the product does, and sends a product question it meets to the plan's owner as an issue.
- [ ] Finding trips: a run reads agent transcripts and the code, plus review records, reverts and failed checks. It looks at what is new since its last run (the last 7 days on a first run) and acts on a trip seen at least twice in the last 30 days. Its transcript method works for any harness: start from raw transcripts, not summaries or memory; check the transcript around each claimed failure and handoff; follow subagent and thread lineage to the root.
- [ ] Fix level: each trip is fixed at the highest level that works: architecture (make it impossible), then a check, then a sign where the agent trips (an error message or help text before a rule in a distant doc). Guardrails are nudges: a better default or message comes before a new gate, and removing a rule whose mistake can no longer happen counts as much as adding one.
- [ ] Triage: garden.md splits the gardener's changes into routine upkeep and recommendations, with examples of each. Routine upkeep is a change whose correctness is evident from the diff and that adds or removes no rule, check, gate, interface or default, at most about 100 changed lines (dead code, residue, a doc line the code contradicts, a broken link): the gardener makes it, has it reviewed by an agent like any change, and merges it. Everything else, and anything the gardener is unsure of, is a recommendation.
- [ ] Recommendations reach the human as one digest per run and repository: an item labelled `needs-human` whose entries each give the trip with its transcript or code evidence, the proposed fix and its level, its size and the alternatives. The human answers each entry (do it, do it differently, no) in a note on that item. The next run reads the answers: accepted entries become issues, it adds to an unanswered digest instead of filing another, and it does not propose a declined recommendation again without new evidence.
- [ ] A run covers one repository, never touches a branch in flight, and reports the window, the trips found, the routine fixes merged, the recommendations made, and the rules and checks added and removed.
- [ ] `references/verify-skill.md`, moved from atelier-l58v (agents-skills 9f775bb), sets the verify-skill standard. The gardener keeps a repository's verify skill working when agents trip on it; authors keep its feature map current with their features.
- [ ] SKILL.md lists `garden`, and garden's read path stays within atelier-qj9d's 300 lines, counted by its rule; the Report gives the count.
- [ ] Trial: a fresh agent given only the skill runs the gardener on this repository, and its output is read by hand. Each trip it reports is checked against its transcript, no change it calls routine should have been a recommendation, and its digest is in the Report. The trial merges and files nothing.

## Scope

`skills/agent-factory` in this repository, after atelier-dkul moves it here, and this item's records. Not the daily schedule (atelier-jy8k).

## Context

Agreed with Sumeet in conversation on 2026-11-08. atelier-l58v's draft gardener (agents-skills 9f775bb, `procedures/garden.md`) mixed in product work: sweeping the backlog and judging whether merged changes were worth their cost. Those went to plan.md's backlog sweep and review's verdict, and the gardener moved here. Gardening is also Sumeet's job, and much of it takes taste, so the agent reads and does routine upkeep and brings the rest as recommendations. atelier-9s9i folds codex-reflect's transcript method into audit.md first; this item moves or links it, whichever fits.

## Unknowns

- Where each harness on this host (Claude Code, Codex, T3 Code) keeps transcripts, and how much a run can read in one session.
