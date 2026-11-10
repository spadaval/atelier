---
schema: atelier.review
schema_version: 2
id: atelier-vcz9
title: finding on atelier-fcb7 @a683a81d
target: atelier-fcb7
identity: gpt-6-astra-amend3-r1
commit: a683a81d21342620e82533bc7bd9ea067ceaeaa3
created_at: 2026-11-10T04:47:53.535923288Z
event:
  event: finding
  recommendation: defer
---

medium docs/strategies/redesign-architecture.md:64: the 116-line commit_records loader is mostly orchestration of retired RecordIndex, WorkflowPolicy, base_policy and ProjectConfig, while its reusable object reader lives in git_freshness.rs (ReadContext, batched object access and read-error handling). Calling that loader a copy understates its dependency closure; commands/git.rs also delegates its actual Git calls there. At the next relevant planning boundary, name the low-level reader/command pieces to copy and describe the record loader as rewritten on them, retaining pinned-commit, nonregular-file and subprocess-failure contracts. Assign the arbitrary-commit read API explicitly to an item; the architecture promises it but no first-step acceptance criterion exercises an arbitrary historical snapshot.
