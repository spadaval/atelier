---
schema: atelier.review
schema_version: 2
id: atelier-j8f8
title: finding on atelier-h3hl @1f516cb2
target: atelier-h3hl
identity: gpt-6.1-sol-h3hl-r1
commit: 1f516cb227031dded6728410b3b7ce5dc73c9d2f
created_at: 2026-11-11T07:10:08.410239224Z
event:
  event: finding
  recommendation: fix_now
---

medium: R2 next/crates/atelier-desk/src/api.rs:90 The new required MissionPage.pending_answers and ItemPage.can_decide, decide_at and pending_answers fields break the typed consumers already on the integration base. On current master, desk/e2e/fixtures.ts constructs MissionPage at lines 89 and 181 without pending_answers, and its itemPage helper at line 205 returns an ItemPage without the three new fields. The partial spread cannot supply required fields. Integrating this branch therefore makes pnpm typecheck fail in the desk CI job, as the Report also acknowledges. Update both mission fixtures and the item helper with valid defaults and run the Desk typecheck before integration. This needs the orchestrator to place the fixture change because the item scope excludes that file; leaving a known build regression for after merge is not fit to integrate. Evidence here is direct inspection of current master and the generated types; TypeScript was not available locally to rerun that check.
