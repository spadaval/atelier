---
schema: atelier.review
schema_version: 2
id: atelier-3z7m
title: finding on atelier-qr5s @a3f31d62
target: atelier-qr5s
identity: gpt-6-astra-plan-r2
commit: a3f31d623604faed81b1e75b12a60cdedb99cba9
created_at: 2026-11-10T01:17:35.413758970Z
event:
  event: finding
  recommendation: fix_now
---

high atelier-wd1i and docs/strategies/redesign-architecture.md:313-317: invoking the new binary by path does not switch the reviewer child's hooks or instructions. .claude/settings.json still invokes installed atelier status/land, and AGENTS.md still mandates smoke-repos.sh, which invokes removed doctor/work/review-request commands and compares against unconverted clones. The first review/install therefore meets the old tool on new records or a guaranteed failed smoke. Specify temporary hook/PATH routing, cutover-specific smoke replacement and instruction updates before this review, restoring them on rollback.
