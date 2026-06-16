---
created_at: "2026-06-13T04:18:41.293399805+00:00"
id: "atelier-5rat"
evidence_type: "review"
captured_at: "2026-06-13T04:18:41.293368705+00:00"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-dxy1"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Agent Factory bounded-proof delegation template review"
updated_at: "2026-06-13T04:19:14.951592497+00:00"
---

Agent Factory bounded-proof delegation template review.

File-change review:

- `/root/.agents/skills/agent-factory/procedures/orchestrate.md` now requires
  one role/subskill, exact tracker IDs, active mission and parent epic, owned
  files/workflows, out-of-scope boundaries, expected proof, evidence
  destination, independence requirement, model choice and rationale, dirty
  worktree rule, and a compact final handoff schema.
- `/root/.agents/skills/agent-factory/procedures/orchestrate.md` now directs
  orchestrators to delegate bounded scout, audit, validation,
  transcript-capture, fixture-review, and docs-drift work early, and requires
  durable evidence, tracker notes, or blocker/follow-up updates instead of
  prose-only private summaries.
- `/root/.agents/skills/agent-factory/procedures/implement.md` now mirrors the
  required handoff schema: result, issue ID, subskill, changed files, evidence
  IDs, commands run, dirty state, branch/commit, blockers, and exact follow-up
  recommendation.

Weak `atelier-tcmr` pattern compared:

- `docs/architecture/quality/mission-log-insights.md` records that long
  `atelier-tcmr` controller threads performed many bounded implementation,
  validation, transcript capture, and fixture-review loops directly. That
  pattern let the main agent collect proof in private context instead of giving
  each bounded slice an evidence-producing assignment.
- The revised template would have exposed the proof gap earlier because each
  delegated slice must name the exact issue ID, owned workflow, expected
  observable proof, evidence destination, independence requirement, and final
  handoff schema before work starts. Missing proof would therefore show up as
  an absent evidence ID, missing command transcript, or explicit blocker in the
  worker handoff instead of being discovered during closeout.

Dogfood handoff using the required schema:

```text
result: pass - Agent Factory delegation template now requires bounded proof
  assignments and evidence-producing handoffs.
issue ID: atelier-dxy1
subskill: implement
changed files:
  /root/.agents/skills/agent-factory/procedures/orchestrate.md
  /root/.agents/skills/agent-factory/procedures/implement.md
evidence IDs: atelier-5rat
commands run:
  sed -n '1,240p' /root/.agents/skills/agent-factory/SKILL.md
  sed -n '1,260p' /root/.agents/skills/agent-factory/procedures/implement.md
  atelier issue show atelier-dxy1
  atelier mission show atelier-19mc
  atelier mission status atelier-19mc
  atelier issue update atelier-dxy1 --claim
  atelier start atelier-dxy1
  atelier rebuild
  git diff --check -- '*.md'
  git -C /root/.agents diff --check -- skills/agent-factory/procedures/orchestrate.md skills/agent-factory/procedures/implement.md
dirty state: shared checkout was dirty before this work; unrelated repo changes
  were preserved. This slice adds .atelier/evidence/atelier-5rat.md and tracker
  updates for atelier-dxy1.
branch/commit: branch unknown from evidence capture; no commit made.
blockers: atelier start atelier-dxy1 was blocked by pre-existing dirty source
  files, so tracked start could not be recorded safely.
exact follow-up recommendation: run atelier-29yn to dogfood this policy on a
  bounded multi-agent run with at least one real subagent handoff.
```
