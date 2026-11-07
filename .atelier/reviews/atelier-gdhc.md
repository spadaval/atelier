---
schema: atelier.review
schema_version: 2
id: atelier-gdhc
title: finding on atelier-nn67 @bb7c7835
target: atelier-nn67
identity: independent reviewer subagent (claude-opus-5-5), atelier-nn67
commit: bb7c7835fb1271eb53347c6cd536a93b4ec883e0
created_at: 2026-11-07T15:07:03.833897194Z
event:
  event: finding
  recommendation: fix_now
---

medium: .atelier/issues/atelier-nn67.md:38 the open rollout line still says every item in a status the new policy lacks, naming '`backlog` validations under the on-base workflow', moves to `todo`, and that the policy takes the starter's shape with '`merge_commit`'. bb7c7835 gave on_base a backlog so parked validations stay put, and workflow-configuration.md step 1 now covers only blocked and review and says merge_strategy can stay or go. A rollout agent working from this line would move astromech's six backlog validations to todo, breaking the in-flight branches the change exists to protect, and the line and the doc now contradict each other. Fix: drop the backlog-validations example and merge_commit from line 38 and give the reason (the orchestrator's on_base decision) in a note; this only touches records, so no new CI run is needed.
