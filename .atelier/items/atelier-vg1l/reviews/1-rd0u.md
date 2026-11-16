---
schema: atelier.review
schema_version: 3
round: 1
commit: 436c07ec53bf6f757d622b30758bd09f77888c99
reviewers:
- thread:delegated-task:command%3Amcp%3A8548b04c-4c44-4c84-8a48-800a41f2d328%3Adelegate-task%3Ae6712e01-a514-499a-855c-0284ef60aee8
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-16T03:00:10.889860204Z
---

## Summary

Complexity: low. The mode adds a router basename and a small alternate shell;
it reuses the existing responsive page layouts and API reads.

Value: high. Removing the Desk's frame is necessary for the mission's panel
layout, which is already a path the human uses.

Risk of merging: low. The mode is opt-in, failures are visible, and the
change is small enough to revert. The shared disputed-page repair fixes an
existing grid and class-name collision.

The value warrants this scope. I found no change that should be shrunk or
rejected and no correctness finding.

I reviewed master...436c07ec53bf6f757d622b30758bd09f77888c99 in an
independent detached worktree against every Done when line, read the
mission's scope and authority, and traced the router, shell, breadcrumbs,
Markdown links, API paths and server's app fallback. The /embed prefix stays
on internal links and reloads, while API and external links retain their
own addresses. The alternate shell removes the requested chrome and keeps
the inbox available. Both documents describe the switch and host frame.

The disputed-page repair also changes standalone rendering, as the Report
explicitly acknowledges. Its glyphs fill the existing 18-pixel grid column,
and the renamed class prevents dispute metadata from inheriting sidebar
positioning. I accept this narrow repair; it is necessary for that page to
fit and remain readable, and adds no new behavior or action.

Validation in the detached worktree:

- pnpm install --frozen-lockfile and pnpm build passed; the build includes
  TypeScript checking.
- pnpm test passed all 43 tests.
- pnpm e2e --workers=4 passed all 82 tests. The existing pages.spec.ts and
  actions.spec.ts have no diff from the base.
- Additional Playwright checks at both 420 and 720 pixels covered /embed
  with and without its trailing slash, the repository rail, mission Work
  and Documents tabs, an expanded finished item's Report, a proposed
  mission's stop confirmation, an active item, both Agents routes, an
  unreadable repository, an unknown mission and an unknown route. Every
  view had zero document-width overflow, no visible control outside the
  viewport's width, and an inbox link under /embed.
- git diff --check passed. GitHub CI's changes and desk jobs passed for the
  reviewed commit.

The interface checks use the fixture API and built app, not the running
Desk service or the actual T3 extension. I did not test host messaging or
theming, which belong to later mission items, or claim a pixel comparison
of every standalone page. No source or existing test was changed during
review.
