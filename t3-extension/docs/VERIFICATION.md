# Verification

## Missions in T3's thread list (atelier-ytor), Windows, November 16, 2026

T3 Code Nightly 0.1.10-nightly.20261115, test instance (port 9333), boring
sidebar on, Desk `http://buildbot:3780`; the daily instance was not touched.
The test instance's only threads come from a paired read-only environment, so
T3 refuses to settle any of them.

`node run-tests.mjs` passes on Linux (the fixture test's 30 checks and the update
test). Each new check was run with only its piece undone, and failed:

| Piece | Undone | Failure |
| --- | --- | --- |
| Settle press stopped | No capture-phase stop | "a press on Settle reached T3, or the click did not" |
| T3's fading copies skipped | Copies walked as list rows | "A fading copy among the folder's rows" timed out (the mission's other threads fell out of it) |
| Orders cleared when off | `clearLayout` leaving `order` on T3's rows | "T3's list whole with the views off" timed out |
| Rows follow a new list | Rows left in T3's old, detached list | "A new list laid out" timed out |

`node test-contract.mjs --port 9333 --thread <atelier test thread>` passes with
every probe ok, including the new `dom:projectFolder`, `dom:folderButton`,
`dom:settleButton` and `layout:threadList`.

In the test instance, through its debug port with CDP mouse input:

- Before any change, pressing a row's Settle button and dragging two rows down
  marked all three rows swept (`bg-sidebar-row-selected`); Escape cancelled it.
