---
created_at: "2026-06-13T17:29:11.074782112+00:00"
id: "atelier-z04a"
labels:
- "mission"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-9t3z"
    type: "advances"
  - kind: "issue"
    id: "atelier-fmri"
    type: "advances"
  - kind: "issue"
    id: "atelier-q5r6"
    type: "advances"
schema: "atelier.mission"
schema_version: 1
status: "closed"
title: "Implement repo-defined issue workflows"
updated_at: "2026-06-13T19:56:45.442000645+00:00"
---

## Intent

Implement repo-defined issue workflows so Atelier no longer advertises workflow policy that it does not enforce. The mission turns issue status into configured workflow state, uses a fixed tracked .atelier/workflow.yaml policy file, adds workflow init/check/migration commands, replaces hardcoded start/close/readiness checks with configured transitions and built-in validators, and preserves agent orientation through category, status, transition, blocker, and next-action output.

## Constraints

- V1 workflow scope is issues only; mission, milestone, plan, and evidence lifecycles remain outside this mission.
- Use a fixed tracked `.atelier/workflow.yaml` policy file.
- Keep built-in issue types in v1; custom issue types, custom validators, expressions, hooks, triggers, post-functions, waivers, and workflow projection tables are deferred.
- Do not add compatibility aliases, staged deprecations, fallback readers, or old-command shims unless a human explicitly asks for them.

## Risks

- Changing issue status semantics touches ready queues, closeout, mission summaries, import/export, lint, and normal agent orientation.
- Over-strict starter policy could add red tape; under-strict policy could preserve the current proof and status ambiguity.
- Removing `finish` and `workflow validate` is intentionally breaking and requires docs/help/Agent Factory drift cleanup.

## Validation

- `atelier-09sx` is resolved by implemented, enforced repo-defined workflow behavior rather than docs-only wording.
- Workflow init, check, migration, transition execution, close gating, ready queue, status surfaces, and abandon behavior have focused tests or transcripts.
- `CONTEXT.md`, product docs, and an ADR capture the workflow-state terminology and architecture decisions.
- Independent validation classifies starter policy, migration, start, blocked transition, close with evidence, lightweight spike close, archive, missing YAML, and unmigrated-record failures.
- Mission closeout maps validation criteria to linked work and attached evidence, then records final `atelier lint`, `atelier export --check`, `atelier doctor`, focused workflow tests, and `git diff --check` proof.
