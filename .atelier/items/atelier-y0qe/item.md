---
schema: atelier.item
schema_version: 3
id: atelier-y0qe
title: Say in the skill which roles work with the human (plan, garden, groom, spike) and which run unattended
status: draft
mission: null
labels:
- agent-factory
priority: high
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-11T00:10:02.965394078Z
---

## Goal

The skill should say which roles work with the human and which run unattended. A mission is a large unit of work that runs unattended. The human's part is to plan it and to keep the ground such that it can succeed, so `plan`, `garden` and `groom` are done with the human, and every other role runs without waiting on one. Work whose goal is not known until something is seen fits neither, and needs a third mode, the spike.

Today `SKILL.md` lists twelve subskills alike. `plan` implies a conversation without saying so. `garden` and `groom` are written as batch jobs that run alone and leave a digest in a queue, where 6 of 63 entries have been answered.

## Done when

- [ ] `SKILL.md` lists the subskills in groups, each with one sentence on what its mode means. With the human: `plan`, `garden`, `groom` and `spike`; the agent shows drafts, numbers and tradeoffs, and waits for direction. Unattended: the rest; the agent never waits on the human, and escalates or aborts as `orchestrate` says.
- [ ] `plan.md` says planning is a conversation: the planner brings the human drafts, tradeoffs and questions as it goes, and does not hand over a finished plan to approve.
- [ ] `garden.md` and `groom.md` say a run has two halves: the agent does the bulk reading and routine fixes alone, then works through what it found with the human. The digest records what they agreed and what is still open. It is not a queue of entries to answer one by one.
- [ ] A new `procedures/spike.md` covers exploratory work with the human in the loop: when to use it (the goal is not known until something is built or seen), how light it is (a branch off the base and no items, with an agent's review before anything merges), and how it ends (a plan, a strategy amendment, or a discarded branch). It says when to stop and plan. `SKILL.md`'s line on small work, multi-step work and missions names it.
- [ ] atelier-jy8k is reworded to schedule only the reading and upkeep half of a gardener run, or declined.
- [ ] A fresh agent given only the new skill answers these with no contradiction: which roles may ask the human during the work, what an unattended role does with a question, what a spike produces, and what a gardener run brings the human.

## Scope

`skills/agent-factory`: `SKILL.md`, `plan.md`, `garden.md`, `groom.md` and a new `spike.md`. It changes guidance, so it comes after atelier-8ep4, which only moves text. The redesigned gardener's page (numbers, causes, at most three proposals) is atelier-9dku's; this item changes only who is present in today's procedure.

## Context

Sumeet's direction, 2026-11-10, in conversation while reviewing the deck spike (`spike/mission-control`). The reasoning: nothing in the middle of a mission waits on the human, which removes the main cause of stalls, and running unattended forces the plan to be explicit. The split protects correctness better than rightness, since whether the plan was right is checked only at approval and when the human reads the result. A spike is where rightness is found before a mission is planned. The deck itself is the example: it was redirected five times, each time by something built.

## Unknowns

- Whether `decide` and `install` belong with the human. Sumeet named `plan`, `garden` and `groom`; these two also end in a human's answer.
