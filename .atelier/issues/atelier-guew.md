---
created_at: "2026-10-17T01:10:57.969862662+00:00"
id: "atelier-guew"
issue_type: "bug"
labels:
- "bundle"
- "cli-output"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-2mky"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Make bundle preview run the same staged-graph validation as apply"
updated_at: "2026-10-17T01:10:57.969862662+00:00"
---

## Goal

`atelier bundle preview` runs only `load_bundle`, `validate_bundle` and `BundleGraphPlan::from_bundle` (crates/atelier-cli/src/commands/bundle.rs:15-20). The staged canonical check runs only in `apply_bundle_file` (:706-728). Preview therefore prints `Bundle preview is valid.` for bundles that apply then rejects. Reproduced in a scratch clone:

- Two new issues that are `blocked_by` each other: preview says valid; apply fails with `Error: Adding this dependency would create a circular dependency chain`.
- A new task with `"blocks": [{"id": "<mission in in_progress>"}]`: preview says valid; apply fails with `Bundle apply rejected the staged canonical state: workflow_mission_dependency_bypass ...`.

The documented loop is "inspect preview, then apply", so the word "valid" makes a promise preview does not check.

## Done when

- Both bundles above fail `atelier bundle preview` with the same error (code and text) that `bundle apply --yes` gives, and preview writes nothing.
- An integration test asserts that preview and apply reject the same staged-graph violations (at least cycle and mission dependency bypass), with an unchanged canonical tree after preview.

## Scope

Preview stays non-mutating: stage into a temporary directory or validate the in-memory staged index. Do not reserve IDs. If full staging in preview turns out too costly, the fallback is to print `Input valid; graph checks run at apply` instead of `valid`, which needs the human's agreement.

## Context

docs/spec/bundle/schema.md (Preview and apply report the same normalized plan); docs/product/cli-surface.md:92-93.
