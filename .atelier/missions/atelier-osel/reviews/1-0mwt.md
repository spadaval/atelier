---
schema: atelier.review
schema_version: 3
round: 1
commit: 35a63b1a89e5947daa4f0304360ad0c8c4ac50ba
reviewers:
- 01a1eafc-0aa0-7322-bb60-156ec0a5fb34
model: grok-4.6
verdict: changes
created_at: 2026-11-17T19:29:59.876735235Z
---

## Summary

Checked the plan at 35a63b1: `mission.md` and all eight items; the blocker graph; `docs/product/desk.md` and `docs/architecture/desk.md` (T3 views, host contract, `useFollowThread`, API table); `desk/src/App.tsx`, `Shell.tsx`, `lib/host.ts`, `pages/ThisThread.tsx`, `MissionControl.tsx`, `lib/mission.ts`; `t3-extension/custom.js` (layer, T3 comments), `host-adapter.js`, `t3-custom.mjs` `buildBundle`, `install.py`, `test-extension.mjs`, `test-update.mjs`, `run-tests.mjs`; `scripts/t3-extension-release.sh`; `crates/atelier-desk/src/server.rs` and `api.rs`; and `t3-extension/spike/mockups/mission-concepts.html` on `spike/extension-react`. Did not open the mockup in a browser, run the spike inside T3 Code, reach Legend's test instance, or read another reviewer's file.

- Complexity: high. A Vite/React root, a T3-component copy pipeline, a new mission page, a typed rewrite of the 1,177-line script, two more native pages, and a Desk cutover that waits on a human install.
- Value: high. Sumeet directed the three outcomes on 2026-11-17: mission control wastes the T3 window, embedding those three pages has run its course, and `custom.js` is past one file.
- Risk of merging it: medium. A bad page shows in T3 and an old extension release rolls it back; yy4j waits on the install so This thread is not blanked. The value is worth that once the holes below are closed.

Could not check: whether every T3 component This thread and the project home need will copy as cleanly as `badge`/`kbd`; menus, popovers and dialogs, which the spike did not try.

## Finding 1: Authority's mockup rule covers pages that have no mockup
Recommendation: fix now
The mockups named in Why (`t3-extension/spike/mockups/mission-concepts.html` on `spike/extension-react`) are three concepts for the mission page. atelier-9l8h and atelier-wvst have no mockup; each says to keep what that Desk page shows today (`docs/architecture/desk.md` This thread and the project home).

Authority (`mission.md`:147-149) tells an implementer to ask before "showing something on a page that the mockups do not, or leaving out something they show". That sentence has no page limit. Drawing This thread or the project home as they are today is showing things those mockups do not contain, so each of those items would have to stop and ask for the whole page. Restrict the mockup rule to atelier-ut89, and keep 9l8h and wvst on the architecture lists they already cite.

## Finding 2: host:thread still steers the framed Desk to /t after This thread is native
Recommendation: fix now
`EmbeddedShell` in `desk/src/components/Shell.tsx`:117-129 runs `useFollowThread`: every `host:thread` that names another thread `navigate`s to `/t`. `docs/architecture/desk.md`:400-410 is that contract. The default panel today loads `/embed/` and that follow is how This thread appears (`t3-extension/custom.js`:173).

atelier-9l8h draws This thread natively and opens an item or the inbox in a frame "with a way back" (item.md:27-28). Its Context says the frame is then only for a page the human asked for; that is not a Done when line, and it does not stop `host:thread` or `useFollowThread`. A frame that still receives `host:thread` on a thread switch is sent to `/t`, the old This thread page, on top of the native one.

atelier-yy4j then deletes `ThisThread.tsx` with its routes (item.md:31-32) and drops `host:thread` "or the whole message if nothing else reads it". `useFollowThread` lives in `Shell.tsx`, which stays, because the inbox and item pages stay framed. After `/t` is gone, a remaining frame that still gets `host:thread` opens the unknown-page route. 9l8h must stop sending `host:thread` into a frame opened for a chosen page, and yy4j must remove `useFollowThread` with `/t`, not only `ThisThread.tsx`.

