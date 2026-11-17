---
schema: atelier.review
schema_version: 3
round: 3
commit: a374930344618ca1e024c2c45af85a2da2adadcf
reviewers:
- 01a1eb0d-3758-7c52-b15a-de3f3cf9d0b5
model: grok-4.6
verdict: changes
created_at: 2026-11-17T19:48:58.470678428Z
---

## Summary

Checked the plan at a374930: `mission.md` and all eight items; the round-2 fix commits 69750027 and 21f4ee69; the blocker graph; `docs/architecture/desk.md` (This thread, mission control, project home, host contract, API table); `desk/src/pages/ThisThread.tsx`, `ProjectHome.tsx`, `MissionControl.tsx`, `components/Glance.tsx`, `lib/api.ts`, `lib/mission.ts`; `t3-extension/t3-custom.mjs` `deskReadUrl`; `custom.js` `openLayer` and the three `/home/` openers; `test-extension.mjs` panel bootstrap, `host:thread` checks, and layer `src`; `crates/atelier-desk/src/server.rs` item `pull_request`, `fetch.rs` recent/open maps (`RECENT` 200); and `t3-extension/spike/mockups/mission-concepts.html` on `spike/extension-react`. Did not open the mockup in a browser, run the spike inside T3 Code, or reach Legend.

- Complexity: high. A Vite/React root, a T3-component copy pipeline, a new mission page, a typed rewrite of the 1,177-line script, two more native pages, and a Desk cutover that waits on a human install.
- Value: high. Sumeet directed the three outcomes on 2026-11-17: mission control wastes the T3 window, embedding those three pages has run its course, and `custom.js` is past one file.
- Risk of merging it: medium. A bad page shows in T3 and an old extension release rolls it back; yy4j waits on the install so This thread is not blanked. The value is worth that once Finding 3 is closed.

Round 2's two fix-now findings are fixed in the items. One hole remains: wvst converts two of the three clicks that open the project home, and leaves the strip's project name on `/embed/home`.

Could not check: whether every T3 component This thread and the project home need will copy as cleanly as `badge`/`kbd`; menus, popovers and dialogs, which the spike did not try.

## Finding 1: 9l8h and wvst leave test-extension.mjs asserting the framed This thread and project home
Recommendation: fix now
Was: 2-h00o.9
Now: fixed
atelier-9l8h (item.md:31) converts the panel's `/embed/`, `host:thread` and `/embed/t` checks into native This thread with no `iframe` and no `host:thread`, and keeps the framed record-link checks. atelier-wvst (item.md:29) converts the breadcrumb and "more missions" `/embed/home/...` checks into the native page with no `iframe`.

## Finding 2: Specify how the retro handles merged PRs the Desk cannot return
Recommendation: fix now
Was: 2-zm4k.9
Now: fixed
The mission leaves historical PR lookup out of scope (`mission.md`:133-134) and the retro criterion opens a Desk page where the Desk gives no PR (`mission.md`:100-101). atelier-ut89 (item.md:32) lists that merged item, says the pull request is not known, and opens its Desk page framed. Its done-page fixture (item.md:38) includes a merged item whose `pull_request` is null. `server.rs`:596 and `fetch.rs`:175 still look up by an extant branch in the cached open/recent maps (`RECENT` 200), so the null is a normal response.

## Finding 3: wvst leaves the strip's project name asserting framed project home
Recommendation: fix now
Three calls open the project home through `openLayer` / `toggleLayer` with `/home/<repo>`, which `openLayer` (`custom.js`:711-717) loads as `${desk}/embed${path}`:

- the "more missions" row (`custom.js`:903)
- the breadcrumb (`custom.js`:1126)
- the strip's project name (`custom.js`:1082-1083), titled "Open the … project home"

`test-extension.mjs` asserts `iframe.src` (`:480`) for all three: more-missions (`:645`), breadcrumb (`:652`), and the strip (`:693`).

atelier-wvst (item.md:29) converts the first two into native-page checks with no `iframe`. It does not name the strip. atelier-40dc rewrites that strip (item.md:26) and freezes its `test-extension.mjs` checks unchanged (item.md:31), and its Out leaves the project home framed until wvst. atelier-yy4j (item.md:33) then makes the Desk answer `/embed/home/...` as an unknown page.

After wvst, either the layer draws `/home/` natively and `:693` fails, or the strip still frames `/embed/home` and yy4j leaves that click on a dead route. Give wvst the same line the other two openers got: the strip's project-name check becomes a check that it opens the native page with no `iframe`, and 40dc's unchanged-check line excepts it.
