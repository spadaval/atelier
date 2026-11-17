# Atelier for T3 Code

An external T3 Code extension using Atelier Desk as its backend. The official
T3 application stays untouched. The local injector owns T3-specific presentation
and context; Desk owns repository records, item/mission data, and its web UI.

It lives in the `atelier` repository beside the Desk, so a change to their
contract is one item and one review. It is not part of the `atelier` release.
How to write its code is in [AGENTS.md](AGENTS.md).

It works on Windows and macOS. It uses private T3 interfaces and the Chrome
DevTools Protocol rather than a supported extension API; every private hook it
uses is checked by name at startup (see [T3 hooks](#t3-hooks)). Last verified on
Windows against T3 Code Nightly `0.1.10-nightly.20261115.4749`.

Three layers:

- `t3-custom.mjs` (loader): launches T3 with a debugging port, attaches, injects,
  reloads on file change. It knows nothing about Atelier. Launching, process
  detection, dialogs and the control channel differ per platform, all in its
  `platform` object.
- `src/host-adapter.js`: the only code coupled to T3 internals.
- `src/custom.js`: the Atelier extension, written against the adapter and Desk's HTTP
  API. The Desk panel is an iframe showing the Desk's embedded mode, and the two
  talk over the Desk's host contract
  ([docs/architecture/desk.md](../docs/architecture/desk.md), "The embedded
  mode" and "The host contract").

The adapter and the extension are modules under `src/`, which Vite builds
(`pnpm build`) into one classic script, `dist/custom.js`, from the entry
`src/main.ts`. That script is what installs as `custom.js` and what the loader
injects: it defines `T3Custom`, and the loader calls `T3Custom.start(api)`.

## Install

Requires T3 Code Nightly, Python 3 for the installer and for updates, and an
Atelier Desk server reachable from this machine. At run time the loader uses
T3's own executable in Node mode, so no separate Node installation is needed.

**On a new device, from a release.** No clone is needed. Download
`t3-extension-<version>.zip` and its `.sha256` from the newest release tagged
`t3-extension-v<version>` on
[the repository's releases](https://github.com/spadaval/atelier/releases)
(it is never marked latest, which stays the newest `atelier` release), unpack
it and run its installer:

```sh
sha256sum -c t3-extension-0.1.1.zip.sha256   # macOS: shasum -a 256 -c
python3 -m zipfile -e t3-extension-0.1.1.zip .
python3 t3-extension-0.1.1/install.py        # Windows: py t3-extension-0.1.1\install.py
# Edit config.json in the install folder (below): desk.urls.
```

The installer copies the bundle's files and its `VERSION` into the install
folder, keeps an existing `config.json`, and records in it the Python that ran
it, which `t3-custom update` uses for later installs. After that the extension
updates itself from releases (see [Updates](#updates)); the unpacked folder can
be deleted.

**For development, from a checkout.** The installer copies the checkout's files
and its build instead, and the install reports its release as `development`.
Building needs Node.js and pnpm; the installer refuses, naming the command,
when `dist/custom.js` is missing or older than a file in `src/`:

```sh
git clone https://github.com/spadaval/atelier.git
cd atelier/t3-extension
pnpm install && pnpm build   # writes dist/custom.js
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
files (`t3-custom.mjs`, the built `custom.js`, `README.md`), keeping a
`custom.js` edited since the last install as `custom.js.previous`
(`custom.js.sha256` records the one installed), and removes the
`host-adapter.js` an install from before the build left, which `custom.js`
now holds. It keeps `custom.css` and `config.json`, only adding missing
top-level keys. A running injector picks up the new `custom.js` at once;
restart it (`stop`, then the launcher) after a change to `t3-custom.mjs`.

`--app <path>` names T3 Code's app and overrides `appPath`; on macOS (and on
Linux, where only tests run it) the path is an app bundle whose
`Contents/Info.plist` names its executable in `Contents/MacOS`. The release
workflow's check installs into an empty home on a Linux runner this way, with a
stand-in bundle whose executable runs Node. An install from a release bundle
copies its `VERSION` file into the install folder, and `t3-custom check` and
`status` report it as `release`; an install from a checkout reports
`development`.

## Updates

```sh
t3-custom update --check   # report the newest release; change nothing
t3-custom update           # install it if it is newer, then reload
```

`update` reads the releases of this repository from GitHub's API
(`releases` in `config.json` overrides the URL; tests point it at a local
server). Of those it takes only releases that are neither drafts nor
prereleases and whose tag is exactly `t3-extension-v<major>.<minor>.<patch>`,
so `atelier`'s own `v*` releases are ignored, and of those the highest version.
If that is newer than the installed one, it downloads the release's
`t3-extension-<version>.zip` and `.sha256`, refuses the zip unless its SHA-256
matches, unpacks it into a temporary folder, and runs the bundle's own
`install.py --home <install folder>` with the recorded Python. That keeps
`config.json` and `custom.css` as any reinstall does. It then asks a running
injector to reload. Versions compare as three numbers; `development` and
`0.0.0-dev.<commit>` (a bundle built on a pull request) count as `0.0.0`, older
than every release, so a checkout install is always offered the newest release.
The checksum proves the download is the file the release published, not who
published it: trust rests on GitHub and this repository's release workflow.

The injector checks when it starts and then once a day. It installs a newer
release by itself only when `settings.autoUpdate` is `true`, which is off by
default and is the "Install new extension releases by themselves" toggle in
Customizations; otherwise it only reports it. This is the extension's own
setting, apart from T3 Code's auto-update.

Each lookup, from the injector or from `t3-custom update`, writes its outcome
to `update.json` in the install folder: when it checked, the installed and the
newest version, whether that is newer, and the error, if any. `t3-custom status`
returns it as `update`, and the Customizations section shows it: the installed
release, the newest one, when it checked, a failed lookup, download, checksum
or install by name (for example `Looking up releases: GET <url>: HTTP 503
Service Unavailable`), and an **Update to <version>** button when a newer
release exists. A failure there changes nothing else: the installed version
stays, and the injector and the extension keep running.

Known limit: an update that changes `t3-custom.mjs` takes effect only when the
injector restarts. `reload` reinjects `custom.js` and `custom.css`, but the
running injector keeps the loader it started with. Status then
reports `restartNeeded: true` and the Customizations section says so; run
`t3-custom stop` and open T3 Custom again. An update never restarts T3 Code or
the injector by itself.

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

**Following the thread.** The extension sends `host:thread` as
`{thread, repository, children, mentioned}`: `thread` is T3's thread ID, whole,
as in the route; `repository` names the project's Desk repository (absent for an
unmapped project), so a thread that holds nothing shows its project and not the
system inbox; `children` lists the threads this one started that T3 knows, in
any connected environment, by `lineage.parentThreadId` (a `delegate_task`
subagent) or `delegatedFrom.threadId` (work a linked environment's agent started
here); `mentioned` lists the item and mission IDs the reference scanner finds in
that thread's own conversation (none while reference links are off). It is sent
after each `desk:ready`; at once when T3's route changes to another thread, with
the mentions last scanned for that thread (none if it was never scanned); and
when `children` or `mentioned` change (T3's thread list is read again every 3
seconds). The conversation is scanned 250 ms after it stops changing, and at
least once a second while it keeps changing, so a thread that streams is
scanned too.

A scan is credited only to the thread whose conversation it read, since T3 can
show the previous thread's conversation for a while after a switch. T3 wraps each
assistant message in an element naming its environment and thread
(`data-assistant-citation-environment` and `data-assistant-citation-thread`):
when the conversation has one, a scan counts only if it names the current
thread. A conversation with no assistant message is taken as the previous
thread's while its root node and first row node (`data-timeline-row-id`) are the
ones the last credited scan read, and as the current thread's once either has
changed. The same message
is never sent twice in a row to the same Desk, so a page the human moves to in
the Desk stays until something changes. Every `desk:ready` is a fresh Desk that
knows no thread, so it is told again, including when T3 re-renders its panel slot
and moving the iframe reloads it. Two loads keep their page instead: a reference
clicked in the conversation opens that record, and Refresh reloads the page
shown; the Desk is not told the current thread on those, only the next change.

## The sidebar and the layer

**Inbox** is one row under T3's sidebar search, drawn like T3's rows, with the
count of what waits on the human across every Desk in `desk.urls`: needs-human
items, proposed plans and waiting threads, from `GET /api/inbox`. A failed read
shows `!`, with the Desk and error in the row's tooltip.

**Missions sit in T3's own thread list**, around T3's own thread rows, so there
is one list. They need T3's boring sidebar (Settings → Labs → Boring sidebar),
which draws one flat list: a header per project folder with that folder's
thread rows after it, then the Snoozed and Settled shelves. In each open
folder whose threads include mission threads, the list reads:

- the folder's header;
- per open mission (from each Desk's `GET /api/sidebar`, in the Desks' order)
  with a thread in this folder: a header with a progress ring, the title, items
  done of total and a chevron, then those threads, indented on a guide line,
  each with its role after it (`lead`, `plan`, `build`, `review`, `decide`).
  Threads T3 started from a mission's threads, by `lineage.parentThreadId` or
  `delegatedFrom.threadId`, go under that mission too, with no role;
- one **N more missions** row for the repository's other open missions, with a
  dot when one of them waits on the human; it opens the project home;
- the folder's other threads, in T3's order.

The pinned rows, other folders and the shelves stay as T3 draws them. A
mission's title opens mission control; its chevron folds it, hiding its
threads, and a folded mission shows how many of its threads are here, and a dot
when something in it waits on the human. Which missions are folded is kept in
localStorage (`t3-custom:atelier:missions:v1`); a mission starts open. A closed
folder (its button's `aria-expanded="false"`) shows no mission rows; the one
row T3 keeps under it, the open thread's, stays a plain row. A mission
thread T3 does not list in the folder (unknown to this window, or pinned,
snoozed or settled) is simply not under its mission.

The extension never moves, removes or edits T3's nodes. It appends its own
`li` rows to T3's list and sets a CSS `order` on every child, read from T3's own
DOM order each time T3 changes the list (before the browser paints), and a
generated stylesheet keyed on `data-thread-item` indents, labels and folds the
mission threads. When T3 draws a new list (a search, a route change), the rows
move to it. With the boring sidebar off (no folders) nothing is reordered, and
a note under Inbox says how to turn it on; the extension does not change T3's
setting.

The Desks are read when the extension starts, every minute, and whenever the T3
window is shown again (a covered window throttles timers); T3's threads are read
again every 3 seconds.

**Gestures that take a run of rows.** Two of T3's gestures act on every row
between two others, in T3's own order, which is no longer the order drawn:
dragging from a row's Settle button settles them, and Shift-click selects them,
for bulk Settle, Snooze, Delete or a drag to the composer. While a mission is
placed, both are stopped before T3 sees them: a press on Settle, and every
click with Shift held on a thread row, whatever other key is held, since which
of those T3 takes as a range depends on the platform. A plain click on Settle
still settles that one thread, and a click and Ctrl- or Cmd-click on a row still
open and select it.

T3 fades a row it removes by appending a copy of it, and does so for the
extension's rows too. The extension removes copies of its own rows as they
appear, when the view turns off and when it stops.

**The layer.** The Inbox row, a mission's title and N more missions open the
Desk over T3's main area (`/embed/inbox`, `/embed/mission/<repo>/<id>`,
`/embed/home/<repo>`), leaving the sidebar usable; the
row shows as selected. The layer is one frame filling the main area, with a
close button and nothing else of the extension's. The frame takes focus as the
layer appears, so the Desk's keys work at once. It closes on Escape in T3,
`desk:key {key: "Escape"}` from the Desk (Escape pressed inside its frame), the
close button, its row again, a click on a thread row in T3's sidebar, any route
change, and `desk:open-thread`, which also navigates T3. It speaks the same host
contract as the panel (`host:theme` on `desk:ready` and on theme changes), and a
message counts only from the layer frame's own window with its Desk's origin.

## The project strip

On T3's new-thread screen (`#/draft/<id>`), for a project with a Desk repository,
a strip 176 pixels high and as wide as the composer sits under the composer's
controls; T3 keeps the composer where it puts it. It reads
`GET /api/launch?repo=<repo>` when the screen opens, again after a minute, and
when the window is shown again. Fields the Desk does not send count as empty.

- Its header line: the project's name, which opens the project home in the
  layer, and **Plan a mission**, which puts "Let's plan a mission for " in the
  composer.
- **Waiting on you**, with the count: the first two waiting items, then "N
  more". Any of it opens the inbox in the layer.
- **In motion**, with the count: up to three items, each with its ID, title,
  stage and how long, and the holder's thread title under it. A click opens the
  holder's thread when T3 knows it, else the item in the layer.
- **Next up**, with the count: up to three ready items. A click puts "Start
  <id>: run `atelier start <id>` in a fresh checkout and follow what it prints."
  in the composer, never sending it. The prompt is made only from an ID of
  Atelier's form (lowercase letters and digits in dash-separated parts); a row
  with any other ID is shown but disabled.

The composer is T3's ProseMirror editor: the text goes in with
`execCommand('insertText')` over a selection of its content, which fires the
input events the editor handles, so T3's draft keeps it. It goes only into the
composer of the draft the strip was drawn for: T3 must show that draft, and
afterwards T3's draft store must hold the text for that draft and no other
draft's prompt may have changed. While T3 renders, the editor on the page can
still be another thread's; a write that reached another draft is undone there,
and the click is reported as an error naming both drafts. When the composer is
narrower than 700 pixels the In motion card is dropped and the other two share
the width. A project with no Desk repository gets no strip.

## The project home

The project's name in T3's thread breadcrumb opens the Desk's project home,
`/embed/home/<repo>`, in the layer instead of starting a new thread, for a
project with a Desk repository. For any other project T3's own action runs.

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
- the toggles: the Atelier views (the Desk panel, the sidebar views, the layer
  and the project strip), reference links and hovercards, and Follow the current
  T3 project (off: a pinned repository is used), and Install new extension
  releases by themselves (off by default);
- the current project's repository, Automatic (by identity, naming the match) or
  a manual override, and the list of overrides with a Remove button each,
  including any `desk.repositories` entries;
- the installed release, in T3's own `Badge` (the extension's first part drawn
  with T3's components, see
  [Drawing with T3 Code's components](#drawing-with-t3-codes-components)), the
  last release check and an Update button (see [Updates](#updates));
- injector status and errors, Reload scripts, Refresh status (also rereads the
  Desks), Open customization folder, Open Desk, each view with whether it is on
  or which hooks it lacks, and every T3 hook probe.

The toggles, manual mappings and pinned repository are kept in `config.json`
under `settings`:

```json
"settings": {
  "enabled": true, "references": true, "followProject": true,
  "autoUpdate": false, "pinnedRepo": null,
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
had the Desk open (`t3-custom:atelier:view:v1`) and which missions are folded
(`t3-custom:atelier:missions:v1`), and
sessionStorage the last
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
advertise cross-origin access to the T3 renderer. The bridge (`deskReadUrl` in
`t3-custom.mjs`) permits only `GET`, only from a Desk in `desk.urls`, and only of
these paths:

- with no query: `/api/repos`, `/api/inbox`, `/api/sidebar`, `/api/motion`,
  `/api/records/{id}`, `/api/repos/{repo}/missions` and
  `/api/repos/{repo}/items|missions/{id}`;
- with exactly these parameters, in this order: `/api/launch?repo=`,
  `/api/history?repo=&days=` or `?repo=&ids=`, `/api/checks?repo=&pr=`,
  `/api/roadmap?repo=`, `/api/thread?id=` or `?id=&full=1`, and
  `/api/ties?thread=`.

A repository segment may not hold `/`, `\`, `?`, `#` or `%`, and the URL the
injector would fetch must have exactly the path and query asked for, with no
fragment, so a dot segment, an encoded dot or a backslash cannot turn an allowed
path into another. Query values are checked too: `repo` is letters, digits, `.`,
`_` and `-`; `days` and `pr` are digits; `ids` is record IDs joined by commas;
`full` is `1`; `id` and `thread` are a T3 thread identity, encoded. An unlisted
or repeated parameter is refused, naming the forms the path takes. The bridge
lives in the injector, so a change to it takes effect only once the injector is
restarted (`t3-custom stop`, then open T3 Custom again); reloading the scripts is
not enough. It does not proxy arbitrary
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

- `custom.js`: the built extension (UI, settings, project behavior, and
  references) with the adapter, the boundary coupled to T3's private DOM and
  modules. Its source is `src/` in a checkout.
- `custom.css`: optional CSS for T3 itself.
- `config.json`: app path, port, URL matching, Desk URLs, optional repository
  overrides, the extension's settings, and optional `appEnv` (environment for a
  test instance, Windows only).
- `t3-custom.mjs`: launcher, file watcher, CDP attachment, and read bridge.
- `injector.log`: timestamped operations and concrete errors.
- `VERSION`: the installed release; absent for an install from a checkout.
- `update.json`: the last release lookup or update and its error, if any.

Saving `custom.js`, `custom.css` or `config.json` reapplies them without
refreshing T3. To work on the extension, leave a build watching `src/` that
writes into the install folder; each save in `src/` rebuilds `custom.js` there
and the injector reapplies it:

```sh
pnpm watch --outDir "<install folder>"   # in t3-extension/
```

The next `install.py` keeps that `custom.js` as `custom.js.previous`, since it
differs from the one installed.
A change to `t3-custom.mjs` needs the injector restarted (`stop`, then
`launch`).
Reload scripts forces reinjection even when files did not change. A T3 reload
reruns the registered script after refreshing the document. Syntax errors keep
the last working customization. Script runtime errors appear in diagnostics.
`window.__t3Custom.diagnostics()` reports the hook probes, current project,
selected repository, reference override, panel selection, the last `host:thread`
sent, the layer's state and each view's missing hooks, without exposing
connection credentials.

```sh
t3-custom status   # injector state, attached pages, and every hook probe
t3-custom check    # validate the files without injecting
t3-custom update   # install the newest release (--check: only report it)
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

`src/host-adapter.js` names each private hook and probes it at startup:

- `module:workspace-panel-store`, `module:atom-registry`,
  `module:thread-snapshot-atom`, `module:environment-atoms`,
  `module:composer-draft-store`: found by the shape of their exports, not by
  chunk or minified export names (the registry is recognised by Effect's
  `~effect/reactivity/AtomRegistry` type id; the environment atoms by
  `environmentIdsAtom` and `connectedEnvironmentIdsAtom`; the draft store by a
  state with `draftThreadsByThreadKey`). Discovery only imports modules the
  renderer itself loads as modules (its entry script and `modulepreload` links),
  never workers or other scripts, and stops as soon as all five are found. The last seen chunk names are tried first, so it
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
- `route:draft`: the `#/draft/<id>` route of T3's new-thread screen, whose
  project comes from the draft store's `draftThreadsByThreadKey`.
- `layout:threadList`: T3's boring sidebar list, the parent of its project
  folders, is one flex column holding every folder, each with its button
  (`hostListProblem`). Checked each time missions are laid out; if it fails,
  the missions view turns off and T3's list is left as T3 drew it, with no
  order, row or rule of the extension's.
- `state:thread-origin`: the `lineage` field on T3's thread shells
  (`lineage.parentThreadId`), with `delegatedFrom.threadId` where a linked
  environment started the thread, read for children and Other threads.
- `dom:*`: the DOM attributes the panel, references, settings and views attach
  to. They are confirmed when first seen, since T3 renders them only on demand.
  Opening Desk checks that T3's panel renders its content slot within 3 seconds;
  if not, `dom:panel` or `dom:panelContent` is reported missing. The views'
  hooks are reported missing when T3 should show them and they are still absent
  3 seconds later: `dom:sidebarSearch` (T3's sidebar search field, whose group
  the Inbox row follows), `dom:mainInset` (`main[data-slot=sidebar-inset]`, the
  main area the layer covers) on any thread or new-thread screen;
  `dom:threadItem` (`li[data-thread-item]`) and `dom:sidebarRow`
  (`[data-sidebar-row]`) when T3 lists threads; `dom:settleButton` (a thread
  row's `button[aria-label="Settle thread"]`) when a project folder lists
  threads. `dom:projectFolder` (`li[data-testid="sidebar-project-folder"]`)
  and `dom:folderButton` (its `button[data-sidebar-row^="folder:"]` with
  `aria-expanded`) are confirmed when seen and never reported missing: without
  folders the boring sidebar is off, which the note under Inbox says; `dom:composerOverlay`,
  `dom:composerStack` and `dom:composerEditor` on a project's new-thread screen;
  `dom:breadcrumbProject` (the first button of `nav[aria-label="Thread
  breadcrumb"]`) on a thread. `dom:conversationThread`
  (`[data-assistant-citation-thread]`) and `dom:timelineRow`
  (`[data-timeline-row-id]`) tell which thread a rendered conversation belongs
  to (see Following the thread); without the first only the node rule applies,
  and without the second only the conversation's root node counts.

A missing hook is reported by name in Customizations, in `t3-custom status`, and
in `injector.log`, and turns off only the surface that needs it: the Desk panel
needs the panel store, the registry, the thread snapshot atom and the panel
slot; opening a thread from the Desk also needs the environment atoms, and says
in the panel when they are missing; the Desk's controls go after
`dom:panelTabList`; references need the conversation DOM, and open Desk in the
browser while the panel is off; the settings section needs the settings page.
The other views each need only their own: the Inbox row `dom:sidebarSearch`;
missions in T3's list `layout:threadList` and `dom:settleButton` (without a
Settle button to stop, a drag could settle the wrong rows), and, to know which
rows are mission threads, the registry, thread snapshot and environment atoms
and `state:thread-origin`; the layer `dom:mainInset` (without it, its pages open
in the browser); the project strip the draft store, `route:draft`,
`dom:composerOverlay` and `dom:composerStack`, and filling the composer
`dom:composerEditor`; the project home `dom:breadcrumbProject`.

## Known limits

- Missions need the boring sidebar. A project with open missions but no T3
  folder (no thread in the list) is not shown, and N more missions is shown
  only in a folder with a thread of a project mapped to a Desk repository.
- T3's next and previous thread shortcuts, and the thread it opens after the
  open thread is settled, follow T3's order, not the order drawn, so in a
  folder with missions they can jump between mission threads and loose
  threads. The Ctrl+1–9 jump shortcuts count in T3's order too, but T3 draws
  each hint on its row, so the row labelled is the row opened.
- While a mission is placed, a click with Shift held on a thread row does
  nothing, Ctrl+Shift-click included; select several threads with Ctrl- or
  Cmd-click. A thread selected before its mission is
  folded stays selected while hidden, and bulk actions include it.
- A copy T3 makes of one of the extension's rows after the extension has
  stopped is T3's to remove; it goes when T3's fade ends, which a covered
  window delays until it is shown.
- T3 animates its list from positions it measures before the extension
  reorders it, so a row whose place changes can slide in from the wrong spot
  for T3's 150 ms motion. Not seen on the test instance, whose covered window
  runs no animations.
- While a mission is placed, Settle cannot be dragged across rows; each thread
  is settled with its own click. If T3's Settle button is not found within 3
  seconds of a folder listing threads (an environment without settling, or a
  renamed button), missions in the list turn off rather than leave the drag
  live.
- A mission thread's item is not shown: T3's row is not changed, and the role
  is drawn from a stylesheet.
- Threads started with `t3_thread_launch` record no parent: T3 stores the caller
  only as `senderThreadId` on the first message, which the thread list does not
  carry. They appear in `children` and Other threads only when a linked
  environment started them (`delegatedFrom`).
- A mission thread is placed under its mission only when this T3 window lists
  it, so only when it is connected to the environment that runs it; the Desk
  names each by the identity its T3 server reports.
- Escape reaches the extension from inside the layer only through the Desk's
  `desk:key`; a Desk page that does not send it keeps Escape to itself.
- The layer and the strip always use the first Desk in `desk.urls` for the
  inbox.

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

## Releases

The extension is released on its own tags, apart from `atelier`. To release
the newest `master`, push a tag naming the next version:

```sh
git fetch origin && git push origin origin/master:refs/tags/t3-extension-v0.1.1
```

The tag starts `.github/workflows/t3-extension-release.yml`. It builds no
Rust and runs no test of its own. It refuses a tag on a commit that is not a
merge on `master`, and one where CI did not pass, with the `extension` job run
in it, on the last merge at or before the tag that changed something that job
tests (`scripts/t3-extension-release.sh check`, which reads the rule from
`scripts/ci-jobs.sh`). A tag on a later merge that changed only records is
accepted on the strength of that one. If CI is still running, re-run the
workflow once it passes. The release holds `t3-extension-<version>.zip` and its
`.sha256`. The zip holds the files of `t3-extension/` at the tag but the build's
inputs (`src/`, `package.json`, `pnpm-lock.yaml`, `tsconfig.json`,
`vite.config.ts`), the built `dist/custom.js` and `VERSION`, and installs with
no Node beyond T3's own executable. It is
never marked the repository's latest release, which stays the newest `atelier`
one, and its notes list the items merged since the previous extension tag that
changed `t3-extension/`. On a pull request that changes the extension or the
workflow, and when started by hand, it builds and checks the bundle as the
run's artifact and publishes nothing.

## Drawing with T3 Code's components

The extension draws with T3 Code's own React components rather than imitations
of them. They are copied, unchanged but for their imports, from a T3 Code
checkout into `src/t3/` by `sync-t3.mjs`, with T3's `cn`, its Tailwind variants,
theme mapping, utilities and the base-layer defaults they rely on
(`src/t3/theme.css`), its whole stylesheet for the check below
(`src/t3/index.css`), its MIT licence (`src/t3/LICENSE`) and the commit copied
from (`src/t3/SOURCE`). Nothing in `src/t3/` is edited by hand. The components
are `badge`, `kbd`, `tooltip`, `collapsible-section-header` and `button`; a page
that needs another adds its name to `components` in `sync-t3.mjs`.

A React tree is drawn by `mountReact(host, api)` (`src/react/mount.tsx`) in a
shadow root of its own, which adopts the built stylesheet, so T3's page and the
extension cannot restyle each other. The colours are T3's variables, inherited
from its page, so a theme or Appearance change in T3 reaches them. Three things
make a shadow root draw as T3 does, each done once and checked below:

- T3 marks dark mode with a class on `<html>`, which a selector in a shadow root
  cannot see. `sync-t3.mjs` rewrites T3's `dark`, `light` and `wco` variants to
  `:host-context(...)`, and stops, naming the line, if T3's form of one changes.
- Tailwind's `@property` rules, which give its shadow and ring variables their
  initial values, do nothing in a shadow root; `mountReact` registers them on the
  document once and removes them when the extension goes.
- Base UI portals a tooltip to `<body>`, outside the stylesheet. `mountReact`
  gives each tree a portal container inside its shadow root, and `sync-t3.mjs`
  points T3's `TooltipPopup` at it. T3's tooltip also reads its title bar height
  from T3's app; `src/react/windowControlsOverlay.ts` restates that hook.

T3's plain `@theme` (its font variables, which Settings → Appearance changes on
`<html>`) is copied as `@theme reference`, so the shadow root reads T3's values
rather than declaring defaults over them. A tree reads the Desk with
`useDeskRead(desk, path)` (`src/react/desk.ts`), through the injector's gate,
again every 15 seconds while the page is visible; a failed read is an `error`,
with the last data kept, never empty data. The roots unmount when the extension
is reloaded or disposed.

**After a T3 Code update**, copy again and check:

```sh
git -C ~/projects/t3code pull                 # or any T3 Code checkout
node sync-t3.mjs ~/projects/t3code            # refuses a checkout with local changes to what it copies
pnpm build && node run-tests.mjs
git diff src/t3/                              # what T3 changed, to review with the copy
```

`test-components.mjs`, which `run-tests.mjs` runs, proves the copy still draws
as T3 does. It builds `check/Probe.tsx`, every copied component in each variant
and size, with the extension's own build, and T3's stylesheet as last copied
the way T3 builds it. A page carrying T3's stylesheet, with a custom font on
`<html>` as Appearance would set, draws the tree twice: in the page itself, as
T3 would, and through `mountReact` in a shadow root. It compares 47 computed
properties of every element and its `::before` and `::after` (colours, borders,
outline, radius, shadows, font, size, spacing, icon stroke) in light and in
dark, and fails naming the element and property where they differ. It also
hovers a tooltip trigger in the shadow root and fails if the tooltip opens
outside it or unstyled. What it does not prove: that T3's running app has the
stylesheet last copied (copy again after an update), or anything about
behaviour beyond the tooltip opening.

## Development and verification

The build needs Node.js and pnpm: `pnpm install`, then `pnpm build` writes
`dist/custom.js`, `pnpm watch` rebuilds it on each change in `src/`, and
`pnpm typecheck` runs TypeScript in strict mode on the TypeScript sources
(the JavaScript ones are not checked yet). The tests need Node.js 22 or newer
and run the built script, so build first; `run-tests.mjs` refuses without a
build. `test-components.mjs` builds what it checks itself (see
[Drawing with T3 Code's components](#drawing-with-t3-codes-components)).

`node run-tests.mjs` runs every test here that needs no real T3 Code: each
`test-*.mjs` but `test-contract.mjs`, one at a time. It exits non-zero and
names each test that failed. A new test is a new `test-*.mjs` file; the runner
and CI find it. `test-extension.mjs` drives the extension in a fixture page in
headless Chrome. `test-update.mjs` needs no browser or Desk, but needs
`python3`: it builds release bundles from this folder and its build, installs
one into an empty home with a stand-in for T3 Code's app, and runs the installed
`t3-custom update` and injector against a local releases list. `test-fixture-desk.mjs` checks that the fixture Desk's
address is read only from a whole "serving on" line, however its output is
split.

**In CI.** The `extension` job in `.github/workflows/ci.yml` installs the
extension's packages from its lockfile, runs `pnpm typecheck` and `pnpm build`,
then runs `node run-tests.mjs` on Linux in headless Chrome, after building
`atelier-desk` (`cargo build -p atelier-desk`) and the Desk's app
(`pnpm build` in `desk/`) from the same commit, so a change to the Desk that
breaks the extension fails its own pull request. It runs when a change
touches `t3-extension/`, `desk/`, `crates/atelier-desk/`, the workspace's
`Cargo.toml`, `Cargo.lock` or `rust-toolchain.toml`, `.github/workflows/ci.yml`
or `scripts/ci-jobs.sh` (`scripts/ci-jobs.sh` holds the rule). A change to
`crates/atelier-core` alone does not run it, on purpose: the fields the
extension reads are defined in `atelier-desk`, and the `rust` job tests the
core.

**On your machine**, either way:

```sh
# Let the test start a Desk from this checkout:
cargo build -p atelier-desk
pnpm --dir desk install && pnpm --dir desk build
pnpm --dir t3-extension install && pnpm --dir t3-extension build
node t3-extension/run-tests.mjs

# Or start the fixture Desk yourself, anywhere that has the build, and point
# the test at it, such as from Windows at a Desk on another host:
node t3-extension/fixture-desk.mjs     # prints the Desk's URL
DESK=http://127.0.0.1:<port> node t3-extension/run-tests.mjs
```

`fixture-desk.mjs` writes the fixture repository into a temporary directory
and serves it with `atelier-desk` (`ATELIER_DESK` overrides the binary,
`DESK_APP` the app) on a free loopback port, fetching nothing. A Desk named by
`DESK` must serve that repository: the test checks that it does and stops if
not. On Windows set `BROWSER` if Edge is elsewhere; on Linux it defaults to
`/usr/bin/google-chrome`.

`node test-contract.mjs --port 9333 --thread <environmentId>/<threadId>` checks
every hook against a running T3 (use a test instance): it opens the thread, the
workspace panel, a new-thread screen (the first draft that names a project, or
`--draft <id>`) and Settings, prints each probe, and exits non-zero naming each
broken hook, or if discovery imported a script that is not a declared renderer
module (such as a worker). It also reads the theme (`css:theme`), finds the
thread among the connected environments (`state:find-thread`), reads who
started each thread (`state:thread-origin`), and on the new-thread screen fills
T3's composer with the same checked write the project strip uses and checks that
T3's own draft store holds the text for that draft alone
(`state:composer-prompt`), then puts the draft's prompt back. If T3 does not
render the draft's own editor within 10 seconds it reports that and writes
nothing. Run
it after every T3 update.

`node test-extension.mjs` tests the extension logic in an isolated browser
fixture with T3's panel, sidebar, breadcrumb and new-thread DOM, T3's content
security policy, and the real Desk app and data. The fixture serves modules shaped like T3's hooks, so the real
discovery code runs, and a worker script that must never run in the window. The
Desk is reached through two local proxies on another site that forward to the
fixture Desk (or `DESK`), except that they answer `GET /api/embed` with the
fixture's origin, so the real Desk app performs the host contract with the
fixture page; each proxy records every path asked for. The second proxy is a
second Desk serving the same repositories, and the first answers 404 for the
closed mission's record. The first forwards `/api/sidebar` to the real Desk,
whose fixture repository holds one open mission led by thread-b, with a plan
reviewer T3 does not know and an item held by thread-a, and a second open
mission with no threads and a needs-human item, and answers
`/api/launch?repo=atelier` itself, since its rows include IDs no record can
have; the second lists no missions. The fixture's T3 state has a subagent of thread-a
(`lineage.parentThreadId`) and a thread started from thread-b by a linked
environment (`delegatedFrom`). Its sidebar is shaped like T3's boring sidebar: one
flex column of a pinned header, two project folders with their thread rows and
a Settled header, whose folders close by removing their rows and leaving a
faded copy of each, as T3 does, and whose Settle buttons record a press and a
click. The test's own `config.json` lists both and has
no repository table and no settings. It uses Edge on Windows, Chrome on macOS
and `/usr/bin/google-chrome` elsewhere; set `BROWSER` to use another Chromium.
It checks:

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
- `host:thread` on `desk:ready` (the real Desk opens `/embed/t` and reads the
  thread, its child's ties and the mentioned record), on each switch to another
  thread, again to a Desk reloaded because its mount was moved, never twice in a
  row with the same content (Settings and back), and for a thread switched to
  while a reference was still loading (the first Desk holds back its
  `/api/embed` answer to make the window wide);
- the whole `host:thread` message: the repository, children by lineage and by
  `delegatedFrom`, the mentioned IDs, and that it is sent again when a mention
  or a child comes or goes;
- `desk:open-thread` going to `#/env/thread-b`, and the notice for an unknown
  thread;
- the bridge's allow-list: each read the native pages make reaches the Desk
  exactly as asked, and no action endpoint, unconfigured Desk, path that URL
  parsing would change, fragment, backslash, unlisted or repeated parameter
  gets through; refused settings that would break the
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
- the Inbox row under T3's search group and its count from both Desks; the
  layer: exactly T3's main area, the frame focused once its Desk is ready,
  closing on `desk:key` Escape (and focus leaving the frame), Escape, the close
  button, the row again, a thread row and `desk:open-thread` (which navigates);
  that `desk:key` and `desk:open-thread` are ignored from the Desk's origin in
  another window and from the layer's window with another origin; a reread of
  the inbox and missions when the window is shown;
- missions in T3's list, read as drawn (each visible child by its top edge):
  in the atelier folder the mission's header, its threads in T3's order,
  indented, with their role (none on the threads they started), "1 more
  mission" with its dot, then the folder's loose thread; the other folder
  untouched; folding (kept in localStorage, and after a reinjection) to the
  thread count and dot; a press on Settle and a click on a row with Shift and
  any other keys not reaching T3 while a mission is placed, and reaching it when none is, and a
  click on Settle and a Ctrl-click reaching it; copies of a mission header
  removed while the view is on and after it turns off; a
  row T3 adds in place, a folder closed (no mission rows, the open thread's row
  kept plain), opened by its button alone, and opened again,
  T3's fading copies of removed rows, and a new list, each laid out again; the
  boring sidebar off and the views off, each leaving no order, row or rule of
  the extension's in T3's list, the first with its note; a mission's title
  opening mission control in the layer, selected and focused, and N more
  missions the project home;
- the breadcrumb's project opening the project home without T3's own action;
- the project strip: its three cards and their rows from the fixture's launch
  data, as wide as the composer and under it; a Next up row filling the
  composer, nothing sent, a row whose ID is not an item ID disabled; Plan a
  mission; In motion opening its holder's thread or the item, Waiting the inbox,
  the project name its home; the narrow layout under 700 pixels; nothing for an
  unmapped project and no launch read for it;
- that without T3's search field the Inbox row turns off by name while
  missions, the panel and references keep working; that a thread list not
  shaped as missions need turns the missions view off by name and leaves no
  order, row or rule of the extension's in it; and that the settings section
  names each view and its hooks;
- Settings reinjection, native-surface coexistence, the renamed panel slot
  fallback, and clean removal of every view.

Its records are those `fixture-desk.mjs` writes, with fixed IDs, titles and
statuses: repository `atelier` with identity `example.invalid/fixture/atelier`,
open mission `atelier-fm01` with items `atelier-fi01` (building, on its branch),
`atelier-fi02` (needs-human) and `atelier-fi03` (done), open mission
`atelier-fm03` with item `atelier-fi04` (needs-human), closed mission
`atelier-fm02`, and item `atelier-rf01` in no mission, which the conversations
name. Every other read reaches the real Desk, except what the proxies answer
themselves: `/api/embed`, `/api/launch`, the unreadable repository added to
one `/api/inbox` answer, and the 404 for `atelier-fm02` on the first Desk. A
change to those in the Desk does not fail this test. Screenshots go
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
