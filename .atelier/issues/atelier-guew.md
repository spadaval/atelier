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
    id: "atelier-uf97"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T03:55:30.285844366+00:00"
status: "declined"
title: "Make bundle preview run the same staged-graph validation as apply"
updated_at: "2026-11-11T03:55:30.285844366+00:00"
---

## Goal

`atelier bundle preview` runs only `load_bundle`, `validate_bundle` and `BundleGraphPlan::from_bundle` (crates/atelier-cli/src/commands/bundle.rs), while `bundle apply` also checks its staged records as one batch against the rules `atelier check` applies (atelier-ilba's write guard). Preview therefore prints that the bundle is valid when apply then rejects it, for example two new issues `blocked_by` each other, or a new task that `blocks` a mission in progress. The documented loop is "inspect preview, then apply", so "valid" makes a promise preview does not keep.

## Done when

- [ ] Both bundles above fail `atelier bundle preview` with the same error (code and text) that `bundle apply --yes` gives, and preview writes nothing: preview runs apply's write-time validation on the staged records.
- [ ] An integration test asserts that preview and apply reject the same staged-graph violations (at least a cycle and a mission dependency bypass), with an unchanged canonical tree after preview.

## Scope

Preview stays non-mutating: validate the staged records with the guard (`atelier_app::record_guard`) without writing them, or stage into a temporary directory. Do not reserve IDs.

## Context

docs/spec/bundle/schema.md (preview and apply report the same normalized plan); docs/product/cli-surface.md. Decision atelier-mzpn meant this to build on atelier-ilba, now merged.

## Report

Dropped in the cutover sort (atelier-5zre), reason (a). It changes `bundle preview` validation, and the What is removed table removes `bundle`.
