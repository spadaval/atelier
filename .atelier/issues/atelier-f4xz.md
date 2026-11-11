---
created_at: "2026-11-07T15:11:37.638954318+00:00"
id: "atelier-f4xz"
issue_type: "bug"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T03:55:50.936929387+00:00"
status: "declined"
title: "Judge a work branch's records against the merged result, not the base's policy alone"
updated_at: "2026-11-11T03:55:50.936929387+00:00"
---

## Goal

On a work branch, `atelier merge --check` and `atelier land` validate the branch's own records against the base's policy. When the base changes its policy to drop a status that a record on an in-flight branch still uses, every such branch refuses with `workflow_issue_status_invalid` until it merges the base in, even when the merge would leave the record valid (found reviewing astromech-zk7z: astromech-278z in `backlog` on three branches). Judge records the way the merge would leave them.

## Done when

- [ ] A test: the base drops a status and moves a record out of it; an in-flight branch that edited the same record merges, and `merge --check` and `land` on that branch report no status error when the merged record is valid.
- [ ] A record the merge would leave invalid is still refused.

## Context

atelier-nn67 avoided this by keeping `backlog` on the on-base workflow. crates/atelier-cli/src/commands/merge.rs, session.rs.

## Report

Dropped in the cutover sort (atelier-5zre), reason (a). Its case is a base policy dropping a status, and the What is removed table removes `workflow.yaml`; the new lifecycle is fixed in the binary.
