---
created_at: "2026-10-16T19:38:32.522240674+00:00"
id: "atelier-qohr"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Drop prune retention from the config refusal wording"
updated_at: "2026-10-16T19:38:32.522240674+00:00"
---

## Goal

The land/merge config refusal says gates read custom link types and prune retention from config.toml; only custom_context_types reaches a gate, and prune retention is read only by atelier prune.

## Done when

The message and comment in crates/atelier-cli/src/commands/git.rs (require_committed_config) and atelier-v44v's Design item 1 name only the custom link types.

## Context

Deferred finding atelier-vwez from the atelier-v44v re-review (approval atelier-0whm).
