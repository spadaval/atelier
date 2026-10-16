---
schema: atelier.review
schema_version: 1
id: atelier-4hv0
title: 'ADR:37-42 transcript figures cannot be reproduced from the stated method, and for this repository the denominator counts commands that never ran atelier. Parsing Bash tool_use inputs (deduplicated by id, 2026-10-16 UTC, before the fix commit d52b2a2d at 23:43Z), I get 121 matches for atelier work|status|prime|issue show|issue list here, close to the ADR''s 119. But about half are python heredocs and transcript greps that only contain the text (doc edits, the counting scripts themselves). Excluding those leaves about 62 actual invocations. Depending on whether ''2>&1 | head'' counts as piped, 16-45 of them are piped (26-73%), not 26/119 (22%). Astromech: 221 matches exactly, but piped comes out 98-148 (sed 82-105) against the ADR''s 113/89. The direction holds and gets stronger. Failure: a Proposed ADR''s evidence paragraph gives exact figures that a reader cannot reproduce. Fix: state the cutoff and counting rule and exclude non-invocations, or give rounded ranges (''roughly a quarter to two thirds here; about half to two thirds in astromech'').'
target: atelier-bw0b
identity: Independent reviewer 2 (Claude subagent)
commit: 0db4b1845dab4da518fe334854b54f50a9a95345
created_at: 2026-10-16T23:50:25.465427547Z
event:
  event: finding
  recommendation: fix_now
---

ADR:37-42 transcript figures cannot be reproduced from the stated method, and for this repository the denominator counts commands that never ran atelier. Parsing Bash tool_use inputs (deduplicated by id, 2026-10-16 UTC, before the fix commit d52b2a2d at 23:43Z), I get 121 matches for atelier work|status|prime|issue show|issue list here, close to the ADR's 119. But about half are python heredocs and transcript greps that only contain the text (doc edits, the counting scripts themselves). Excluding those leaves about 62 actual invocations. Depending on whether '2>&1 | head' counts as piped, 16-45 of them are piped (26-73%), not 26/119 (22%). Astromech: 221 matches exactly, but piped comes out 98-148 (sed 82-105) against the ADR's 113/89. The direction holds and gets stronger. Failure: a Proposed ADR's evidence paragraph gives exact figures that a reader cannot reproduce. Fix: state the cutoff and counting rule and exclude non-invocations, or give rounded ranges ('roughly a quarter to two thirds here; about half to two thirds in astromech').
