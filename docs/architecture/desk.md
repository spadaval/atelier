# The Desk's API

Status: the contract for mission atelier-z4cr (`docs/strategies/desk.md`),
written by item atelier-1ie1 before any server code. It says what each page
of the Desk is sent, and for every field, which function of the redesigned
core in `next/` gives it, or that none does yet. The decisions are in
[ADR 0032](../adr/0032-the-desk-is-the-humans-product.md); what the Desk is
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
an item (`atelier_app::git::branches`, psjw). Nothing reads a working tree:
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

## Routes

All responses are JSON, typed for the app with `ts-rs`. `{repo}` is the
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
| `POST /api/act` | Any | Runs `ready` or `decide` |

`?done=true` on `/missions` lists done and dropped missions, as
`Filter::done` does for `list` (master).

## Responses, field by field

Every response also has `commit`: the base's tip it was read at
(`Repo::base_tip`, psjw).

### `GET /api/repos`

| Field | Given by | State |
| --- | --- | --- |
| `name`, `path` | The Desk's configuration | Desk |
| `base` | `Repo::base`, from `config.toml` at `HEAD` | M1: `Repo::open_committed` |
| `base_tip` | `Repo::base_tip` | psjw |
| `in_progress` | Count of missions where `atelier_app::in_progress(mission, &entries)` | master |
| `drafts` | Count of `MissionDir` with `MissionStatus::Draft` | master |
| `waiting` | Item count of `atelier_app::list(&entries, &missions, &caller, &Filter { needs_human: true, .. })` | master |
| `error` | The error text when the repository does not open or a record does not read (`atelier_records::ReadError`, master) | Desk |

### `GET /api/repos/{repo}/missions`

`missions[]`, one per open mission, in ID order, as `list` orders groups:

| Field | Given by | State |
| --- | --- | --- |
| `id`, `title`, `status`, `labels`, `lead`, `ready {by, at}`, `created_at` | `atelier_core::Mission`, read by `CommitReader::mission` | master |
| `in_progress` | `atelier_app::in_progress` | master |
| `counts` | `items_of(id, &entries)` grouped by `Entry::status` in `LIFECYCLE` order, as `show <mission>` groups them | master |
| `waiting` | Items of `items_of` with `Entry::needs_human`, plus those whose record has a question (`acts::question`) | master |

`loose`: the item count of the group with `id: None` from `list` (master),
the spike's "work outside missions".

### `GET /api/repos/{repo}/missions/{id}`

