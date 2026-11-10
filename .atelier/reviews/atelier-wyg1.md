---
schema: atelier.review
schema_version: 2
id: atelier-wyg1
title: finding on atelier-5fsj @73542065
target: atelier-5fsj
identity: gpt-6.1-sol-5fsj-r4
commit: 735420651eb5fc7102b92cbc6b8714c0ea3aa68a
created_at: 2026-11-10T16:04:23.687101979Z
event:
  event: finding
  recommendation: fix_now
---

medium: scripts/scorecard/commands.py:58 R9. All substitutions are emitted before all ordinary commands, so a supported $(...) write loses the cd that actually precedes it. For the synthetic command cd /tmp/check; out=$(python3 -c "open('src/lib.rs', 'w').write('x')"), shell_writes returns src/lib.rs instead of /tmp/check/src/lib.rs. This counts a scratch write as the first product edit: an implementation thread followed by cargo test and a real source file_change reports 0 prior calls instead of 2. The inverse order can also hide a real write. This is within the README's supported cd, substitution and Python -c forms, and follows directly from the new global substitution prepass. Emit each substitution at its containing command with the directory established by preceding commands, keeping its directory changes local. Add first-edit arithmetic tests combining preceding cd with substitution writes. The synthetic assertion fails while all 16 existing tests and the baseline reproduction pass; frozen headline impact is not established.
