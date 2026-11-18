---
schema: atelier.review
schema_version: 3
round: 3
commit: 3999705b5aa0d2747978f455e3dde3e4f5aef5f6
reviewers:
- thread:delegated-task:command%3Amcp%3A591f5cbe-f9e2-482c-89ac-8e9787397ee0%3Adelegate-task%3Ac7221beb-e410-42f9-a5ee-f34545fd39b2
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-18T05:00:51.668183518Z
---

## Summary

Complexity: medium. This adds a React page and its model and changes the shared layer mount and navigation.
Value: high. The native project home completes one of the mission's required outcomes.
Risk of merging it: medium. It changes the page people use to choose work, but failures are visible and the change can be reverted.
The value warrants the size; keeping the framed home would not deliver the required outcome.

Reviewed `master...3999705b5aa0d2747978f455e3dde3e4f5aef5f6`, the changes since round 2 and the merge with master. Compared the native home with the Desk renderer, API types and handlers, shared read hook, layer navigation and mission authority. The lane rules, item ordering, holders, priorities and seven-day strip preserve the existing home. The shared native mount also preserves the mission page and works with the newly merged This thread page. The README conflict resolution keeps both pages' test documentation.

`pnpm typecheck`, `pnpm build`, `git diff --check master...3999705b5` and all seven test files in `node t3-extension/run-tests.mjs` passed. The tests used a fresh `TMPDIR` under `/home/supadava/.cache`, given the previous review's report of the host's `/tmp` quota. Integration checks prove that the breadcrumb, project strip and more-missions row each open the native home without an iframe, that items and proposed plans open framed with Back, and that an agreed mission opens the native mission page. Fixture checks cover all three lanes, an empty repository, folds and holders, history and navigation, read failures and retained data, priorities, and widths from 500 to 1,400 pixels. No extension source requests `/embed/home`.

Checked every Done when line against the code, tests, documentation and supplied light and dark Nightly screenshots. I inspected both screenshots but did not independently drive the Nightly instance. PR #210's extension and test jobs were still pending when inspected; the local suite passed on the reviewed head.

The previous findings remain fixed. Failed refreshes show a timestamped warning beside retained roadmap, history and mission items, including an empty mission; loose items retain their priority indicators. The round 2 documentation finding is also fixed: `docs/architecture/desk.md` now distinguishes an initial failure that omits a section from a failed refresh that retains the last answer. No new findings.
