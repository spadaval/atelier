---
created_at: "2026-10-01T21:19:17.694344067+00:00"
id: "atelier-wp90"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-xz11"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-01T22:45:47.852942578+00:00"
status: "done"
title: "Complete and dogfood session commands"
updated_at: "2026-10-01T22:45:47.852942578+00:00"
---

## Goal

Complete prime, land, setup and cross-branch orientation so an agent can resume and finish work with the rebuilt tracker.

## Done when

CLI tests exercise context, dirty-record and invariant refusal, harness installation and cross-branch views. One real session uses prime and land and records its observations; supported harness hooks pass official-format fixtures.

## Scope

Session commands, harness hooks and focused tests.

## Context

docs/strategies/atelier-rebuild.md, S6.

## Close Reason

Session commands and supported hooks pass fresh tests and an actual Codex prime/land dogfood; R1–R8 are repaired and independently reviewed.
