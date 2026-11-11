---
created_at: "2026-10-17T01:10:57.970791265+00:00"
id: "atelier-fb1i"
issue_type: "bug"
labels:
- "check"
- "cli-output"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-uf97"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T03:55:33.148267581+00:00"
status: "superseded"
title: "Show graph errors in atelier check even when a record fails to parse"
updated_at: "2026-11-11T03:55:33.148267581+00:00"
---

## Goal

With a YAML error in one record (atelier-wtnp, `status: todo: bad`) and an `advances` link to a missing `atelier-nope` in another (atelier-gfi8), `atelier check` reports only the YAML error. The missing reference appears only after the YAML is fixed and check is rerun. The output never says graph checks were skipped. atelier-idae (done) made check report every independent graph error, but only once every record parses. The Agent Factory skill's Atelier reference (`skills/agent-factory/references/atelier.md` in spadaval/.agents, Administration) says check "stops at the first structural error", which matches neither idae's claim nor this behaviour.

## Done when

- [ ] That two-fault fixture reports both problems in one run. If graph checks cannot run safely, check instead prints `Graph checks skipped: fix the records above first`.
- [ ] A test covers the two-fault fixture.

## Context

Extends atelier-idae.

## Report

Dropped in the cutover sort (atelier-5zre), reason (b). The new binary's `check` already reports a record that does not parse and the other records' missing references in one run (run in a scratch clone: one unparseable item, one unknown mission and one unknown blocker gave three problems in one report).
