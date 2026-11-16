# Verification

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
