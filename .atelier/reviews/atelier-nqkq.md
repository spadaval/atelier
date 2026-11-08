---
schema: atelier.review
schema_version: 2
id: atelier-nqkq
title: finding on atelier-9s9i @d6b69340
target: atelier-9s9i
identity: claude-code-reviewer-9s9i (opus-5.5)
commit: d6b69340389c36d87c3b4e0b578dc7fabac2ed9a
created_at: 2026-11-08T22:49:46.025718141Z
event:
  event: finding
  recommendation: fix_now
---

low: skills/agent-factory/procedures/audit.md:29-38 and the Report's 'Left out' list. Two harness-neutral rules from codex-reflect were dropped without being listed, against the Goal 'retiring it loses nothing' and the Done when line 'The Report lists what was left out and why'. (1) SKILL.md 'Evidence and Caveats': Git evidence is only what the transcript recorded; never present the repository's present branch or commit state as the session's history. A retrospective agent that runs git log now and reports it as what the run did gets silent wrong history. (2) SKILL.md 'Resolve Related Threads' / metrics.md: a created or forked thread is a related peer, not a parent; only subagent spawns are ancestry, and the primary thread is chosen from the task, not from links or recency. The new 'Follow subagent and thread links back to the root session' invites walking a peer thread up as if it were the root. Fix: add about two lines to Session Retrospectives covering both (path stays near 245 of 300), and list in the Report, with a reason, the general points still omitted: bounding the analysis window when the root spans several tasks, treating detected links and automatic failure/recovery signals as candidates until the transcript confirms them, and keeping mission outcome separate from operational rates.
