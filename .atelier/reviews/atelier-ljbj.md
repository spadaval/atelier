---
schema: atelier.review
schema_version: 2
id: atelier-ljbj
title: finding on atelier-fcb7 @d4033fc2
target: atelier-fcb7
identity: gpt-6-astra-plan-r2
commit: d4033fc224792781cc6d6b86a65f0068efeded77
created_at: 2026-11-10T01:18:21.671063148Z
event:
  event: finding
  recommendation: fix_now
---

medium atelier-yptz: the required scenario closes a mission while its second item stays draft, but both specifications require every item done or dropped before close. The scenario cannot pass without violating the lifecycle. Keep the draft during the readiness/start assertion, assert close refuses while it remains draft, then explicitly drop or finish it before successful mission closure.
