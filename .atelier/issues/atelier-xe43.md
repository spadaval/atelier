---
created_at: "2026-10-15T18:45:05.884861487+00:00"
id: "atelier-xe43"
issue_type: "bug"
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
title: "Report branches without a tracker concisely in prime and status"
updated_at: "2026-10-15T18:45:05.884861487+00:00"
---

## Goal

`prime` and `status` do not print raw git errors for local branches that have no committed tracker.

## Done when

A test with a fresh `atelier init` and pre-existing local branches shows concise "no tracker" reporting (or omission) and no `git cat-file` text, while genuine errors on tracked branches are still reported.

## Scope

Local branch summaries in prime/status.

## Context

Evaluation output: "route-clipboard: unavailable — Git record object missing: git cat-file -t <sha>:.atelier/workflow.yaml failed …" for every pre-existing branch.
