---
created_at: "2026-10-01T22:11:08.982471751+00:00"
id: "atelier-idae"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-15T19:06:43.726759122+00:00"
status: "done"
title: "Report every graph error in atelier check"
updated_at: "2026-10-15T19:06:43.726759122+00:00"
---

## Goal

Report every graph error found by atelier check in one invocation.

## Done when

A fixture with multiple independent malformed graph edges reports each error while keeping a nonzero exit status. Valid graphs still pass, and mutation commands still refuse invalid records before writing.

## Scope

Graph diagnostics in check; preserve existing fail-closed validation and mutation behavior.

## Context

Independent rebuild review identified that check stops at the first graph error. The rebuilt command correctly refuses malformed graphs; collecting all diagnostics is a later usability improvement.

## Close Reason

check reports each independent graph error: per-record isolation, plus the first cross-record error (e.g. a cycle) once faulty records are set aside. Mutations still fail closed. Test check_reports_every_independent_graph_error_in_one_run (two missing targets plus a cycle, 3 findings, nonzero exit).
