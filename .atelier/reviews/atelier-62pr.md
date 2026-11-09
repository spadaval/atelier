---
schema: atelier.review
schema_version: 2
id: atelier-62pr
title: finding on atelier-yn69 @38c7c1c1
target: atelier-yn69
identity: gpt-6.1-sol-r1
commit: 38c7c1c179431d9864ccf95622a7cedc1cfa1981
created_at: 2026-11-09T18:46:39.579230623Z
event:
  event: finding
  recommendation: fix_now
---

Low: crates/atelier-site/web/src/pages/Brief.tsx:91-97,127: Scope and Open Human Decisions silently omit each item's priority. These ItemLine calls pass id, type, status and title but never item.issue.priority / issue.priority; ItemLine has no priority prop. The checked-in alpha-m1 brief fixture returns priority=high for alpha-a1 and alpha-a2, and the CLI's brief renderer passes those priorities to ItemLine (commands/mission_review.rs:2176,2222), but rendering BriefView yields 'alpha-a1 task todo Load records from a commit' and 'alpha-a2 decision todo Choose the server' without high in either section. This loses API data the corresponding Markdown command prints, violating Done when 2 and hiding the priorities while reviewing a plan. Show the returned priority on both kinds of row and assert it per item in a fixture-based test; the current 22 Vitest tests pass despite the omission.

Verdict: merge after this small fix. Sizes: complexity medium (embedded app and second-language build), value high (phone-readable mission reviews), risk medium (release/install path). Contract: fail on R1's omitted data. Fitness: otherwise pass; the pages use the API-selected review and do not derive work state.

Proof at 38c7c1c1: pnpm install --frozen-lockfile, typecheck, all 22 Vitest tests and pnpm build passed; cargo nextest run -j 4 -p atelier-site passed 8 tests; cargo nextest run -j 4 -p atelier-cli --test serve passed 20 tests, with the fixture-writer test intentionally skipped. Both Rust runs used TMPDIR under the requested cache and a separate review-sol-target there. Playwright with 4 workers passed 22 tests, including actual built JS/CSS MIME types and missing-asset 404. Fresh-binary host Playwright passed 5 repositories, 42 missions and 84 pages with no errors. Direct scratch execution of build.rs proved missing dist builds with pnpm, current dist skips pnpm, newer sources require pnpm, missing pnpm fails with the build instructions, and debug needs no pnpm. cargo fmt, cargo metadata --locked, diff whitespace, all 18 ci-jobs cases, systemd-analyze verification and dist generate --check passed. Independently rendered BriefView from the checked-in fixture to confirm the missing priorities.

Open questions: none. Limits: no full dist or cargo install build rerun here; inspected the recorded clean-install/boat receipts and release wiring. The installed atelier currently lacks serve, so fresh host validation used the debug binary built in the review target directory rather than replacing the installed CLI. No code edited. R1 is the sole finding; no hardening against transient Git failures requested.
