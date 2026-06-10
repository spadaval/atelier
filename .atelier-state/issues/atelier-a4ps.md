---
acceptance: []
blocks:
- "atelier-zd4d"
created_at: "2026-06-10T03:51:15.735448190+00:00"
depends_on:
- "atelier-e2vh"
- "atelier-hdhk"
- "atelier-m25t"
- "atelier-po2n"
- "atelier-tfn8"
- "atelier-z6sc"
evidence_required: []
id: "atelier-a4ps"
issue_type: "validation"
labels:
- "closeout"
- "migration"
- "validation"
links: []
parent: "atelier-zd4d"
priority: "P2"
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Validate Markdown-first migration and closeout"
updated_at: "2026-06-10T03:51:15.735448190+00:00"
---

Close out the Markdown-first RecordStore / ProjectionIndex / RuntimeState migration with scenario-centered proof.

Scope:
- Validate that canonical mutations write Markdown first and refresh or repair ProjectionIndex.
- Validate that a fresh runtime database rebuilds from discovered Markdown records without manifest.json or graph.json.
- Validate that local RuntimeState can be absent or reset without losing canonical issue behavior.
- Run residue searches for stale architecture language that treats SQLite as canonical record storage or export as the normal durability step.
- Record explicit follow-up issues for deferred compatibility paths.

Acceptance:
Command transcripts or automated tests prove create/update/close/dependency/link flows, stale projection recovery, rebuild from .atelier-state, export --check, lint, doctor, and ready work discovery; docs and tracker issues reflect any deferred work; the epic can be closed with no unowned acceptance criterion.

Validation:
- cargo fmt -- --check
- cargo test
- git diff --check
- ./target/debug/atelier lint
- ./target/debug/atelier export --check
- ./target/debug/atelier doctor
