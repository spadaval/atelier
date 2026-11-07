---
schema: atelier.review
schema_version: 2
id: atelier-63vf
title: approval on atelier-nn67 @85952f77
target: atelier-nn67
identity: independent reviewer subagent (claude-opus-5-5), atelier-nn67
commit: 85952f7754ac07f0e395b4a178c567388d45b9af
created_at: 2026-11-07T15:07:48.590558446Z
event:
  event: approval
---

Reviewed 2ffb6751..85952f77. The first four Done when lines hold: branch_base is optional on read and still written, and a record parses with it or without it; migrate after-merge keeps an item's last stand-in-less line and names the item, and a rerun is a no-op; workflow-configuration.md gives a policy-move order that strands nothing (scratch check: switching first makes check and transitions refuse with workflow_issue_status_invalid); ADR 0028 names atelier-hfme. Light-path fix: policy and config count as code, as is_record_path counts them; on a scratch repo a policy edit after approval makes the approval stale and merge refuses, and merge gates use the base's policy. Fixes verified: atelier-xb4j (merge_strategy wording), atelier-kctm (tracker records wording), atelier-gdhc (the rollout line and a decision note, records only). The on_base backlog change stays in the strategy: no start, done_on_base and classification unchanged, and the installed binary reads the new policy. Ran atelier-workflow and atelier-app 146/146 and atelier-cli loose_ends, light_path and session 158/158; fmt and diff --check clean; atelier check passes. Not run by me: the full suite and test-hooks; CI at bb7c7835 was pending when checked.
