---
acceptance: []
blocks: []
created_at: "2026-06-08T22:34:34.830966082+00:00"
depends_on: []
evidence_required: []
id: "atelier-0019"
issue_type: "task"
labels:
- "agent-factory"
- "task"
- "tracker"
- "validation"
links: []
parent: "atelier-0016"
priority: "P3"
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Clarify unblock command success output"
updated_at: "2026-06-09T17:20:17.141915982+00:00"
---


Validation in atelier-z1p.6 showed atelier issue unblock removes the dependency edge, but prints the same success text as block. Adjust output so Agent Factory transcripts clearly distinguish add and remove.
