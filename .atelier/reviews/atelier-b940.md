---
schema: atelier.review
schema_version: 1
id: atelier-b940
title: finding on atelier-hhce @0c3835b9
target: atelier-hhce
identity: claude-reviewer-hhce
commit: 0c3835b9ff81af67b5fbe9c2015749f453f1b0a4
created_at: 2026-11-04T23:36:50.146668353Z
event:
  event: finding
  recommendation: defer
---

low: docs/adr/0024-missions-are-themed-containers.md:49 The ADR, mission-authoring.md:14 and work-model.md:17 link spadaval/.agents main#shaping-a-mission and #ready-missions (and .atelier/workflow.yaml ready cites plan.md Ready Missions), but origin/main plan.md has neither section yet; they exist only on the local agent-factory/mission-shape branch (65f7077, e428837), which has no PR. The links dangle until it lands. Fix: land the Agent Factory branch with or before this PR.
