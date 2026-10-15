---
created_at: "2026-10-15T18:45:05.916848779+00:00"
id: "atelier-cmi9"
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
title: "Measure non-test Rust lines for S5"
updated_at: "2026-10-15T18:45:05.916848779+00:00"
---

## Goal

An honest, reproducible measurement of non-test Rust lines for S5, excluding tests/ files and inline cfg(test) modules.

## Done when

Evidence records the count method, the commit, and the result against the 20k limit; the independent validation (atelier-xz11) checks it.

## Scope

Measurement only; changing S5 needs the human.

## Context

A rough count in the external evaluation found about 26k lines outside tests/ dirs, including inline cfg(test) modules. scripts/count_non_test_rust.py exists.
