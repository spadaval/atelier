---
created_at: "2026-10-16T19:38:32.522240674+00:00"
id: "atelier-qohr"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-qohr
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Drop prune retention from the config refusal wording"
updated_at: "2026-10-16T20:08:36.279059088+00:00"
---

## Goal

The land/merge config refusal says gates read custom link types and prune retention from config.toml; only custom_context_types reaches a gate, and prune retention is read only by atelier prune.

## Done when

The message and comment in crates/atelier-cli/src/commands/git.rs (require_committed_config) name only the custom link types. atelier-v44v's Design item 1 is corrected in this record rather than edited (see Design).

## Design

atelier-v44v's Design item 1 is left as it is: editing a done record's body counts as a new completion claim (`completion_claim_matches` compares the whole body, atelier-jy9t), which reopened atelier-v44v's gates and failed land, even after a revert. The correction lives here instead: gates read only custom context link types from config.toml; prune retention is read only by `atelier prune`.

## Context

Deferred finding atelier-vwez from the atelier-v44v re-review (approval atelier-0whm).
