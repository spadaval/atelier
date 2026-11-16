# Atelier for T3 Code

An external T3 Code extension using Atelier Desk as its backend. The official
T3 application stays untouched. The local injector owns T3-specific presentation
and context; Desk owns repository records, item/mission data, and its web UI.

It works on Windows and macOS. It uses private T3 interfaces and the Chrome
DevTools Protocol rather than a supported extension API; every private hook it
uses is checked by name at startup (see [T3 hooks](#t3-hooks)). Last verified on
Windows against T3 Code Nightly `0.1.10-nightly.20261115.4749`.

Three layers:

- `t3-custom.mjs` (loader): launches T3 with a debugging port, attaches, injects,
  reloads on file change. It knows nothing about Atelier. Launching, process
  detection, dialogs and the control channel differ per platform, all in its
  `platform` object.
- `host-adapter.js`: the only code coupled to T3 internals.
- `custom.js`: the Atelier extension, written against the adapter and Desk's HTTP
  API. The Desk panel is an iframe.

## Install

Requires T3 Code Nightly, Python 3 for the installer, and an Atelier Desk server
reachable from this machine. At run time the loader uses T3's own executable in
Node mode, so no separate Node installation is needed.

```sh
git clone git@github.com:spadaval/t3-code-atelier.git
cd t3-code-atelier
cp config.example.json config.json
# Edit config.json: desk.baseUrl and repository identities/slugs.
python3 install.py        # Windows: py install.py
```

| | macOS | Windows |
|---|---|---|
| T3 found at | `/Applications/T3 Code (Nightly).app` | `%LOCALAPPDATA%\Programs\t3code\T3 Code (Nightly).exe` |
| Installed to | `~/Library/Application Support/T3 Custom` | `%APPDATA%\T3 Custom` |
| Launcher | `~/Applications/T3 Custom.app` | Start menu: **T3 Custom** |
| Control command | `t3-custom` in the install folder | `t3-custom.cmd` in the install folder |
| Control channel | Unix socket `injector.sock` | named pipe `\\.\pipe\t3-custom-<hash of folder>` |

`config.json` is local and ignored by Git. Set `desk.baseUrl` to a URL reachable
from both the injector and the embedded browser. Each repository entry needs
Desk's repository name, its Atelier `slug`, and T3's canonical repository key.
An explicit project mapping in Settings can also select a configured repository.
If `appPath` does not exist, the installer fills in the T3 it finds.

Running the installer again upgrades an installation: it replaces the managed
files (`t3-custom.mjs`, `host-adapter.js`, `custom.js`, `README.md`), keeping a
modified `custom.js` as `custom.js.previous`. It keeps `custom.css` and
`config.json`, only adding missing top-level keys. A running injector picks up
the new `custom.js` and adapter at once; restart it (`stop`, then the launcher)
after a change to `t3-custom.mjs`.

## Use

Open the launcher. It starts the official T3 Code Nightly with debugging on
`127.0.0.1:9222` and starts or reuses one background injector. Opening the
launcher again forces reinjection. It never quits T3: if T3 is already running
without the debugging port, the launcher says so and leaves it running; quit T3
once and open the launcher. On Windows the check is that a second T3 process
hands off to the running one and exits. The original icon and update relaunches
are not intercepted.

**Desk** is available beside T3's panel controls and in its right-panel tab strip.
It uses the existing workspace-panel content area and follows the host's resize,
maximize, and close behavior. Selecting a native tab restores native content.
This is an injected surface adapter, not a registered public T3 plugin type.
Existing native surfaces remain in T3's state; their DOM is left mounted and is
hidden/inert only while Desk occupies the slot.

The panel header has only Desk controls: Refresh, Open externally, and Close.
A small bottom-right gear opens **Settings → General → Customizations**. That
section contains injector status/errors, the state of each T3 hook, Reload
scripts, Open customization folder, feature toggles, project mapping, and the
follow-project/pinned setting. The controls are created before the T3 hooks are
looked up, so a broken hook still leaves a path to diagnostics and reloading.

## Project context

The adapter reads the current environment/thread route, then resolves project
metadata using T3's existing authenticated connection. It keeps those credentials
inside T3's renderer. Neither Desk nor its iframe receives T3 credentials.

`config.json` maps canonical repository identities to Desk repository names and
Atelier slugs. The example maps T3's `github.com/spadaval/atelier` to Desk's
`atelier` repository. Names and local directory paths are not used as identity.
Manual mappings are keyed by T3 environment ID and project ID. Worktrees or
clones with the same repository identity resolve to the same Desk repository.

Threads in the same repository preserve iframe navigation. Switching to an
unmapped project shows an explicit empty state instead of the previous
repository's content. Link that project in Customizations, or turn off Follow
current project and choose a pinned repository. Clicking a valid cross-project
reference explicitly opens that reference, without silently changing the mapping.

Project mappings, feature preferences, and per-thread Desk selection live in
T3's localStorage. Settings retain the last project context across reinjection
using sessionStorage. The adapter makes T3 metadata reads and local UI-store
changes. Desk retains its own existing user-authorized actions.

## Smart references

The configured slug recognizes item/mission tokens such as `atelier-plr9`.
Suffixes follow Atelier's lowercase-letter/digit format; four characters are not
assumed. Unknown prefixes are not interpreted as Atelier references.

Rendered conversation text, including inline code, is highlighted. Fenced code,
links, controls, and editors are excluded. DOM Ranges and CSS Highlights leave
React's text nodes intact; separate keyboard links provide focus and activation.
Mouse selection and copying continue to operate on the original message text.
Visible link geometry updates on scrolling/resizing and references are rescanned
as messages stream. Existing focus/hover targets are retained across scans.

References need only T3's conversation DOM, so they keep working when the hooks
behind the Desk panel are missing; clicks then open Desk externally.

Hover or keyboard focus opens a card with title, state, priority, holder,
blockers, completion checks, and an optional goal excerpt. Click opens the record
in Desk. Direct activation resolves the record kind even before a hovercard has
loaded. Lookup tries the item route, then the mission route only on a 404.
Requests are deduplicated and cached for 30 seconds; Refresh/Retry bypasses the
cache. API failure and not-found states are shown explicitly.

Desk reads go through the background injector because Desk does not currently
advertise cross-origin access to the T3 renderer. The bridge permits only
`GET /api/repos` and configured repositories' item/mission reads. It does not
proxy arbitrary URLs, action endpoints, credentials, or shell commands. The
other bridge actions are injector status and opening its fixed local folder.

## Edit and control

Installed files, in the install folder above:

- `custom.js`: extension UI, settings, project behavior, and references.
- `custom.css`: optional CSS for T3 itself.
- `host-adapter.js`: the boundary coupled to T3's private DOM and modules.
- `config.json`: app path, port, URL matching, Desk backend, repository/slug map,
  and optional `appEnv` (environment for a test instance, Windows only).
- `t3-custom.mjs`: launcher, file watcher, CDP attachment, and read bridge.
- `injector.log`: timestamped operations and concrete errors.

Saving client/adapter/CSS/config files reapplies them without refreshing T3.
Reload scripts forces reinjection even when files did not change. A T3 reload
reruns the registered script after refreshing the document. Syntax errors keep
the last working customization. Script runtime errors appear in diagnostics.
`window.__t3Custom.diagnostics()` reports the hook probes, current project,
selected repository, reference override, and panel selection without exposing
connection credentials.

```sh
t3-custom status   # injector state, attached pages, and every hook probe
t3-custom check    # validate the files without injecting
t3-custom reload
t3-custom stop
```

A clean stop removes custom DOM, highlights, native visibility/inert overrides,
listeners, and future-document injection, without reloading T3. Quit and reopen
T3 normally to remove the debugging listener too. To uninstall, stop, then
remove the launcher and the install folder.

Only main page targets matching `t3code://app/` are injected. Preview pages and
Desk's iframe are not injected separately. T3's own content security policy is
left in force: it already allows `frame-src http: https:` for the Desk iframe,
and CDP-injected scripts are not subject to `script-src`. Desk reads go through
the injector because Desk sends no CORS headers. No main-process inspector or
network-facing debugging listener is enabled.

## T3 hooks

`host-adapter.js` names each private hook and probes it at startup:

- `module:workspace-panel-store`, `module:atom-registry`,
  `module:shell-state-atom`: found by the shape of their exports, not by
  chunk or minified export names (the registry is recognised by Effect's
  `~effect/reactivity/AtomRegistry` type id). Discovery only imports modules the
  renderer itself loads as modules (its entry script and `modulepreload` links),
  never workers or other scripts, and stops as soon as all three are found. The
  last seen chunk names are tried first, so it normally imports 4 modules; if
  they change it searches the rest (80 of 320 on the current build).
  `__t3Custom.diagnostics().modules` lists what it imported.
- `route:thread`: the `#/<environment>/<thread>` route, checked when a thread is
  opened. The thread's project and repository come from T3's own shell state
  atom, so the adapter makes no request to the T3 server and needs none of its
  credentials.
- `dom:*`: the DOM attributes the panel, references and settings attach to.
  They are confirmed when first seen, since T3 renders them only on demand.
  Opening Desk checks that T3's panel renders its content slot within 3 seconds;
  if not, `dom:panel` or `dom:panelContent` is reported missing.

A missing hook is reported by name in Customizations, in `t3-custom status`, and
in `injector.log`, and turns off only the surface that needs it: the Desk panel
needs the panel store, the registry, the connection atom and the panel slot;
references need the conversation DOM, and open Desk in the browser while the
panel is off; the settings section needs the settings page.

## A test instance (Windows)

A second instance of the installed T3 can run beside the daily one with its own
data, so changes can be tried without touching the daily app. T3 takes its
Electron data folder from `APPDATA` (its main process sets it, so
`--user-data-dir` is not the switch to use) and its
server state from `T3CODE_HOME`; separate values give it its own single-instance
lock, server and port. Install a second copy of the extension with `--home`
(this creates no Start menu entry) and give it its own port and environment in
that copy's `config.json`:

```json
"debugPort": 9333,
"appEnv": {
  "APPDATA": "C:\\Users\\you\\AppData\\Local\\T3 Custom Test\\Roaming",
  "T3CODE_HOME": "C:\\Users\\you\\AppData\\Local\\T3 Custom Test\\t3home",
  "T3CODE_DISABLE_AUTO_UPDATE": "1"
}
```

```bat
py install.py --home "%LOCALAPPDATA%\T3 Custom Test\custom-home"
"%LOCALAPPDATA%\T3 Custom Test\custom-home\t3-custom.cmd" launch
```

Keep auto-update off in a test instance: an update installs over the shared
program folder. Before its first launch, create an empty
`<T3CODE_HOME>\userdata\cli-command-first-launch`; otherwise T3 adds the test
instance's `t3` command to your user PATH (its Settings can remove it again).
To see the daily server's threads in it, mint a pairing code with `t3 pair`
(for example `--scope orchestration:read`) and add `http://127.0.0.1:3773` under
Settings → Environments → Add environment.

## Development and verification

No npm dependencies are needed. The tests need Node.js 22 or newer.

`node test-contract.mjs --port 9333 --thread <environmentId>/<threadId>` checks
every hook against a running T3 (use a test instance): it opens the thread, the
workspace panel and Settings, prints each probe, and exits non-zero naming each
broken hook, or if discovery imported a script that is not a declared renderer
module (such as a worker). Run it after every T3 update.

`node test-extension.mjs` tests the extension logic in an isolated browser
fixture with T3's panel DOM contract, T3's content security policy, and real
Desk reads. The fixture serves modules shaped like T3's hooks, so the real
discovery code runs, and a worker script that must never run in the window. It
also renames the panel slot and checks that references then open Desk
externally. It uses Edge on Windows and Chrome on macOS; set `BROWSER` to use
another Chromium. It checks docking/resizing, project matching and manual
mappings, safe reference rendering, keyboard hovercards, click-through, Settings
reinjection, native-surface coexistence, and clean removal. It reads Desk
repository `atelier`, item `atelier-plr9` and mission `atelier-g961` from the
configured Desk; replace those fixtures to test another dataset. Screenshots go
under the ignored `artifacts/` directory. `devtools.mjs` is a development helper
and is not installed. See [verification notes](docs/VERIFICATION.md).

## Proposed Desk backend increment

This extension is the T3-specific client; the following belong in Desk:

1. **Repository metadata for integrations.** Expose the authoritative Atelier
   `slug` and a stable repository identity in `/api/repos`. The server already
   reads `.atelier/config.toml`; integrations should not infer prefixes from
   existing IDs or equate server paths with client paths.
2. **Compact reference resolution.** Resolve an ID to item or mission in one
   read, returning repository identity, kind, canonical URL, title, status,
   priority, holder, blockers, completion counts, source revision, and read-from
   information. Distinguish unknown, missing, and ambiguous references. Support
   cache validation (for example ETag) before adding bulk lookup or live events.
3. **An embedded Desk mode.** Provide a compact, repository-scoped layout that
   fits a side panel, with theme selection and less redundant global navigation.
   Global Inbox/Proposed/Motion/Disputed views need an explicit repository filter;
   simply deep-linking the repository page does not scope every other view.
4. **A narrow host navigation protocol.** Add a documented, origin-checked
   `postMessage` handshake for selecting a repository/item, reporting navigation,
   and requesting host links. Validate both source window and exact origin. This
   would preserve iframe history without changing src to convey host context.
5. **An explicit integration read contract.** Document stable errors, versioning,
   and the intended cross-origin/auth model for previews. Keep read previews
   separate from action authorization; do not broadly enable mutation access
   merely to make hovercards work.

No Desk changes above are required by this client increment. Desk must already
be reachable at the configured URL; the extension does not configure its server
or create network tunnels.
