---
schema: atelier.review
schema_version: 3
round: 1
commit: 5634c758d02e51fc09403b141b997850632210f0
reviewers:
- thread:delegated-task:command%3Amcp%3A0588fba7-e995-4861-85ba-d4260b2f89e6%3Adelegate-task%3A61827169-7310-43f2-a756-5867fcb02388
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-18T00:35:48.772180101Z
---

## Summary

Complexity: medium. Existing sidebar and layer behavior moves across typed modules, and the strip gains a React lifecycle and polling hook.

Value: high. This completes the mission's single typed codebase and removes the last unchecked view script.

Risk of merging: medium. These are user-facing navigation and composer surfaces, but failures are visible and the change is reversible.

The value warrants the rewrite, and the module boundaries follow the existing responsibilities. I found one layout problem that needs fixing before merge.

Reviewed `master...5634c758d02e51fc09403b141b997850632210f0`, comparing the new modules with the deleted script. Checked every Done when line: DOM sidebar rows, React strip using copied T3 components, native mission and framed Inbox targets, strict TypeScript with no source exclusions, deletion of the handwritten scripts, preserved host-behavior comments, and unchanged test assertions. The separate fixture-origin fix matches the Desk's embed contract.

`pnpm typecheck`, `pnpm build`, `node run-tests.mjs` (all five test files), and `git diff --check master...5634c758` pass. The integration test covers folding, fading copies, range gestures, folder reopening, view degradation, navigation, frame origin/window validation, and cleanup. Only shadow-root queries changed in that test.

Started a fresh isolated T3 Code Nightly `v0.1.11-nightly.20261117.4851` instance with this build. `test-contract.mjs` passed all 35 hooks. Also exercised its mission title (Fixture mission, no iframe), Inbox row (framed `/embed/inbox`, pressed), and draft strip (Waiting on you 2, In motion 1, Next up 1), with no reported extension error. Stopped the instance afterwards.

An additional temporary probe used the integration fixture with a third held In motion item and measured the clipping described below. It changed no repository test or source. I did not check Windows or macOS, or visually inspect the strip in both themes; the component comparison test passes in light and dark.

## Finding 1: Fit all three In motion rows inside the strip's card
Recommendation: fix now

`t3-extension/src/strip/ProjectStrip.tsx:130` gives each row `shrink-0`, including the two-line In motion rows built at lines 55–63. The strip is fixed at 176px (`src/strip/strip.tsx:16`), leaving a 144px card; its header and padding consume 34px, while three held rows consume 129.5px including gaps. The card uses `overflow-hidden` at `ProjectStrip.tsx:112`, so the third item is cut off rather than made fully available.

Reproduced with the built extension in the existing headless integration fixture, adding a third item with a holder to `launchFixture.in_motion`. At the wide layout, the card ran from y=570 to y=714. The third row ran from y=691 to y=733.5; its holder line ran from y=714.75 to y=728.5 and was entirely hidden, and its item line was also clipped at the bottom. The existing fixture has only two In motion rows, so its passing assertions do not cover this supported three-row state.

Make the three two-line rows fit the available card height, or otherwise make their full content reachable without clipping. Add a geometry check using three held items, so the third row and its holder remain within the visible card.
