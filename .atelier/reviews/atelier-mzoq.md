---
schema: atelier.review
schema_version: 1
id: atelier-mzoq
title: approval on atelier-hhce @0c3835b9
target: atelier-hhce
identity: claude-reviewer-hhce
commit: 0c3835b9ff81af67b5fbe9c2015749f453f1b0a4
created_at: 2026-11-04T23:36:56.981950830Z
event:
  event: approval
---

Read the 233d3cd5..0c3835b9 diff. Lint, workflow validation and IssueSections::for_type no longer require mission criteria or write a placeholder. A strategy with no heading now parses as no criteria. brief --html drops the empty tile, TOC link and section; debrief, work and evidence already skipped empty criteria. Exercised target/debug/atelier in a scratch repo on an Outcome-only mission, a strategy-linked mission whose strategy has no heading, and a legacy placeholder mission. check, check <id>, issue transition ready, brief, brief --html, work mission, work missions, debrief and debrief --html all pass with no criteria text, placeholder or warning. evidence --criterion is refused with 'does not define'. The new brief.rs tests would fail without the change. ADR 0024 matches atelier-rc8j (S6 unproven, 26 runs, split to p1vz), 1w8c S6, 4pzj S5 and l63c S3, and agrees with Agent Factory plan.md Shaping a Mission and Ready Missions. The docs grep finds no remaining claim that criteria are required. Passed: diff --check, fmt, metadata --locked, nextest -p atelier-app -p atelier-core (118) and filtered cli_integration (121). CI was still pending at review time. Findings atelier-b940 and atelier-hknd are low, deferred.
