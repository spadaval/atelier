---
created_at: "2026-10-30T06:18:25.809733950+00:00"
id: "atelier-svi6"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-svi6
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-30T06:25:48.677013319+00:00"
status: "done"
title: "Stop vendoring the Agent Factory skill; agents load the installed copy"
updated_at: "2026-10-30T06:25:48.677013319+00:00"
---

## Goal

Stop vendoring the Agent Factory skill in this repository. The human decided (2026-10-30) that the skill has exactly one git-tracked copy, in spadaval/.agents, and agents load the installed copy. The vendored `.agents/skills/agent-factory` has already drifted from it, and copies that drift silently undo the human's decisions.

## Done when

- `.agents/skills/agent-factory` no longer exists, and `git grep` finds no live reference to it outside historical records (closed issues, review and evidence records, superseded strategies).
- `docs/adr/0023-*.md` records the decision and supersedes ADR 0020 decision 2, and ADR 0020's status says so.
- AGENTS.md and the product and architecture docs point at the installed skill or its upstream repository instead of the vendored path.
- `atelier review request` no longer names a repository path for the review procedure that the repository does not contain; the brief-rendering unit test covers the new text and fails against the old.
- `cargo fmt -- --check`, `cargo nextest run` and `atelier check` pass.

## Scope

Docs, ADRs, AGENTS.md, the active r6bt strategy's guidance line, and `review_request.rs` with its test. Closed records and superseded strategies keep their historical paths.

## Context

- ADR 0020 decision 2 (`docs/adr/0020-agent-factory-targets-atelier-only.md`).
- Canonical skill: `~/projects/agents-skills/skills/agent-factory` (spadaval/.agents). Its commit bee1476 carries the vendored copy's unsynced 10-23 guidance.

## Notes

- 2026-10-30 [decision] atelier review request no longer names a review procedure path
  Reason: With no vendored copy, Atelier cannot know where a reader's skill is installed; a repository path it does not contain would mislead, and the brief already carries the requirements and range
  Rejected: Point at ~/.agents/skills/agent-factory, which ties the CLI to one install layout
  Rejected: Always name Agent Factory's review procedure, which is wrong in repositories without the skill

## Close Reason

The repository no longer vendors Agent Factory: `.agents/skills/agent-factory` is removed, ADR 0023 supersedes ADR 0020 decision 2, AGENTS.md and the docs point at the installed skill or `spadaval/.agents`, and `atelier review request` no longer names the vendored review procedure.

### Deviations
Open issues atelier-0xcr, atelier-fb1i and atelier-pg2b also cited the vendored path; review finding atelier-ul8r had them repointed at the canonical skill. Closed records and the superseded adaptive-missions strategy keep their historical paths.

### Decisions
See this item's decision note on the review brief's procedure line.

### Surprises
The local spadaval/.agents checkout is several commits ahead of GitHub (bee1476 and later are unpushed), so the new GitHub links show an older skill until it is pushed.

### Open items
None in this repository. Pushing spadaval/.agents is for the thread that owns that work.
