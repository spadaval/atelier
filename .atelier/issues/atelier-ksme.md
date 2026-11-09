---
created_at: "2026-11-09T06:20:33.781256763+00:00"
id: "atelier-ksme"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "review"
    id: "atelier-juiz"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "R4 low: skills/agent-factory/SKILL.md:35 Core Rules still say workers report findings and the agent accountable for the…"
updated_at: "2026-11-09T06:30:37.481555868+00:00"
---

## Goal

Deferred from finding atelier-juiz on atelier-ss4y @ca39f2ff, by `atelier review defer`.

R4 low: skills/agent-factory/SKILL.md:35 Core Rules still say workers report findings and the agent accountable for the outcome decides what they mean. Independent Review's Findings bullet (SKILL.md:73) says author and reviewer settle correctness and scope stays with the accountable agent. Constitution §5 already says "for current scope"; the Core Rule dropped those words. A cold orchestrator reading Core Rules first can think it still records dispositions. Restore "for current scope" so the Core Rule matches constitution and the Findings split. Not blocking because Findings on the same page is specific.

## Done when

- [x] SKILL.md's Separate evidence from scope rule says the accountable agent decides what findings mean "for current scope", matching constitution §5 and Independent Review's Findings bullet.

## Report

Fixed in passing in 2504f748 on atelier-ss4y's branch ("decides what they mean for current scope"). Re-reviewed with that commit by codex-reviewer (gpt-6.1-sol), approval atelier-acli at 1bfb2fa5, which says this issue is fixed.
