---
created_at: "2026-06-13T20:55:09.103395868+00:00"
id: "atelier-3iom"
issue_type: "feature"
labels:
- "agent-readiness"
- "validation"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Encode proof and closeout policy in Atelier gates"
updated_at: "2026-06-13T20:55:09.103395868+00:00"
---

## Description

Move repeated proof-routing and mission closeout requirements out of Agent Factory prose and into Atelier-owned workflow validators, mission status, mission audit, lint, and product documentation.

## Outcome

- Proof-routing rules and mission closeout gates are enforced or reported by Atelier-owned validators, status output, audit output, lint, or documented workflow policy.
- Agent Factory guidance can defer to those Atelier surfaces instead of restating detailed evidence placement and closeout requirements.
- Missing proof, missing validation issues, stale advanced checks, and incomplete mission audit coverage produce actionable Atelier output.

## Evidence

- Before and after AGENTFACTORY.md review shows detailed proof/closeout prose replaced by references to Atelier-owned behavior.
- Mission status or audit transcript demonstrates actionable proof and closeout gaps.
- `atelier lint`, `atelier export --check`, `atelier doctor`, and focused workflow tests or transcripts pass.