| Field | Given by | State |
| --- | --- | --- |
| `id`, `title`, `status`, `labels`, `lead`, `ready`, `created_at` | `Mission` | master |
| `in_progress` | `atelier_app::in_progress` | master |
| `sections[] {heading, markdown}` | `MissionDir::section` for Why, Outcome, Success criteria, Scope, Authority and Result, those present | master |
| `dropped` | `MissionDir::dropped` | master |
| `can_ready` | `acts::check_mission(Act::ReadyMission, &facts).is_ok()`; it shows Say go | master |
| `documents[] {path, title}` | Every file of `.atelier/missions/<id>/` but `mission.md` and `reviews/`; `title` is its first `# ` heading, else its name | M2: `CommitReader::mission_documents(commit, id)` |
| `counts` | As on the rail | master |
| `items[]` | `items_of(id, &entries)`, each as an item row (below) | master |
| `questions[] {item, question, finding, answer {by, text}, outcome}` | `ItemDir::decisions` of each item's record | M3: `MissionView::questions` |
| `plan_reviews[]` | `MissionDir::reviews`, each as a review (below) | master |
| `plan_reviews[].changed_since[]` | The plan's paths that differ between the commit reviewed and the base's tip | M6: `atelier_app::plan::plan_changed_at(&mut repo, id, commit, base_tip)` |
| `threads[]` | The join in [Tying a thread to an item](#tying-a-thread-to-an-item) | master, T3 Code |
| `pending_answers[]` | Answers the Desk gave with `decide` that no commit holds yet | Desk memory |

An **item row**, in a mission, the inbox and the rail's loose work:

| Field | Given by | State |
| --- | --- | --- |
| `id`, `title`, `status`, `priority`, `labels` | `Entry` | master |
| `needs_human`, `startable` | `Entry::needs_human`, `Entry::startable` | master |
| `blockers[] {id, status}`, `waiting_on[]` | `Entry::blockers`, `Entry::waiting_on` | master |
| `holder`, `reviewer` | `Entry::holder`; `Entry::reviewer`, set only while in review | master |
| `cannot_start[]` | For a ready item, `say::condition(id, c)` for each of `Entry::unmet` | master values, psjw text |
| `needs[]` | For a started item, `say::condition` for each of `Entry::needs`, what its holder's next act lacks | master values, psjw text |
| `branch {merged}` | `Located::branch`; absent when there is no branch | psjw |
| `question` | `acts::question(record)`, the oldest unanswered | master |
| `report_written`, `dropped` | `ItemDir::report_written`, `ItemDir::dropped` | master |
| `approved_at` | `Entry::approved`, which `atelier_app::annotate` fills, from `atelier_app::git::approved_at`; `mission_view` and `item_view` annotate their entries | M4 |

A **review**, of an item or a mission's plan:

| Field | Given by | State |
| --- | --- | --- |
| `name`, `round` | `ReviewFile::name` | master |
| `commit`, `reviewers`, `model`, `verdict`, `created_at` | `atelier_core::Review` | master |
| `findings[] {name, title, recommendation, was, now, decided, settlement}` | `ReviewFile::findings` | master |
| `findings[].text` | `ReviewFile::finding_text` | master |
| `findings[].open`, `findings[].deferred` | `atelier_core::is_open`, `is_deferred`, over all the record's findings and its decisions (none for a mission) | master |

### `GET /api/repos/{repo}/doc?path=<path>`

`path` is from the repository's root. The Desk normalizes it and refuses one
that leaves the root or does not end in `.md`. `markdown` is
`CommitReader::text(base_tip, path)` (master). Links in a mission's files
are relative to the file (`../../../docs/adr/...` after conversion), so the
app resolves them against `path`, as the spike's `DocReader` does.

### `GET /api/repos/{repo}/items/{id}`

| Field | Given by | State |
| --- | --- | --- |
| `id`, `title`, `status`, `mission`, `labels`, `priority`, `blocked_by`, `implementers`, `ready`, `trivial`, `caused_by`, `created_at` | `atelier_core::Item` of `Located::record` | master |
| `read_from` | Whether the record is the base's or the branch's | M7: `ItemView::read_from`, `Located::read_from` |
| Item row fields | As above, from `entry(&repo::facts(..))` with `Repo::links` | psjw |
| `body` | `Item::body`, the whole Markdown | master |
| `goal`, `scope`, `report` | `ItemDir::section` | master |
| `done_when[] {ticked, text}` | `ItemDir::done_when` | master |
| `decisions[]`, `responses[]` | `ItemDir::decisions`, `ItemDir::responses` | master |
| `reviews[]` | `ItemDir::reviews`, each as a review | master |
| `unsettled[]` | `acts::unsettled`, for a done item | master |
| `decide_at` | `git::checkout_of(root, id)`: the checkout with the item's branch out, where `decide` writes | psjw |
| `can_decide` | `acts::check(Act::Decide, &facts)` for the human caller | master values, psjw `facts` |
| `threads[]`, `pending_answers[]` | As on the mission | master, Desk |

`show <item>` also shows a settlement written in the base checkout and not
committed. The Desk does not, since it reads commits only.

### `GET /api/inbox`

`repos[]`, each with `repo`, `commit` and:

| Field | Given by | State |
| --- | --- | --- |
| `needs_human[] {mission, mission_title, items[]}` | `list(.., &Filter { needs_human: true, .. })`: `Group::id`, `Group::mission`, `Group::items` as item rows | master |
| `drafts[]` | Missions with `MissionStatus::Draft`, as on the rail | master |
| `questions[] {item, mission, question, finding}` | `acts::question` of each started item's record | master |

`needs_human` lists the same items as `atelier list --needs-human`, which
the criterion desk-matches-cli checks. `drafts` and `questions` are in
addition. A question is listed because the human may answer any question
(atelier-t6o6), not because only he can.

### `GET /api/threads`

The server polls T3 Code's MCP server, `t3_thread_list` with `allProjects`,
`includeSubagents`, `settled: false` and the live statuses (`preparing`,
`queued`, `starting`, `running`, `waiting`), with a read-only token, as the
2026-11-10 note on atelier-aab1 describes.

| Field | Given by | State |
| --- | --- | --- |
| `id`, `title`, `status`, `provider`, `model`, `parent`, `branch`, `pull_requests`, `link`, `updated_at` | T3 Code: `threadId`, `title`, `status`, `providerInstanceId`, `model`, `parentThreadId`, `branch`, `pullRequests`, `link`, `updatedAt` | T3 Code |
| `holds[] {repo, kind, id, role}` | The join below, over every repository | master |

Effort, context use and token counts are not shown: the MCP server gives
none (atelier-aab1), and the strategy puts them out of scope.

### `POST /api/act`

The request is one of:

```json
{ "repo": "atelier", "act": "ready", "ids": ["atelier-z4cr"] }
{ "repo": "atelier", "act": "decide", "id": "atelier-xxxx",
  "answer": "One line.", "settle": null }
```

`settle` is `null`, `"defer"` or `"no_action"`, for
`atelier_core::Outcome`. The app sends the header the strategy's Access
section names. The response is `200` with one of:

| Result | Fields | From |
| --- | --- | --- |
| `done` | `text`, `paths[]`, `commit` (`null` for `decide`) | `atelier_app::plan::Done` (psjw) |
| `refused` | `rules[]`, `text`, `next` (may be `null`) | `atelier_app::Refused` (psjw) |

A failure that is not a refusal, such as Git failing, is `500` with
`error`. Every page is read again after a `done`.

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

For `ready` the server opens the checkout `checkout_of(root, base)` names;
for `decide`, the one `checkout_of(root, id)` names. Where none has the
branch out, it opens the repository's root, and the function refuses with
its own text and next step (`say::off_base` or `say::elsewhere`, psjw), so
the Desk writes no refusal of its own. `by` for `ready` is the same name as
the caller. The skip is always `Skip::new(None)`.

## The brief and the debrief on the mission page

What the mission page shows in place of each part of `atelier brief` and
`atelier debrief` (today's binary, `crates/atelier-app/src/mission_review.rs`,
`Brief` and `Debrief`, served by `crates/atelier-site/src/api.rs`):

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
| M3 | A mission's page as `show <mission>` prints it: the `MissionDir`, `in_progress`, its entries, its questions with the item each is on, its plan reviews | `atelier_app::mission_view`, returning `MissionView` (plan reviews in `MissionView::dir.reviews`); `show` calls it | Low | High: keeps the Desk and `show` agreeing, which desk-matches-cli checks | Low |
| M2 | The paths of a mission's documents at a commit: the files of its directory but `mission.md` and `reviews/` | `atelier_records::CommitReader::mission_documents` | Low | High: the strategy is a document now | Low |
| M4 | An approved item's approval time on `Entry` | `atelier_app::annotate` and `annotate_all`, with `atelier_app::git::approved_at`; the CLI calls them | Low | Medium | Low |
| M7 | An item's page as `show <item>` prints it, with whether its record came from the base or the branch | `atelier_app::item_view`, returning `ItemView`, and `Located::read_from` (`ReadFrom::Base` or `ReadFrom::Branch`); `show` calls it | Low | Medium | Low |
| M6 | What in a mission's plan changed between a reviewed commit and the base's tip | `atelier_app::plan::plan_changed_at`, comparing two commits; `plan_changed`, which `ready` and `review` use, still compares the checkout | Low | Medium: tells the human at go whether the review is stale | Low |
| M5 | The history of acts with the commit, time and thread of each, as `atelier log` is specified (`redesign-architecture.md`, The log), for a mission and its items | Missing. The old debrief's reader follows first parents only | High: a Git walk of both merge parents and branch tips, with record diffs per commit | Medium: the timeline, planned against added, and when an item merged | Medium: the walk is unsettled in the specification |

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
  page shows, not a failed request.
- The Desk always acts with `Skip::new(None)`: a skip is an agent's
  reason carried in a commit trailer, and the human acting from a page is
  shown the refusal and its next step instead.
- `can_ready` and `can_decide` decide only whether a button shows; the act
  function checks again when it runs.
- Documents are shown whole, not split into Sizing or Risks: a mission's
  planning documents have no required shape (`redesign-architecture.md`,
  Records), so lifting out sections would be a rule only the Desk had.
- The inbox adds draft missions and open questions to the needs-human list,
  because saying go and answering are the two acts the Desk offers.
- Live threads are polled as a list and joined to records on the server,
  so a page needs one request for its agents.
