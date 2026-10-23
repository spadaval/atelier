---
created_at: "2026-10-23T17:59:09.745161217+00:00"
id: "atelier-sf0e"
evidence_type: "validation"
commit: "cd790a67a0fd34914afe25337408d4b88a439af4"
result: "pass"
captured_at: "2026-10-23T17:59:09.744840578+00:00"
agent_identity: "independent validator (Claude)"
target:
  kind: "issue"
  id: "atelier-1o56"
  role: "validates"
criteria:
- slug: "S4"
  result: "pass"
criteria_mission: "atelier-rc8j"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-1o56"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "S4 pass: review list <id> shows one row per event in time order, then 'Approval: ✓ current / ⚠ stale — ... changed since by 1 commit: <sha> <subject>: <path> / ⚠ missing — ... not independent / ✓ accepted on main', then 'Findings: N disposed/open' with each finding and its disposition chain ('✓ s1-7l10 fix_now → fix_now s1-2cr5, fixed at 799e0fcb'); an item with none says 'No review events for <id>.'. review show is a 2-5 line view with linked finding/disposition. Recording prints one line ('Approval s1-zx0k on s1-rpjn @8b0cceb9', 'Finding ... Dispose: <command>') and an author identity got '⚠ identity matches a Git author; this will not satisfy review.current'. New titles are 'finding on s1-rpjn @df897224', not the body. Limits: review show of an author-identity approval says only 'not accepted on main', not why; child-to-owner view (jw1n) covered by tests only."
updated_at: "2026-10-23T17:59:09.746092735+00:00"
---

S4 pass: review list <id> shows one row per event in time order, then 'Approval: ✓ current / ⚠ stale — ... changed since by 1 commit: <sha> <subject>: <path> / ⚠ missing — ... not independent / ✓ accepted on main', then 'Findings: N disposed/open' with each finding and its disposition chain ('✓ s1-7l10 fix_now → fix_now s1-2cr5, fixed at 799e0fcb'); an item with none says 'No review events for <id>.'. review show is a 2-5 line view with linked finding/disposition. Recording prints one line ('Approval s1-zx0k on s1-rpjn @8b0cceb9', 'Finding ... Dispose: <command>') and an author identity got '⚠ identity matches a Git author; this will not satisfy review.current'. New titles are 'finding on s1-rpjn @df897224', not the body. Limits: review show of an author-identity approval says only 'not accepted on main', not why; child-to-owner view (jw1n) covered by tests only.
