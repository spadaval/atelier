---
schema: atelier.review
schema_version: 2
id: atelier-43bx
title: finding on atelier-dkul @e9d6bd3d
target: atelier-dkul
identity: plan-rereviewer-g6rp
commit: e9d6bd3d2535398cf53af3fafc98e5fbb746edf5
created_at: 2026-11-08T06:00:18.008496631Z
event:
  event: finding
  recommendation: fix_now
---

[low] atelier-b2m5's fix puts the 'moved, frozen' notice only in spadaval/.agents' README, and that repository has no README (main holds only .gitignore, .skill-lock.json, skills/ and evals/), so the notice lands in a new root file an agent editing skills/agent-factory there is unlikely to open. The finding asked for the README and the skill's maintainers/ too; the agent the notice is for edits the skill and reads maintainers/ (skill-evaluation.md, finding-disposition-pressure-tests.md). Fix: say 'adds a README' rather than presuming one, and put the same line at the top of skills/agent-factory/maintainers/skill-evaluation.md in that commit (not SKILL.md, which is the live skill until bxs7).
