---
schema: atelier.review
schema_version: 3
round: 4
commit: f2bbdbf79026f9aad6c4d580cea268a6aea697c1
reviewers:
- 01a1eb13-c867-77f0-8e26-8eb4fd002c2f
model: grok-4.6
verdict: approve
created_at: 2026-11-17T19:55:41.095347861Z
---

## Summary

Checked the plan at f2bbdbf: `mission.md` and all eight items; the round-3 fix (wvst only); the blocker graph; `docs/architecture/desk.md` (This thread, mission control, project home, host contract, API table); `desk/src/pages/ThisThread.tsx`, `ProjectHome.tsx`, `MissionControl.tsx`, `components/Glance.tsx`, `lib/api.ts`; `t3-extension/t3-custom.mjs` `deskReadUrl`; `custom.js` `openLayer` and the three `/home/` openers; `test-extension.mjs` layer `src` for more-missions, breadcrumb and the strip; and `t3-extension/spike/mockups/mission-concepts.html` on `spike/extension-react`. Did not open the mockup in a browser, run the spike inside T3 Code, or reach Legend.

- Complexity: high. A Vite/React root, a T3-component copy pipeline, a new mission page, a typed rewrite of the 1,177-line script, two more native pages, and a Desk cutover that waits on a human install.
- Value: high. Sumeet directed the three outcomes on 2026-11-17: mission control wastes the T3 window, embedding those three pages has run its course, and `custom.js` is past one file.
- Risk of merging it: medium. A bad page shows in T3 and an old extension release rolls it back; yy4j waits on the install so This thread is not blanked. The value is worth that.

Round 3's remaining fix-now finding is fixed in wvst. No new hole.

Could not check: whether every T3 component This thread and the project home need will copy as cleanly as `badge`/`kbd`; menus, popovers and dialogs, which the spike did not try.

## Finding 1: wvst leaves the strip's project name asserting framed project home
Recommendation: fix now
Was: 3-tvgf.3
Now: fixed
atelier-wvst (item.md:26) names each of the three ways to the project home, including the project's name in the strip under the composer. item.md:29 converts the `test-extension.mjs` checks for the breadcrumb, the strip's project name (`:693`) and the "more missions" row into native-page checks with no `iframe`, and says no code path of the extension asks for `/embed/home` afterwards. atelier-40dc still rewrites the strip with the project home framed (item.md:31, 39) and runs before wvst, which is the right order for that check.