## Finding 3: yy4j can delete the Desk HTTP reads the native pages still call
Recommendation: fix now
The three T3 views are React routes (`desk/src/App.tsx`:23-25). The crate has no extra HTTP route for `/t`, `/home/:repo` or `/mission/:repo/:id`. What those views added on the server is `GET /api/ties`, `/api/thread`, `/api/history`, `/api/checks` and `/api/roadmap` (`crates/atelier-desk/src/server.rs`:367-374; `docs/architecture/desk.md`:562, "reads added for a host's views").

atelier-ut89, atelier-9l8h and atelier-wvst keep calling those paths through `desk-get`. atelier-yy4j's Done when (item.md:31) deletes "every hook, read and route of `crates/atelier-desk` that only they used". An implementer following that line deletes the handlers the native pages still need. Say the SPA pages, `/embed/mission`, `/embed/t`, `/embed/home`, `desk:resized` and `useFollowThread` go, and that those JSON reads stay for `desk-get`.

## Finding 4: A quiet live thread would show in both Needs you and At work
Recommendation: fix now
atelier-ut89 (item.md:28-29) puts each quiet thread in *Needs you* (T3 says live, `updated_at` unchanged for 30 minutes, the rule This thread uses) and also "a line per live thread" under *At work*. A quiet thread is live, so it would appear twice.

Concept A of `mission-concepts.html` puts the stalled review only in *Needs you*; *At work* is the threads that are still moving. `needs-you-first` wants a row for the quiet thread and a line per live thread that has changed. Tell *At work* to list live threads that are not already in *Needs you*.

## Finding 5: 3dip cannot both bundle host-adapter and keep every test assertion
Recommendation: fix now
atelier-3dip builds `src/` into one classic script whose entry imports `custom.js` and `host-adapter.js` (item.md:24), and requires `node run-tests.mjs` to pass with no change to what any test asserts (item.md:30). `run-tests.mjs` runs `test-update.mjs`.

The loader today concatenates two files into the injected IIFE (`t3-custom.mjs`:253-303: `fs.readFile` of `custom.js` and `host-adapter.js`, then `` `${adapter}\n${js}` ``). `install.py`:38 copies `host-adapter.js` by name. `test-update.mjs`:107 asserts that an installed home contains `host-adapter.js`, and :165 that an update replaces `custom.js` with a file ending `// release 0.2.0`.

A Vite bundle that includes the adapter is one file. The loader must stop concatenating `host-adapter.js` or it would run twice. Then `install.py` no longer installs that name, and `test-update.mjs`:107 fails. Pick one in atelier-3dip: keep installing `custom.js` and `host-adapter.js` by those names, with the loader still wrapping both (Vite then compiles `custom.js` only), or change `t3-custom.mjs`, `install.py` and the assertions in `test-update.mjs` in this item.

## Finding 6: 40dc's layer rewrite does not keep the native mission page
Recommendation: defer
atelier-ut89 mounts the native mission page into the layer `custom.js` already draws (item.md:45). That layer is an iframe whose `src` is `/embed/...` (`custom.js`:709-717). atelier-40dc, after ut89, rewrites "the layer over T3's main area that holds a native page or a framed one" (item.md:26) and asks that `test-extension.mjs` checks for these parts pass unchanged (item.md:30).

`test-extension.mjs`:633-637 is one of those checks: a mission title opens the layer with `src` `/embed/mission/...` and focus on `[data-t3-layer] iframe`. After ut89 that assertion has to change. 40dc's "unchanged" line would restore the iframe. Give 40dc a Done when that a mission title still opens the native page with no iframe, and split the layer tests so inbox-framed and mission-native are separate. The lead can order that when 40dc starts; it need not block the mission.
