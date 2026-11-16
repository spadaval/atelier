# Verification

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
