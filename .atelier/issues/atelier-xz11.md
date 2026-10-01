---
created_at: "2026-10-01T21:19:17.757643119+00:00"
id: "atelier-xz11"
issue_type: "validation"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "blocked"
title: "Validate rebuild success criteria independently"
updated_at: "2026-10-01T23:04:17.725496985+00:00"
---

## Goal

Independently check the strategy's S1–S8 acceptance claims against the final integrated candidate.

## Done when

Each criterion has an explicit result and concrete evidence: gate and clean merge scenarios, freshness tests, a release benchmark, the command list and Rust source count, a dogfooded session, typed record lint and fixtures, and this tail tracker. Any failure or blocker is reported before a done claim.

## Scope

Read-only acceptance checks, evidence records, findings and their dispositions.

## Context

docs/strategies/atelier-rebuild.md, Success Criteria; docs/adr/0019-verified-repo-native-tracker.md.
