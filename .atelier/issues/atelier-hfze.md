---
created_at: "2026-10-17T01:10:57.972320540+00:00"
id: "atelier-hfze"
issue_type: "feature"
labels:
- "cli-output"
- "policy"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-nn9y"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Add a view of the workflow policy"
updated_at: "2026-10-17T01:10:57.972320540+00:00"
---

## Goal

No command shows which statuses, transitions, checks, required close fields, merge checks and base branch apply to each issue type. Required body sections are not in the policy at all: they are hard-coded and documented only in `atelier issue create --help` (`--body`). docs/product/workflow-configuration.md tells users to "Inspect actual policy", but the only route is the YAML. In Claude Code transcripts for atelier and astromech (excluding audit sessions), agents read `.atelier/workflow.yaml` through the shell 95 times, e.g. `cat .atelier/workflow.yaml | head -80` and `sed -n 80,200p .atelier/workflow.yaml`. Check names such as `blockers.transitive_none_open`, `work.terminal` and `decision.recorded` are explained nowhere in the CLI.

## Done when

One command (name to decide: `atelier policy` or `atelier help workflow`) prints, per workflow, one line per transition, with the checks in plain words and the body sections per type. Sketch:

```
work (bug, feature, spike, task, validation) · starts todo · done: done, superseded
  start    todo|blocked → in_progress   no open blockers, incl. transitive
  close    in_progress|review → done    --reason · no open blockers · children done
  defer    todo|blocked → backlog
  body: Goal, Done when (+ Scope, Context, Unknowns, Design, Background)
merge: fast_forward_only into master · shell: git diff --check HEAD^ HEAD
```

- When the working policy is invalid, it reports the fallback to the last valid policy.
- A snapshot test runs it on the starter policy from `atelier init`.

## Unknowns

Command name, and whether validator descriptions come from the policy file or from built-in text for each check.
