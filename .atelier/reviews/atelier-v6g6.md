---
schema: atelier.review
schema_version: 2
id: atelier-v6g6
title: finding on atelier-v3nm @969a7c46
target: atelier-v3nm
identity: codex-plan-reviewer (gpt-6.1-sol)
commit: 969a7c464fa8662a5bded59be4a841cd9c5940f2
created_at: 2026-11-09T04:04:19.511634060Z
event:
  event: finding
  recommendation: fix_now
---

medium: .atelier/issues/atelier-v3nm.md:38 The validation must replan on the base and produce a new docs/architecture design document (line 39). Planning on the Base in references/atelier.md permits direct base commits only for records and linked strategy files; architecture documents require a branch, independent review, CI and atelier merge. A validation item itself has no branch or merge. Following this body either breaches policy or stalls when the planner needs to integrate its design. Specify the branch/light-path review and integration of the real design document, or run the planner in a disposable experiment and have the independent validator assess its artifact while the real replan is separate work. Keep the validation result on the base and state how the generated plan is reviewed without exempting architecture docs.
