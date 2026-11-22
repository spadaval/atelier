# The Desk's API

Status: the contract for mission atelier-z4cr, closed on 2026-11-11,
written by item atelier-1ie1 before any server code, and corrected by items
atelier-uwxy, atelier-e05n and atelier-h3hl where the server as built
differs. Item atelier-v958 (mission atelier-to83) pointed it at the live
repositories, read from branches local and fetched, added the pages of what
is proposed, in motion and disputed, fetching, the stop and being told, and
removed "say go" and the answers to questions. It says what each page
of the Desk is sent, and for every field, which function of the redesigned
core in `crates/` gives it, or that none does yet. The decisions are in
[the design record](../design.md#decisions); what the Desk is
for is in [its product intent](../product/desk.md).

## Verdict

The core can feed every page. Nothing in the strategy's Design has to
change. Two facts shape the order of work:

- **Reads need atelier-psjw too, not only actions.** On master, `Located`,
  `Entry` and `entries` are in `atelier-core`, but the code that builds a
  `Located` for every item (the base's record plus the branch named for it,
  `Repo::everything`) is in `atelier-cli/src/repo.rs`. The atelier-psjw
  branch moves `Repo` into `atelier-core`. Until it merges, the Desk cannot
  read items the way `status`, `list` and `show` do without copying CLI code.
- **Seven views were missing**, listed with sizes in
  [Missing views](#missing-views). Six were small moves or exposures of code
  that existed, and atelier-l5ai has added them. One, the history of actions (`atelier log`), was added
  later, by atelier-js4a; only the debrief's timeline and its
  plan-against-actual need it, so the mission page shipped without them.

In the tables below, **master** means the function exists on master today
(3715b5f9), **psjw** that it exists on the atelier-psjw branch (55d88b72)
and not on master, and **M1** to **M7** name a view that was missing.
atelier-psjw has since merged, so its marks now mean on master too, and
atelier-l5ai added M1, M2, M3, M4, M6 and M7: where a table names one of
those, the function that now gives it follows. atelier-js4a added M5.

## How the server reads

For each repository under its configured roots, a request does this, inside
one blocking task:

1. Open the repository for reading with the human's identity as caller:
   `atelier_core::Repo::open_committed(dir, caller, &skip)` (M1).
   The caller is the name the Desk is configured with, such as `ada`,
   never a value from the request.
2. `Repo::everything()` gives a `Located` for every item on the base, on
   the branch named for it, local or the forge's remote-tracking one, and on
   every unmerged named branch that adds or changes it (atelier-dciw).
   `Repo::all_missions()` does the same for missions, and
   `Repo::published_stops` marks dropped a mission the forge's base holds
   dropped, as `list` does.
3. `atelier_core::entries(&items, &missions, &caller)` (master) gives one
   `Entry` per item, with the record the cross-branch rule picks
   (`Located::record`, master).
4. Pages are built from those values and the record readers below.

The cache key is the base's tip and the tip of every local and
remote-tracking branch (one `git for-each-ref` of `refs/heads` and
`refs/remotes`), with the checkout opened and its `HEAD`, whose commit holds
the config read, so a commit or a fetch builds a response again. A request opens the
repository and reads those tips; while they are unchanged it is answered
from the response kept for them, and a new commit on any of them builds the
response again, with no restart. Nothing reads a working tree:
a `Repo` opened with `open_committed` reads `.atelier/config.toml` from the
commit `HEAD` names, and the item of the branch the opened checkout has out
from that branch's tip, where `Repo::open` reads both from the checkout's
files. `HEAD` can still be an old commit: on this host the main atelier
checkout is detached far behind master, so its `HEAD` holds the config of
that old commit, not the base's. The server opens the checkout that has the
base out, `atelier_core::git::checkout_of(root, base)`, where there is one.
Planning's and the actions' methods of `Repo` (`store`, `checkout_item`,
`locate_planned`, `planned`, `new_id`, `uncommitted`) read the checkout,
and the server does not call them.

`Repo` holds `&Skip`, and `Skip` holds a `RefCell`, so a `Repo` is not
`Send`. The server makes the `Skip` and opens the `Repo` inside the
blocking task that uses them.

## Running the server

The crate is `crates/atelier-desk`, and its binary is `atelier-desk`. It
is not part of the tracker's release: its manifest sets
`[package.metadata.dist] dist = false`, and no other crate depends on it,
which its `dependencies` test checks.

It takes one argument, the configuration file:

```console
$ cd next && cargo build -p atelier-desk
$ target/debug/atelier-desk --config ~/.config/atelier-desk/config.toml
atelier-desk: built from 66af635eef253e133ab4630aba02285b943431f6, serving on http://127.0.0.1:3780
```

It serves until it gets SIGINT or SIGTERM, then revokes its T3 Code
session and exits with success (`tests/signals.rs`). The commit it names is
the checkout's `HEAD` when it was built (`build.rs`), and `GET /api/repos`
gives it too; changes not committed are not marked. The configuration is
TOML:

```toml
# The name the human acts under; every view is computed for it.
caller = "ada"
# Repositories served, each under its directory name, first. One that does
# not open, or holds a record that does not read, is listed by /api/repos
# with its error.
repos = ["~/projects/atelier", "~/projects/life-on-rails"]
# Each direct child of a root that is a Git checkout whose HEAD holds
# .atelier/config.toml is served under its directory name too; a child with
# no Atelier config is skipped. Optional; at least one of repos and roots.
roots = []
# How often, in seconds, each repository's remote is fetched: the forge's
# remote, else origin. 0 fetches nothing. Default 300.
fetch_seconds = 300
# The Desk's own directory: news.json, what it has told of, and work/, the
# clones it writes a stop in. Default ~/.cache/atelier-desk.
state = "~/.cache/atelier-desk"
# The Desk's address as the human opens it, for the links it sends.
# Optional; without it a link is a path.
url = "http://buildbot.tail47515a.ts.net:3780"
# The gh command, for pull requests on a GitHub forge. Default gh.
gh = "gh"
# Addresses to listen on. Default: 127.0.0.1:3780.
listen = ["127.0.0.1:3780"]
# The names a request's Host header may give, without its port. Any other
# is refused with 403. Default: localhost, 127.0.0.1 and [::1].
hosts = ["localhost", "127.0.0.1"]
# The built app's directory, served for every path outside /api, with its
# index.html for a path that names no file. Without it only /api is served.
app = "/path/to/atelier/desk/dist"
# The origins of the pages that may frame the Desk in its embedded mode and
# talk to it (The host contract). Each is scheme://host[:port], with no
# path; `*` and `null` are refused. Default: none, and the framed Desk then
# sends and reads nothing.
embed_origins = ["t3code://app"]

# How to reach T3 Code. Without this table the Desk makes no call to T3
# Code and every response reports its threads as unavailable. Both keys
# are needed.
[t3]
# The t3 binary that matches the running T3 Code, which the Desk runs for
# a token: not the older t3 on PATH. The service names a wrapper that
# finds it (see Running it as a service).
command = "~/.local/share/atelier-desk/t3"
# T3 Code's MCP server.
url = "http://127.0.0.1:3773/mcp"
# The first message of a planning session's thread. {mission} and {repo}
# are replaced by the mission's ID and the repository's name. Optional; the
# default is given under POST /api/sessions.
session_message = "Start a planning session for mission {mission} ..."

# How the human is told when a plan is proposed or merges. Optional; without
# it the inbox lists the news and nothing is sent.
[notify]
# The program and its first arguments; the Desk adds a title, a line of
# text and a link to the plan's page, as three more arguments.
command = ["~/.local/share/atelier-desk/notify-mail", "personal"]
```

A path may start with `~/`, and a relative one is read from the
configuration file's directory; that holds for `t3.command` and the program
of `notify.command` too. When two repositories have the same name, the first
is served and the second is listed with an error.

The app's TypeScript types are generated from `src/api.rs` into
`desk/src/api/`, one file per type, and committed. The test
`the_committed_typescript_types_are_current` fails when they differ, and
writes them again when run with `ATELIER_UPDATE_TYPES=1`:

```console
$ cd next && ATELIER_UPDATE_TYPES=1 cargo nextest run -p atelier-desk --test types
```

## Running it as a service

On this host the Desk runs as the systemd user service `atelier-desk`,
reached over the tailnet at <http://buildbot.tail47515a.ts.net:3780/>
(item atelier-va99). It serves the five live repositories under
`~/projects/` (atelier-v958): atelier, astromech, demiurge, life-on-rails and
cosplay-studio. Everything it needs is in `desk/service/`:

| File in the repository | Installed as | What it is |
| --- | --- | --- |
| `atelier-desk.service` | `~/.config/systemd/user/atelier-desk.service` | The unit |
| `t3` | `~/.local/share/atelier-desk/t3` | The `[t3] command`: runs the `t3` of the T3 Code version that is running |
| `notify-mail` | `~/.local/share/atelier-desk/notify-mail` | The `[notify] command` on this host, off until the human turns it on |
| `notify-mail.test.sh` | | Checks `notify-mail` against a `gog` stand-in; nothing is sent |
| `install.sh` | | Builds, installs or updates, restarts and checks the service |
| (written by `install.sh` once) | `~/.config/atelier-desk/config.toml` | The configuration |
| (the release build) | `~/.local/share/atelier-desk/atelier-desk` | The server |
| (`desk/dist`) | `~/.local/share/atelier-desk/app/` | The built app |
| (the Desk's state) | `~/.cache/atelier-desk/news.json`, `~/.cache/atelier-desk/work/<repo>/` | What it has told of, and the clones it writes a stop in |

**The unit** starts at boot (`WantedBy=default.target`, with lingering
enabled for the user) and has the limits of `atelier-serve.service`:
`MemoryMax=1G`, `CPUQuota=100%` and `Nice=10`, so it cannot starve T3 Code.
`Restart=on-failure` with `RestartSec=5` restarts it after a crash or a
SIGKILL; `systemctl --user stop` sends SIGTERM, on which it exits cleanly
and stays stopped. `StartLimitIntervalSec=0` lets systemd restart it
without limit.

**At boot**, the tailnet address exists only once `tailscaled`, a system
service, has brought `tailscale0` up, and a user unit cannot be ordered
after a system unit. Until then binding `100.124.153.80:3780` fails, the
server exits with an error, and systemd starts it again five seconds later,
until the address exists. Loopback is not served in that time either.

**The configuration** listens on `127.0.0.1:3780` and the tailnet address
`100.124.153.80:3780`, never on `0.0.0.0`, and answers the hosts
`localhost`, `127.0.0.1`, `100.124.153.80`, `buildbot` and
`buildbot.tail47515a.ts.net`; any other `Host` is refused with 403. It is
not published with `tailscale serve` or `funnel`. Port 7341 is the old
site's and port 80 on the tailnet address is T3 Code's. `install.sh` writes
it from `tailscale status` when it does not exist and never overwrites it.
It holds no token: the server gets each from the `t3` command and keeps it
in memory.

**The `t3` command after a T3 Code update.** An update adds a directory
under `~/.t3/runtime/versions/` and removes old ones, so the configuration
does not name a version. It names the wrapper `~/.local/share/atelier-desk/t3`,
which reads `activeVersion`, the version T3 Code runs, from
`~/.t3/runtime/service-state.json` and runs that version's `t3`. The Desk
runs it for each new token, so after an update the next token comes from
the new version with no step of ours. `~/.local/share/atelier-desk/t3
--version` prints the version it finds.

### Install

Once, from a clean checkout of the branch to install:

```console
$ desk/service/install.sh --caller ada
```

`install.sh` builds `atelier-desk` (release, `-j
4`, under a minute on this host) and the app, copies them and the wrapper
into `~/.local/share/atelier-desk/`, writes the configuration, installs and
enables the unit, restarts it, and fails unless the server answers on
loopback with the commit it was built from. It refuses a checkout with
changes not committed, since the commit it reports would not mark them.

### Update

From a clean checkout of `master` after a merge, such as the one with
`master` out:

```console
$ desk/service/install.sh
```

`install.sh` keeps the configuration. The server reads each repository's
commits and refs on every request, so a commit or a fetch shows at once.
A configuration written before atelier-v958 names the converted clones in
`roots`; replace that line with the `repos`, `fetch_seconds`, `state` and
`url` lines `install.sh` writes now.

### Check

```console
$ systemctl --user status atelier-desk
$ curl -s -H 'Host: localhost' http://127.0.0.1:3780/api/repos | jq -r .built_from
$ curl -s -o /dev/null -w '%{http_code}\n' http://buildbot.tail47515a.ts.net:3780/api/repos
$ curl -s -o /dev/null -w '%{http_code}\n' -H 'Host: elsewhere.example' http://127.0.0.1:3780/api/repos   # 403
$ journalctl --user -u atelier-desk
```

The two checks against the live service, `desk-matches-cli` and
`action-is-a-record`, are commands in `desk/live/`; `desk/README.md` says how
to run them.

## The app

The app is `desk/` (item atelier-94pv; `desk/README.md` has its commands).
Its routes are `/` (repositories), `/inbox`, `/proposed`, `/motion`,
`/disputed`, `/r/{repo}` and
`/r/{repo}/mission/{id}` (the rail and one mission, with its Overview,
Work and Documents tabs as `?tab=`, and a document as `?doc=<path>`),
`/r/{repo}/item/{id}`, and `/agents` and `/r/{repo}/agents` (T3 Code's live
threads from `GET /api/threads`; the second shows only the threads, and the
holds, of that repository). Three more are views a host shows beside its
threads, described under [Views a host shows](#views-a-host-shows): `/t`
(this thread), `/mission/{repo}/{id}` (mission control) and `/home/{repo}`
(the project home). Where it reads the API differently from what the tables
below suggest:

- A document is listed under its file name, with its first heading
  beneath, since the converted `design.md` files begin with `# Sizing`. Its
  YAML front matter is shown as fields, not as Markdown.
- A relative link to a Markdown file, in a document, a mission's section or
  an item's text, opens that file in the mission's Documents tab, so any
  Markdown file of the repository can be shown there, listed or not. A link
  from an item in no mission, or to a file that is not Markdown, is shown as
  text.
- A done item's Report and its findings fixed in review are read from
  `/items/{id}` when the human opens them on the Work tab; the item row has
  only `report_written`.
- An agent with no `thread` is shown with "not running in T3 Code" only
  when `t3` has a list to judge by (`reachable`, or an `age_seconds` from
  before a failed poll). With no list read, the page says threads are
  unavailable and names the identities without that claim.
- An identity is shown whole, cut short on screen only, with the whole
  value in its text and title. No link holds one.
- "Stop this plan" and Start a planning session are the app's only action
  controls. The stop asks for their reason, on one line, before it sends; a
  session asks once. A stop that was pushed and whose pull request did not
  close stays on the page as the server gave it, with the error and the `gh
  pr close` that finishes it, under the note that the stop is not
  published yet. After a `done`, the app reads every page on screen
  again before showing the action's text. An agreed mission's page shows the
  commands that stop it instead (`stop_commands`), folded.
- A mission's state is its pill: Proposed, Agreed (with "Work in progress"
  when an item is started), Stop not published yet, Stopped or Done. Its
  page names the ref it was read from and the pull request of a plan's
  branch, and gives its plan's last round in one line.
- A proposed plan's own documents are read with `at` set to `read_at`, the
  tip of its branch; any other Markdown file at the base's tip.
- An item's Decisions section, from before `escalate` was removed, is shown
  as the record has it, with no form.
- Every page that reads a repository ends with when its remote was last
  fetched (`fetched`).

### The embedded mode

A host that shows the Desk in a panel, such as T3 Code, opens it under
`/embed`: `/embed/inbox`, `/embed/r/{repo}/item/{id}`, and so on for every
route above (item atelier-vg1l). The app's router takes `/embed` as its
base, so every link inside the Desk keeps the prefix and a reload stays in
the mode; the server already answers any path outside `/api` with the app.
Without the prefix nothing changes.

In the mode the Desk draws content only. It removes the sidebar with its
brand, page links and repository list, the theme toggle, the footer with the
commit the server was built from, and the menu button that opens the sidebar
on a phone. The top bar keeps the breadcrumb, the way up from any page, and
gains a link to the inbox with its count of what waits on the human. Pages
take their narrow layouts from the width media queries, since the panel is
the browser's viewport; `desk/e2e/embedded.spec.ts` checks the inbox,
repositories, mission, item, proposed, in motion and disputed pages at 420
and 720 pixels, and `desk/e2e/views.spec.ts` and `desk/e2e/host.spec.ts`
the views a host shows at the widths it shows them.

### The host contract

The embedded Desk and the page that frames it, its host, talk over
`postMessage` (item atelier-fnm5; `desk/src/lib/host.ts`). Each message is
an object with a `type`; a message whose `type` the Desk does not know, or
that is not an object, is ignored, so a host may send messages a newer Desk
understands.

**Who talks.** The app reads the configuration's `embed_origins` from
`GET /api/embed` once, when it opens under `/embed` inside a frame. It talks
only to its parent window, and only when the parent's origin is listed: it
learns that origin from `location.ancestorOrigins`, else from
`document.referrer`, else from the first message the parent sends from a
listed origin. It posts with that origin as the target, never `*`, and it
reads a message only when `event.source` is its parent window and
`event.origin` is that origin. With an empty list, or a parent whose origin
is not listed, the Desk sends nothing, reads nothing, and works as it does
unframed.

**From the Desk to the host:**

| `type` | Fields | When |
| --- | --- | --- |
| `desk:ready` | `version`: `1` | Once, when it has a listed parent |
| `desk:navigated` | `path`: the Desk's route without `/embed`, with its query, such as `/r/atelier/item/atelier-fnm5`; `title`: the page's title, as the browser tab would show it | After `desk:ready`, and whenever the path or the title changes, so a page whose title arrives with its data is told twice |
| `desk:open-thread` | `thread`: the thread's identity as the records hold it (`Agent.identity`, `LiveThread.id`, a dispute's `Side.identity`, a started session's `thread_id`) | When the human follows a thread link |
| `desk:resized` | `height`: the page's content height in CSS pixels, rounded up | On mission control, whenever its content's height changes, so the host can size its frame to it |
| `desk:key` | `key`: `Escape` | When Escape is pressed and nothing in the Desk handled it, so the host can close its panel or overlay. A Desk control that uses Escape calls `preventDefault`, and then the host is not told |

With a listed parent a thread link is a button that sends `desk:open-thread`
and opens no tab, popup or page; the host decides how to show the thread.
Without one it links to T3 Code's web address in a new tab, as before.

**From the host to the Desk:**

| `type` | Fields | What the Desk does |
| --- | --- | --- |
| `host:theme` | `colors`: any of `background`, `foreground`, `surface`, `border`, `muted` (secondary text) and `accent`, each a CSS colour; `font`: a CSS font family list; `scheme`: `light` or `dark` | Sets its tokens from them, without a reload |
| `host:thread` | `thread`: the identity of the thread the host is showing, as the records hold it (the form `desk:open-thread` sends); `repository`, optional: the name of the Desk repository the host's project maps to, as `GET /api/repos` names it; `children`, optional: the identities of the threads it started; `mentioned`, optional: the record IDs its conversation names | Opens "This thread" (`/t`, below) when it names another thread than the last `host:thread` did; otherwise shows what is new on that page where it is open. The T3 extension no longer sends it: it draws This thread itself, so a page framed in its panel stays where it is |

The theme maps onto the tokens at the top of `desk/src/styles.css`:
`background` to `--bg`, `foreground` to `--ink`, `surface` to `--panel`,
`--raised` and `--paper`, `border` to `--line`, `muted` to `--ink-3`,
`accent` to `--you` and `--you-text`, and `font` to `--sans`. The tokens the
host does not send are derived by mixing `--bg` and `--ink` (`--canvas`,
`--hover`, `--line-strong`, `--ink-2`, and the surface, line and muted
tokens when those are not sent). `scheme` picks the Desk's light or dark set
for the colours a host cannot send, those of state (blue, green, red,
violet); without it the scheme follows the background's luminance, read by
painting the colour on a canvas so every colour form the browser parses
counts. A later
message merges over the earlier ones: what it sends replaces, what it omits
stays, and a new `background` without a `scheme` has its scheme read from
the colour. A value that does not parse as a colour or a family list, or
that refers outside itself (`var()`, `env()`, `url()`, `inherit`, or
`currentColor` anywhere in it), is
dropped and its token keeps its value.

**Following the thread** (item atelier-cdxg; `useFollowThread` in
`desk/src/components/Shell.tsx`). On a `host:thread` that names another
thread than the last one did, the Desk opens "This thread" (`/t`), which
reads the thread, its ties, its children and the records it names from the
message ([Views a host shows](#views-a-host-shows)). It replaces the frame's
history entry rather than adding one, so the host's back button does not
walk through the threads it showed. A `host:thread` that names the thread
the last one named opens nothing, so a page the human has moved to stays;
when it says something new of that thread, such as another child or
another record named, "This thread" shows it where that page is open.
`children` and `mentioned` keep only their non-empty strings, and
`mentioned` each ID once; a field that is not a list counts as none.

Until atelier-cdxg the Desk chose one page from the thread's current ties
instead: the item it implements or reviews, the mission it leads, plans or
reviews the plan of, else its project's page or the inbox. "This thread"
shows all of those at once, with the threads it started and the records it
names, which one page could not.

What a host sends is only displayed. No message makes the Desk write or
act, fetch from another origin, or go outside its own routes: a theme sets
tokens, and a thread identity or a record ID is only looked up, through
`GET /api/ties`, `GET /api/thread` and `GET /api/records/{id}`, each
percent-encoded in a query or a path segment.

### Views a host shows

A host such as T3 Code shows three views of the Desk beside its threads,
and the inbox laid out for a wide frame (atelier-cdxg, from a spike the human
settled on 2026-11-16). Each is an ordinary route under `/embed`; nothing in
them writes or acts.

**This thread** is drawn by the T3 extension, not the Desk
(`t3-extension/src/thread/`), in T3 Code's panel beside a thread, 400 to
750 pixels wide, in React with T3 Code's own components and no frame. The
thread, the threads it started (T3's lineage) and the record IDs its
conversation names (the extension's reference scanner) come from the
extension's own state, so it sends no `host:thread`; the records come from
the Desk's reads through the injector's gate, each every 30 seconds while
the page is shown. A link on it to an item or the inbox, or a proposed
plan's title, opens that Desk page framed in the panel
(`/embed/r/{repo}/item/{id}`, `/embed/inbox`,
`/embed/r/{repo}/mission/{id}`), with a bar whose **This thread** goes
back; the framed page stays where it is when T3 shows another thread. The
title of an agreed or done mission opens the mission page. The Desk's own
page (`/t`, `desk/src/pages/ThisThread.tsx`) and `host:thread` stay for
other hosts until atelier-yy4j removes them. Before the host names a thread
that page says it is waiting for one. Both show, for the thread:

- The thread's title, from `GET /api/thread`, and, when it holds nothing
  now, what it did before ("Earlier it implemented ...").
- *Waiting on you*: the needs-human items, proposed plans and waiting
  threads, from `GET /api/inbox`, among the records this page is about:
  what the thread and its children hold now, the items of missions it
  holds or planned or leads, and the records it names.
- *Holds*: each item it implements or reviews now, with its Done when
  lines, its pull request and its reviewer; and each mission it leads,
  plans or reviews the plan of, with its open items.
- *Work under this thread*: each mission it planned or leads, open or done,
  with every item and a chip for each implementer and reviewer thread,
  which opens that thread in the host. Its title opens mission control on
  the Desk's page, and the mission page, or a proposed plan's record page,
  in the extension's.
- *Other threads it started*: the `children` no record names, each with its
  status, model and age from `GET /api/thread`, marked quiet when it has
  not changed in 30 minutes while it says it runs.
- *Mentioned*: each record in `mentioned` that it does not hold, read with
  `GET /api/records/{id}`, the done and dropped ones folded after four.
- With none of these, the project at a glance for the project's
  `repository` (what waits on them, open missions, items in motion, what
  can start next). On the Desk's page with no repository it is one line
  saying the thread holds nothing; the extension shows how to link the
  project instead.

A read that fails leaves its part out and the page still draws: ties that
cannot be read are named in a banner, and the page then does not claim the
thread holds nothing. The extension's page names each read that failed, the
path and the error, and claims the thread holds nothing only when the ties
of the thread and of each child, the inbox and each named record were all
read; a named record the Desk does not have (404) is not one it names
(`t3-extension/src/thread/model.ts`, `holdsNothing`).

**The mission page** is drawn by the T3 extension, not the Desk
(`t3-extension/src/mission/`), over T3 Code's main area, in React with T3
Code's own components and no frame. The extension no longer opens the Desk's
mission control below; that page stays until atelier-yy4j removes it. A
mission's title in T3's sidebar opens the mission page when the mission is
agreed or running; a proposed plan's title opens its record page
(`/embed/r/{repo}/mission/{id}`) framed. The page reads, through the
injector's gate, `GET /api/repos/{repo}/missions/{id}` every 15 seconds,
`GET /api/history?repo=&ids=` for the mission and its items every 30, and,
while it runs, `GET /api/thread?id=&full=1` for each thread T3 Code lists;
once done, the page of each merged item every 60, for its pull request.
Its model is `t3-extension/src/mission/model.ts`.

- Running (an open mission in progress or with an item started; else
  *Agreed*): the title, a state badge, items merged of total and a
  **Lead** button with its key, `L`, which opens the lead's thread, else
  the planner's. One sentence: how many items are being built (building or
  changes requested), in review and merging (approved), then what waits on
  the human and how many threads are quiet, or "Nothing needs you".
  *Needs you*: a row for each open item labelled `needs-human`, then each
  live thread waiting on a person or quiet, each with a button that opens
  the item's Desk page, framed, or the thread. A thread is live when T3
  Code lists it, says it is live and its status is not an ended one; it is
  waiting when T3 Code says `waiting` or its full read has a request of
  the human pending (`pending_requests`), such as an approval, whatever
  status T3 Code gives it (one stopped on an approval is `running`),
  listed at once whatever its age, and
  quiet when its `updated_at` is 30 minutes old, This thread's rule, so a
  long stage with a thread still changing is not quiet. A thread both
  waiting and quiet is one row. A thread whose read fails keeps its last
  answer, with a note naming the error; one with no answer yet is listed
  under *At work* as not read, and the sentence says whether it needs the
  human is not known rather than "Nothing needs you". Whether a thread is
  live and waiting comes from T3 Code's list as the mission's read gives
  it, which the Desk polls more often than it refreshes a full answer;
  either saying `waiting` lists it. The full answer gives the latest line
  and times. With none, there is no *Needs you*. *At
  work*: each other live thread, with its item, latest line and how long
  its latest run has gone. *Not started*, *Merged* and *Outcome and
  success criteria* are each one folded line that opens in place; an item
  in them opens its Desk page, framed. *Not started* counts the open items
  that can start and those that wait, and says of each that it can start or
  what it waits on: its blockers, the branch that adds it while the base
  does not hold it, or else the condition of `start` that does not hold.
- Done (or dropped): the state and when it closed, and how long after the
  plan merged; the Result as written; a lane per item, dropped ones
  included and ended where they were dropped, from the plan's merge
  to the close, with its building, review and merging spans, a legend, and
  its review rounds and time from start to merge at its end; and the
  merged items, most review rounds first, each opening its pull request on
  the forge where the item's page gives one (`pull_request`). One whose
  pull request has left the forge's recent list has none: it says its pull
  request is not known and opens its Desk page, framed.
- The lanes and the merge times come from commit subjects, as the progress
  log below does, and say so. The page has no log of steps.
- `L` opens the lead's thread, `Enter` presses the focused row's button and
  `Esc` closes the page; each key is shown on the button it presses. A Desk
  page opened from it is framed with a **Back** button to it.
- It holds at 728 pixels wide and stacks below 560 (a row's button goes
  under its text and a thread's latest line is hidden); from 1,100 *Needs
  you* and *At work* sit side by side, the lanes take the full width, and
  no line of text is wider than 75 characters. Widths are of the page, by
  container queries, not of the window.
- Records' sections are drawn from their Markdown by a small reader of
  paragraphs, lists, headings, inline code, emphasis and links (as text),
  since the extension does not take on `react-markdown`.

**Mission control** (`/mission/{repo}/{id}`,
`desk/src/pages/MissionControl.tsx`), over the host's main area, until
atelier-yy4j; the T3 extension no longer opens it. One screen
of bordered panes after Factory's Mission Control: a top bar (the
repository and mission, when the plan merged, how many items are live and
how many quiet) and a state line (`RUNNING`, `AGREED`, `PROPOSED` or
`DONE`) with a bar of the items done; *Active Item*, the item selected, with
its stage (for an open one, whether it can start) and how long it has been
in it, its holder and reviewer, what it cannot start until or, once
started, its blockers, Done when, goal, pull request and CI
(`GET /api/checks`);
*Items*, every item with its stage, marked when it has sat in a moving stage
for 30 minutes; *Progress Log*, the mission's steps newest first, paged to
the rows its pane holds; *Active Threads*, the live threads on the mission,
else the last three on the selected item, each with its role, item, model,
time and latest line (`GET /api/thread` with `full=1`); and a footer of
working keys:

| Key | Does |
| --- | --- |
| `↓` or `j`, `↑` or `k` | Selects the next or previous item, kept in `?item=` |
| `Enter` | Asks the host to open the selected item's thread: its reviewer while it is in review, else its holder, else the last thread on it |
| `L` | Asks the host to open the mission's lead, else its planner |
| `I` | Opens the selected item's page |
| `]` or `PageDown`, `[` or `PageUp` | Pages the log |
| `Esc` | Not handled, so the host is told (`desk:key`) and closes the view |

The keys are ignored with a modifier held or in a text field. The panes hold
down to 860 pixels wide, about a host's main area beside its sidebar, and
the page does not scroll; below 860 they stack and the page scrolls. The top
bar leaves 44 pixels clear at its right, where T3 Code draws a close button
over the frame. Mission control draws its own top bar, so the embedded
shell shows no breadcrumb or inbox link on it, and it tells the host its
height (`desk:resized`).

What is approximate there:

- The progress log, the stages' ages and an item's review round come from
  commit subjects (`GET /api/history`), not from the records: a step
  committed under another subject is missed, and a subject that only looks
  like a step is counted. An item with no step in the history is dated from
  when an old record says it was readied, and otherwise has no age.
- "Waited on you" marks only two things: the steps on items labelled
  `needs-human`, and the ready marks old records hold of the human, from
  before items were open on 2026-11-22: those whose `by`
  starts with the Desk's `caller` (from `GET /api/repos`), shown as one line
  per batch readied together. It knows of nothing else that waited on them.

**The project home** is drawn by the T3 extension, not the Desk
(`t3-extension/src/home/`), over T3 Code's main area, in React with T3 Code's
own components and no frame, from the same reads through the injector's gate.
The extension no longer opens the Desk's page (`/home/{repo}`,
`desk/src/pages/ProjectHome.tsx`), which stays until atelier-yy4j removes it;
both follow the rules here. The project's name in T3's thread breadcrumb, the
project's name in its project strip and the "N more missions" row in its
sidebar open it. It is the project as a roadmap without dates. *Now*
holds each agreed mission with any item started or done, and started items
in no mission; *Next*, agreed missions with nothing started, and every open
item in no mission as one list, by priority and then ID; *Later*, proposed
plans (`GET /api/roadmap` gives the items in no mission, in no such order,
so each page sorts them). A mission folds open to
its items with their holders, read from its page
(`GET /api/repos/{repo}/missions/{id}`) as it opens; those under *Now* start
open, and those under *Next* too when *Now* has no mission and *Next* two at
most. On the Desk's page a mission's title opens mission control; in the
extension an agreed or done mission's title opens the mission page, a
proposed plan's its record page framed, and an item its Desk page framed,
each framed page with a way back to the project home. *This week*
is a strip of the last seven days, each listing the items started and
merged, plans agreed and missions closed that day, from
`GET /api/history?days=7`, so it is as approximate as the progress log. In
the extension, a read of the items in no mission or of the week that fails
before it has answered leaves its part out, with a note naming the failure.
One that fails after answering, as can a mission's items, keeps what it last
gave, with a note saying it is as of that read.

**The inbox**, embedded in a frame wider than one comfortable column, flows
its repositories into columns of at least 560 pixels, up to three, each
kept whole.

## Routes

All responses are JSON, typed for the app with `ts-rs`. Times are RFC 3339
strings, and enums are their record names, such as `in_review`. A refused
`Host` is `403`, an ID that does not validate or a document path that
leaves the root is `400`, an unknown repository, record or route is `404`,
and a read that fails is `500`; each has `error`. `{repo}` is the
name the Desk's configuration gives a repository. An item or mission ID is
checked with `atelier_core::records::validate_id` (master) before use. A thread
identity never appears in a path: it can hold `/`, `:` and `%`, as
`thread:delegated-task:command%3Amcp%3A...` does.

| Route | Page | Gives |
| --- | --- | --- |
| `GET /api/repos` | Repositories | Each repository with its counts |
| `GET /api/embed` | Embedded mode | `origins`: the configuration's `embed_origins`, the origins the Desk talks to (The host contract) |
| `GET /api/repos/{repo}/missions` | Mission rail, the project home | The repository's missions and loose work |
| `GET /api/repos/{repo}/missions/{id}` | Mission, the extension's mission page and project home | One mission, wherever it is read from: sections, documents, items, plan reviews, threads, its pull request and its stop |
| `GET /api/repos/{repo}/missions/{id}/history` | Mission, Activity tab | The actions on the mission and its items, from the records (M5) |
| `GET /api/repos/{repo}/doc?path=<path>&at=<commit>` | Mission, Documents tab | One Markdown file at the base's tip, or at `at` |
| `GET /api/repos/{repo}/items/{id}` | Item | One item in full |
| `GET /api/inbox` | Inbox | What waits on the human, in every repository |
| `GET /api/threads` | Live threads | T3 Code's live threads, each with what it holds in the records |
| `GET /api/records/{id}` | A host that frames the Desk | What an ID names, in whichever repository served holds it |
| `GET /api/ties?thread=<identity>` | A host that frames the Desk | Every record that names a thread identity, live or not, and whether each tie is current |
| `GET /api/thread?id=<identity>&full=1` | This thread, mission control, the extension's mission page | Any thread T3 Code knows, live or ended; with `full`, its latest run and line |
| `GET /api/sidebar` | A host's sidebar | Every open mission in every repository, with the threads on it and on its items |
| `GET /api/launch?repo=<repo>` | A host's project strip | What can start, what waits on the human and what moves, in one repository |
| `GET /api/history?repo=<repo>&days=<n>&ids=<id,...>` | Mission control, the project home, the extension's mission page | The records' steps, read from commit subjects |
| `GET /api/checks?repo=<repo>&pr=<number>` | Mission control | CI on a pull request |
| `GET /api/roadmap?repo=<repo>` | The project home | The open items in no mission |
| `GET /api/proposed` | Proposed | Proposed plans and record changes on branches, in every repository |
| `GET /api/motion` | In motion | Started items and missions in progress, with their threads |
| `GET /api/disputed` | Disputed | What `atelier list --disputed` gives, with each side's model |
| `POST /api/action` | Mission | Stops a proposed plan |
| `POST /api/sessions` | Mission | Starts a planning session as a T3 Code thread |

`?done=true` on `/missions` lists done and dropped missions, as
`Filter::done` does for `list` (master).

The reads added for a host's views (atelier-cdxg) take the repository as the
query parameter `repo`, by the name `/api/repos` gives it. Those that run a
command (`git log`, `git for-each-ref`, `gh pr view`) or call T3 Code
(`t3_thread_read`) give it a deadline, keep its answer a while, and never
put a value from the request on a command line unchecked: `repo` is looked
up among the repositories served, a pull request is a number, and a thread
identity is sent to T3 Code as JSON. The pages leave out what such a read
cannot give and still draw.

The two history routes are two readers. A mission's reads the records'
changes, so it is exact; `/api/history` reads commit subjects, and its
section below says why it still does.

## Responses, field by field

Every response for one repository also has `commit`: the base's tip it was
read at (`Repo::base_tip`, psjw). `/api/repos` gives each repository's
`base_tip` instead, and `/api/inbox` a `commit` for each repository.

Every response that reads one repository, and each repository of
`/api/repos`, `/api/inbox`, `/api/proposed`, `/api/motion` and
`/api/disputed`, has `fetched {remote, at,
error, every_seconds}`: the remote the Desk fetches (the forge's, else
`origin`; `null` when there is neither), when the clone last fetched, from
`git::fetched_at` (the newest `FETCH_HEAD`), why the Desk's last fetch or
its last read of the forge's pull requests failed, and the configured
period. It is joined on each request, not
cached, and so are pull requests and T3 Code's threads.

### `GET /api/repos`

| Field | Given by | State |
| --- | --- | --- |
| `name`, `path` | The Desk's configuration | Desk |
| `identity` | The `origin` remote (`git remote get-url origin`) as `host/path`, lower case, with no scheme, user, port or `.git`: `https://github.com/spadaval/atelier.git`, `git@github.com:spadaval/atelier.git` and `ssh://git@github.com:22/spadaval/atelier` are all `github.com/spadaval/atelier`, the key T3 Code's `repositoryKeyFromRemote` gives a project's repository (`fetch::identity`). `null` without an `origin`, or for one with no host, such as a local path or a `file://` URL. Given for a repository that does not read too | Desk |
| `base` | `Repo::base`, from `config.toml` at `HEAD` | M1: `Repo::open_committed` |
| `base_tip` | `Repo::base_tip` | psjw |
| `in_progress` | Count of missions where `atelier_core::in_progress(mission, &entries)` | master |
| `proposed` | Count of missions whose `state` is `proposed` | Desk |
| `waiting` | Item count of `atelier_core::list(&entries, &missions, &caller, &Filter { needs_human: true, .. })` | master |
| `error` | The error text when the repository does not open or a record does not read (`atelier_core::records::ReadError`, master); then `base` and `base_tip` are `null` and the counts `0` | Desk |

Beside `repos`, `built_from` is the commit the server was built from
(`atelier_desk::BUILT_FROM`), or `unknown`. The app shows its first nine
characters in the sidebar's footer, with the whole ID in its title.
`caller` is the configuration's `caller`, the name the human acts under,
by which mission control tells the ready marks old records hold of them.

### `GET /api/repos/{repo}/missions`

`missions[]`, one per mission that is not done or stopped, proposed ones
included, in ID order, as `list` orders groups:

| Field | Given by | State |
| --- | --- | --- |
| `id`, `title`, `status`, `labels`, `lead`, `created_at` | `atelier_core::model::Mission` of `Located::record`, the copy that counts | master |
| `state` | `proposed` when `Located::proposed` or an old `draft`; `stop_pending` for a drop on an unmerged branch of an agreed mission, or a plan dropped on its branch whose drop is not on the forge's copy of the branch or whose pull request is still open; `stopped` once dropped and published; `open` (shown "Agreed"); `done` | Desk, from the reader |
| `branch` | `Located::source`'s `name`: the ref the record was read from; `null` for the base | atelier-dciw |
| `in_progress` | `atelier_core::in_progress` | master |
| `counts[] {status, count}` | `items_of(id, &entries)` grouped by `Entry::status` in `LIFECYCLE` order, as `show <mission>` groups them; a status with no item is left out | master |
| `waiting[]` | The IDs of the open items of `items_of` with `Entry::needs_human` | master |

`loose`: the item count of the group with `id: None` from `list` (master),
the spike's "work outside missions", with the same `done` flag. With
`?done=true` the rows are the done and stopped missions. The server
settles each row's state with its pull request before it keeps the
finished or the unfinished, so a plan whose stop is pushed and whose pull
request is still open stays with the unfinished.

### `GET /api/repos/{repo}/missions/{id}`

The page is read from `atelier_core::mission_view`, as `show <mission>` is.
Its item rows also need each item's records, which `MissionView::located`
gives (added by atelier-uwxy).

| Field | Given by | State |
| --- | --- | --- |
| `mission` | The mission's row, as on the rail | Desk |
| `sections[] {heading, markdown}` | `MissionDir::section` for Why, Outcome, Success criteria, Scope, Authority and Result, those present, of the copy that counts | master |
| `dropped` | `MissionDir::dropped` | master |
| `read_at` | The commit its record and documents were read at: the tip of the branch it was read from, else the base's | Desk |
| `pull_request {number, url, state}` | The pull request whose head is the branch it was read from, or, for one not proposed that is read from the base, the branch the convention names for it, which `merge` deleted; from the forge's list (see Fetching) | Desk, `gh` |
| `round {round, reviews[] {name, reviewer, model, verdict}}` | The last round of `MissionDir::reviews` | Desk |
| `stop_on` | For `stop_pending` on an agreed mission, the unmerged branch that drops it | Desk |
| `can_stop` | Whether the page offers the stop: `proposed`, with a forge remote and a state directory configured | Desk |
| `stop_commands[]` | For an agreed mission, the commands that stop it by the record-only path | Desk |
| `documents[] {path, title}` | Every file of `.atelier/missions/<id>/` but `mission.md` and `reviews/`, at `read_at`; `title` is its first `# ` heading, else its name | M2: `CommitReader::mission_documents(commit, id)` |
| `items[]` | `MissionView::items`, each as an item row (below) | master |
| `plan_reviews[]` | `MissionDir::reviews`, each as a review (below) | master |
| `plan_reviews[].changed_since[]` | The plan's paths that differ between the commit reviewed and `read_at`; `null` when this clone lacks that commit | M6: `atelier_core::plan::plan_changed_at(&mut repo, id, commit, read_at)` |
| `threads[]` | Its planners, its lead and the reviewers of its plan reviews, with `item: null`, then each of its items' agents (below) with `item` naming the item, so a mission with no lead still shows who works on its items; joined as in [Tying a thread to an item](#tying-a-thread-to-an-item) | Desk, T3 Code |
| `t3` | Whether T3 Code could be read, as on `/api/threads` | Desk |

An **item row**, in a mission, the inbox and the rail's loose work:

| Field | Given by | State |
| --- | --- | --- |
| `id`, `title`, `status`, `priority`, `labels` | `Entry` | master |
| `needs_human`, `startable` | `Entry::needs_human`, `Entry::startable` | master |
| `blockers[] {id, status}`, `waiting_on[]` | `Entry::blockers`; the IDs of `Entry::waiting_on` | master |
| `holder`, `reviewer` | `Entry::holder`; `Entry::reviewer`, set only while in review | master |
| `cannot_start[]` | For an open item, `say::condition_text(id, c)` for each of `Entry::unmet` | master values, psjw text |
| `needs[]` | For a started item, `say::condition_text` for each of `Entry::needs`, what its holder's next action lacks | master values, psjw text |
| `branch {merged, name}` | The branch the record was read from, else the one named for it; absent when there is none | atelier-dciw |
| `report_written`, `dropped` | `ItemDir::report_written`, `ItemDir::dropped` | master |
| `approved_at` | `Entry::approved`, which `atelier_core::annotate` fills, from `atelier_core::git::approved_at`; `mission_view` and `item_view` annotate their entries, and the inbox annotates its rows | M4 |

A **review**, of an item or a mission's plan:

| Field | Given by | State |
| --- | --- | --- |
| `name`, `round` | `ReviewFile::name` | master |
| `commit`, `reviewers`, `model`, `verdict`, `created_at` | `atelier_core::model::Review` | master |
| `findings[] {name, title, recommendation, was, now, decided, settlement}` | `ReviewFile::findings` | master |
| `findings[].text` | `ReviewFile::finding_text` | master |
| `findings[].open`, `findings[].deferred` | `atelier_core::model::is_open`, `is_deferred`, over all the record's findings and its decisions (none for a mission) | master |
| `changed_since` | On a plan review, as above; `null` on an item's | M6 |

### `GET /api/repos/{repo}/doc?path=<path>`

`path` is from the repository's root. The Desk normalizes it and refuses one
that leaves the root or does not end in `.md`. The response's `path` is the
normalized one. `markdown` is
`CommitReader::text(base_tip, path)` (master). Links in a mission's files
are relative to the file (`../../../docs/design.md`, say), so the
app resolves them against `path`, as the spike's `DocReader` does.

### `GET /api/repos/{repo}/items/{id}`

| Field | Given by | State |
| --- | --- | --- |
| `row` | The item row, as above, from `ItemView::entry`; it holds `id`, `title`, `status`, `priority` and `labels` | psjw |
| `mission`, `blocked_by`, `implementers`, `ready` (who said go, in an old record; else `null`), `trivial`, `caused_by`, `created_at` | `atelier_core::model::Item` of `Located::record` | master |
| `read_from` | `base` or `branch`: whether the record is the base's or the branch's | M7: `ItemView::read_from`, `Located::read_from` |
| `body` | `Item::body`, the whole Markdown | master |
| `goal`, `scope`, `report` | `ItemDir::section` | master |
| `done_when[] {ticked, text}` | `ItemDir::done_when` | master |
| `decisions[]`, `responses[]` | `ItemDir::decisions`, from before `escalate` was removed, and `ItemDir::responses` | master |
| `reviews[]` | `ItemDir::reviews`, each as a review | master |
| `unsettled[]` | The names of `actions::unsettled`, for a done item | master |
| `threads[]` | One agent per identity and role the record names: its implementers, the reviewers of its reviews and the deciders of its answers, each with `item: null` | Desk, T3 Code |
| `t3` | As on `/api/threads` | Desk |
| `pull_request {number, url, state}` | The pull request of its branch, or for a done or dropped item with none, of the branch the convention names for it, where the forge's last list has one (Fetching); `null` for another item with no branch, or none found | Desk, the forge |

An **agent**, in `threads[]` of a mission or an item: `identity`, whole as
the record holds it; `role`, one of `planner`, `implementer`, `reviewer`, `lead` or
`decider` (who answered an old question); `item`; and `thread`, the live thread's `title`, `status`,
`provider`, `model` and `link` when T3 Code lists the identity as a live
thread, else `null`, and then the page shows the identity as text with no
link. An identity named in two roles is listed once for each.

`show <item>` also shows a settlement written in the base checkout and not
committed. The Desk does not, since it reads commits only.

### `GET /api/inbox`

`repos[]`, each with `repo`, `commit`, `error` (as on `/api/repos`; the
lists are then empty) and the fields below. The response also has `t3`, as
on `/api/threads`, and `news[] {repo, mission, title, kind, at, link}`:
plans proposed and merged lately, newest first, the last 50 the Desk saw
(see [Being told](#being-told)).

| Field | Given by | State |
| --- | --- | --- |
| `needs_human[] {mission, mission_title, items[]}` | `list(.., &Filter { needs_human: true, .. })`, with the proposed missions as `list` passes them: `Group::id`, `Group::mission`, `Group::items` as item rows | master |
| `proposed[]` | Missions whose `state` is `proposed`, as mission rows | Desk |
| `threads[]` | Live threads whose status is `waiting`, which T3 Code gives a thread that waits on a person, and that hold something in this repository's records; each as on `/api/threads`, with only this repository's `holds` | Desk, T3 Code |

`needs_human` lists the same items as `atelier list --needs-human`, which
`desk-matches-cli` checks. `proposed` is in addition: a plan they may stop.

### `GET /api/threads`

The server reads T3 Code's thread list every 10 seconds on a thread of its
own and keeps it in memory, so no request waits on T3 Code and none calls
it. Each poll calls the MCP tool `t3_thread_list` with `allProjects`,
`includeSubagents`, `settled: false`, the live statuses (`preparing`,
`queued`, `starting`, `running`, `waiting`) and `limit: 100`, following
`nextCursor` to the last page. A poll that fails keeps the last list read,
which every response then serves with its age.

The client is `atelier_desk::t3::Client`. It runs the configured command as
`auth session issue --subject mcp-client --scope orchestration:read --ttl
1h --label atelier-desk --json` and keeps the token in memory only: it is
never logged, written to disk or put in a response or an error. It opens
an MCP session (`initialize`, then `notifications/initialized`) and sends
the token as a bearer token with each request. A token within two minutes
of the expiry `issue` printed, or one the server refuses with `401` or
`403`, is replaced, the old session revoked with `auth session revoke`, and
the call made once more; so is an MCP session the server no longer knows
(`400` or `404`). Stopping the server revokes the session it holds. The
token can read and do nothing else: T3 Code refuses it on
`t3_thread_launch` with `capability_denied` (atelier-aab1).

| Field | Given by | State |
| --- | --- | --- |
| `threads[].id`, `title`, `status`, `provider`, `model`, `parent`, `pull_requests[] {repository, number, url, state}`, `link`, `updated_at` | T3 Code: `threadId`, `title`, `status`, `providerInstanceId`, `model`, `parentThreadId`, `pullRequests`, `link`, `updatedAt` | T3 Code |
| `threads[].holds[] {repo, kind, id, role}` | The join below, over every repository served; `kind` is `item` or `mission` | Desk |
| `t3 {reachable, age_seconds, error}` | Whether the last poll read the list; the age in seconds of the list served (`null` before one is read); and why the last poll failed, or that the configuration has no `[t3]` table | Desk |

`t3_thread_list` gives no `branch`, so the response has none. T3 Code's
`t3_thread_read` gives one, but it is `null` for a thread working on a
branch, as the example below shows. `link` is the T3 Code web app's address
for the thread, such as `https://app.t3.codes/threads/<environment>/<thread
ID, percent-encoded>`.

Effort, context use and token counts are not shown: the MCP server gives
none (atelier-aab1), and the strategy puts them out of scope.

### `GET /api/records/{id}`

What an ID names, for a host that has an ID and no repository, such as T3
Code's extension reading one from a thread's title. The Desk looks the ID
up in every repository served, in the order its configuration gives them,
and answers with the first record that has it. A repository that does not
read is passed over; `/api/repos` gives its error. An ID that does not
validate is `400`; one that validates and names nothing is `404`.

| Field | Given by |
| --- | --- |
| `repo` | The name the Desk's configuration gives the repository |
| `kind` | `item` or `mission` |
| `id`, `title` | The record |
| `status` | As the record reads: an item's (`Status`, such as `open`, which an old `draft` or `ready` reads as, or `in_review`) or a mission's (`MissionStatus`: `open`, `done`, `dropped` or the old `draft`) |

Records are read as `/api/threads` reads them: each item by the record the
cross-branch rule picks (`Located::record`), so a started item's status is
its branch's, and each mission by its record that counts, with a stop on
the forge's base marked dropped (`Repo::published_stops`). The index both
this route and `/api/ties` read is built once per repository and kept
until its commits change, as every response is.

### `GET /api/ties?thread=<identity>`

What a thread identity is tied to, whether or not T3 Code lists the thread
as live, so a host can show what any thread worked on. The identity is the
query parameter `thread`, percent-encoded, never a path segment, since it
can hold `/`, `:` and `%`. It is compared whole, as in [Tying a thread to
an item](#tying-a-thread-to-an-item). A `thread` that is missing or empty
is `400`; an identity no record names gets `ties: []`, not `404`.

| Field | Given by |
| --- | --- |
| `thread` | The identity asked about |
| `ties[] {repo, kind, id, title, status, role}` | One per record and role that names the identity, in every repository served: the record as on `/api/records/{id}`, and `role` as on `/api/threads` (`planner`, `implementer`, `reviewer`, `lead` or `decider`). Items come before missions within a repository |
| `ties[].current` | Whether the identity works on the record now. On an item: its last implementer (`Implementers::holder`) while it is `building`, `in_review`, `changes_requested` or `approved` (`actions::STARTED`); the holder of a review of its latest round while it is `in_review` and no review of that round has a verdict, as `Entry::reviewer` finds it. On a mission that is not `done` or `dropped`: its lead and its planners, and the holder of each review of its plan's latest round while a review of that round has no verdict, the rule `actions::plan_round` keeps a round open by. Otherwise false: a replaced implementer, a reviewer of a closed round, a decider, and every tie to a done or dropped record |

`/api/threads` and its `holds` are unchanged: they list every tie, current
or not, with no title or status.

### `GET /api/thread?id=<identity>`

Any thread T3 Code knows, live or ended, for a page that names a thread
whose title the records do not hold. A live thread is answered from the
last poll, with no call. Any other is read with the MCP tool
`t3_thread_read` (`limit: 1`) and kept a minute, a thread T3 Code does not
know as long, so it is not asked for on every request. With `full=1` it is
read with `view: "activity"`, its last four items (`beforePosition` past
its end, since without a position T3 Code gives a client with no thread of
its own a thread's first items), and kept 20 seconds.
The Desk keeps at most 500 threads read this way. The identity goes to T3
Code as a value in a JSON request, never on a command line; one that is
empty, longer than 512 characters or holds a control character is `400`.
A thread T3 Code does not know, or any thread while T3 Code is not
configured or cannot be read, is `404`.

| Field | Given by |
| --- | --- |
| `id`, `title`, `status`, `updated_at`, `parent`, `model` | T3 Code: `threadId`, `title`, `status`, `updatedAt`, `parentThreadId`, `model` |
| `live` | Whether the last poll listed it |
| `started_at` | With `full=1`: the `startedAt` of its newest run (`recentRuns`), else its `createdAt`; otherwise `null` |
| `last_line` | With `full=1`: its newest assistant message or tool call in one line of at most 160 characters, a tool call as `<tool>: <description, command, path, pattern, query, prompt or URL>`; reasoning and messages sent to it are passed over. Otherwise `null` |
| `pending_requests` | How many requests of the human, such as approvals, it has pending: `pendingRequestCount` of `t3_thread_read`, 0 where T3 Code omits it. `null` for a live thread answered from the last poll, since `t3_thread_list` does not give it; `full=1` always reads the thread |

### `GET /api/sidebar`

Every open mission in every repository, for a host's sidebar, in the
configuration's order. `repos[]`, each with `repo`, `missions[]`, and
`error` only when the repository does not read (then `missions` is empty).
A mission is listed when it is proposed or agreed and open.

| Field | Given by |
| --- | --- |
| `missions[].id`, `title` | The mission |
| `missions[].state` | `in_progress` (agreed, with an item started), `agreed` or `proposed` |
| `missions[].done`, `total` | Its items done, and all its items |
| `missions[].waiting` | Its items waiting on the human (`MissionRow::waiting`), and the threads on it or its items that T3 Code's last poll shows `waiting` |
| `missions[].threads[] {identity, role}` | The agents on the mission itself: its lead, planners and plan reviewers |
| `missions[].items[] {id, title, status, threads[]}` | Each item, with its implementers and reviewers as `{identity, role}` |

### `GET /api/launch?repo=<repo>`

One repository for a host's project strip: what can start now, what waits
on the human there, and what is in motion. An unknown repository is `404`.

| Field | Given by |
| --- | --- |
| `ready[] {id, title, priority}` | `/api/motion`'s `ready`, the open items that can start, without the needs-human ones |
| `waiting` | The needs-human items, the proposed plans and `threads_waiting`, counted |
| `threads_waiting[] {identity, title}` | Live threads that hold work here and that T3 Code shows `waiting` |
| `waiting_items[] {id, title}` | The first three things waiting on them: proposed plans, then needs-human items by priority |
| `in_motion[] {id, title, stage, age_seconds, holder, live}` | Each started item, those whose holder is live first, then the most recently moved. `stage` is `building`, `in review <round>`, `changes requested` or `approved`, the round counting its submits in the last 60 days of history; `age_seconds` is since its latest step in that history, else since its branch's last commit; `holder` is its reviewer while it is in review, else its implementer, as `{identity, title}`, the title from `/api/thread` or `null`; `live` is whether that thread is live |

The history is read as `/api/history` reads it, and the page still gets an
answer when it cannot be: the ages then come from the branches.

### `GET /api/repos/{repo}/missions/{id}/history`

What `atelier log <id>` gives for a mission, its plan reviews and its items,
oldest first, from `atelier_core::log::history_until` on a repository opened
with `Repo::open_committed` (M5). It is kept until the repository's commits
change, under the cache key above, and given 15 seconds: the walk stops
then, and the request stops waiting whatever hung. A read past that is
`500`. An unknown repository, or an ID that is no mission's, an item's
included, is `404`; an ID that does not validate `400`.

| Field | Given by |
| --- | --- |
| `actions[].timestamp`, `commit` | The commit's committer time, in UTC, and the commit as Git abbreviates it |
| `actions[].id` | The mission, or the item of it |
| `actions[].action` | `Happened::name`, as `atelier log` prints it: `plan`, `plan-merge` and the others in [the log's table](../design.md#the-log) |
| `actions[].thread` | The thread the log shows, or `null` |
| `actions[].detail[]` | What changed, in the log's parts (`Happened::changed`), such as `approved -> done`, or `amendment` on a `plan-merge` |
| `actions[].skips[]`, `skipped_by` | Each rule the action's commit passed, as `<rule>: <reason>`, and the thread that passed them |
| `passed_over[] {timestamp, commit, id, path, problem}` | Each version of those records the log passed over, as it does not read |
| `commit`, `fetched` | As on every response for one repository |

Records have no history from before their conversion at the cutover, so a
mission converted then has no `plan`, and no `plan-merge` of its first plan.

### `GET /api/history?repo=<repo>&days=<n>&ids=<id,...>`

This route, and `/api/launch` with it, stays on commit subjects until
`t3-extension/fixture-desk.mjs` is updated: that fixture builds its closed
mission's history from empty commits, which change no record, and
`t3-extension/` is frozen in mission atelier-ww7w. Subjects also reach back
before the cutover, where the records' history does not.

The records' steps, newest first, read from the subjects of the newest
20,000 commits on the local branches and the fetched remote's. Only the
subjects Atelier's commands write count, so the history is approximate:
a step committed under another subject is missed, and a subject that only
looks like a step is counted. A step recorded on two branches before a
merge is listed once. The list is read with one `git log`, given 15 seconds,
and kept until the repository's commits change. `days` keeps the last
`days` days; `ids`, a comma-separated list, keeps the steps on those IDs. An
unknown repository is `404`, a `days` that is not a number `400`, and a
`git log` that fails `500`.

| Field | Given by |
| --- | --- |
| `events[].commit`, `at`, `subject` | The commit, its committer time, and its subject |
| `events[].kind`, `id` | From the subject: `planned` (`Plan mission <id>`), `plan_review` (`Review the plan of <id>, round <n>: <verdict>`), `started` (`Start <id>`), `submitted` (`Submit <id> ...`), `review` (`Review <id>, round <n>: <verdict>`), `merged` (`Merge <id>: ...` or `Merge <remote>/<id> ...`), `readied` (`Ready <id>, <id> ...`, one for each ID, in history from before items were open), `closed` (`Close <id> ...`), `dropped` (`Drop <id>`) and `led` (`Lead <id>`) |
| `events[].round`, `verdict` | A review's round and its verdict as the subject words it, such as `changes requested`; otherwise `null` |
| `events[].record`, `title` | `item` or `mission` and the record's title, where the repository holds the ID; otherwise `null` |

### `GET /api/checks?repo=<repo>&pr=<number>`

CI on a pull request, read with `gh pr view <number> --json
statusCheckRollup` in the repository's directory, given 15 seconds, and kept
a minute. `pr` is parsed as a number before it reaches the command line; one
that is not is `400`. An unknown repository is `404`, and a `gh` that fails,
such as for a pull request it cannot find, `500`.

| Field | Given by |
| --- | --- |
| `checks[] {name, state}` | Each check's name, or its context, and its conclusion as GitHub words it (`SUCCESS`, `FAILURE`, `SKIPPED`, ...), or its status while it runs |
| `state` | `none` with no check; `failing` when one failed, errored, was cancelled, timed out or needs action; `passing` when every one succeeded, was skipped or was neutral; otherwise `pending` |

### `GET /api/roadmap?repo=<repo>`

`repo`, and `loose[]`: every item in no mission that is not done or
dropped, as item rows, for the project home's Now and Next. An
unknown repository is `404`.

### `GET /api/proposed`

`repos[]`, each with `repo`, `commit`, `error`, `fetched` and:

| Field | Given by |
| --- | --- |
| `plans[] {mission, amends, pull_request, round}` | Each mission whose `state` is `proposed`, as on its page, and each agreed mission read from its own unmerged branch, an amendment (`amends: true`). A plan dropped on its branch is `stopped` or `stop_pending`, and not listed |
| `changes[] {branch, pull_request, records[] {kind, id, title, change}}` | By branch: each record an unmerged branch adds (`Located::proposed`, from `Located::source` and `Located::also_added`) or changes (`Located::changes`), with `change` `added`, `changed`, `dropped` or `removed`. A plan's own branch, proposed or stopped there, whose records are its plan, and a started item's own branch, which is work in motion, are left out; an agreed mission's own branch is not: what that branch holds decides, a drop there, not merged yet, listed as `dropped`, or an amendment, listed under `plans` |

### `GET /api/motion`

`repos[]`, each with `repo`, `commit`, `error`, `fetched` and:

| Field | Given by |
| --- | --- |
| `items[] {id, title, status, mission, branch, holder, reviewer}` | Each entry in a started status (`actions::STARTED`), its status from the record that counts, which for a started item is its branch's, local or fetched; `branch` is the ref read; `holder` and `reviewer` are agents (`Entry::holder`, `Entry::reviewer`) |
| `items[].changed_at` | When `branch` last had a commit, from one `git for-each-ref` over the local and fetched branches, given 10 seconds; `null` where it cannot be read |
| `missions[] {mission, lead}` | Each agreed mission with a started item (`in_progress`), and its lead as an agent |
| `ready[]` | Up to five open items that can start now, needs-human ones included, as item rows, highest priority first, then by ID |

An agent's `thread` is T3 Code's live thread with that identity, or `null`.
Whether T3 Code says the thread is alive is that: a thread T3 Code lists as
live, or, with a list read, one it does not (the page says "not running in
T3 Code"); with no list read, the page says threads are unavailable. The
response's `t3` says which.

### `GET /api/disputed`

`repos[]`, each with `repo`, `commit`, `error` and what
`atelier_core::disputes(&items, &missions, None)` gives, which `atelier list
--disputed` prints:

| Field | Given by |
| --- | --- |
| `rebuttals[] {item, mission, finding, title, fact, implementer, raised_by, judged[] {finding, judged, reviewer}}` | `Rebuttal`: the implementer is the item's holder; `raised_by` the reviewer of the rebutted finding's review; each later finding that judged it, with what it made of it and its review's reviewer |
| `plans[] {mission, round, reviews[] {name, reviewer, verdict}}` | `SplitRound`: each review of a round whose reviewers' verdicts differ |

Each side is `{identity, model, provider, link}`. `model` is the review's
`model` where it records one; otherwise, and for the implementer, T3 Code's
model for the thread, where T3 Code has listed it since the Desk started.
To know threads that are no longer live, every sixth poll also reads T3
Code's last 100 threads of any status (`t3_thread_list` with no status
filter), and the Desk keeps the provider, model and link of every thread
it has seen.

### `POST /api/action`

The request is a stop of a proposed plan:

```json
{ "action": "stop", "repo": "atelier", "id": "atelier-xxxx", "reason": "One line." }
```

A field the action does not take is refused. The caller is never in the
request: the stop runs under the configured `caller`. The types are
`ActionRequest` (`StopAction`) and `ActionResponse` in `desk/src/api/`.

The response is `200` with one of:

```json
{ "result": "done", "text": "Dropped the proposed plan of mission atelier-xxxx: ...\nPushed atelier-xxxx to origin and closed its pull request. The plan is stopped.\n",
  "paths": [".atelier/missions/atelier-xxxx/mission.md"], "commit": "76cbb6c..." }
{ "result": "refused", "rules": [], "text": "Mission atelier-xxxx is agreed: ...", "next": "..." }
```

| Result | Fields | From |
| --- | --- | --- |
| `done` | `text`, `paths[]`, `commit`, the drop's commit as pushed | `atelier_core::plan::Done` and [The stop](#the-stop) |
| `refused` | `rules[]`, `text`, `next` (may be `null`) | `atelier_core::Refused`, or the stop's own checks |

A stop is refused, changing nothing, when the reason is empty or not one
line, the mission is not found, is stopped already, or is agreed (its stop
waits on atelier-yclc), when the repository names no forge remote, and when
the plan's branch is not on the remote as last fetched. An `id` that is not
an ID is `400`, an unknown repository `404`, a body that is not JSON `400`
or `415`, JSON that is not the shape above `422`, and a failure that is not
a refusal, such as a push the remote refuses, `500`; each with `error`.
Actions run one at a time.

#### Only from the app

`POST /api/action` and `POST /api/sessions` are refused with `403` unless the
request has the header `x-atelier-desk: 1`, and its `Origin`, when it has
one, names a host in the configuration's `hosts` (its port is ignored). A
page of another site cannot add the header without the browser asking the
server first, which it does not answer, and a request from such a page
carries that page's `Origin`. The check runs before the method's, so a
`GET` of either route is `403` too. There is no sign-in: anyone who can
reach the Desk at a host it answers to can act as its caller (strategy,
Access).

### `POST /api/sessions`

Starts a planning session for a mission: a new T3 Code thread in the
repository's T3 project, whose first message names the mission. It writes
no record. The request is `SessionRequest`:

```json
{ "repo": "atelier", "mission": "atelier-z4cr" }
```

The mission must be on the base or on a branch (`404` otherwise). The
response is `200` with `SessionResponse`, one of:

```json
{ "result": "started", "thread_id": "...", "link": "https://app.t3.codes/threads/...",
  "project_id": "..." }
{ "result": "refused", "text": "No T3 Code project has /path as its workspace root, ..." }
```

`link` is the one `t3_thread_launch` gives, and may be `null`. It is
refused, and nothing is started, when the configuration has no `[t3]`
table, when T3 Code cannot be reached, when no project's workspace root is
the repository, or when T3 Code refuses the launch.

How it starts the thread:

1. **Find the project.** The server lists T3 Code's projects with
   `t3_project_list` (every page, deleted ones left out), under the token
   that only reads, and takes the first whose `workspaceRoot` is the
   served repository's directory, the one under a configured root,
   compared as canonical paths. A project rooted at another checkout of
   the same repository is not taken: a T3 project names one checkout, and
   the thread starts in it.
2. **Get a token that may operate.** Only now, the server runs the
   configured `t3` command as `auth session issue --subject mcp-client
   --scope orchestration:operate --ttl 5m --label atelier-desk --json`.
   The token is held in memory for this one request: it is never logged,
   written to disk, or put in a response or an error, and no other route
   uses it. The read token cannot launch: T3 Code refuses it with
   `capability_denied` (atelier-aab1).
3. **Launch.** It calls `t3_thread_launch` with `projectId`, `title`
   `Plan <mission>: <mission title>`, `message`, and a `clientRequestId`
   of its own, so that the client's one retry after a refused token
   replays the launch rather than starting a second thread. It names no
   model or runtime mode, so T3 Code picks them as it does for a caller
   outside a thread; on this host, on 2026-11-11, that was
   `claude-fable-5-2` in `approval-required` mode, although the scratch
   project's default model was another. The thread runs in the project's
   checkout (`workspaceStrategy` `root`).
4. **Revoke.** It revokes the operating token's session with `auth session
   revoke`, whether the launch worked or not.

The first message is `[t3] session_message`, with `{mission}` and `{repo}`
replaced. Its default (`atelier_desk::config::SESSION_MESSAGE`) is:

> Start a planning session for mission {mission} in this repository
> ({repo}). Load the Agent Factory skill (/agent-factory) and plan
> {mission} with me: read the mission and its documents, then work with me
> on its plan.

## Fetching

The tracker's commands fetch no more than the base and a lost claim's
branch (docs/design.md, No command fetches); the
Desk does, for the human. On a thread of its own, every `fetch_seconds`, it
runs `git fetch --quiet --prune <remote>` in each served repository, with no
terminal prompt and SSH in batch mode (`atelier_core::git::unattended`).
The remote is the forge's, as `config.toml` at `HEAD` names it, else `origin`; a repository with neither
is not fetched. A fetch writes only the repository's remote-tracking refs
and `FETCH_HEAD`, never a checkout. A failed fetch keeps the refs as they
were and its error is shown in `fetched.error`. Where the forge is GitHub's,
the same round reads its last 200 pull requests of any state and up to 1,000
open ones with `gh pr list --json number,url,state,headRefName`
(`atelier_core::forge::list_pull_requests`). It keeps
the newest of each list for each head branch, the open ones apart, so a
newer closed pull request never hides an older open one of the same branch.
The open list is complete when it holds fewer than 1,000, and only then does
a branch missing from it count as having none open; a page's `pull_request` is the one whose head is the branch the
record was read from. A failed read keeps the last list. After each round
the Desk looks for news (below). With `fetch_seconds = 0` it fetches nothing
and looks for news every five minutes.

## The stop

A proposed plan's stop, `POST /api/action` with `action: stop`, is carried through
by the Desk, as atelier-2vdq defines a stop: the drop on the plan's branch,
pushed, and the pull request closed (`crates/atelier-desk/src/stop.rs`).

1. **Check.** It opens the served repository for reading and finds the
   mission with `Repo::find_mission`. It refuses unless the mission is
   proposed (`Located::proposed`), the configuration names a forge remote,
   and the plan's branch is on that remote as last fetched.
2. **A clone of its own.** Its writes go through a checkout of the branch
   they change, never a served repository's own checkout: the clone
   `<state>/work/<repo>`, made on first use with `git clone --no-checkout`
   from the served repository, its remote's URL and push URL and its
   committer's name and email set as the served repository has them. It
   fetches the plan's branch and the base from the remote, checks the
   branch out at the remote's tip, and removes any file a failed attempt
   left.
3. **The record.** It opens the clone with `Repo::open`, naming the
   configured caller. The clone has the forge's base as it is now, so it
   refuses, changing nothing, a plan that merged since the served
   repository last fetched: that is an agreed mission's stop. Otherwise it
   calls `atelier_core::plan::drop_mission(&mut repo,
   id, reason)`, the function `atelier drop <mission>` calls, which writes
   the mission dropped with `Dropped: <reason>` first in its Result and
   commits nothing. A refusal from it is the response.
4. **Commit and publish.** It commits the paths it wrote with the message
   `Drop <id>`, the one `atelier drop` prints, pushes the branch without
   force, so a branch that moved since the fetch refuses it, and closes the
   branch's pull request with `gh pr close <branch> --comment "Stopped from
   the Desk by <caller>: <reason>"`. A branch with no pull request is
   stopped once pushed; a `gh` failure is said in the response, which names
   the command that finishes it.
5. **Read again.** It fetches the served repository and reads its pull
   requests again, so the page shows the plan stopped.

The page shows the plan `stopped` once the drop is on the forge's copy of
the branch and a list of pull requests read with the branch at that tip has
its pull request closed or none, and `stop_pending` before. The Desk keeps,
with each list, the tip of every remote branch as fetched just before it was
read; a list counts for a branch only while the branch is still at that tip.
So a drop pushed after the last list, by the Desk or by another clone, is
pending until a list is read with it fetched, whether the list before said
closed, open or none, and a page read while that list is being read says
pending too. A list that fails keeps the last one, and `fetched.error` says
why; a plan whose stop that last list confirmed stays stopped. An agreed
mission dropped on any unmerged branch, its own too, is `stop_pending`
until that branch merges, whatever its pull request says.

An agreed mission's stop reaches the base by `atelier start --backlog`,
`atelier drop`, a commit, `atelier submit` and `atelier merge`. `submit` and
`merge` live in `atelier-cli`, and the Desk runs no `atelier` binary (ADR
0032), so the Desk does not make it until atelier-yclc moves them into the
core; the page lists the commands (`stop_commands`). Someone else's stop is
read like any record: the agreed mission is `stop_pending`, with `stop_on`
naming the unmerged branch that drops it, until that branch merges, and
`stopped` once the base, or the forge's base as last fetched, holds it
dropped.

The tests `a_proposed_plan_is_stopped_from_its_page_and_shown_stopped_once_published`
and `an_open_missions_stop_is_pending_until_its_branch_merges_and_the_desk_does_not_make_it`
(`crates/atelier-desk/tests/actions.rs`) work against a bare remote with a
`gh` stand-in. The first stops a plan through `POST /api/action`, finds the
drop pushed and the pull request closed, finds the record equal to the one
`drop_mission` writes in another clone, and that the served repository's
`HEAD`, files and local branches are unchanged. `action-is-a-record` in
`desk/live/` does the same through the built app and the binaries.

## Being told

After each round of fetches the Desk compares each repository's missions
(`Repo::all_missions`) with what it saw last (`crates/atelier-desk/src/news.rs`):

- a mission proposed now that was neither proposed nor agreed before is
  news that a plan was proposed;
- a mission agreed now (on the base) that was not before is news that it
  merged, whether or not a look saw it proposed: a plan proposed and merged
  between two looks, or while the Desk was down, is still told.

The first look at a repository remembers and tells nothing, so starting the
Desk tells nothing. What it has seen, and the last 50 pieces of news, are
kept in `<state>/news.json`, so a restart tells nothing twice. Each piece is
listed in the inbox's `news` and, with a `[notify]` table, sent: the Desk
runs the configured command with three more arguments, a title ("Plan
proposed: <title>" or "Plan merged: <title>"), a line of text, and a link to
the mission's page (`url` and `/r/<repo>/mission/<id>`). A command that
fails is logged; the news stays in the inbox.

On this host the command is `notify-mail personal`, which mails the human
from the Gmail account `gog` knows by that alias to the same address. The
address is looked up in `gog`'s keyring, so no address is in the
configuration. The installed configuration leaves `[notify]` commented out
until they turn it on.

## Action functions

What the server calls for the stop, and where it stands:

| Step | Function | Returns |
| --- | --- | --- |
| Make the invocation's skip | `atelier_core::Skip::new(None)` | A `Skip` that passes nothing |
| Open for reading only | `atelier_core::Repo::open_committed(dir: &Path, caller: Thread, skip: &Skip)` | `anyhow::Result<Repo>`; refuses a caller with a line break or `: ` |
| Open the Desk's clone by path, naming the person | `atelier_core::Repo::open(dir: &Path, caller: Thread, skip: &Skip)` | The same |
| Drop the mission | `atelier_core::plan::drop_mission(&mut Repo, id: &str, reason: &str)` | `anyhow::Result<Result<Done, Refused>>`. `Done` has the text and the paths written, and commits nothing |
| Commit the paths written | `atelier_core::git::commit_paths` | The commit, with no trailer, since the skip passes nothing |
| Close the pull request | `atelier_core::forge::close_pull_request(gh: &Path, dir: &Path, branch: &str, comment: &str)` | `anyhow::Result<bool>`: `false` when the branch has no open pull request |

The caller is `Thread(caller)` with `caller` from the configuration, whole.
The skip is always `Skip::new(None)`. The server runs no `atelier` binary
and reads no command's output. It writes only in its own clone. It starts
`git` through `atelier_core::git`, as the command does: its fetch, clone,
config, commit and push, and the other reads above. Its fetches and the
stop's calls in its clone it makes with `atelier_core::git::unattended`, so
a remote that wants a password or a host key fails instead of waiting on a
terminal; the clone copies from the served repository's path, which asks
nothing. It starts `gh`, the program its configuration names, through
`atelier_core::forge`: `list_pull_requests` on each fetch and
`close_pull_request` in the stop.

## The brief and the debrief on the mission page

What the mission page shows in place of each part of the old tool's mission
brief and debrief (its `atelier-app`'s `mission_review.rs`, `Brief` and
`Debrief`, served by its `atelier-site`'s `api.rs`):

| Old part | On the mission page | From |
| --- | --- | --- |
| Brief: header, status, Generated from | Title, status pill, `in_progress`, `commit` | `Mission`, `in_progress`, base tip |
| Brief: Outcome | The Outcome section | `MissionDir::section` |
| Brief: Strategy | The Documents tab. The converter moved each open mission's strategy into its directory as `design.md`, with `notes.md` beside it | M2, `CommitReader::text` |
| Brief: Success criteria, unproven marks | The Success criteria section as written. No proof marks: the new records have no criteria parser or proof receipts | `MissionDir::section` |
| Brief: Sizing, Derisking, Risks and Unknowns | Read in `design.md` on the Documents tab, not lifted out | `CommitReader::text` |
| Brief: Scope (items, size, blockers, unknowns) | Items grouped by status, with blockers and labels | `items_of`, `Entry` |
| Brief: human decisions | Waiting on you: needs-human items | `Entry::needs_human` |
| Brief: planning review events | Plan reviews, with findings and what changed since each | `MissionDir::reviews`, M6 |
| Brief and debrief: Next | The stop for a proposed plan, the commands for an agreed mission; the action's own `next` on a refusal | `state`, `Refused::next` |
| Debrief: status line | Status, counts per status | `Mission`, `items_of` |
| Debrief: The Problem (Why) | The Why section | `MissionDir::section` |
| Debrief: What we set out to do | The Success criteria section | `MissionDir::section` |
| Debrief: Results (proof marks, delivered by) | The Result section `close` requires | `MissionDir::section("Result")` |
| Debrief: What we did (report, fixed in review, closed at) | Each done item's Report and its fix-now findings carried as fixed; when it merged needs the history | `ItemDir::section`, `ReviewFile::findings`; M5 for the time |
| Debrief: Validation | The item's Report, whose `### Validation` the app shows within it | `ItemDir::section("Report")` |
| Debrief: Planned work, Plan against actual (added, found, dropped, left) | Dropped items with their reason and open items, now. What was planned at go, and what joined after, need the history | `ItemDir::dropped`, `Entry`; M5 |
| Debrief: found work | Items taken from a deferred finding, named in its `Taken` settlement | `ReviewFile::findings` |
| Debrief: Needs your review (asks) | Waiting on you | As above |
| Debrief: decisions | Each item's Decisions from before `escalate` was removed, on its page; disagreements on the Disputed page | `ItemDir::decisions`, `disputes` |
| Debrief: timeline events | The Activity tab | M5 |
| Debrief: base exemptions | Items merged trivially, by their `trivial` reason | `Item::trivial` |

## Tying a thread to an item

A record holds a thread's identity in five places, and nowhere else:
`Item::implementers` (the last holds the item, `Entry::holder`), each
review's `reviewers` (the open round's holder is `Entry::reviewer`),
`Mission::lead`, `Mission::planners`, and `Answer::by` in a Decisions entry
from before `escalate` was removed. Under T3 Code the
identity is `T3CODE_THREAD_ID` (`atelier_core::identity`, master), which is
the thread's `threadId`. The server builds one map from identity to what it
holds, `{repo, kind, id, role}` with role `planner`, `implementer`, `reviewer`,
`lead` or `decider`, and looks each T3 thread's `threadId` up in it, comparing
whole strings. An identity that is no T3 thread, such as a Claude Code
session ID, `human`, or a Codex subagent's `<parent>/<thread>`, is shown as
text with no link. It is never split to find its parent.

Example, from a converted clone, from before atelier-v958 served the live
repositories. The converted clone at
`~/.cache/atelier-desk/clones/atelier` holds no thread ID at all: the
converter writes `implementers: []` and `lead: null` and carries no review,
so all 169 items show none. To get one, a second clone of it at
`~/.cache/atelier-desk/scratch/atelier-1ie1-threads` ran the new binary's
`start atelier-1ie1` inside this item's T3 thread, which made commit
`ba8cd9f8` on branch `atelier-1ie1`:

```console
$ git show atelier-1ie1:.atelier/items/atelier-1ie1/item.md | sed -n '/^implementers/,/^ready/p'
implementers:
- thread:delegated-task:command%3Amcp%3Ad4040a34-c28b-4672-8918-87b1ffbc9fe0%3Adelegate-task%3A4e47a5ad-9a52-4091-8f48-b4a628778aef
ready:
```

T3 Code's `t3_thread_read` with that string as `threadId` answered with
the thread: title "Implement atelier-1ie1: the Desk's API document",
`status: running`, `providerInstanceId: claudeAgent`,
`model: claude-opus-5-5`, `branch: null`. So the item page shows under
Agents: that title as a link, "Implementer", running, Claude Opus 5.5. T3
Code gives `branch: null` although the thread works on branch
`atelier-1ie1`, so a thread's branch cannot do the join, and neither can
its title.

A live thread that no record names is tied to no repository: on
`/api/threads` it is listed with empty `holds`, and it is in no
repository's inbox. The Desk does not guess from where the thread runs.
A T3 project's workspace root names one checkout, such as
`~/projects/atelier` for the project `atelier`, while its
threads work in worktrees elsewhere. A thread that holds nothing in the
records is doing no tracked work, so tying it to a repository would show
it beside work it is not doing.

An item with none, such as `atelier-psjw` in the same clone
(`implementers: []`), shows "No thread has started this item." An open item
says that and nothing else; a started one whose holder is no live thread
shows the holder's identity and "not running in T3 Code".

## Missing views

Each is added to `atelier-core`, where the CLI uses it
too, in the order the mission page needs them. Sizes: complexity, value,
risk. Item atelier-l5ai added all but M5; "Given by" names the function.
The mission page's data is `atelier_core::mission_view(&mut repo, id)`, a
`MissionView`, and the item page's `atelier_core::item_view(&mut repo, id)`,
an `ItemView`; `show` prints each.

| ID | What it must compute | Given by | Complexity | Value | Risk |
| --- | --- | --- | --- | --- | --- |
| M1 | Open a repository for reading by path and caller, with the base branch from `config.toml` at a commit (the commit `HEAD` names) and every item read from commits, never from the opened checkout's files | `atelier_core::Repo::open_committed` | Low | High: the strategy's rule | Low |
| M3 | A mission's page as `show <mission>` prints it: the `MissionDir`, `in_progress`, its entries, its disputes, its plan reviews | `atelier_core::mission_view`, returning `MissionView` (plan reviews in `MissionView::dir.reviews`, and its items' records in `MissionView::located`, which atelier-uwxy added); `show` calls it | Low | High: keeps the Desk and `show` agreeing, which desk-matches-cli checks | Low |
| M2 | The paths of a mission's documents at a commit: the files of its directory but `mission.md` and `reviews/` | `atelier_core::records::CommitReader::mission_documents` | Low | High: the strategy is a document now | Low |
| M4 | An approved item's approval time on `Entry` | `atelier_core::annotate` and `annotate_all`, with `atelier_core::git::approved_at`; the CLI calls them | Low | Medium | Low |
| M7 | An item's page as `show <item>` prints it, with whether its record came from the base or the branch | `atelier_core::item_view`, returning `ItemView`, and `Located::read_from` (`ReadFrom::Base` or `ReadFrom::Branch`); `show` calls it | Low | Medium | Low |
| M6 | What in a mission's plan changed between a reviewed commit and the base's tip | `atelier_core::plan::plan_changed_at`, comparing two commits; `plan_changed`, which `review` uses, still compares the checkout | Low | Medium: tells the human at go whether the review is stale | Low |
| M5 | The history of actions with the commit, time and thread of each, as `atelier log` is specified ([the design record](../design.md#the-log)), for a mission and its items | `atelier_core::log::history_until`, returning `History`; `log` calls it through `history` | High: a Git walk of both merge parents and branch tips, with record diffs per commit | Medium: the timeline, planned against added, and when an item merged | Medium |

M1 to M4 come before the mission page can match `show`. M5 followed it
(atelier-js4a) as a route: the app does not draw it yet.

## Spike pages left for a later mission

The spike's Health page, its Insights page and the gardener and backlog
digest view (`Digest.tsx`) are out of this mission (strategy, Scope: the
next mission's health, feed and database). They are not designed here.
The spike's activity feed (`Feed.tsx`, `/r/:repo/activity`), built from
`git log` subjects, waited for M5, and is left for that mission too.

## Choices made here

- Repository routes nest under `/api/repos/{repo}`, so a repository's
  name never collides with a route name.
- One `POST /api/action`, with the action named in the body, so atelier-yclc's
  stop of an agreed mission is the same request.
- A refusal is `200` with `result: refused`, because it is an answer the
  page shows, not a failed request. The same holds for a session that
  cannot start.
- The app's header is `x-atelier-desk: 1`, as the spike's was
  `x-atelier-deck: 1`.
- The Desk always acts with `Skip::new(None)`: a skip is an agent's
  reason carried in a commit trailer, and the human acting from a page is
  shown the refusal and its next step instead.
- `can_stop` decides only whether a button shows; the stop checks again
  when it runs.
- Documents are shown whole, not split into Sizing or Risks: a mission's
  planning documents have no required shape (`redesign-architecture.md`,
  Records), so lifting out sections would be a rule only the Desk had.
- The inbox adds proposed plans to the needs-human list, because a plan
  goes ahead unless they stop it.
- The Desk writes a stop in a clone of its own rather than a worktree of
  the served repository, so it adds no worktree, branch or ref to a
  repository a person or a thread works in; the served repository is only
  read and fetched.
- Pull requests are read on the fetch timer, with one `gh pr list` per
  repository, not on each request, so no page waits on the forge.
- Live threads are polled as a list and joined to records on the server,
  so a page needs one request for its agents.
- A response read from records is cached by its commits and refs without
  its threads, pull requests or fetch time; each request joins those to it,
  so none is older than the last poll or fetch.
