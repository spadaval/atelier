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
  API. The Desk panel is an iframe showing the Desk's embedded mode, and the two
  talk over the Desk's host contract (in the `atelier` repository,
  `docs/architecture/desk.md`, "The embedded mode" and "The host contract").

## Install

Requires T3 Code Nightly, Python 3 for the installer, and an Atelier Desk server
reachable from this machine. At run time the loader uses T3's own executable in
Node mode, so no separate Node installation is needed.

```sh
git clone git@github.com:spadaval/t3-code-atelier.git
cd t3-code-atelier
cp config.example.json config.json
# Edit config.json: desk.urls.
python3 install.py        # Windows: py install.py
```

| | macOS | Windows |
|---|---|---|
| T3 found at | `/Applications/T3 Code (Nightly).app` | `%LOCALAPPDATA%\Programs\t3code\T3 Code (Nightly).exe` |
| Installed to | `~/Library/Application Support/T3 Custom` | `%APPDATA%\T3 Custom` |
| Launcher | `~/Applications/T3 Custom.app` | Start menu: **T3 Custom** |
| Control command | `t3-custom` in the install folder | `t3-custom.cmd` in the install folder |
| Control channel | Unix socket `injector.sock` | named pipe `\\.\pipe\t3-custom-<hash of folder>` |

`config.json` is local and ignored by Git. `desk.urls` lists the Desks (one
today), each reachable from both the injector and the embedded browser; the older
single `desk.baseUrl` is still read. Each Desk must list `t3code://app` in its
`embed_origins`, or its embedded mode will not talk to T3. No repository table is
needed: each Desk's `GET /api/repos` gives every repository's `identity`, which
is matched against T3's `repositoryIdentity.canonicalKey`. `desk.repositories`
is now only an override: an entry `{ "repo", "slug", "repositoryKeys" }` adds
keys a repository should also match, or a reference prefix other than its name.
`settings` holds what the Customizations section edits (see
[Settings](#settings)). If `appPath` does not exist, the installer fills in the
T3 it finds.

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

**Desk** is an icon button beside T3's panel controls and a tab in its
right-panel tab strip. It uses the existing workspace-panel content area and
follows the host's resize, maximize, and close behavior. Selecting a native tab
restores native content. This is an injected surface adapter, not a registered
public T3 plugin type. Existing native surfaces remain in T3's state; their DOM
is left mounted and is hidden/inert only while Desk occupies the slot.

The panel holds only the Desk, opened at `/embed/` of the project's Desk (for
example `http://buildbot:3780/embed/`), so the Desk draws no sidebar, theme
toggle or footer of its own, and the Desk chooses the page (see Following the
thread below). The Desk tab looks like T3's own tabs: a close icon,
then the title of the Desk page shown, from the Desk's `desk:navigated` message.
Three icon buttons follow T3's tab list while the Desk is shown: Refresh (reloads
the Desk page shown), Open externally (the same page at its normal, non-embedded
address) and Customizations (opens **Settings → General → Customizations**).

On `desk:ready` the extension sends `host:theme` with T3's colours, font and
scheme, and sends it again whenever T3's theme changes; the Desk restyles without
reloading. The colours are T3's `--background`, `--foreground`, `--card`
(surface), `--border`, `--muted-foreground` (muted) and `--primary` (accent),
each resolved to a computed colour; the font is the page's; the scheme is dark
when T3's root has its `dark` class. Messages are posted with the Desk's origin
as the target, and a message counts only when it comes from the Desk iframe's
window with the Desk's origin.

When the Desk asks to open a thread (`desk:open-thread`, such as a thread link on
an item), T3 goes to `#/<environment>/<thread>` for the connected environment
whose threads include it. A thread in no connected environment opens nothing, and
the panel says so in a line at its top for 8 seconds.

