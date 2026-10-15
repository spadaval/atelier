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
closed_at: "2026-10-15T19:06:43.576012479+00:00"
status: "done"
title: "Report branches without a tracker concisely in prime and status"
updated_at: "2026-10-15T19:06:43.576012479+00:00"
---

## Goal

`prime` and `status` do not print raw git errors for local branches that have no committed tracker.

## Done when

A test with a fresh `atelier init` and pre-existing local branches shows concise "no tracker" reporting (or omission) and no `git cat-file` text, while genuine errors on tracked branches are still reported.

## Scope

Local branch summaries in prime/status.

## Context

Evaluation output: "route-clipboard: unavailable — Git record object missing: git cat-file -t <sha>:.atelier/workflow.yaml failed …" for every pre-existing branch.

## Close Reason

prime/status summarise branches without a committed workflow.yaml as 'No tracker on: …' and tracked branches without started work as 'No started work on: …'. Real read and policy errors are still reported. Test branches_without_a_tracker_are_summarised_without_git_errors.
