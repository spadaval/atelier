---
schema: atelier.review
schema_version: 2
id: atelier-xn6i
title: finding on atelier-orb2 @3ba4a62f
target: atelier-orb2
identity: GPT-6.1 Sol independent planning reviewer
commit: 3ba4a62f0323cb3b6ccb1b651d76b07691e30d81
created_at: 2026-11-11T03:22:23.590815918Z
event:
  event: finding
  recommendation: fix_now
---

.atelier/issues/atelier-orb2.md:43 (also :35), severity medium: The no-restatement criterion conflicts with the Review rules the role files must contain. Line 35 requires roles/implementer.md to state fix, rebut once, or escalate, including escalation when a fix leaves scope, and roles/reviewer.md to state the one-try rule. But the current binary already prints those rules to the implementer: next/crates/atelier-cli/src/output.rs:298 prints the three exits and the out-of-scope escalation rule, and :111 prints that a kept rebuttal has had its one try and leaves fix or escalate. Line 43 forbids any role-file sentence restating status, refusals or help, without an exception. atelier-1oel will also print the three-round escalation rule, while Sumeet expressly leaves these rules with the skill. Consequently the required role text fails line 43, and deleting it fails line 35 and the owner's decision; the Done when lines cannot all hold. Exempt the explicitly retained Review policy from the ban, and apply the ban to duplicate command mechanics and orientation instead, keeping the policy in its assigned role files. This changes no binary behavior or mission scope.
