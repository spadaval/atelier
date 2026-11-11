---
schema: atelier.review
schema_version: 2
id: atelier-1hf0
title: finding on atelier-a4rs @c756d0b3
target: atelier-a4rs
identity: GPT-6.1 Sol independent planning reviewer
commit: c756d0b375f05423ac5c96a553dc6cc4613d4ced
created_at: 2026-11-11T04:38:31.184776932Z
event:
  event: finding
  recommendation: fix_now
---

.atelier/issues/atelier-a4rs.md:52-53 at c756d0b3: medium, FIX NOW. The remaining part of atelier-9x7a is not resolved: serial threaded scenarios and a waiting delegate bound only that launch path. Native Codex spawn_agent is explicitly allowed, but its children are only counted from the session record; no admission limit or mandatory waiting policy is specified for them. One serial root can start three native children, or a root can keep working while its native child starts another; two scenarios classified as non-threaded can also exceed two workers if either starts a native child. Reporting the peak afterwards cannot keep the owner limit of two eval agents from being exceeded. The present harness has no native-child bound: run.py:125-129 builds codex exec without one, :430-433 limits only root jobs, and delegate.py:27-30 controls only delegate launches. Settle a policy that prevents excess active workers across native and delegate launches, including nested and mixed launches and the claimed one-agent non-threaded cases. Prefer a small verified restriction over a new scheduler. Require a non-agent test that attempts the excess launches and proves they wait or are refused before another worker starts; counting alone is not enforcement. The smoke runs and t747 matrix must use that policy.
