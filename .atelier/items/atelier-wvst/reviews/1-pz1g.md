---
schema: atelier.review
schema_version: 3
round: 1
commit: 593f6dc37170cacbe6bca45c742aa2b26be5e2ae
reviewers:
- thread:delegated-task:command%3Amcp%3A4a4c7c1e-1d22-4657-a9be-245f255abba4%3Adelegate-task%3A7d8acf65-2f24-4888-ace7-ccb5bcf1f7a1
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-18T01:36:20.030248184Z
---

## Summary

Complexity: medium. This adds a React page and its model, and changes the shared layer mount and navigation.
Value: high. Drawing the project home without a frame is one of the mission's required outcomes.
Risk of merging it: medium. It changes the project home people use to see and choose work, but the change is visible and can be reverted.
The value warrants the size; keeping the old framed page would not deliver the required outcome.

Reviewed `master...593f6dc37170cacbe6bca45c742aa2b26be5e2ae`, comparing the native page with `desk/src/pages/ProjectHome.tsx`, the API types, the layer and read hook, and the documented rules. Ran `pnpm typecheck`, `pnpm build`, `node t3-extension/run-tests.mjs` and `git diff --check master...593f6dc`; all passed, including all six test files. The integration checks cover all three entrances with no iframe, items and proposed plans framed with Back, and agreed missions opening natively. The fixtures cover the three lanes, empty repository, holders, week and widths. Inspected the supplied light and dark Nightly screenshots; I did not independently drive the Nightly test instance. PR #210's extension and test jobs were still pending when inspected.

Also built a temporary browser fixture from the submitted components. After successful reads, made the roadmap, history and expanded mission reads return HTTP 502 and dispatched `visibilitychange` to trigger a refresh. This reproduced Finding 1. Comparing the old loose-item renderer with the new one establishes Finding 2. The page's main navigation and the documentation changes meet the Done when lines, but these regressions need correction.

## Finding 1: Handle failed refreshes alongside retained data
Recommendation: fix now

`t3-extension/src/home/ProjectHome.tsx:207-219` only checks `page.error` while `page.data` is absent. `useDeskRead` deliberately retains the last successful data on failure (`src/react/desk.ts:43`), so after the first success every failed mission refresh is silent. The expanded mission continues presenting old statuses, blockers and holders as current. This also happens when that mission was empty: the early return at line 215 hides the failure.

The same hook contract is mishandled at lines 67-68: the roadmap and history notes say their contents are "left out", but line 45 still uses the retained roadmap and line 90 still renders the retained history. In the browser reproducer, all three loose items and the week remained visible under those notes; the expanded mission retained its five items with no warning at all.

Handle errors even when data exists. For retained data, identify the failed read and say it is from the last successful read, as the missions warning at line 66 already does. If a section really is omitted, make the rendering agree. Add a successful-read-then-failed-refresh check for the expanded mission, roadmap and history; initial-failure fixtures do not exercise this case.

## Finding 2: Keep the priority indicators on items in no mission
Recommendation: fix now

`t3-extension/src/home/ProjectHome.tsx:259-260` routes every loose item through `ItemLine`, whose output at lines 235-246 never reads `row.priority`. The existing Desk home renders `<Priority priority={row.priority} />` for each loose item (`desk/src/pages/ProjectHome.tsx:125`). Consequently, the native home drops the distinction between critical, high, medium and low work from Now, Next and Later, where people choose what comes next.

Restore the priority indicator for loose items. This is a migration of the existing home, with a new design out of scope, and the mission's Authority explicitly requires agreement before leaving out something the old page shows. The Report records no such change. Use loose fixtures with different priorities to verify that the distinction survives the migration; the current fixtures give every item medium priority and never assert its display.
