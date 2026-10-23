---
created_at: "2026-10-23T00:06:54.647091599+00:00"
id: "atelier-aufe"
issue_type: "task"
labels:
- "review"
workflow_branch:
  branch_base: master
  work_branch: task/atelier-aufe
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
  - kind: "issue"
    id: "atelier-prjr"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "State Agent Factory's one review standard for code and prose"
updated_at: "2026-10-23T00:07:19.850566144+00:00"
---

## Goal

On 2026-10-22 the human decided review keeps one standard: every non-trivial change, prose included, gets independent review, and only the trivial exemption (correctness fully evident from the diff) skips it; tracker records follow the tracker's own rules. Atelier enforces this, but the vendored Agent Factory skill still limits review to code: SKILL.md (core rule, scaling table, Independent Review: "Prose-only documentation is not code"), constitution.md, procedures/orchestrate.md, procedures/implement.md and maintainers/skill-evaluation.md. An agent following the skill skips review on non-trivial prose that land then refuses. Found by acceptance validation of atelier-k8h8 (atelier-prjr, receipt atelier-om3m, S8 fail).

## Done when

Every statement in .agents/skills/agent-factory/ that limits independent review to code says non-trivial changes instead, with prose (docs, ADRs, plans) named as needing review and tracker records left to the tracker's rules; the trivial exemption is unchanged. The Constitution amendment cites the human's decision. `grep -rn 'non-trivial code\|is not code' .agents/skills/agent-factory` finds only code-specific guidance (good-code, review's code reading). The scenario evals in the upstream repository are named as not run.

## Notes

- 2026-10-22 [decision] State one review standard by extending 'what needs review' to prose and leaving the trivial exemption and code-specific guidance (good-code, review's code reading) as they are
  Reason: The human's 2026-10-22 decision: review keeps one standard for code and prose; only the trivial exemption skips it. The smallest change that removes every code-only limit
  Rejected: A separate review budget for prose (superseded atelier-1mn2)
- 2026-10-22 [note] Scenario evals in spadaval/.agents evals/agent-factory were not run for this wording change; the change follows a human decision rather than an observed agent failure, and the skill-evaluation method's baseline step would only confirm agents currently skip review on prose. Limit stated for review.
