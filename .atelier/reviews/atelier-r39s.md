---
schema: atelier.review
schema_version: 2
id: atelier-r39s
title: finding on atelier-qr5s @378eb60a
target: atelier-qr5s
identity: grok-4.6-plan-r2
commit: 378eb60a7695bdea855a47332bda78b1218b3445
created_at: 2026-11-10T01:23:20.475748881Z
event:
  event: finding
  recommendation: fix_now
---

high docs/strategies/redesign-architecture.md:304-321: cutover step 3 merge with a forge pushes converted master, and step 4 only then installs the binary, moves the skill, and runs setup. .claude/settings.json and AGENTS.md invoke the installed atelier, whose Stop hook is land. After the push, schema 3 records are on origin/master and 0.4.0 cannot read them; submit's handoff also prints atelier review, which is 0.4.0; the work host breaks if it fetches. Smallest fix: install the branch-built binary and run setup without committing (AGENTS.md already names --binary) before merge publishes, print that path in the handoff, and tell the work host not to fetch until it converts.