**Following the thread.** The extension sends `host:thread` with T3's thread
ID, whole, as in the route: after each `desk:ready` and whenever T3's route
changes to another thread. It names the project's Desk repository with it,
so a thread that holds nothing shows its project and not the system inbox. The Desk opens the item or mission that thread holds
now, or the inbox, with one line naming the thread's part ("This thread
implements this item.", "This thread holds no open work."). The same thread is
never sent twice in a row to the same Desk, so a page the human moves to in the
Desk stays until T3 shows another thread. Every `desk:ready` is a fresh Desk that
knows no thread, so it is told again, including when T3 re-renders its panel slot
and moving the iframe reloads it. Two loads keep their page instead: a reference
clicked in the conversation opens that record, and Refresh reloads the page
shown; the Desk is not told the current thread on those, only the next one.

## Spike: missions sidebar, layer, quick launch (branch `spike/t3-home`)

A rough build to look at, against the spike Desk (`http://buildbot:3781`); untested
beyond the screenshots in `artifacts/spike/` and `artifacts/spike/round2/`.

- **Layer.** One Desk frame over T3's main area (`main[data-slot=sidebar-inset]`),
  sidebar left usable: the inbox, mission control or the project home. Esc, the
  close button, the same row again, any T3 route change, `desk:open-thread` and
  `desk:key {key: "Escape"}` from its frame close it. The frame gets focus as the
  layer appears, so the Desk's keys work at once.
- **Sidebar.** Under T3's search: the Inbox row (count from `GET /api/inbox`),
  then a Missions section from `GET /api/sidebar`, both read every minute. A
  mission row opens `/embed/mission/<repo>/<id>` in the layer; its chevron
  expands it (kept in localStorage `t3-custom:atelier:missions:v1`) to its
  threads by item, then mission threads, then "Other threads": T3 threads whose
  `lineage.parentThreadId` or `delegatedFrom.threadId` is one of the mission's.
  A thread T3 knows shows its live title and status and opens on click.
- **Hiding.** T3's own list items (`li[data-thread-item="<env>:<thread>"]`) of
  mission threads are hidden by a stylesheet; "Show all threads" turns it off.
  T3's Working and Settled counts still include them; the Missions title says
  "N hidden from T3's list" instead.
- **Project strip.** On the new-thread screen of a project with a Desk
  repository, a 176px strip under T3's composer controls, as wide as the
  composer, from `GET /api/launch?repo=<repo>`: the project name (opens the
  project home layer) and "Plan a mission" on its header line, then three cards,
  Waiting on you (opens the inbox), In motion (opens the holder's thread when T3
  knows it, else the item in the layer) and Next up (puts a start prompt in the
  composer with `execCommand('insertText')`; never sends). Under 700px of
  composer width the In motion card goes. Missing fields count as empty.
- **Project home.** The project button in T3's thread breadcrumb opens
  `/embed/home/<repo>` in the layer instead of starting a new thread.
- `host:thread` is `{thread, repository, children, mentioned}`; `children` uses
  both creator links above.
- Hooks, each a probe: `module:composer-draft-store`, `route:draft`,
  `state:thread-origin`, `dom:composerOverlay`, `dom:composerStack`,
  `dom:composerEditor`, `dom:sidebarSearch`, `dom:mainInset`, `dom:sidebarRow`,
  `dom:threadItem`, `dom:breadcrumbProject`. The bridge also allows
  `GET /api/inbox`, `/api/sidebar` and `/api/launch?repo=<name>`.

## Project context

The adapter reads the current environment/thread route, then resolves project
metadata using T3's existing authenticated connection. It keeps those credentials
inside T3's renderer. Neither Desk nor its iframe receives T3 credentials.

A project's Desk repository is, in order: a manual mapping for that project in
`settings.mappings`; else the repository whose Desk `identity` (or a
`desk.repositories` override key) equals the project's canonical repository key.
So a T3 project whose repository a Desk serves needs no configuration: T3's
`github.com/spadaval/atelier` opens the Desk's `atelier`. Names and local
directory paths are not used as identity. Manual mappings are keyed by T3
environment ID and project ID. Worktrees or clones with the same repository
identity resolve to the same Desk repository.

The project's repository picks which Desk the panel shows and whether it shows
one; the page within it is the Desk's choice. Switching to an unmapped project
shows an explicit empty state instead of the previous repository's content. Link that project in Customizations, or turn off Follow
current project and choose a pinned repository. Clicking a valid cross-project
reference explicitly opens that reference, without silently changing the mapping.

