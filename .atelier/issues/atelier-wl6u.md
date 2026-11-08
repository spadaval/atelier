---
created_at: "2026-11-08T23:03:38.890123921+00:00"
id: "atelier-wl6u"
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
    id: "atelier-nz6a"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Port cross-repo-regression and vacuous-test-review to Atelier fixtures, so they test the skill again"
updated_at: "2026-11-08T23:03:38.890123921+00:00"
---

## Goal

Since atelier-nz6a the skill steps aside when `atelier status` says "Not an Atelier repository". The fixtures of `cross-repo-regression` and `vacuous-test-review` are not Atelier repositories, so their skill arm now runs without the skill. In the nz6a matrix both still passed, but only because each run read `procedures/review.md` in the same command as `atelier status` before stepping aside.

## Done when

- [ ] Both fixtures are Atelier repositories (`atelier init` in `setup.sh`), keeping each scenario's pressure: cross-repo-regression's shared tracker and sibling repository, and vacuous-test-review's vacuous test and false "it failed first" claim.
- [ ] Each runs 3 times on the skill arm with Opus, read by hand, with no run stepping aside, and the results are recorded in `evals/agent-factory/results/`.

## Scope

`evals/agent-factory/scenarios/cross-repo-regression`, `evals/agent-factory/scenarios/vacuous-test-review` and the eval README.
