# The Desk's API

Status: the contract for mission atelier-z4cr, closed on 2026-11-11,
written by item atelier-1ie1 before any server code, and corrected by items
atelier-uwxy, atelier-e05n and atelier-h3hl where the server as built
differs. It says what each page
of the Desk is sent, and for every field, which function of the redesigned
core in `crates/` gives it, or that none does yet. The decisions are in
[the design record](../design.md#decisions); what the Desk is
for is in [its product intent](../product/desk.md).

## Verdict

The core can feed every page. Nothing in the strategy's Design has to
change. Two facts shape the order of work:

- **Reads need atelier-psjw too, not only acts.** On master, `Located`,
  `Entry` and `entries` are in `atelier-app`, but the code that builds a
  `Located` for every item (the base's record plus the branch named for it,
  `Repo::everything`) is in `atelier-cli/src/repo.rs`. The atelier-psjw
  branch moves `Repo` into `atelier-app`. Until it merges, the Desk cannot
  read items the way `status`, `list` and `show` do without copying CLI code.
- **Seven views were missing**, listed with sizes in
  [Missing views](#missing-views). Six were small moves or exposures of code
  that existed, and atelier-l5ai has added them. One, the history of acts (`atelier log`), is specified and
  not built, and only the debrief's timeline and its plan-against-actual
  need it; the mission page ships without them first.

In the tables below, **master** means the function exists on master today
(3715b5f9), **psjw** that it exists on the atelier-psjw branch (55d88b72)
and not on master, and **M1** to **M7** name a view that was missing.
atelier-psjw has since merged, so its marks now mean on master too, and
atelier-l5ai added M1, M2, M3, M4, M6 and M7: where a table names one of
those, the function that now gives it follows. Only M5 is still missing.

## How the server reads

For each repository under its configured roots, a request does this, inside
one blocking task:

1. Open the repository for reading with the human's identity as caller:
   `atelier_app::Repo::open_committed(dir, caller, &skip)` (M1).
   The caller is the name the Desk is configured with, such as `sumeet`,
   never a value from the request.
2. `Repo::everything()` (psjw) gives a `Located` for every item on the base
   and on each local branch named for one. `Repo::missions()` (psjw) gives
   every `MissionDir` at the base's tip.
3. `atelier_app::entries(&items, &missions, &caller)` (master) gives one
   `Entry` per item, with the record the cross-branch rule picks
   (`Located::record`, master).
4. Pages are built from those values and the record readers below.

The cache key is the base's tip and the tip of every local branch named for
an item (`atelier_app::git::branches`, psjw), with the checkout opened and
its `HEAD`, whose commit holds the config read. A request opens the
repository and reads those tips; while they are unchanged it is answered
from the response kept for them, and a new commit on any of them builds the
response again, with no restart. Nothing reads a working tree:
a `Repo` opened with `open_committed` reads `.atelier/config.toml` from the
commit `HEAD` names, and the item of the branch the opened checkout has out
from that branch's tip, where `Repo::open` reads both from the checkout's
files. `HEAD` can still be an old commit: on this host the main atelier
checkout is detached far behind master, so its `HEAD` holds the config of
that old commit, not the base's. The server opens the checkout that has the
base out, `atelier_app::git::checkout_of(root, base)`, where there is one.
Planning's and the acts' methods of `Repo` (`store`, `checkout_item`,
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
$ cargo build -p atelier-desk
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
caller = "sumeet"
# Each direct child of a root that is a Git checkout whose HEAD holds
# .atelier/config.toml is served under its directory name. A child with no
# Atelier config is skipped; one that does not open, or holds a record that
# does not read, is listed by /api/repos with its error.
roots = ["~/.cache/atelier-desk/clones"]
# Addresses to listen on. Default: 127.0.0.1:3780.
listen = ["127.0.0.1:3780"]
# The names a request's Host header may give, without its port. Any other
# is refused with 403. Default: localhost, 127.0.0.1 and [::1].
hosts = ["localhost", "127.0.0.1"]
# The built app's directory, served for every path outside /api, with its
# index.html for a path that names no file. Without it only /api is served.
app = "/path/to/atelier/desk/dist"

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
```

A path may start with `~/`, and a relative one is read from the
configuration file's directory; that holds for `t3.command` too. When two roots have a child of the same
name, the first is served and the second is listed with an error.

The app's TypeScript types are generated from `src/api.rs` into
`desk/src/api/`, one file per type, and committed. The test
`the_committed_typescript_types_are_current` fails when they differ, and
writes them again when run with `ATELIER_UPDATE_TYPES=1`:

```console
$ ATELIER_UPDATE_TYPES=1 cargo nextest run -p atelier-desk --test types
```

## Running it as a service

On this host the Desk runs as the systemd user service `atelier-desk`,
reached over the tailnet at <http://buildbot.tail47515a.ts.net:3780/>
(item atelier-va99). It serves converted scratch clones of the five
repositories until this host's cutover; pointing it at the live
repositories is the follow-up item atelier-qxmr. Everything it needs is in
`desk/service/`:

| File in the repository | Installed as | What it is |
| --- | --- | --- |
| `atelier-desk.service` | `~/.config/systemd/user/atelier-desk.service` | The unit |
| `t3` | `~/.local/share/atelier-desk/t3` | The `[t3] command`: runs the `t3` of the T3 Code version that is running |
| `install.sh` | | Builds, installs or updates, restarts and checks the service |
| `make-clones.sh` | | Makes or refreshes the converted clones |
| `make-clones.test.sh` | | Checks that `make-clones.sh` refuses a path inside a source repository before writing |
| (written by `install.sh` once) | `~/.config/atelier-desk/config.toml` | The configuration |
| (the release build) | `~/.local/share/atelier-desk/atelier-desk` | The server |
| (`desk/dist`) | `~/.local/share/atelier-desk/app/` | The built app |
| (`make-clones.sh`'s output) | `~/.cache/atelier-desk/clones/<repo>/` | The served clones; `~/.cache/atelier-desk/clones.work/` holds their logs |

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
$ cargo build -j 4 -p atelier-cli
$ desk/service/make-clones.sh ~/.cache/atelier-desk/clones
$ desk/service/install.sh --caller sumeet
```

`make-clones.sh` clones each repository's base branch from `~/projects/`
(atelier `master`, astromech `main`, demiurge `master`, life-on-rails
`main`, cosplay-studio `main`), disables pushes, converts the records with
`scripts/cutover/convert_records.py`, commits and verifies the conversion,
and runs the new `atelier check`, printing one line per step. It only reads
the repositories under `~/projects/`: before its first write it refuses a
clones path that, with symbolic links resolved, lies inside `~/projects/`
or a source repository. It moves a clone into place only when every step
passed. `install.sh` builds `atelier-desk` (release, `-j
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

To refresh the clones as well, which the service needs no restart for:

```console
$ cargo build -j 4 -p atelier-cli
$ desk/service/make-clones.sh ~/.cache/atelier-desk/clones
```

`install.sh` keeps the configuration. The server reads each clone's
commits on every request, so a refreshed clone shows at once.

### Check

```console
$ systemctl --user status atelier-desk
$ curl -s -H 'Host: localhost' http://127.0.0.1:3780/api/repos | jq -r .built_from
$ curl -s -o /dev/null -w '%{http_code}\n' http://buildbot.tail47515a.ts.net:3780/api/repos
$ curl -s -o /dev/null -w '%{http_code}\n' -H 'Host: elsewhere.example' http://127.0.0.1:3780/api/repos   # 403
$ journalctl --user -u atelier-desk
```

The mission's two success criteria each have a command in `desk/live/`;
`desk/README.md` says how to run them.

## The app

The app is `desk/` (item atelier-94pv; `desk/README.md` has its commands).
Its routes are `/` (repositories), `/inbox`, `/r/{repo}` and
`/r/{repo}/mission/{id}` (the rail and one mission, with its Overview,
Work and Documents tabs as `?tab=`, and a document as `?doc=<path>`),
`/r/{repo}/item/{id}`, and `/agents` and `/r/{repo}/agents` (T3 Code's live
threads from `GET /api/threads`; the second shows only the threads, and the
holds, of that repository). Where it reads the API differently from what
the tables below suggest:

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
- Say go, the answer form and Start a planning session are the app's only
  act controls. Say go and a session each ask once before they send. The
  answer form answers an item's oldest open question only, as `decide`
  does, and offers a settlement only when the question names a finding.
  After a `done`, the app reads every page on screen again before showing
  the act's text.
- `pending_answers` carries no place among the item's decisions, so the
  app matches each to the first unanswered question of the same item with
  the same text and finding, in order. The inbox has no
  `pending_answers`: it drops an answered question. The app keeps an
  answer given on that visit beneath the questions, marked as not
  committed yet, for as long as the item's own `pending_answers` (read
  from `/items/{id}`) lists it.

## Routes

All responses are JSON, typed for the app with `ts-rs`. Times are RFC 3339
strings, and enums are their record names, such as `in_review`. A refused
`Host` is `403`, an ID that does not validate or a document path that
leaves the root is `400`, an unknown repository, record or route is `404`,
and a read that fails is `500`; each has `error`. `{repo}` is the
name the Desk's configuration gives a repository. An item or mission ID is
checked with `atelier_records::validate_id` (master) before use. A thread
identity never appears in a path: it can hold `/`, `:` and `%`, as
`thread:delegated-task:command%3Amcp%3A...` does.

| Route | Page | Gives |
| --- | --- | --- |
| `GET /api/repos` | Repositories | Each repository with its counts |
| `GET /api/repos/{repo}/missions` | Mission rail | The repository's missions and loose work |
| `GET /api/repos/{repo}/missions/{id}` | Mission | One mission: sections, documents, items, questions, plan reviews, threads |
| `GET /api/repos/{repo}/missions/{id}/history` | Mission, Activity tab | The acts on the mission and its items (M5; 404 until it exists) |
| `GET /api/repos/{repo}/doc?path=<path>` | Mission, Documents tab | One Markdown file at the base's tip |
| `GET /api/repos/{repo}/items/{id}` | Item | One item in full |
| `GET /api/inbox` | Inbox | What waits on the human, in every repository |
| `GET /api/threads` | Live threads | T3 Code's live threads, each with what it holds in the records |
| `POST /api/act` | Mission, Item | Runs `ready` or `decide` |
| `POST /api/sessions` | Mission | Starts a planning session as a T3 Code thread |

`?done=true` on `/missions` lists done and dropped missions, as
`Filter::done` does for `list` (master).

The route for `/history` is not served yet (M5): it answers `404`.

## Responses, field by field

Every response for one repository also has `commit`: the base's tip it was
read at (`Repo::base_tip`, psjw). `/api/repos` gives each repository's
`base_tip` instead, and `/api/inbox` a `commit` for each repository.

### `GET /api/repos`

| Field | Given by | State |
| --- | --- | --- |
| `name`, `path` | The Desk's configuration | Desk |
| `base` | `Repo::base`, from `config.toml` at `HEAD` | M1: `Repo::open_committed` |
| `base_tip` | `Repo::base_tip` | psjw |
| `in_progress` | Count of missions where `atelier_app::in_progress(mission, &entries)` | master |
| `drafts` | Count of `MissionDir` with `MissionStatus::Draft` | master |
| `waiting` | Item count of `atelier_app::list(&entries, &missions, &caller, &Filter { needs_human: true, .. })` | master |
| `error` | The error text when the repository does not open or a record does not read (`atelier_records::ReadError`, master); then `base` and `base_tip` are `null` and the counts `0` | Desk |

Beside `repos`, `built_from` is the commit the server was built from
(`atelier_desk::BUILT_FROM`), or `unknown`. The app shows its first nine
characters in the sidebar's footer, with the whole ID in its title.

### `GET /api/repos/{repo}/missions`

`missions[]`, one per open mission, in ID order, as `list` orders groups:

| Field | Given by | State |
| --- | --- | --- |
| `id`, `title`, `status`, `labels`, `lead`, `ready {by, at}`, `created_at` | `atelier_core::Mission`, read by `CommitReader::mission` | master |
| `in_progress` | `atelier_app::in_progress` | master |
| `counts[] {status, count}` | `items_of(id, &entries)` grouped by `Entry::status` in `LIFECYCLE` order, as `show <mission>` groups them; a status with no item is left out | master |
| `waiting[]` | The IDs of the open items of `items_of` with `Entry::needs_human`, or whose record has an unanswered question (`acts::question`) | master |

`loose`: the item count of the group with `id: None` from `list` (master),
the spike's "work outside missions", with the same `done` flag.

### `GET /api/repos/{repo}/missions/{id}`

The page is read from `atelier_app::mission_view`, as `show <mission>` is.
Its item rows also need each item's records, which `MissionView::located`
gives (added by atelier-uwxy).

| Field | Given by | State |
| --- | --- | --- |
| `mission` | The mission's row, as on the rail: `id`, `title`, `status`, `labels`, `lead`, `ready`, `created_at`, `in_progress`, `counts`, `waiting` | master |
| `sections[] {heading, markdown}` | `MissionDir::section` for Why, Outcome, Success criteria, Scope, Authority and Result, those present | master |
| `dropped` | `MissionDir::dropped` | master |
| `can_ready` | `acts::check_mission(Act::ReadyMission, &facts).is_ok()`; it shows Say go | master |
| `documents[] {path, title}` | Every file of `.atelier/missions/<id>/` but `mission.md` and `reviews/`; `title` is its first `# ` heading, else its name | M2: `CommitReader::mission_documents(commit, id)` |
| `items[]` | `MissionView::items`, each as an item row (below) | master |
| `questions[] {item, question, finding, answer {by, text}, outcome}` | `ItemDir::decisions` of each item's record | M3: `MissionView::questions` |
| `plan_reviews[]` | `MissionDir::reviews`, each as a review (below) | master |
| `plan_reviews[].changed_since[]` | The plan's paths that differ between the commit reviewed and the base's tip; `null` when this clone lacks that commit | M6: `atelier_app::plan::plan_changed_at(&mut repo, id, commit, base_tip)` |
| `threads[]` | Its lead and the reviewers of its plan reviews, with `item: null`, then each of its items' agents (below) with `item` naming the item, so a mission with no lead still shows who works on its items; joined as in [Tying a thread to an item](#tying-a-thread-to-an-item) | Desk, T3 Code |
| `t3` | Whether T3 Code could be read, as on `/api/threads` | Desk |
| `pending_answers[]` | Answers the Desk gave with `decide` to its items' questions that no commit holds yet, each as in `questions[]`; see [Answers not yet committed](#answers-not-yet-committed) | Desk memory |

An **item row**, in a mission, the inbox and the rail's loose work:

| Field | Given by | State |
| --- | --- | --- |
| `id`, `title`, `status`, `priority`, `labels` | `Entry` | master |
| `needs_human`, `startable` | `Entry::needs_human`, `Entry::startable` | master |
| `blockers[] {id, status}`, `waiting_on[]` | `Entry::blockers`; the IDs of `Entry::waiting_on` | master |
| `holder`, `reviewer` | `Entry::holder`; `Entry::reviewer`, set only while in review | master |
| `cannot_start[]` | For a ready item, `say::condition_text(id, c)` for each of `Entry::unmet` | master values, psjw text |
| `needs[]` | For a started item, `say::condition_text` for each of `Entry::needs`, what its holder's next act lacks | master values, psjw text |
| `branch {merged}` | `Located::branch`; absent when there is no branch | psjw |
| `question` | `acts::question(record)`, the oldest unanswered, with the date its heading gives | master |
| `report_written`, `dropped` | `ItemDir::report_written`, `ItemDir::dropped` | master |
| `approved_at` | `Entry::approved`, which `atelier_app::annotate` fills, from `atelier_app::git::approved_at`; `mission_view` and `item_view` annotate their entries, and the inbox annotates its rows | M4 |

A **review**, of an item or a mission's plan:

| Field | Given by | State |
| --- | --- | --- |
| `name`, `round` | `ReviewFile::name` | master |
| `commit`, `reviewers`, `model`, `verdict`, `created_at` | `atelier_core::Review` | master |
| `findings[] {name, title, recommendation, was, now, decided, settlement}` | `ReviewFile::findings` | master |
| `findings[].text` | `ReviewFile::finding_text` | master |
| `findings[].open`, `findings[].deferred` | `atelier_core::is_open`, `is_deferred`, over all the record's findings and its decisions (none for a mission) | master |
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
| `mission`, `blocked_by`, `implementers`, `ready`, `trivial`, `caused_by`, `created_at` | `atelier_core::Item` of `Located::record` | master |
| `read_from` | `base` or `branch`: whether the record is the base's or the branch's | M7: `ItemView::read_from`, `Located::read_from` |
| `body` | `Item::body`, the whole Markdown | master |
| `goal`, `scope`, `report` | `ItemDir::section` | master |
| `done_when[] {ticked, text}` | `ItemDir::done_when` | master |
| `decisions[]`, `responses[]` | `ItemDir::decisions`, `ItemDir::responses` | master |
| `reviews[]` | `ItemDir::reviews`, each as a review | master |
| `unsettled[]` | The names of `acts::unsettled`, for a done item | master |
| `decide_at` | `git::checkout_of(root, id)`: the checkout with the item's branch out, where `decide` writes; `null` when none has it out, and then `decide` is refused. Read on every request, not cached, since a checkout is added without a commit | psjw |
| `can_decide` | `acts::check(Act::Decide, &atelier_app::repo::facts(..))` for the human caller, from the records the page shows | master values, psjw `facts` |
| `threads[]` | One agent per identity and role the record names: its implementers, the reviewers of its reviews and the deciders of its answers, each with `item: null` | Desk, T3 Code |
| `t3` | As on `/api/threads` | Desk |
| `pending_answers[]` | As on the mission, for this item | Desk memory |

An **agent**, in `threads[]` of a mission or an item: `identity`, whole as
the record holds it; `role`, one of `implementer`, `reviewer`, `lead` or
`decider`; `item`; and `thread`, the live thread's `title`, `status`,
`provider`, `model` and `link` when T3 Code lists the identity as a live
thread, else `null`, and then the page shows the identity as text with no
link. An identity named in two roles is listed once for each.

`show <item>` also shows a settlement written in the base checkout and not
committed. The Desk does not, since it reads commits only.

### `GET /api/inbox`

`repos[]`, each with `repo`, `commit`, `error` (as on `/api/repos`; the
lists are then empty) and the fields below. The response also has `t3`, as
on `/api/threads`.

| Field | Given by | State |
| --- | --- | --- |
| `needs_human[] {mission, mission_title, items[]}` | `list(.., &Filter { needs_human: true, .. })`: `Group::id`, `Group::mission`, `Group::items` as item rows | master |
| `drafts[]` | Missions with `MissionStatus::Draft`, as mission rows on the rail | master |
| `questions[] {item, mission, question, finding}` | `acts::question` of each started item's record, less a question the Desk has answered and no commit holds yet | master |
| `threads[]` | Live threads whose status is `waiting`, which T3 Code gives a thread that waits on a person, and that hold something in this repository's records; each as on `/api/threads`, with only this repository's `holds` | Desk, T3 Code |

`needs_human` lists the same items as `atelier list --needs-human`, which
the criterion desk-matches-cli checks. `drafts` and `questions` are in
addition. A question is listed because the human may answer any question
(atelier-t6o6), not because only he can.

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

### `POST /api/act`

The request is one of:

```json
{ "act": "ready", "repo": "atelier", "ids": ["atelier-z4cr"] }
{ "act": "decide", "repo": "atelier", "id": "atelier-xxxx",
  "answer": "One line.", "settle": null }
```

`settle` is `null` or left out, `"defer"` or `"no_action"`, for
`atelier_core::Outcome`. A field the act does not take is refused. The
caller is never in the request: `ready` and `decide` run under the
configured `caller`, which is also `ready`'s `by`. The types are
`ActRequest` (`ReadyAct` or `DecideAct`) and `ActResponse` in
`desk/src/api/`.

The response is `200` with one of:

```json
{ "result": "done", "text": "Made ready, as sumeet said: atelier-z4cr.\n...",
  "paths": [".atelier/missions/atelier-z4cr/mission.md"], "commit": "76cbb6c..." }
{ "result": "refused", "rules": ["..."], "text": "`atelier ready` refused; ...",
  "next": "Correct what each names, ..." }
```

| Result | Fields | From |
| --- | --- | --- |
| `done` | `text`, `paths[]`, `commit` (`null` for `decide`) | `atelier_app::plan::Done` |
| `refused` | `rules[]` (empty when no rule refused it), `text`, `next` (may be `null`) | `atelier_app::Refused` |

The Desk writes none of this text. A `decide` whose `id` is not an ID is
`400`, an unknown repository `404`, a body that is not JSON `400` or `415`,
JSON that is not one of the shapes above `422`, and a failure that is not a refusal, such as Git
failing, `500`; each with `error`. Acts run one at a time. Every page is
read again after a `done`, since `ready`'s commit moves the base's tip.

#### Only from the app

`POST /api/act` and `POST /api/sessions` are refused with `403` unless the
request has the header `x-atelier-desk: 1`, and its `Origin`, when it has
one, names a host in the configuration's `hosts` (its port is ignored). A
page of another site cannot add the header without the browser asking the
server first, which it does not answer, and a request from such a page
carries that page's `Origin`. The check runs before the method's, so a
`GET` of either route is `403` too. There is no sign-in: anyone who can
reach the Desk at a host it answers to can act as its caller (strategy,
Access).

#### Answers not yet committed

`decide` writes the answer into the item's record in the checkout with the
item's branch out and commits nothing; the implementer's `submit` commits
it. Pages are read from commits, so after a `done` the server keeps the
answer in memory, as the record in that checkout then holds it, with the
item's mission. The item's page and its mission's page list it in
`pending_answers`, and the inbox leaves the question out of `questions`.
An answer is known by its item and its place among the item's decisions,
since a heading may repeat. Once the page's records show the decision at
that place answered, which happens when a commit on the item's branch
holds the answer, the server forgets it. A restart forgets every one; the answer is still in the
checkout, and shows once committed.

### `POST /api/sessions`

Starts a planning session for a mission: a new T3 Code thread in the
repository's T3 project, whose first message names the mission. It writes
no record. The request is `SessionRequest`:

```json
{ "repo": "atelier", "mission": "atelier-z4cr" }
```

The mission must be in the base's last commit (`404` otherwise). The
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
> ({repo}). Load the Agent Factory skill (/agent-factory) and use its
> `plan` procedure on {mission}: read the mission and its documents, then
> work with me on its plan until it is ready for me to say go.

## Act functions

What the server calls for each act, and where it stands:

| Step | Function | Returns | State |
| --- | --- | --- | --- |
| Make the invocation's skip | `atelier_app::Skip::new(None)` | A `Skip` that passes nothing | psjw |
| Find the checkout | `atelier_app::git::checkout_of(&root, branch)` | `Option<PathBuf>`: the worktree with `branch` out | psjw; on master only in `atelier-cli/src/git.rs` |
| Open it by path, naming the person | `atelier_app::Repo::open(dir: &Path, caller: Thread, skip: &Skip)` | `anyhow::Result<Repo>`; refuses a caller with a line break or `: ` | psjw; master's `Repo::open()` in `atelier-cli` takes no arguments and reads the environment |
| Say go | `atelier_app::ready(&mut Repo, ids: &[String], by: &str)` | `anyhow::Result<Result<Done, Refused>>`. `Done` has the text, the paths written and the commit `ready` made of only those paths | psjw; master's `plan::ready` in `atelier-cli` returns text and leaves the commit to its caller |
| Answer a question | `atelier_app::decide(&mut Repo, id: &str, answer: &str, settled: Option<Outcome>)` | The same types. `Done::paths` is the item's `item.md`; `commit` is `None`, since `submit` commits it | psjw; master's `questions::decide` in `atelier-cli` |
| Open for reading only | `atelier_app::Repo::open_committed(dir: &Path, caller: Thread, skip: &Skip)` | `anyhow::Result<Repo>`, as `open` | M1 |

For `ready` the server opens the checkout `checkout_of(root, base)` names,
with `base` read by `Repo::open_committed` on the served directory; for
`decide`, the one `checkout_of(root, id)` names. Where none has the branch
out, it opens the served directory, and the function refuses with its own
text and next step (`say::off_base` or `say::elsewhere`), so the Desk writes
no refusal of its own. The caller is `Thread(caller)` with `caller` from the
configuration, whole, and `by` for `ready` is the same string. The skip is
always `Skip::new(None)`. The server runs no `atelier` binary and reads no
command's output, and makes no worktree, moves no ref and makes no commit
of its own: `ready` commits only the paths it wrote, so a file staged in
that checkout before stays staged.

The test `saying_go_from_the_desk_leaves_the_commit_the_cli_leaves`
(`crates/atelier-desk/tests/acts.rs`) checks the strategy's
act-is-a-record criterion: it says go through `POST /api/act` in one clone
and calls `atelier_app::ready` as the command does in another clone of the
same commit, and finds one new commit in each with the same message and
paths, and the same mission record but for `ready.at`.

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
| Brief: human decisions | Waiting on you: questions and needs-human items | `ItemDir::decisions`, `Entry::needs_human` |
| Brief: planning review events | Plan reviews, with findings and what changed since each | `MissionDir::reviews`, M6 |
| Brief and debrief: Next | The Say go button for a draft; the act's own `next` on a refusal | `check_mission`, `Refused::next` |
| Debrief: status line | Status, counts per status | `Mission`, `items_of` |
| Debrief: The Problem (Why) | The Why section | `MissionDir::section` |
| Debrief: What we set out to do | The Success criteria section | `MissionDir::section` |
| Debrief: Results (proof marks, delivered by) | The Result section `close` requires | `MissionDir::section("Result")` |
| Debrief: What we did (report, fixed in review, closed at) | Each done item's Report and its fix-now findings carried as fixed; when it merged needs the history | `ItemDir::section`, `ReviewFile::findings`; M5 for the time |
| Debrief: Validation | The item's Report, whose `### Validation` the app shows within it | `ItemDir::section("Report")` |
| Debrief: Planned work, Plan against actual (added, found, dropped, left) | Dropped items with their reason and open items, now. What was planned at go, and what joined after, need the history | `ItemDir::dropped`, `Entry`; M5 |
| Debrief: found work | Items taken from a deferred finding, named in its `Taken` settlement | `ReviewFile::findings` |
| Debrief: Needs your review (asks) | Waiting on you | As above |
| Debrief: decisions | Each item's Decisions, with answer and outcome | `ItemDir::decisions` |
| Debrief: timeline events | The Activity tab | M5 |
| Debrief: base exemptions | Items merged trivially, by their `trivial` reason | `Item::trivial` |

## Tying a thread to an item

A record holds a thread's identity in four places, and nowhere else:
`Item::implementers` (the last holds the item, `Entry::holder`), each
review's `reviewers` (the open round's holder is `Entry::reviewer`),
`Mission::lead`, and `Answer::by` in a Decisions entry. Under T3 Code the
identity is `T3CODE_THREAD_ID` (`atelier_app::identity`, master), which is
the thread's `threadId`. The server builds one map from identity to what it
holds, `{repo, kind, id, role}` with role `implementer`, `reviewer`, `lead`
or `decider`, and looks each T3 thread's `threadId` up in it, comparing
whole strings. An identity that is no T3 thread, such as a Claude Code
session ID, `human`, or a Codex subagent's `<parent>/<thread>`, is shown as
text with no link. It is never split to find its parent.

Example, from a converted clone. The converted clone at
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
`/home/supadava/projects/atelier` for the project `atelier`, while its
threads work in worktrees elsewhere, and until the cutover the Desk serves
converted clones, not those checkouts. A thread that holds nothing in the
records is doing no tracked work, so tying it to a repository would show
it beside work it is not doing.

An item with none, such as `atelier-psjw` in the same clone
(`implementers: []`), shows "No thread has started this item." A ready item
says that and nothing else; a started one whose holder is no live thread
shows the holder's identity and "not running in T3 Code".

## Missing views

Each is added to `atelier-app` or `atelier-records`, where the CLI uses it
too, in the order the mission page needs them. Sizes: complexity, value,
risk. Item atelier-l5ai added all but M5; "Given by" names the function.
The mission page's data is `atelier_app::mission_view(&mut repo, id)`, a
`MissionView`, and the item page's `atelier_app::item_view(&mut repo, id)`,
an `ItemView`; `show` prints each.

| ID | What it must compute | Given by | Complexity | Value | Risk |
| --- | --- | --- | --- | --- | --- |
| M1 | Open a repository for reading by path and caller, with the base branch from `config.toml` at a commit (the commit `HEAD` names) and every item read from commits, never from the opened checkout's files | `atelier_app::Repo::open_committed` | Low | High: the strategy's rule | Low |
| M3 | A mission's page as `show <mission>` prints it: the `MissionDir`, `in_progress`, its entries, its questions with the item each is on, its plan reviews | `atelier_app::mission_view`, returning `MissionView` (plan reviews in `MissionView::dir.reviews`, and its items' records in `MissionView::located`, which atelier-uwxy added); `show` calls it | Low | High: keeps the Desk and `show` agreeing, which desk-matches-cli checks | Low |
| M2 | The paths of a mission's documents at a commit: the files of its directory but `mission.md` and `reviews/` | `atelier_records::CommitReader::mission_documents` | Low | High: the strategy is a document now | Low |
| M4 | An approved item's approval time on `Entry` | `atelier_app::annotate` and `annotate_all`, with `atelier_app::git::approved_at`; the CLI calls them | Low | Medium | Low |
| M7 | An item's page as `show <item>` prints it, with whether its record came from the base or the branch | `atelier_app::item_view`, returning `ItemView`, and `Located::read_from` (`ReadFrom::Base` or `ReadFrom::Branch`); `show` calls it | Low | Medium | Low |
| M6 | What in a mission's plan changed between a reviewed commit and the base's tip | `atelier_app::plan::plan_changed_at`, comparing two commits; `plan_changed`, which `ready` and `review` use, still compares the checkout | Low | Medium: tells the human at go whether the review is stale | Low |
| M5 | The history of acts with the commit, time and thread of each, as `atelier log` is specified ([the design record](../design.md#the-log)), for a mission and its items | Missing. The old debrief's reader follows first parents only | High: a Git walk of both merge parents and branch tips, with record diffs per commit | Medium: the timeline, planned against added, and when an item merged | Medium: the walk is unsettled in the specification |

M1 to M4 come before the mission page can match `show`. M5 can follow the
first mission page: its parts show "not available yet" until then.

## Spike pages left for a later mission

The spike's Health page, its Insights page and the gardener and groom
digest view (`Digest.tsx`) are out of this mission (strategy, Scope: the
next mission's health, feed and database). They are not designed here.
The spike's activity feed (`Feed.tsx`, `/r/:repo/activity`), built from
`git log` subjects, waits for M5.

## Choices made here

- Repository routes nest under `/api/repos/{repo}`, so a repository's
  name never collides with a route name.
- One `POST /api/act` for both acts, with the act named in the body, since
  the strategy allows only two and both return the same types.
- A refusal is `200` with `result: refused`, because it is an answer the
  page shows, not a failed request. The same holds for a session that
  cannot start.
- The app's header is `x-atelier-desk: 1`, as the spike's was
  `x-atelier-deck: 1`.
- The Desk always acts with `Skip::new(None)`: a skip is an agent's
  reason carried in a commit trailer, and the human acting from a page is
  shown the refusal and its next step instead.
- `can_ready` and `can_decide` decide only whether a button shows; the act
  function checks again when it runs.
- Documents are shown whole, not split into Sizing or Risks: a mission's
  planning documents have no required shape ([the design
  record](../design.md#records)), so lifting out sections would be a rule only the Desk had.
- The inbox adds draft missions and open questions to the needs-human list,
  because saying go and answering are the two acts the Desk offers.
- Live threads are polled as a list and joined to records on the server,
  so a page needs one request for its agents.
- A response read from records is cached by its commits without its
  threads; each request joins the threads of the last poll to it, so a
  thread's status is never older than the last poll.