The adapter makes T3 metadata reads and local UI-store changes. Desk retains its
own existing user-authorized actions.

## Settings

The Customizations section (Settings → General) shows:

- each Desk URL, whether it is reachable, and which repositories it claims, with
  their identities;
- the toggles: the Desk surface, reference links and hovercards, and Follow the
  current T3 project (off: a pinned repository is used);
- the current project's repository, Automatic (by identity, naming the match) or
  a manual override, and the list of overrides with a Remove button each,
  including any `desk.repositories` entries;
- injector status and errors, Reload scripts, Refresh status (also rereads the
  Desks), Open customization folder, Open Desk, and every T3 hook probe.

The toggles, manual mappings and pinned repository are kept in `config.json`
under `settings`:

```json
"settings": {
  "enabled": true, "references": true, "followProject": true,
  "pinnedRepo": null,
  "mappings": { "<environmentId>:<projectId>": "http://buildbot:3780/r/atelier" }
}
```

A mapping or pinned repository names the repository by its Desk page, since two
Desks may serve repositories of the same name; `""` means not linked. A bare
name, as an earlier version saved, still works while one Desk is configured;
with several it is ignored, since it cannot say which Desk it meant. A pinned
repository that cannot be found shows as not linked and never falls back to
another. The injector refuses a save whose keys are not exactly these settings
(inherited names such as `toString` included), whose toggles are not `true` or
`false`, whose `pinnedRepo` is not a string or `null`, or whose `mappings` is not
an object of strings; `config.json` is then left as it was and the section shows
the error.

A change in the section takes effect in the page at once and is sent to the
injector as a `save-settings` request, which checks each field and rewrites
`config.json`. Saves run one at a time in the order they arrive, and each writes
a temporary file beside `config.json` and renames it over it, so the last change
wins and no reader sees half a file. The injector recognises its own write (the file is exactly what it
wrote and nothing but `settings` changed) and only updates the script a reloaded
T3 document will run: the running extension and the Desk iframe are not
reloaded (`injector.log`: "Settings saved; applied in place"). Any other edit of
`config.json`, by hand or by another program, reapplies the extension as before.
With several T3 windows, the others pick up a saved setting when they next
reload. T3's localStorage keeps only view state, which threads
had the Desk open (`t3-custom:atelier:view:v1`), and sessionStorage the last
project context while Settings is shown. Settings kept in localStorage by an
earlier version are not carried over.

## Smart references

Each Desk repository's name, or the `slug` of its `desk.repositories` override,
recognizes item/mission tokens such as `atelier-plr9`. The Desk does not report
an Atelier slug, so its repository name stands in for it; they are the same for
every repository the Desk serves today. Suffixes follow Atelier's
lowercase-letter/digit format; four characters are not assumed. Unknown prefixes
are not interpreted as Atelier references.

Rendered conversation text, including inline code, is highlighted. Fenced code,
links, controls, and editors are excluded. DOM Ranges and CSS Highlights leave
React's text nodes intact; separate keyboard links provide focus and activation.
Mouse selection and copying continue to operate on the original message text.
Visible link geometry updates on scrolling/resizing and references are rescanned
as messages stream. Existing focus/hover targets are retained across scans.

References need only T3's conversation DOM, so they keep working when the hooks
behind the Desk panel are missing; clicks then open Desk externally.

An ID is resolved with one read, `GET /api/records/{id}`, which names its
repository, kind (item or mission), title and status; with several Desks, each is
asked in `desk.urls` order until one has it. Hover or keyboard focus opens a card
with that title and status at once, then priority, holder, blockers, completion
checks and a goal excerpt from the record's own item or mission route. Click
opens the record in Desk, in its repository even from another project, and on
the Desk that answered for it. Direct
activation needs only the one read. Requests are deduplicated and cached for 30
seconds; Refresh/Retry bypasses the cache. API failure and not-found states are
shown explicitly.

