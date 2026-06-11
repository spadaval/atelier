---
acceptance: []
blocks:
- "atelier-vqsb"
created_at: "2026-06-11T02:45:01.354284754+00:00"
depends_on:
- "atelier-kaei"
evidence_required: []
id: "atelier-vfqo"
issue_type: "task"
labels: []
links: []
parent: "atelier-zjb5"
priority: "P1"
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Rewrite tests that asserted JSON command output"
updated_at: "2026-06-11T02:45:01.354284754+00:00"
---

Replace JSON command-output assertions with human-output, quiet-output, or direct state/projection assertions according to the inventory. Acceptance: tests cover removal behavior and the intended replacement workflow without scraping broad human text as a machine contract.
