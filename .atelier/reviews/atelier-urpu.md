---
schema: atelier.review
schema_version: 2
id: atelier-urpu
title: finding on atelier-orb2 @ea6ff0ee
target: atelier-orb2
identity: GPT-6.1 Sol independent planning reviewer
commit: ea6ff0ee0165cd82b315020fb7d5be1b3816f070
created_at: 2026-11-11T02:51:19.908933532Z
event:
  event: finding
  recommendation: fix_now
---

.atelier/issues/atelier-orb2.md:35 (also :59), severity medium: This Done when requires SKILL.md to carry all the detailed Review rules, including when the implementer must escalate and who decides, although every role reads SKILL.md (:34). Its final sentence simultaneously requires each rule to live in the file whose reader acts on it. Those requirements conflict and reproduce the common-file problem atelier-8ep4 specifically identifies: its Goal point 2 and second Done when remove arranging review and deciding findings from the instructions every role reads. The strategy requires step A to meet that intent (redesign-3-cutover.md, What mission 2 showed), and its step-A row puts live rules into role files. Keep the shared independent-review invariant in SKILL.md; explicitly assign implementer escalation/rebuttal actions and reviewer/lead/decider actions to the files whose readers perform them, with one authoritative statement each. Rewrite this criterion so a role-isolated implementation can satisfy it, rather than making the implementer choose which requirement to violate.
