---
schema: atelier.review
schema_version: 2
id: atelier-4qlp
title: finding on atelier-g6rp @859f41a2
target: atelier-g6rp
identity: plan-reviewer-g6rp
commit: 859f41a2454c4278bc33d18868729b139e7e90f7
created_at: 2026-11-08T05:58:07.249815213Z
event:
  event: finding
  recommendation: fix_now
---

[medium] A false fact about spadaval/.agents. Its origin/main (23a6bed) holds only skills/agent-factory, artifact-hub, codex-reflect, generate-html-plan, pr-review-explorer, evals/agent-factory, .gitignore and a stale .skill-lock.json. The firecrawl* skills are plain directories in ~/.agents/skills and are not in the repository. After atelier-6oxt the repository holds no skills, yet the Outcome says it 'keeps only skills that are still used' and Scope and 6oxt say the firecrawl skills and the repository 'stay'. Sumeet's two questions (merging the firecrawl skills; retiring the repository, which 6oxt leaves empty) are not recorded as an open decision or needs-human item, as plan.md's Ready Work requires. Fix: correct the Outcome, Scope and 6oxt's Scope, and record both questions for Sumeet.