- With the extension, a synthetic mission naming two of the instance's threads
  (put in for this check through a temporary copy of `custom.js`, since none of
  the Desk's missions has a thread this instance lists) showed as in the
  mockup: header with ring and `2/5`, the two threads indented with `build` and
  `lead`, the folder's other thread after them, and "2 more missions" with its
  dot in the atelier folder. The same drag swept no row, with Escape or
  released; a plain click on Settle reached T3, which answered "Failed to
  settle thread. This connection cannot change threads or projects."
- Fold and unfold, a folder closed (no mission rows) and opened, a search
  (T3's new list: no folders, no rows, no orders) and back (rows on the new
  list), and N more missions opening the project home in the layer all worked.
- Found and fixed: T3 fades a removed row by appending an inert, absolutely
  placed copy (`Sidebar.motion.ts`), the extension's own rows included. In the
  covered test window those animations never finish, so copies stayed; the
  layout now leaves them out of its walk.
- The real `custom.js` was put back; the instance shows "1 more mission".

Not verified: that a settle actually happens on a click (the environment
refuses it, after T3's handler ran), T3's slide animations against the
reordered list (the covered window runs none), and macOS.

## Re-review fixes, against the merged Desk, Windows, November 16, 2026

The Desk work has merged: these runs used the real Desk at
`http://buildbot:3780`, which now serves every route, and the test instance
(port 9333) points at it again. Same T3 build; the daily instance was not
touched.

`DESK=http://buildbot:3780 node test-extension.mjs` passes, and
`node test-contract.mjs --port 9333 --thread <atelier test thread>` passes with
every probe ok, including `dom:conversationThread`, `dom:timelineRow` and
`state:composer-prompt` through the checked write. Each new check was then run
with only its fix undone, and failed:

| Finding | Fix undone | Failure |
| --- | --- | --- |
| 1. A streaming thread is never announced | The send waits for the thread's first scan, and the scan timer restarts on every change | "host:thread on thread switches" timed out |
| 1. (the scan alone) | Only the scan timer restarting on every change | "Mentions of a thread while it streams" timed out |
| 2. The previous conversation credited to the new thread | Every scan credited to the current thread | "thread-b's conversation was credited to thread-a" |
| 3. The composer of another draft filled | `fillComposer` writing without its checks | "The refusal reported" timed out |

In the real T3, the strip's Plan a mission put "Let's plan a mission for " in
the draft store exactly, with no error, through the checked write.


## Review fixes for the sidebar views, Windows, November 16, 2026

Fixes for an independent review of `776be61..4bb7348`; same T3 build, test
instance (port 9333) and Desk (`http://buildbot:3781`) as below. The daily
instance was not touched.

`DESK=http://buildbot:3781 node test-extension.mjs` passes. Each new check was
then run with only its fix undone, and failed:

| Finding | Fix undone | Failure |
| --- | --- | --- |
| 1. Hiding outlived Missions | Hiding regardless of the views toggle and the sidebar hook | "T3's list whole with the views off" timed out |
| 2. Suffix selectors | `[data-thread-item$=":<thread>"]` again | "Mission threads hidden from T3's list, and only they" timed out (`env:other:thread-a` was hidden) |
| 3. Stale `mentioned` | One global list, sent before the new thread's scan, kept with references off | thread-b's messages were not exactly one carrying its own references |
| 4. Unreadable repository | `repos[].error` ignored | "Inbox count marked for an unreadable repository" timed out |

The route-switch check now waits for thread-b's message before switching back:
a thread is announced once its own conversation has been scanned.

`node test-contract.mjs --port 9333 --thread <atelier test thread>` passes with
the new `state:composer-prompt` probe: text put in the composer of a new-thread
screen reached T3's draft store, and the prompt was put back. Its first version
filled the composer before T3 had rendered the draft's own editor and wrote into
the test thread's draft instead; the check now waits for the draft's editor, and
the stray text was removed.


## Sidebar views, layer and project strip, Windows, November 16, 2026

T3 Code Nightly `0.1.10-nightly.20261115.4749`, test instance (port 9333, own
`APPDATA` and `T3CODE_HOME`); the daily instance was not touched. The Desk was
the reworked one at `http://buildbot:3781` (Atelier item atelier-cdxg, under
review), which serves `/api/sidebar`, `/api/launch`, `/embed/t`,
`/embed/mission/...` and `/embed/home/...`; the Desk at 3780 does not serve them
yet.

- `DESK=http://buildbot:3781 node test-extension.mjs` passes (25 checks). Its
  new checks: the whole `host:thread` (repository, children by lineage and by
  `delegatedFrom`, mentioned, resent when a mention or child comes or goes); the
  Inbox row and its count; the layer (cover, focus, every way it closes,
  `desk:open-thread`, spoofed `desk:key` and `desk:open-thread` from another
  window and from the layer window sent to another origin); the reread on
  `visibilitychange`; Missions (rows, expand and remember across reinjection,
  known and unknown threads, Other threads); hiding and Show all threads;
  mission control in the layer; the breadcrumb's project; the project strip
  (cards, a filled composer and nothing sent, a non-item ID refused, the narrow
  layout, nothing for an unmapped project); a missing sidebar hook leaving the
  panel and references working; the settings section naming each view.
- Replaced on purpose: the check that the Desk opens the inbox with "This thread
  holds no open work." for a thread holding nothing. With `repository` in
  `host:thread` (776be61) the merged Desk opens the project instead, and the
  Desk under test opens `/embed/t`; the check is now that the Desk opens
  `/embed/t` and reads the thread, its child's ties and the mentioned record.
  Open externally is checked against `/t` for the same reason.
- `node test-contract.mjs --port 9333 --thread <atelier test thread>`: all 28
  probes ok, including `module:composer-draft-store`, `route:draft`,
  `state:thread-origin` and the seven new `dom:` hooks; 5 of 325 renderer
  modules imported.
- In the test instance: the Inbox row (72), 16 missions from the Desk, the
  project strip on the atelier new-thread screen with real `waiting_items`,
  `in_motion` and `ready`; a mission row opened mission control with its frame
  focused, and a real Escape key sent into the frame closed the layer through
  the Desk's `desk:key`, focus back in T3. Every view reported on.
- Found by the new checks and fixed: the strip was 4 pixels wider than the
  composer (its padding added to the width set).

Not verified: mission threads hidden from the real T3's list, their live titles
and Other threads in the real T3 (every mission thread runs on buildbot's T3,
which the test instance is not paired with; the fixture covers them), and macOS.
While the test window is covered, Chromium marks the page hidden and throttles
its timers; the views reread when it is shown again, and the layer opens and
closes without waiting for a timer.

## Review fixes, Windows, November 15, 2026

Fixes for an independent review of `b50a8ab..9019ad1`; same T3 build, test
instance (port 9333) and Desk as below. The daily instance was not touched.

`node test-extension.mjs` passes (18 checks). Each new check was then run with
only its fix undone, and failed:

| Finding | Fix undone | Failure |
| --- | --- | --- |
| 1. Concurrent saves | Saves unserialised, `config.json` written in place (the old code) | "an older concurrent save won" (20 saves at once) |
| 2. Validator | `key in checks`, no value checks | `{"mappings":null}` was saved |
| 3. Switch during a reference load | Suppress whichever thread is current at `desk:ready` | host:thread for the thread switched to timed out |
| 4. Wrong Desk | Open on the first Desk serving the name | the mission did not open on the second Desk, which answered |
| 5. Path normalisation | Old segment pattern, no check of the parsed URL | `/api/repos/..\..\api\action?/items/x` was read |
| 6. Binding context | Accept reports from any context | a same-site frame used the injector bridge |
| 7a. Window check | Origin check only | the Desk origin from another window: `desk:open-thread` accepted |
| 7b. Origin check | Window check only | the Desk window with another origin: `desk:open-thread` accepted |

Removing only the serialisation (keeping the temporary file and rename) also
failed, on two saves renaming the same temporary file.

In the test instance: `test-contract.mjs` reports all 17 probes ok; the injector
accepts the extension's reports (the top frame's default context matches the
page target's ID in Electron too: "Customizations applied"); the Customizations
section read the Desk's 5 repositories through the bridge; the Desk still
follows the thread (inbox with "This thread holds no open work."), and toggling a
setting twice left the extension and the Desk frame in place ("Settings saved;
applied in place" twice).

Not verified in the real T3: a second configured Desk, and a same-site frame
calling the binding (T3 has none); both are covered only by the fixture.

## Following the thread and in-place settings, Windows, November 15, 2026

Same T3 build and test instance (port 9333) as below; Desk at
`http://buildbot:3780` with `host:thread` (atelier item atelier-knx8,
`desk/src/lib/follow.ts`). The daily instance was not touched.

- `node test-extension.mjs` passes (16 checks, three runs in a row). The real
  Desk app in the fixture received `host:thread` for `thread-a` on ready, read
  `/api/ties?thread=thread-a`, and opened the inbox with "This thread holds no
  open work."; a frame listener saw exactly `thread-b`, `thread-a` for the
  switches and nothing more after going to Settings and back to `thread-a`;
  removing the mount (as a T3 re-render does) reloaded the Desk and it was told
  `thread-a` again; after a reference click the Desk stayed on the item. A
  mapping saved from Settings left the extension root and the iframe in place
  (no new `/embed` document request); an outside edit of `config.json`
  reapplied the extension.
- `node test-contract.mjs --port 9333 --thread <atelier test thread>`: all 17
  probes ok, 5 of 320 renderer modules imported.
- In the test instance, in the "Atelier extension test thread" of the `atelier`
  project, opening the Desk loaded `http://buildbot:3780/embed/` and the Desk
  went to `/embed/inbox` with "This thread holds no open work." (the thread holds
  no Atelier work) (`artifacts/follows-thread.png`). After moving to
  `/embed/r/atelier` inside the Desk, switching to a thread of another project
  and back opened the inbox and its line again. Turning "Show Atelier reference
  links" off and on in Customizations kept the extension root and the Desk
  frame's window (a marker set in it survived); `injector.log` shows "Settings
  saved; applied in place" twice and no reload.

Found and fixed during this run: when T3 unmounted and re-rendered its panel slot
(switching to a thread whose panel was closed and back), the moved iframe
reloaded and the fresh Desk was not told the thread, and a post to the detached
frame threw. Each `desk:ready` now decides afresh whether to send.

Not verified: a thread that holds an item or mission in the test instance (the
only such threads are on the Desk's own T3 server, which the test instance is
not paired with), so the item and mission pages and their lines were seen only
in the Desk's own tests.

## Embedded Desk and host contract, Windows, November 15, 2026

T3 Code Nightly `0.1.10-nightly.20261115.4749`, Atelier Desk at
`http://buildbot:3780` at atelier commit `6aea272a` (embedded mode, host
contract, `/api/records`, `identity` in `/api/repos`, `embed_origins =
["t3code://app"]`). The real-T3 checks used the second T3 instance (port 9333,
own `APPDATA` and `T3CODE_HOME`); the daily instance and its installed extension
were not touched.

- `node test-extension.mjs` passes (15 checks, Edge headless). The real Desk app
  runs in the fixture through a proxy that answers `/api/embed` with the
  fixture's origin, so the theme, title, spoofing and open-thread checks
  exercise the Desk's own `host.ts`, not a stand-in.
- `node test-contract.mjs --port 9333 --thread <atelier test thread>` reports
  all 17 probes ok, including `module:environment-atoms`, `css:theme`,
  `dom:panelTabList` and `state:find-thread`; discovery imported 5 of 325
  renderer modules (56 before `toast-*` was added to the chunk-name hint).
- In the test instance, in a thread of the `atelier` project with the test
  home's `config.json` still in the old `desk.baseUrl` form: the panel loaded
  `http://buildbot:3780/embed/r/atelier`, matched by the Desk's identity
  `github.com/spadaval/atelier`; the Desk drew no sidebar or footer; the tab
  showed the Desk's page title; Open externally pointed at
  `http://buildbot:3780/r/atelier`; there was no header row and no gear
  (`artifacts/embedded-dark.png`).
- Switching T3 to light mode in its Appearance settings restyled the Desk
  without reloading it: its `--bg` became T3's `oklch(0.992 0 0)`, `--sans`
  T3's font list, and its scheme light (`artifacts/embedded-light.png`); switching
  back to dark gave `rgb(0, 0, 0)` again.
- `desk:open-thread` posted from the real Desk frame (origin
  `http://buildbot:3780`, parent `t3code://app`) went to
  `#/<environment>/<thread>` for a thread of the test instance's environment;
  an unknown thread left the route alone and showed "Thread no-such-thread is
  not in any connected T3 environment." in the panel.
- The Customizations section lists the Desk as reachable with its 5
  repositories and identities, the toggles, the current project as
  "Automatic: atelier", the `desk.repositories` entry as an override, and every
  probe (`artifacts/settings.png`).

Not verified: a real thread link clicked in the Desk (the Desk's T3 is another
server, so its thread IDs are not threads of the test instance; the message
itself was posted from the Desk frame instead), more than one Desk URL, and
macOS.

## Windows, November 15, 2026

T3 Code Nightly `0.1.10-nightly.20261115.4749` on Windows 11, Atelier Desk at
`http://buildbot:3780`. All runs used a second T3 instance of the installed app
(own `APPDATA`, `T3CODE_HOME`, debugging port 9333, auto-update off), paired
read-only with the daily T3 server; the daily instance was never attached to.

- Loader: `launch` cold-starts the instance and injects in about 2 seconds;
  `launch` with the instance already debuggable hands off and reinjects;
  `launch` with the instance running without a debugging port shows the
  "already running" dialog (the second process exits with code 130) and leaves
  it running. `status`, `reload` and `stop` work over the named pipe; `stop`
  removes every custom element without reloading the document
  (`performance.timeOrigin` unchanged).
- Injection works with T3's content security policy in force: the Desk iframe
  loads `http://buildbot:3780/r/atelier` and renders; no `Page.reload` is used.
- `test-contract.mjs` reports all 12 hooks present and that discovery imported
  4 of 320 declared renderer modules (80 when the chunk-name hint misses), no
  workers; with a renamed store shape and settings selector, or a non-existent
  thread, it exits 1 naming them.
- With the panel slot attribute renamed in the live instance, a reference click
  opened `http://buildbot:3780/r/atelier/item/atelier-plr9` externally and
  `dom:panelContent` was reported missing.
- `t3-custom.cmd` leaves `ELECTRON_RUN_AS_NODE` unset in the calling `cmd.exe`.
- In a real thread of the `atelier` project (repository key
  `github.com/spadaval/atelier`): the Desk tab, a hovercard for `atelier-plr9`
  with Desk data, and the Customizations section with every probe.
- `test-extension.mjs` passes with Edge (5 of 5 runs). With the previous
  discovery (all resource-timing scripts) it fails: reinjection imports the
  fixture's worker into the window.
- The installer upgrades a modified installed `custom.js` (kept as
  `custom.js.previous`) and the running injector reapplies it.

Not verified: macOS after these changes, and the Start menu shortcut and
injection into the daily instance (that needs the daily T3 restarted).

## macOS prototype, November 15, 2026

Validated against T3 Code Nightly `0.1.10-nightly.20261114.4669` with the
isolated Chromium suite (docking, repository matching, references, hovercards,
reinjection, native-surface restoration, read-bridge restrictions) and the
installed T3 application. The official application bundle was not modified.

The tests use existing backend records; they do not cover Desk mutations.
They are not a compatibility guarantee for other T3 builds. Runtime logs,
machine configuration, screenshots, and temporary browser profiles are local
artifacts and are not versioned.
