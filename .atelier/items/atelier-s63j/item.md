---
schema: atelier.item
schema_version: 3
id: atelier-s63j
title: Strip the old tracker's process text from replay fixtures of any era
status: ready
mission: atelier-kw8u
labels: []
priority: medium
blocked_by: []
implementers: []
ready:
  by: 'the lead of atelier-kw8u: both change what the replayed reviews measure, so they come before the matrix'
  at: 2026-11-19T02:52:02.027431283Z
trivial: null
caused_by: null
created_at: 2026-11-19T02:50:32.559838815Z
---

## Goal

Taken from deferred finding atelier-z8rg/1-yrne.1, in .atelier/items/atelier-z8rg/reviews/1-yrne.md:

> Finding 1: Remove retained historical process instructions in a shared-setup follow-up
> Recommendation: defer
>
> `evals/agent-factory/README.md:123-128` correctly discloses that hierarchy-rules and priority-contract retain old process guidance. I verified their base trees: hierarchy-rules' AGENTS.md explicitly requires the removed `.agents/skills/agent-factory/SKILL.md` and obsolete tracker commands, while priority-contract also loads AGENTFACTORY.md with its own workflow instructions. `replay.strip_process` only removes three differently named headings, so these instructions survive in the fixtures. The retained priority-contract result records a failed `atelier issue show atelier-zivs` against today's tracker, consistent with the Report.
>
> Equal exposure in both arms does not remove the competing workflow instructions or the extra process guidance in the none arm. Extend the shared sanitization in a separate item and protect it with fixture assertions before treating these two cases as clean measurements of the current skill's contribution. This is deferred because changing the shared setup is explicitly outside this item's scope, the limitation is disclosed, and the recorded run still completes its independent review.

## Done when

- [ ] A replay fixture holds none of the old tracker's process text, whatever headings its base's AGENTS.md uses, and no root-level guidance file of that tracker, such as `AGENTFACTORY.md`.
- [ ] `replay_tests.py` asserts this for every replay case, and `hierarchy-rules` and `priority-contract` pass it.
- [ ] The evals README no longer needs its note that those two cases show the old process.

## Scope
