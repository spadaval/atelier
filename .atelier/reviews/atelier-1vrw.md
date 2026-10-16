---
schema: atelier.review
schema_version: 1
id: atelier-1vrw
title: 'The ADR Consequences (ADR:74-79) and atelier-sdb0''s Done when claim to update ''every current reference'' to prime, but they miss three. (1) This repository''s .claude/settings.json, which this branch sets to ''atelier prime --hook claude''. sdb0 must re-run setup claude here, otherwise atelier-jjvc''s planned check (flag hooks that still call prime) fails on Atelier''s own repo. (2) docs/strategies/review-and-introspection.md:59,101 (see the related finding). (3) scripts/bench_read_commands.py:358,367, which benchmarks prime. Fix: add them to sdb0''s Done when.'
target: atelier-bw0b
identity: Independent reviewer 2 (Claude subagent)
commit: 0db4b1845dab4da518fe334854b54f50a9a95345
created_at: 2026-10-16T23:50:25.582655665Z
event:
  event: finding
  recommendation: fix_now
---

The ADR Consequences (ADR:74-79) and atelier-sdb0's Done when claim to update 'every current reference' to prime, but they miss three. (1) This repository's .claude/settings.json, which this branch sets to 'atelier prime --hook claude'. sdb0 must re-run setup claude here, otherwise atelier-jjvc's planned check (flag hooks that still call prime) fails on Atelier's own repo. (2) docs/strategies/review-and-introspection.md:59,101 (see the related finding). (3) scripts/bench_read_commands.py:358,367, which benchmarks prime. Fix: add them to sdb0's Done when.
