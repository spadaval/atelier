---
schema: atelier.review
schema_version: 2
id: atelier-0avz
title: finding on atelier-l58v @9ea0a19b
target: atelier-l58v
identity: claude-code-reviewer (opus-5.5)
commit: 9ea0a19b4fb5a0f3488d5efd40e58bbaeff0d91b
created_at: 2026-11-08T21:49:01.275533048Z
event:
  event: finding
  recommendation: fix_now
---

medium: skills/agent-factory/procedures/review.md:87 and procedures/orchestrate.md:79 (eb14a31). Two new rules turn on a finding's severity (a won't-fix on a high-severity finding goes to the human; a new finding on the fix of a low-severity finding is deferred), but the skill never defines severity or asks a reviewer to give one: the Output shape (review.md:103-112) has no severity, and review.md:101 still says to use that shape 'even where a brief asks for severity labels'. An orchestrator must guess whether a finding is high, and a reviewer following review.md drops the label the human gate needs; it is also unclear whether a wont_fix override of a Reject: finding (review.md:42-43), whose text opens with the verdict rather than a severity, needs the human. Fix: add severity (low, medium, high, with one-line anchors, or reuse Risk's) to the Findings line of the Output shape, drop 'severity labels' from review.md:101, and say a Shrink:/Reject: finding carries one too.
