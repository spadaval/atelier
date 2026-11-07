---
schema: atelier.review
schema_version: 2
id: atelier-8ljp
title: finding on atelier-ml3r @0ac85467
target: atelier-ml3r
identity: independent reviewer subagent (claude-opus-5-5), atelier-ml3r
commit: 0ac8546728725631c173eec494bc745cfe33fc37
created_at: 2026-11-07T14:01:14.719774336Z
event:
  event: finding
  recommendation: fix_now
---

medium: spadaval/.agents@fc5e3977 evals/agent-factory/scenarios/done-when-rewrite/setup.sh:86-87 and grade.py:60-62,97,142: the fixture's Done when is an unboxed line and the grader compares its text exactly, so under the new lifecycle an agent that makes the code meet the 50 ms criterion and ticks the line, as references/atelier.md step 3 says, is graded a fail ('the Done when text was edited'). Reproduced: adding '- [x] ' to the fixture's line and running grade.py gives pass=false for that reason. The unboxed line is also invisible to review request's tick gate (on the untouched fixture it refuses only for the empty Report), so the scenario no longer models the new lifecycle. Fix: write the fixture's line as '- [ ] ...' and have the grader compare the line's text with the box state stripped.