Desk reads go through the background injector because Desk does not currently
advertise cross-origin access to the T3 renderer. The bridge permits only `GET`
of `/api/repos`, `/api/records/{id}` and `/api/repos/{repo}/items|missions/{id}`,
and only from a Desk in `desk.urls`. A repository segment may not hold `/`, `\`,
`?`, `#` or `%`, and the URL the injector would fetch must have exactly the path
asked for, with no query or fragment, so a dot segment, an encoded dot or a
backslash cannot turn an allowed path into another. It does not proxy arbitrary
URLs, action endpoints, credentials, or shell commands. The other bridge actions
are injector status, `save-settings`, and opening its fixed local folder.

**Who may use the bridge.** The extension reaches the injector through a CDP
binding, `__t3CustomReport`, which Chromium exposes in every execution context of
the page, child frames that share its process included. The injector therefore
acts on a report only when it comes from the main world of the top frame: it
tracks the page's execution contexts and accepts the default context whose frame
is the page target's own frame; a report from any other context (a same-site
iframe, an isolated world) is ignored and logged ("Ignored a report from outside
the top frame"). An isolated world for the extension was not used: the extension
must import T3's own module instances and read its atom registry, which an
isolated world cannot share. What this does not cover: any script running in T3's
own main world, T3's code or anything injected into it, can call the binding as
the extension does. The bridge's limits above (reads only, fixed allow-list,
checked settings) are the protection there.

## Edit and control

Installed files, in the install folder above:

- `custom.js`: extension UI, settings, project behavior, and references.
- `custom.css`: optional CSS for T3 itself.
- `host-adapter.js`: the boundary coupled to T3's private DOM and modules.
- `config.json`: app path, port, URL matching, Desk URLs, optional repository
  overrides, the extension's settings, and optional `appEnv` (environment for a
  test instance, Windows only).
- `t3-custom.mjs`: launcher, file watcher, CDP attachment, and read bridge.
- `injector.log`: timestamped operations and concrete errors.

Saving client/adapter/CSS/config files reapplies them without refreshing T3.
A change to `t3-custom.mjs` needs the injector restarted (`stop`, then
`launch`).
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
Desk's iframe are not injected separately; the iframe and T3 talk only through
the Desk's `postMessage` contract. T3's own content security policy is left in
force: it already allows `frame-src http: https:` for the Desk iframe, and
CDP-injected scripts are not subject to `script-src`. Desk reads go through the
injector because Desk sends no CORS headers. No main-process inspector or
network-facing debugging listener is enabled.

## T3 hooks

`host-adapter.js` names each private hook and probes it at startup:

- `module:workspace-panel-store`, `module:atom-registry`,
  `module:thread-snapshot-atom`, `module:environment-atoms`: found by the shape
  of their exports, not by chunk or minified export names (the registry is
  recognised by Effect's `~effect/reactivity/AtomRegistry` type id; the
  environment atoms by `environmentIdsAtom` and `connectedEnvironmentIdsAtom`).
  Discovery only imports modules the renderer itself loads as modules (its entry
  script and `modulepreload` links), never workers or other scripts, and stops
  as soon as all four are found. The last seen chunk names are tried first, so it
  normally imports 5 modules; if they change it searches the rest (56 of 320 on
  the current build, of 320 then). `__t3Custom.diagnostics().modules` lists what it imported.
- `css:theme`: T3's `--background`, `--foreground`, `--card`, `--border`,
  `--muted-foreground` and `--primary` on its root element, read each time the
  theme is sent. A missing one is named and simply not sent; the Desk keeps its
  own value for it.
- `route:thread`: the `#/<environment>/<thread>` route, checked when a thread is
  opened. The thread's project and repository come from T3's own thread
  snapshot atom, so the adapter makes no request to the T3 server and needs none of its
  credentials.
- `dom:*`: the DOM attributes the panel, references and settings attach to.
  They are confirmed when first seen, since T3 renders them only on demand.
  Opening Desk checks that T3's panel renders its content slot within 3 seconds;
  if not, `dom:panel` or `dom:panelContent` is reported missing.

