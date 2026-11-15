# Atelier for T3 Code

An external T3 Code extension using Atelier Desk as its backend. The official
T3 application stays untouched. The local injector owns T3-specific presentation
and context; Desk owns repository records, item/mission data, and its web UI.

This is a rough macOS prototype, tested with T3 Code Nightly
`0.1.10-nightly.20261114.4669`. It uses private T3 interfaces and the Chrome
DevTools Protocol rather than a supported extension API.

## Install

Requires macOS, T3 Code Nightly in `/Applications/T3 Code (Nightly).app`,
Python 3, and an accessible Atelier Desk server. The launcher uses T3's bundled
Electron runtime; a separate Node installation is only needed for development.

```sh
git clone git@github.com:spadaval/t3-code-atelier.git
cd t3-code-atelier
cp config.example.json config.json
# Edit config.json with your Desk URL and repository identities/slugs.
python3 install.py
open "$HOME/Applications/T3 Custom.app"
```

`config.json` is local and ignored by Git. Set `desk.baseUrl` to a URL reachable
from both the Mac's injector and the embedded browser. Each repository entry
needs Desk's repository name, its Atelier `slug`, and T3's canonical repository
key. The supplied `example.com/owner/example` key is a placeholder. An
explicit project mapping in Settings can also select a configured repository.
Existing installed configuration is preserved; edit the installed copy when
updating an existing installation.

## Use

Open `~/Applications/T3 Custom.app`, or drag it into the Dock. The launcher starts
the official T3 Code Nightly with debugging on `127.0.0.1:9222` and starts or reuses
one background injector. Opening the launcher again forces reinjection. It never
quits T3; if the app was launched without debugging, quit it once and use this
launcher. The original icon and update relaunches are not intercepted.

**Desk** is available beside T3's panel controls and in its right-panel tab strip.
It uses the existing workspace-panel content area and follows the host's resize,
maximize, and close behavior. Selecting a native tab restores native content.
This is an injected surface adapter, not a registered public T3 plugin type.
Existing native surfaces remain in T3's state; their DOM is left mounted and is
hidden/inert only while Desk occupies the slot.

The panel header has only Desk controls: Refresh, Open externally, and Close.
A small bottom-right gear opens **Settings → General → Customizations**. That
section contains injector status/errors, Reload scripts, Open customization
folder, feature toggles, project mapping, and the follow-project/pinned setting.
The controls are created before loading the private T3 adapter, so an adapter
failure still leaves a path to diagnostics and reloading.

## Project context

The adapter reads the current environment/thread route, then resolves project
metadata using T3's existing authenticated connection. It keeps those credentials
inside T3's renderer. Neither Desk nor its iframe receives T3 credentials.

`config.json` maps canonical repository identities to Desk repository names and
Atelier slugs. The example uses repository and slug `example`; replace its
placeholder identity for your installation. Names and local directory paths are
not used as identity.
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

The configured slug recognizes item/mission tokens such as `example-i5ph`.
Suffixes follow Atelier's lowercase-letter/digit format; four characters are not
assumed. Unknown prefixes are not interpreted as Atelier references.

Rendered conversation text, including inline code, is highlighted. Fenced code,
links, controls, and editors are excluded. DOM Ranges and CSS Highlights leave
React's text nodes intact; separate keyboard links provide focus and activation.
Mouse selection and copying continue to operate on the original message text.
Visible link geometry updates on scrolling/resizing and references are rescanned
as messages stream. Existing focus/hover targets are retained across scans.

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

Installed files are under `~/Library/Application Support/T3 Custom/`:

- `custom.js`: extension UI, settings, project behavior, and references.
- `custom.css`: optional CSS for T3 itself.
- `host-adapter.js`: the small boundary coupled to T3's private module exports.
- `config.json`: app path, port, URL matching, Desk backend, repository/slug map.
- `t3-custom.mjs`: launcher, file watcher, CDP attachment, and read bridge.
- `injector.log`: timestamped operations and concrete errors.

Saving client/adapter/CSS/config files reapplies them without refreshing T3.
Reload scripts forces reinjection even when files did not change. Command-R also
reruns the registered script after refreshing the document. Syntax errors keep
the last working customization. Script runtime errors appear in diagnostics.
Changes to the injector process itself require restarting that helper.
`window.__t3Custom.diagnostics()` reports the current project, selected repository,
reference override, and panel selection without exposing connection credentials.

```sh
"$HOME/Library/Application Support/T3 Custom/t3-custom" status
"$HOME/Library/Application Support/T3 Custom/t3-custom" check
"$HOME/Library/Application Support/T3 Custom/t3-custom" reload
"$HOME/Library/Application Support/T3 Custom/t3-custom" stop
```

A clean stop removes custom DOM, highlights, native visibility/inert overrides,
listeners, and future-document injection. It refreshes the main renderer to
restore CSP. Quit/reopen the original app to remove the debugging listener too.
To uninstall, stop, then remove the launcher and the Application Support folder.

Only main page targets matching `t3code://app/` are injected. Preview pages and
Desk's iframe are not injected separately. The first attachment and clean stop
refresh the renderer because Chromium establishes CSP when initializing a
new document. `bypassCSP: true` is required for the current iframe implementation;
it bypasses CSP for that inspected renderer session, not just one frame-src rule.
No main-process inspector or network-facing debugging listener is enabled.

## Development and verification

`python3 install.py` installs the runtime and adapter, creates the launcher, and
adds missing configuration defaults. It preserves existing custom JS/CSS/config.
When upgrading the managed extension client, back up and copy `custom.js`
explicitly. No npm dependencies or separate Node installation are needed at run
time: the launcher uses T3's bundled Electron binary in Node mode.

`node test-extension.mjs` requires Node.js 22 or newer and Google Chrome at its
standard macOS application path. It runs an isolated Chromium fixture with T3's panel DOM
contract and real Desk reads. It checks docking/resizing, project matching and
manual mappings, safe reference rendering, keyboard hovercards, click-through,
Settings reinjection, native-surface coexistence, and clean removal. The private
host adapter also needs verification against the actual installed T3 build.
It discovers modules from preload links as well as resource timings and waits
for T3's prepared connection during startup. These private interfaces can change
with T3 updates; adapter errors identify the missing interface or environment.
`devtools.mjs` is a development helper and is not installed in the app.

This suite is an integration test against the configured live Desk backend. It
expects repository `example`, item `example-i5ph`, and mission
`example-bzmd`; replace those fixtures to test another dataset. It uses
only backend reads and isolated browser state. Generated screenshots go under
the ignored `artifacts/` directory. See [verification notes](docs/VERIFICATION.md)
for the initial prototype's validation scope.

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
