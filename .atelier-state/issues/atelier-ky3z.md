---
acceptance: []
blocks:
- "atelier-fkgl"
- "atelier-uuhh"
created_at: "2026-06-11T02:45:00.523808082+00:00"
depends_on:
- "atelier-0se4"
evidence_required: []
id: "atelier-ky3z"
issue_type: "spike"
labels: []
links: []
parent: "atelier-esh8"
priority: "P2"
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Find and update internal consumers that relied on --json"
updated_at: "2026-06-11T02:45:00.523808082+00:00"
---

Identify repository scripts, tests, Agent Factory docs, and workflow examples that currently call atelier commands with --json. Acceptance: each consumer has a migration target: human parsing avoided, quiet output used, direct repo-state/projection read used, or command redesigned.