A missing hook is reported by name in Customizations, in `t3-custom status`, and
in `injector.log`, and turns off only the surface that needs it: the Desk panel
needs the panel store, the registry, the thread snapshot atom and the panel
slot; opening a thread from the Desk also needs the environment atoms, and says
in the panel when they are missing; the Desk's controls go after
`dom:panelTabList`; references need the conversation DOM, and open Desk in the
browser while the panel is off; the settings section needs the settings page.

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
module (such as a worker). It also reads the theme (`css:theme`) and finds the
thread among the connected environments (`state:find-thread`). Run it after
every T3 update.

`node test-extension.mjs` tests the extension logic in an isolated browser
fixture with T3's panel DOM contract, T3's content security policy, and the real
Desk app and data. The fixture serves modules shaped like T3's hooks, so the real
discovery code runs, and a worker script that must never run in the window. The
Desk is reached through two local proxies on another site that forward to the
first Desk in `config.json`, except that they answer `GET /api/embed` with the
fixture's origin, so the real Desk app performs the host contract with the
fixture page; each proxy records every path asked for. The second proxy is a
second Desk serving the same repositories, and the first answers 404 for the
mission's record. The test's own `config.json` lists both and has no repository
table and no settings. It uses Edge on Windows
and Chrome on macOS; set `BROWSER` to use another Chromium. It checks:

- discovery, docking and resizing, and that the project is matched by the
  identity the Desk reports;
- the `/embed` prefix on the iframe and the normal address on Open externally,
  following `desk:navigated`; no header row in the panel and no floating gear;
- `host:theme` on `desk:ready` (colours, font, scheme) and again after a change
  of the fixture's root class and custom properties;
- the tab label from `desk:navigated`;
- that a Desk message is ignored from the Desk's origin in another window (a
  second frame loaded from the Desk), and from the Desk iframe's own window with
  another origin (the frame sent to a page of the second Desk); each check
  fails if only that half of the window-and-origin test is removed;
- `host:thread` on `desk:ready` (the real Desk reads `/api/ties` for the
  fixture's thread and opens the inbox with "This thread holds no open work."),
  on each switch to another thread, again to a Desk reloaded because its mount
  was moved, never twice in a row for the same thread (Settings and back), and
  for a thread switched to while a reference was still loading (the first Desk
  holds back its `/api/embed` answer to make the window wide);
- `desk:open-thread` going to `#/env/thread-b`, and the notice for an unknown
  thread;
- the bridge's allow-list (no action endpoints, no unconfigured Desk, no path
  that URL parsing would change), refused settings that would break the
  extension, and that a same-site frame which sees the binding cannot use it;
- references, keyboard hovercards, click-through (the Desk stays on the
  record) and streaming, and that a mission is resolved with `/api/records` reads
  only, never asked for as an item, and opens on the second Desk, the one that
  answered;
- a manual mapping saved from Settings landing in `config.json` under
  `settings` and applying without reinjecting the extension or reloading the
  Desk iframe, 20 saves sent at once leaving the last one in `config.json`
  without a reinjection, an outside edit of `config.json` reapplying the
  extension, and localStorage holding only view state;
- Settings reinjection, native-surface coexistence, the renamed panel slot
  fallback, and clean removal.

It reads Desk repository `atelier`, item `atelier-plr9` and mission
`atelier-g961`; replace those fixtures to test another dataset. Screenshots go
under the ignored `artifacts/` directory. `devtools.mjs` is a development helper
and is not installed. See [verification notes](docs/VERIFICATION.md).

## What the Desk still lacks

The Desk now gives repository identities, one-read record resolution, an
embedded mode and a host contract, and this extension uses all four. Still open
on the Desk's side:

1. **The Atelier slug in `/api/repos`.** The extension uses the repository name
   as the reference prefix; a repository whose slug differs needs a
   `desk.repositories` override.
2. **Hovercard fields in `/api/records/{id}`.** Priority, holder, blockers and
   completion counts still need a second read of the item or mission route.
3. **CORS for reads.** Reads go through the injector because the Desk sends no
   CORS headers.

Desk must already be reachable at the configured URL; the extension does not
configure its server or create network tunnels.
