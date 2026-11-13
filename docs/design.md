# Atelier: design record

This is Atelier's one design record, edited in place to match the code. It
replaced the ADRs on 2026-11-11, and its [Decisions](#decisions) carry those
of theirs that still govern. The product behaviour it serves is in the
[product specification](product.md). Where this page and the binary
disagree, the page is wrong and is corrected.

One exception stands while mission atelier-to83 runs. Its branch model,
under which only a merge writes the base, was written here on 2026-11-13
before any code changed, so that each of the mission's items is built
against reviewed text. Until an item has merged, the parts of this page it
covers describe what it will build, not what the binary does.

## How the tool was rewritten

The aim was better code, not more code. The tool was rewritten in full, in a
fresh workspace beside the old one. Nothing new depended on anything old.
One reviewed change then deleted the old workspace and put the new one in
its place, and the cutover converted the records. No compatibility layer
was built: no binary reads both record layouts.

A piece of the old code was copied across only when its behaviour was
wanted as it was. The item that copied it named it, trimmed what the new
tool did not call, and it was reviewed as new code. Everything else was
written new.

Non-test Rust is reported at the end of each mission as an indication. It is
not a limit, and no item is shaped to hit a number.

## Why a full rewrite, and what made it safe

This was the tool's third rebuild, and 31% of the Rust ever written here
had survived, so the choice needed its reasons stated.

- The record layout changed under every command at once, so the command
  layer could not be replaced piece by piece.
- What would have stayed in place was small, and it was bound to types that
  went away: the forge code took the workflow policy's config, and the merge
  code took the workflow policy and the record index.
- A fresh workspace could not grow a dependency on the old one by accident,
  and the switch became deleting one directory.

What was different from the earlier rebuilds: the scope was two reviewed
specifications, fixed before any code; the old tool kept tracking the work
until the new one was rehearsed on every repository; the scorecard has a
baseline to judge the result against; and the list of what was tried and
removed is in this document.

## What came from the old code

The old tool had 48,988 non-test lines of Rust in six crates, and 59,627
lines of tests, of which 50,438 were CLI integration tests. None of it was
edited by the rewrite. It was deleted whole at the switch.

| Area | Lines in the old tool | In the new tool |
| --- | --- | --- |
| Record parsing, validation and index | 7,930 | Written new. Front-matter splitting was copied with one change, so that it keeps a record's body byte for byte where the old code stripped a newline at each end. ID generation was copied from `record_id.rs`. |
| Merge, gates, freshness, light path | 6,190 | Written new. Copied and put on the new types: the merged-tree checkout, the merge commit and the push from `merge.rs` and `merge_push.rs`, and the protection around them described under The forge. |
| Issue and work views and output | 5,304 | Written new, as `status`, `show` and `list` |
| Missions, brief, debrief, criteria | 4,753 | Not carried over. The Desk added the views its pages need (atelier-z4cr) |
| Review and findings | 4,669 | Written new, as `review` and `verdict` over one file per review |
| Site, Rust and web | 4,926 | Not carried over. Deleted with the old workspace; the Desk replaces it (see Decisions) |
| Workflow policy | 3,418 | Not carried over; a short config section replaces it |
| Status, land and hooks | 2,438 | Written new. The harness hook installer was copied. |
| Prune and history | 2,410 | Not carried over |
| Evidence | 1,931 | Not carried over |
| Bundle | 1,665 | Not carried over |
| Forge and CI (`atelier-app/src/forge.rs`) | 1,202 | `forge.rs`, 382 lines, was copied with its contract tests, and takes the new config |
| Git and shell adapters (`commands/git.rs`, `git_freshness.rs`) | 3,501 | Copied: the Git calls the new commands use, and the object reader, which reads files at a pinned commit through one `git cat-file --batch` process and reports a missing path, a non-regular file and a failed subprocess. The record loader on top of it was written new; the old one was bound to types that went. |
| Migrate | 883 | Not carried over |
| Setup | 821 | The merging of hook settings into a harness's config was copied; what it installs is new, and it gained Git hooks and Codex hooks |

No old test was ported, except the contract tests that came with a copied
piece. The new tool has its own.

## Workspace and crates

The tool is the Cargo workspace at the repository root, built to
`target/`. It has four crates under `crates/`, those of the old tool less
`atelier-workflow` and `atelier-site`, under the same names, and the Desk's
crate beside them:

| Crate | Owns |
| --- | --- |
| `atelier-core` | Domain values: Mission, Item, Review, Finding, Status, Thread, Config |
| `atelier-records` | Reading and writing the three record kinds and `config.toml`, from a checkout or from a commit; IDs; validation |
| `atelier-app` | Use cases: the status view, each lifecycle act, the merge checks, cross-branch reads, the views the Desk reads; a repository opened by path (`Repo`) and the Git calls it and the acts here make; the history of acts `log` prints; `ready` and `decide` as functions that return what they did or a refusal as data; the refusal and skip policy, and the wording of the refusals and results of the acts here |
| `atelier-cli` | Arguments, dispatch, the other commands' Git and shell calls, on `atelier-app`'s Git helpers, and the forge; output |

The dependency direction is `cli` to `app` to `records` to `core`. The
Desk's crate, `atelier-desk`, depends on `app` and `records`, and nothing
depends on it.

The tool was built in `next/`, a workspace of its own beside the old one,
with its crates at `next/crates/<name>/`, so that the switch moved them to
the root `crates/` without changing a path inside them. While both
workspaces stood, CI proved that nothing in the new one built from the old
the direct way: the job that built and tested `next/` deleted the root
`crates/` directory first.

`scripts/ci-jobs.sh` says which of CI's two jobs a change needs. The Rust
job checks formatting, builds and tests the workspace, and runs the tree's
`atelier check` on the records; it is skipped when the change touches only
Markdown outside `crates/` and the Desk's app outside `desk/src/api/`. The
Desk's job tests its app, when the change touches `desk/`.
`scripts/count_non_test_rust.py` counts the non-test Rust of a commit.

The binary is `target/debug/atelier` in a build of the tree, and `cargo
install --path crates/atelier-cli --locked` installs it. One installed
binary serves every repository on the host.

## Records

All records are Markdown with YAML front matter, `schema_version: 3`.
`atelier-records` writes the front matter in one form: the fields in the
order shown, block style (`labels:` then `- review`, `ready:` then indented
`by:` and `at:`), absent values as `null`, and times in RFC 3339 with their
offset, UTC as `Z`; the examples below show the fields and their values,
not that layout. The body after the closing `---` line is kept byte for
byte, so a record it reads and writes back unchanged is the same file. An
unknown front-matter key or value is refused, naming the key. Headings and
field lines inside a fenced code block in a body are text, never read;
a fence opens and closes as in CommonMark.

### `items/<id>/item.md`

```yaml
schema: atelier.item
schema_version: 3
id: atelier-06jc
title: Put open findings in full in the review brief
status: building        # draft ready building in_review changes_requested approved done dropped
mission: atelier-icv4   # optional; names a mission in the checkout
labels: [review]
priority: high          # critical high medium low
blocked_by: []          # each names an item in the checkout
implementers: [<thread>]  # start appends unless the thread already holds it; the last entry holds the item
ready: {by: sumeet, at: <time>}
trivial: null           # or the reason given to merge --trivial
caused_by: null         # on a bug: an item ID, or a commit hash of 7 to 64 hex digits
created_at: <time>
```

The item lives in a directory named for its `id`. Sections are `##`
headings: Goal, Done when, Scope, Report, Decisions; Goal and Done when are
required. Report has `### Validation` and `### Responses` subsections when
they apply. Responses holds one line per open fix-now finding, optionally
as a list item: `2-m2bn.1: fixed in <commit>` or
`2-m2bn.1: rebutted: <fact>`.

The front matter has no field for why an item ended without a merge, so
the body holds it. `drop` writes `Dropped: <reason>`, the reason on one
line, as the first line under `## Report`, before any other text or
subsection, adding the section at the end when the item has none.
`atelier-records` reads it back (`ItemDir::dropped`): the section's first
line that is not blank, when it starts `Dropped: `. `close` writes an
item's result first under `## Report` the same way.

A Decisions entry is a `### <date>: <question>` heading under Decisions.
Its lines may name a finding (`Finding: 2-m2bn.1`); once answered it has
`Answer by <thread>: <answer>`, and then, for a finding, `Outcome: defer` or
`Outcome: no action`. An `Outcome:` without a `Finding:` and an answer is
refused. `escalate` adds an entry at the end of the section, adding the
section at the end of the body when there is none, dated in UTC, with its
`Finding:` line when given one. `decide` writes its lines after the last
line of the oldest entry with no answer. The question and the answer are
each one line.

### `missions/<id>/mission.md`

```yaml
schema: atelier.mission
schema_version: 3
id: atelier-xxxx
title: ...
status: open            # open done dropped
labels: []
lead: null              # a thread, set by lead; attribution, not a lock
planners: [<thread>]    # start --mission sets it; submit by another thread appends
```

The mission lives in a directory named for its `id`. Sections are `##`
headings: Why, Outcome, Success criteria (optional), Scope, Authority,
Result; Outcome is required, and `check` says when Why is empty, since it
names whose direction the mission started from. `drop` writes `Dropped:
<reason>` first under `## Result`, as on an item, and `MissionDir::dropped`
reads it. Its plan reviews are review files under `reviews/`, read and
validated as an item's are. `notes.md` holds the lead's notes. Any other
file in the directory is a planning document with no required shape, and is
not read.

Proposed and agreed are not stored. A mission whose record says `open` is
proposed when it is read from a branch that has not merged, and agreed when
it is read from the base. The record stores no time: when the mission was
created is the time of the commit that added it, as `log` reads it.

**The old form.** Until the rollout (atelier-ql64) has converted every
repository on the host, the binary reads a mission written in the form
above it, with `ready` and `created_at` and the statuses `draft` and
`ready`. It reads `status: ready` as `open` and ignores the `ready` and
`created_at` fields. It reads `status: draft` as proposed wherever the
record is, the base included, so such a mission cannot be led, closed or
have its items started until the rollout moves it to its branch or its plan
is reviewed and merged. A missing `planners` reads as none, so any thread
may review such a plan. When a command next writes such a record it writes
the new form: `open` for `ready`, the two old fields left out, and
`planners` added. `check` reports a record in the old form as old, not as a
problem, so the pre-commit hook lets one stay as it is. The version stays
3: the two forms differ only in these fields, and an installed binary that
predates the change refuses `open` and `planners` by name, which says what
to install.

### `reviews/<round>-<suffix>.md`, under an item or a mission

```yaml
schema: atelier.review
schema_version: 3
round: 2
commit: <sha reviewed>
reviewers: [<thread>]   # review appends; the last entry holds the review
model: gpt-6.1-sol      # or null; set by verdict --model, never required
verdict: null           # approve or changes, set by verdict
created_at: <time>
```

The suffix is four random base-36 characters, and the front matter's
`round` is the file name's. Creating a review picks a suffix no file of the
item or mission has, so blind reviews of one round never share a name. Sections:
Summary, then `## Finding <k>: <title>`, numbered without repeats, whose
first line is `Recommendation: fix now | defer | no action`. A finding's
name is its review's file stem and its number, as in `2-m2bn.1`. A finding
carried from an earlier round also has `Was: <finding name>`, naming a
finding of an earlier round, and exactly one of `Now: fixed`,
`Now: withdrawn`, `Now: kept`, or `Decided: defer | no action`, which the
tool writes from the Decisions entry and the reviewer cannot change. Every
finding name a record gives, in `Was:`, in Responses or in a Decisions
entry's `Finding:`, must be a finding of the item's reviews. Other lines
of a finding are the reviewer's text, and an HTML comment is text.

`review` writes the file's front matter: the round, the commit at the tip
of the item's branch (for a plan review, the checkout's HEAD), the caller
as reviewer, `model: null` and `verdict: null`. Its body is the template: a `## Summary` heading over a
comment saying what to write, then one finding for each fix-now finding
the earlier rounds leave open, numbered from 1, under that finding's
latest name:

```markdown
## Finding 1: <the open finding's title>
Recommendation: fix now
Was: 1-x7qd.1
Response: fixed in 0123abc
Now: kept
<!-- Set Now: to fixed, withdrawn or kept. -->
```

`Response:` is the author's line for that finding in Responses, or `none
in the Report`. A plan review's carried finding has no `Response:` line,
since a mission has no Report. A carried finding starts kept, so it stays open until its
reviewer closes it. Nobody edits the front matter: `verdict` sets the
verdict, and the model when given `--model`.

The template also carries each finding of an earlier round that a
decision settled and no earlier round carries, once, in the same order:

```markdown
## Finding 2: <the settled finding's title>
Recommendation: fix now
Was: 1-x7qd.1
Decided: defer
<!-- A decision settled it, and the tool wrote `Decided:`. Leave this finding as it is. -->
```

`verdict` is refused while the file does not carry such a finding with the
`Decided:` line the tool wrote, or while a finding of the file has a
`Decided:` line no decision wrote, or carries a settled finding, or a
finding carried as decided, any other way. So no later review reopens
what a decision settled, and a reviewer cannot decide a finding itself.

A fix-now finding is open until a later round carries it as fixed,
withdrawn or decided, or a Decisions entry naming it has an outcome.
Carried as kept, it moves: the carrying finding is then the open one, and
must say `Recommendation: fix now`. A
carrying finding that says fixed, withdrawn or decided is not itself open.
A review whose verdict is approve while one of its own findings is open is
refused.

A finding is deferred when its recommendation is defer or a decision
deferred it, by a `Decided: defer` line or by a Decisions entry naming it
with `Outcome: defer`, unless a later round carries it: then the carrying
finding is the one that stands (`atelier_core::is_deferred`). A deferred
finding is unsettled until its text gains
`Taken: <item> by <thread>`, written by `new --from`, or
`Dropped by <thread>: <reason>`, written by `drop`; `<item>` is an item
ID, and a finding with both lines is refused. The tool writes the line
right after the finding's `Recommendation:` line, which is outside any
fence, so it is read whatever else the finding holds. Across items a
finding is named `<item>/<finding>`, as in `atelier-06jc/2-m2bn.1`: `list
--deferred` prints that name, and `new --from` and `drop` take it. Only a
finding of a done item can be settled, and a settled one is refused. The
line is written in
the checkout the command runs in, like any record write: on a backlog
branch, where it merges as a record-only change, or on the branch of a
gardener's run, where the staleness rule counts it as a
change outside the run's own item, so it is reviewed and reaches the base in
the run's merge. Two branches that settle one finding differently conflict
when the second merges, and Git reports it. These are the only writes to a
review after its verdict, and the only writes to a done item.

`new "<title>" --from <finding>` writes a draft item whose Goal names the
finding and its review file and quotes the finding, heading and text, and
the `Taken:` line, in one run: it validates both first, writes the item and
then the review, and removes the item again if the review cannot be
written. `drop <finding> --reason "<why>"` needs a reason on one line.
A review file is written whole or not at all: into a file beside it, which
then replaces it. `show` on a done item prints each finding's settlement,
the base's or one the checkout holds, committed or not.

`caused_by` is checked for form only. The item that caused a bug may have
left the tree at the cutover, and then its ID or the commit is still worth
recording.

### `config.toml`

```toml
schema_version = 3
slug = "atelier"
base_branch = "master"
branch_prefix = "x/"     # optional; every kind of branch carries it

[[merge_check]]
run = "cargo fmt -- --check"

[forge]
provider = "github"
remote = "origin"
ci_workflow = "ci.yml"   # optional; without it nothing waits for CI
```

`merge` reads this file from the base branch's tip. `branch_prefix`, when
set, is put before every branch the convention names: `x/<item-id>`,
`x/<mission-id>` and `x/backlog/<slug>`. It is a valid Git ref component
ending in `/`, and one function in `atelier-core` turns a branch name back
into the record it names, given the prefix, so that the reader and `start`
agree. `slug` is lowercase
letters, digits and single inner hyphens. `merge` always makes a merge
commit, so there is no merge strategy to set. `provider` has one value,
`github`. `merge_check` and `forge` may be left out. An unknown key is refused with its name.

### IDs

Missions and items keep the old tool's form, `<slug>-<4 characters>`. `show`
accepts either kind.

## Status

Status is a stored field. One table in `atelier-app` (`acts::TABLE`) lists
every act: the command, the role that may run it, the statuses it may run
from, the status it sets, and its conditions. Each command and the `status`
view read that table, so what `status` says a thing needs is what the command
will check.

| Act | Role | From | To | Conditions |
| --- | --- | --- | --- | --- |
| `ready` (item) | Planner | `draft` | `ready` | none |
| `start` | Implementer | `ready` | `building` | the item's record is on the base, so an item of a proposed plan waits for the plan to merge; blockers done; its mission, if any, is not dropped; no branch `<id>` exists, local or on the remote; the item is not dropped |
| `start`, taking over | Implementer | `building`, `in_review`, `changes_requested`, `approved` | unchanged | branch `<id>` exists, local or on the remote; its mission, if any, is not dropped |
| `start --new` | Implementer | none | `building` | none; the item is created on its new branch |
| `start --mission` | Planner | none | `open` | none; the mission is created on its new branch, with the caller in `planners` |
| `start --backlog` | Any | none | none | no branch `backlog/<slug>` exists, local or on the remote |
| `submit` | Implementer | `building`, `changes_requested`, `approved` | `in_review` | Done when ticked; Report written; each open fix-now finding has a line in Responses or an unanswered question about it; no unanswered question. After its commit, where a forge is configured, it pushes the branch and opens its pull request; a failure of either is a warning, and `merge` does it |
| `submit` (mission) | Planner | proposed | unchanged | the mission is not dropped; no record the branch changes is uncommitted. A caller not in `planners` is added to it, in a commit of `mission.md` alone, and takes the plan over |
| `submit` (backlog) | Any | none | none | no record the branch changes is uncommitted |
| `review` | Reviewer | `in_review` | unchanged | the caller is not in the item's `implementers` |
| `verdict changes` | Reviewer | `in_review` | `changes_requested` | the file validates; it has an open fix-now finding; it keeps what decisions settled, as the tool wrote it |
| `verdict approve` | Reviewer | `in_review` | `approved` | the file validates; no fix-now finding of the item is open, the file's or one it does not carry; it keeps what decisions settled |
| `review` of a plan | Reviewer | proposed | unchanged | the caller is not in the mission's `planners` |
| `verdict changes` on a plan review | Reviewer | proposed | unchanged | the file validates; it has an open fix-now finding; no finding has a `Decided:` line |
| `verdict approve` on a plan review | Reviewer | proposed | unchanged | the file validates; no fix-now finding is open among the file's own and those of earlier rounds; no finding has a `Decided:` line |
| `escalate` | Implementer | `building`, `changes_requested`, `approved` | unchanged | none |
| `decide` | Decider | any started status | unchanged | an unanswered question exists; the caller is not in the item's `implementers`, nor, for the oldest unanswered question when it names a finding, in the `reviewers` of the review that raised that finding or of any review whose finding it carries |
| `merge` | Implementer | `approved` | `done`, read from the merge | a review of the latest round approves it and still covers the head; no unanswered question; nothing uncommitted in tracked files or the item's directory; the item and its mission are not dropped |
| `merge --trivial` | Implementer | `building` | `done`, read from the merge | Done when ticked; Report written; no review file; no unanswered question; nothing uncommitted in tracked files or the item's directory; the item and its mission are not dropped |
| `merge` (mission) | Planner | proposed | agreed, by being on the base | two reviews of the current round, from threads not in `planners`, approve, and the plan has not changed since the commit each reviewed; the mission is not dropped on its branch; nothing uncommitted |
| `merge` (backlog) | Any | none | none | every changed path is under `.atelier/`; it adds no mission and changes no mission's plan; it brings no item record in a started status; `atelier check` passes on the merged tree; nothing uncommitted |
| `close` (item) | Any | `draft`, `ready` | `done` | a result text; no branch `<id>` exists |
| `close` (mission) | Lead or planner | `open`, on the base | `done` | every item done or dropped; Result written |
| `drop` | Planner | `draft`, `ready`, and an item's started statuses; a mission's `open` | `dropped` | a reason, on one line; for a started item, a checkout on the item's branch; not an item already merged |
| `new --from`, `drop` on a finding | Planner on a backlog branch, or an implementer on its branch | `done`, the finding's item | unchanged | a review of the item has the finding; it is deferred; it is not settled |
| `lead` | Lead | an `open` mission, on the base | unchanged | none |

`ready` on a mission is no act: it is refused, saying a mission needs no
go-ahead.

Each act's record change belongs on one kind of branch, and either the
command commits it or the agent does with Git. The three commands that act
on Git, `start`, `submit` and `merge`, are the only ones that commit; every
other act writes the file and stops, and prints the paths and the Git
command that commits them.

| Act | Branch its record change belongs on | Committed by |
| --- | --- | --- |
| `start`, `start --new`, a take-over | The item's branch, which `start` makes or switches to | The command: one commit holding the item's record, which is the claim, then a push where a remote is configured |
| `start --mission` | The mission's branch, which it makes | The command, then a push where a remote is configured |
| `start --backlog` | The backlog branch, which it makes | Nothing to commit; the command pushes the branch where a remote is configured |
| `new`, `set`, `ready` (item) | A backlog branch; or the mission's branch while its plan is proposed and the item is one of the plan's | The agent |
| `submit` | The item's branch | The command: the item's directory, with `in_review` |
| `submit` (mission, backlog) | The branch it is on | The command commits only the addition of a planner taking the plan over; it refuses while a record change is uncommitted, and pushes |
| `review`, `verdict` (item) | The item's branch, in the implementer's checkout | The agent: the reviewer commits the review file and `item.md` |
| `review`, `verdict` (plan) | The mission's branch | The agent: the reviewer commits the review file and pushes it to the mission's branch |
| `escalate`, `decide` | The item's branch | The agent, as now; atelier-xbog removes both commands |
| `merge`, `merge --trivial` | Merges the item's branch into the base | The forge's merge of the pull request, or, without a forge, the command's merge commit. `--trivial` first commits `trivial` on the branch |
| `merge` (mission, backlog) | Merges the branch into the base | As for an item |
| `close` (item, mission) | A backlog branch | The agent |
| `drop` of a started item | The item's branch | The agent; the base's copy follows on a backlog branch |
| `drop` of an item not started, or of an agreed mission | A backlog branch | The agent |
| `drop` of a proposed mission | The mission's branch | The agent, who pushes it and closes its pull request |
| `new --from`, `drop` on a finding | A backlog branch, or a gardener's item branch | The agent |
| `lead` | A backlog branch | The agent |

A status written by an act the agent commits is checked when it is
committed: the pre-commit hook refuses a status change the table does not
list, and refuses any record change committed on the base.

How the table is built:

- A row joins the table with its command. The table holds the rows of
  `ready` on an item, `start`, `start --new`, `start --mission`, `start
  --backlog`, `start` taking over, `submit` on each kind of branch,
  `review`, both verdicts, `escalate`, `decide`, `merge` on each kind of
  branch and with `--trivial`, `lead`, `drop` (one row, for an item and a
  mission), `close` on an item and on a mission, `review` and both
  verdicts on a plan review, and settling a finding (`Act::Settle`), whose finding's
  conditions `acts::check_settle` checks after the row's. A mission's row is checked with `acts::check_mission`, from
  the mission's status and, for `close`, the status of each item that names
  it, each found as `Located::record` finds it, and its Result; for a
  verdict, from the caller's plan review. A mission's From is its stored
  status, or proposed, which is `open` read from a branch that has not
  merged; `open` in a row means `open` on the base. `start --new` and
  `start --mission` run from no status, since each creates its record, and
  take the status they set from their rows. The rows of a backlog branch
  have no record to take a status from. `close` on an item and on a mission needs no role, like a planner's
  act.
- A round of a plan review is over once every file of it has a verdict,
  since its reviewers work blind in checkouts of their own
  (`acts::plan_round`). Until then it is the latest round, and the round's
  file is the caller's of those with no verdict, else the first by name,
  which `review` takes over. Once it is over, `review` opens the next
  round, whose file carries the open fix-now findings of the rounds before,
  as an item's does. A plan verdict counts the file's own findings and the
  earlier rounds'; another file of the same round is another reviewer's.
- A round of an item is the latest round of its reviews until one of them
  has a verdict, which ends it; then it is the next round, which has no file
  until `review` creates one. The round's file is the caller's when it holds
  one of the latest round's, else the first by name. The caller is the
  round's reviewer while the item is in review and it holds that file.
- `acts::check` refuses a caller that holds one role on the item and runs
  another role's act, with one line naming its role and that role's
  commands. `review` and `decide` are the exceptions: an implementer,
  current or past, is refused by their condition; `review` shows it the
  brief, and `decide` the handoff that starts a decider. Then `check` refuses an
  act whose From does not hold the item's status, then one that needs a role
  the caller does not hold, then one with an unmet condition, and otherwise
  gives the status to set. An act that binds a role (`start`, `review`)
  needs nothing held, and `decide` binds its decider the same way;
  `submit`, `escalate` and `merge` need the caller to be the item's last
  implementer, and a verdict needs it to be the round's reviewer. No
  thread holds the planner's role on an item, so a planner's act on one
  (`ready`) needs nothing held and is never refused for a role the caller
  holds. On a mission, `submit` and `merge` need the caller to be in
  `planners`, which `submit` adds it to, and `review` of the plan needs it
  not to be.
- `acts::next` names the act the caller would run next: `start` on a ready
  item; as its holder, `submit` from building or changes requested, `merge`
  from approved, and nothing while it is in review; as the round's
  reviewer, `verdict changes` while its file holds an open fix-now finding,
  else `verdict approve`; as anyone else, `review` when it is in review and
  they never implemented it, else nothing when they reviewed it before
  (their review is done, and taking the item would put them on both sides),
  else the take-over. `status` prints that act's
  unmet conditions from `acts::unmet`, the function `check` calls, and ends
  with its command.
- `start` takes the take-over row when the item's status is a started one,
  and the plain row otherwise, so a draft, done or dropped item is refused by
  that row's From.
- `start`'s condition that the base's record is not dropped needs no check of
  its own: the base's dropped record is the item's status (below), and
  `start` does not run from `dropped`. A blocker counts as done only when its
  status, found the same way, is `done`; a blocker that names no item is
  unmet. The mission's condition is met unless its record, found the same
  way, says `dropped`; a mission no record names is unmet.
- Done when is ticked when every checklist line under `## Done when` (`- [ ]`
  or `- [x]`, outside fenced code) is ticked. The Report is written when
  `## Report` holds a line that is neither blank nor a heading; a fenced
  transcript counts.

Two facts are checked live and never stored:

- **The approval covers the head.** Since the reviewed commit, nothing
  changed outside the item's own directory, and the item's Goal, Done when
  and Scope are unchanged. So a Report, a response or a review file never
  makes an approval stale, while code, `config.toml`, a mission's documents,
  another item or the item's own requirements do. If it is stale, `status`
  and `merge` say to run `submit`. `atelier-app`'s `Repo::checkout`
  gathers what this reads, as `acts::Checkout`: the paths that differ between the reviewed commit and
  the head, the item's record at the reviewed commit and at the head, and
  what the checkout has not committed; `acts::unmet` applies the rule. `status` gathers
  it on the item's branch, so it lists what `merge` will check.
- **The merge checks pass.** The configured commands on the merged tree,
  and CI where the forge names a workflow. A CI run counts when it ran on
  the head, or on an earlier commit of the branch from which the head
  differs only inside the item's own directory, so the run `submit` started
  still counts after `verdict approve` commits. The latest finished run that
  counts decides; a cancelled run has not finished. `merge` runs or reads
  them; `status` lists them, from the base's config, and runs none and looks
  up no CI.

Of the rules in the product specification's Review section, the binary
enforces four, through the act table: an author never reviews or decides
its own item; a reviewer of a finding, in any round that carried it, never
decides it; `verdict approve` is refused while a fix-now finding is open;
and a decided finding cannot be reopened. Three it does not enforce, and
leaves to the skill: that a rebuttal goes to a fresh reviewer and gets one
try, that escalation is required after three rounds without an approval,
and that a mission's lead is the one who decides. `status` shows each of
them but the fresh reviewer: an open finding whose rebuttal a reviewer kept says the rebuttal has
had its one try; the implementer is shown the round, and once three rounds
have ended without an approve verdict (`Facts::unapproved_rounds`, counted
from the review files and stored nowhere) it is told on one line that
escalation is required now, though `submit` still opens a fourth, and in
review that it escalates once the round's verdict is in if that asks for
changes too; and a
question names the lead as its decider when the lead may decide it.
Nothing stops one thread reviewing every round, and any thread that is
neither an implementer of the item nor a reviewer of the finding may
decide. The review brief carries no merge check results, since nothing
records them. Where the base's config names a forge it names the branch's
open pull request, where CI's result can be read, looked up with the code
`submit` and `merge` use and bounded as `merge`'s question under its lock
is; when the forge has none or does not answer, the brief says so in one
line and `review` goes on.

## Threads and roles

A thread's identity is `T3CODE_THREAD_ID` when it is set, else the harness's
session ID (`CLAUDE_CODE_SESSION_ID`, `CODEX_SESSION_ID`,
`GROK_SESSION_ID`), else `human`. When `CODEX_THREAD_ID` and
`CODEX_SESSION_ID` are both set and differ, the caller is a Codex subagent,
and its identity is that one, then `/`, then its `CODEX_THREAD_ID`. A
variable set to an empty value counts as unset. Nothing splits an identity
on `/` or reads it as a path; it is compared whole.

A Claude Code subagent is not a new thread. Observed on 2026-11-10
(atelier-67nn): a Claude Code subagent started with its `Agent` tool runs with
its parent's `CLAUDE_CODE_SESSION_ID`, and under T3 Code with its parent's
`T3CODE_THREAD_ID`. Atelier sees it as its parent, and refuses it as the
reviewer of its parent's item. A Codex subagent is one. Measured on
2026-11-11 with Codex 0.167 `codex exec` inside a T3 thread: the root has
`CODEX_THREAD_ID` equal to `CODEX_SESSION_ID`, and its `spawn_agent` child
has the same `CODEX_SESSION_ID` and inherited `T3CODE_THREAD_ID` and a
`CODEX_THREAD_ID` of its own, so it may review or decide its parent's item
and not run its parent's `submit` or `merge`. A thread the harness starts
as a thread, such as T3 Code's `delegate_task`, has an identity of its own
under any harness. `submit`'s handoff and the refusal of an author's
`review` and `decide` say which subagents may be used, to start one with
fresh context, not as a fork, and that starting it is the reader's own job
and part of its task, even as a child or subagent, so it starts it without
waiting to be asked. Told less, a Codex thread waited for its user's word
(atelier-t747), and a Claude Code thread started as a T3 Code child stopped
for its lead (the first real item under the new tool, mission atelier-qr5s).

That rule is the command's. A program that links `atelier-app`, such as the
Desk, names the caller itself when it opens a repository (`Repo::open`),
and that name is the identity the records get: an answer it writes for
`sumeet` reads `Answer by sumeet:`, where the command run by a person with
no harness variable set is `human`. `ready`'s `by`, the person who said go,
is a parameter of the function, which `ready --by` fills. A name the records
cannot write and read back as it is, one with a line break or a `: `, is
refused when the repository is opened, before anything is written.

Entry commands store the identity in the record: `start` appends it to the
item's `implementers`, `review` to the review file's `reviewers`, `start
--mission` and a planner's take-over to the mission's `planners`, and `lead`
sets the mission's `lead`. `decide` stores it with the answer. The last entry
of a list holds the role now. The earlier entries are why a thread that once
implemented an item can never review it.

There is no local state file. A command works out who is calling by looking
for the caller's identity in the records it can see: the item of the current
branch, an open review of that item, and missions as the reader below finds
them, on the base and on their branches. A thread may
hold several things, and `status` lists each. In a Git checkout with no
`.atelier/`, every command but `check`, which reports the missing
`config.toml`, says this is not an Atelier repository. On a branch named
for an item, `status` speaks to the item's holder as its implementer, to a
thread that reviewed an earlier round as done with it, and to anyone else as
a visitor, naming the holder. On a mission's branch it speaks to a planner
as planner, with what the plan still needs to merge, and to anyone else as
a possible reviewer of the plan. A visitor, or a reviewer whose review is
done, is told, before any offer to take
the item over, each unanswered question and `decide`, as its next command
when it may decide them and naming who may when it may not; the lead of the
item's mission is told so and offered no take-over. Anywhere else it lists the
started items the caller holds, with `start <id>` to return to one, the
ready items `start` would accept now (`Entry::startable`, as `list --ready`
reads it), one line counting the other ready items by what stops them (a
blocker not done, a mission dropped, a branch of their name) that names
`list`, and the started items others hold. Holding nothing, it ends with
`start` on an item that can start, or says no ready item can start now and
names no `start`. Wherever it runs it also
lists the record changes the checkout has not committed and the commits of
its branch not yet pushed, with the Git command for each, and each drop
written but not yet published. Away from an
item's branch it also lists the missions in progress and, apart from the
ready items, the items labelled `needs-human` that nobody has started, as
waiting on a human, and the items dropped on their branch whose drop has
not reached the base, each with its reason. A thread that is the `lead` of an agreed
mission is told so, with each of the mission's items by
status, what holds or blocks each, under each started item the conditions
its holder's `status` lists as unmet, read from the branch's last commit and
so without what the holder has not committed, other than questions, which
are listed apart, and the handoff line that starts a thread for each item that can
start now; holding nothing else, it is pointed at no item itself. On the item's branch it says a dropped item
was dropped and why, as does the refusal of any act on it, and it
speaks to the round's reviewer as reviewer, naming its file, tells
the implementer of an item in review which thread holds the round's review,
or that none does yet, and tells the implementer of a building item whose
Goal or Done when lines are empty, as after `start --new`, to write them
first. That line is orientation, not a condition of `submit`. The
implementer's `status` lists each unanswered question as a condition of
`submit` or `merge`, and says who decides the oldest: the lead of the
item's mission, when the mission is agreed and its lead may decide it, else
a new thread, with the handoff `escalate` printed. The lead reads the item
from its branch's last commit, so while the question is not committed the
lead's handoff also gives the Git command that commits it. An open finding whose
rebuttal a reviewer kept says that a rebuttal has had its one try, so the
exits left are a fix or `escalate`. A lead's `status` lists, under each
mission it leads, its items' unanswered questions with the `decide` that
answers each, and `show <mission>` lists them too. An approved item says
since when, in the implementer's and the lead's `status` and in `show`:
the time of the commit on its branch that set the approving review's
verdict, which `git log -S` finds; nothing stores it. Away from the item's
branch, the implementer's list of what it holds gives the same: each
item's unanswered questions, since when an approved one is approved, the
round of one in review or with changes requested, and the line that
escalation is required after three rounds without an approval. With
changes requested, that line names `escalate --finding` with the first open
fix-now finding that no question is about, in its round or any it was
carried from, and it is gone once a question is about each.

Harness hooks give orientation at session start. `atelier setup claude`
writes Claude Code's project hooks into `.claude/settings.json`: a
SessionStart hook runs `atelier status`, whose output Claude Code adds to
the session, and a Stop hook runs `atelier check` at the end of each turn,
which also reports record changes not yet committed. The Stop hook's
command is `atelier check || exit 1`, since a Stop hook that exits 2 makes
Claude Code go on and run it again. `atelier setup codex` writes Codex's
into `.codex/hooks.json`, which Codex 0.167 reads for a project in the same
form: a SessionStart hook runs `atelier status`, whose output Codex adds as
developer context. Codex runs a project's hooks only once the project is
trusted and the user has trusted the hook in its `/hooks` list, and
`setup codex` says so. Each command merges its hooks into the file the
repository has, keeping every other hook there and replacing only those
marked `# atelier-managed:<event>`, the old tool's marker, so that either
tool's `setup` replaces the other's; run again, it changes nothing. A file
that is not valid JSON, or whose hooks are not in the harness's form, is
refused with nothing changed. The merging was copied from the old `setup
claude`. Where no harness hook runs, a project's `AGENTS.md` says to run
`atelier status` first, and the post-checkout Git hook prints it on every
branch switch.

## Where records are read and written

The base is written only by merges. A record changes in the checkout of a
branch, and the act table above says which kind of branch each change
belongs on and who commits it. This section says how a record is found when
the base and several branches hold copies of it, and where each command
reads and writes.

### Finding a record across branches

- **The refs read.** The base's tip is the remote-tracking ref of the base
  (`refs/remotes/<remote>/<base_branch>`) where a forge is configured and
  that ref exists, since the forge makes every merge; otherwise it is the
  local base (`refs/heads/<base_branch>`). A local base that holds commits
  the remote's does not is reported by `status`, since no merge made them.
  The branches read are every local branch, and every remote-tracking branch
  of the forge's remote, whose name less the configured prefix is an item
  ID, a mission ID (the slug, a hyphen and four characters) or
  `backlog/<slug>`. A branch whose tip is in the history of the base's tip
  is merged and is not read: its records are on the base. Every record is
  read at a commit, through the commit reader, except in the checkout a
  command runs in, whose files are read as they stand so that uncommitted
  edits count.
- **Which copy wins** (`Located::record`), in order:
  1. The base's copy when it is final: an item that is `done` or `dropped`,
     or in a started status, which on the base means its branch has merged
     and reads as `done` (see [Merge](#merge)); a mission that is `done` or
     `dropped`.
  2. Else the copy on the record's own branch, the one named for its ID,
     while that branch is unmerged. Where both the local branch and its
     remote-tracking branch exist, the tip that contains the other is read.
     Where they have diverged, a `dropped` on either wins, and otherwise the
     local tip is read, and `status` and `show` say the two have diverged.
  3. Else the base's copy.
  4. A record the base does not hold and that has no branch of its own, such
     as an item filed on a backlog branch or an item of a proposed plan, is
     read from the unmerged branch that adds it, and is proposed. Where
     several do, the first by branch name is read and `show` names the
     others.

  A copy on any other unmerged branch, such as a backlog branch that drops
  a ready item, is a proposed change, not the record. It takes effect when
  that branch merges. `show` names each unmerged branch that changes the
  record, and `list` shows the change under what is proposed.
- **What a reader is told.** The reader gives each record the ref it was read
  from: the base, a local branch, or `<remote>/<branch>`. `show` prints it
  under the title, and for a remote-tracking ref how long ago the remote was
  fetched, from the modification time of the clone's `FETCH_HEAD`, or that
  it was never fetched. `list` prints one line at its top naming the remote
  and the age of the last fetch, and gives each proposed entry, and each
  started item read from a remote-tracking branch, with its branch. `status`
  lists a started item held in another clone, with the thread its branch's
  record names.
- **Reading is cheap enough or it gets a cache.** One `git for-each-ref`
  lists the refs, one `git rev-list` question per ref says whether it is
  merged, and the clone's one `git cat-file --batch` process reads every
  record. atelier-dciw measures `list` with 200 named remote branches; a
  cache on disk is added only if that misses two seconds.
- **No command fetches**, with two exceptions that read back what the command
  just did: `start` fetches the one branch whose claim it lost, to name the
  holder, and `merge` fetches the base after the forge has merged. Fetching
  is otherwise the agent's, or the Desk's on its timer.

### The base's own checkout

The checkout that has the base branch out is where a person or a thread
reads; nothing is written there.

- Every command that writes a record refuses in it and writes nothing,
  naming the `start` that makes the branch the change belongs on: `start
  --backlog <slug>`, `start --mission "<title>"` or `start <id>`. Written
  there, the files could be committed nowhere, since the pre-commit hook
  refuses a record change on the base; they would show in every thread's
  `status`, and `merge` without a forge would refuse to move a base checkout
  holding them. Refusing before writing costs one command; the agent's
  recovery from the other costs several.
- `start` run there makes and claims the branch without switching the
  checkout, and prints the `git worktree add` that gives the branch a
  checkout of its own, beside the repository, for the agent to run.
  Switching the base's checkout has cost us a working tree before. `start`
  does not make the worktree itself: the tool would then own a path and its
  removal, which the harness already owns (T3 Code makes a worktree per
  thread).
- `status`, `show`, `list`, `log` and `check` run there as anywhere.

### Where each command writes

- **A writing command writes the checkout it runs in.** It reads the
  checkout's records and finds the rest as above, validates what it writes
  against every record it can see, and writes nothing when one of them has a
  problem; the pre-commit hook validates them again as they are committed.
  It prints the paths it wrote and ends with a `Then:` line giving the Git
  command that commits those paths alone. Run on a branch of a kind the act
  table does not give for its change, such as `lead` on an item's branch, it
  says which kind the change belongs on; that is a rule, and `ATELIER_SKIP`
  passes it. `new --from` and `drop` on a finding, on the branch of a
  started item whose last implementer is the caller, say that the write
  reaches the base in that item's merge and makes an approval of it stale.
- **`new`** picks an ID that no record has in the checkout, at the base's tip
  or on any branch the reader reads, and that no branch is named for, and
  writes a draft with its section headings and nothing under them; an
  item's priority starts as `medium`. It makes items only; `start
  --mission` makes a mission.
- **`set`'s flags.** `set <id>` takes `--title`, `--priority`,
  `--labels a,b`, `--mission <id>`, `--blocked-by a,b` and
  `--caused-by <item or commit>`, at least one. A list flag replaces the
  list, and an empty value clears a list or an optional field. A mission
  takes only `--title` and `--labels`; an item's other flags on a mission are
  refused. It changes a draft or ready item and is refused for a started
  item, naming its branch, and for a done or dropped one. `ready <id>...
  --by <name>` checks every named ID first and writes nothing when any is
  unknown, a mission, or not a draft, naming each.
- **`drop`.** It writes `dropped` and `Dropped: <reason>` into the record in
  the checkout. An item that is not started, and an agreed mission, are
  dropped on a backlog branch, and the drop counts once that merges. A
  started item is dropped on its own branch, in any checkout of it, by any
  thread. Once that commit is pushed the reader finds it (rule 2 above), so
  the item's `submit`, `merge` and take-over are refused with its reason in
  every clone that has fetched, and a `submit` or forge merge from a clone
  that has not is refused by the forge, since the branch it would push is
  behind. `drop` then prints the commands that carry the base's copy on a
  backlog branch. An item that is only on its branch, as after `start
  --new`, gets its branch's front matter and its Goal, Done when and Scope
  written there, under which its Report holds the reason; its reviews, the
  rest of its Report and its Decisions stay on the branch, so nothing on
  the base names a review it lacks. `status` and `list` say a drop has not
  reached the base until it has. A drop of an item already merged is
  refused. A proposed mission is dropped on its own branch; `drop` prints
  the push and the `gh pr close` that publish it. Dropping a mission
  changes no item; `start` and a take-over on one of its items, and an
  item's `merge`, are refused once the drop is found, saying the mission
  was dropped. The Desk's stop does all of this through the same functions.
- **`close`** on an item with a branch is refused, naming `merge` as the way
  it ends. `close` on a mission runs only on an agreed one, and counts its
  items as the reader finds them.
- **`lead`** writes the caller as `lead` of an agreed mission, in the
  checkout it runs in, and commits nothing; it refuses a mission that is
  proposed, done or dropped. A thread that already leads the mission is
  shown it and writes nothing. It reads its items as the reader finds them,
  so what it says can start now is what `start` will start.
- **`list`'s flags.** With none, every item and mission the reader finds that
  is not done or dropped: those on the base under their mission and then
  under their sorted labels, every such mission even with no item, and,
  apart, what is proposed. `--ready` keeps items whose `start` has no unmet
  condition; `--blocked` items with a blocker that is not done;
  `--needs-human` items with that label; `--proposed` only what is proposed;
  `--mission <id>` that mission's items; `--label <l>` items, and missions,
  with the label; `--mine` items the caller holds as implementer or as the
  round's reviewer, and missions it leads or plans with their items;
  `--done` turns all of this to done and dropped items and missions. Flags
  narrow together; with `--ready`, `--blocked` or `--needs-human` a mission
  is shown only for an item kept under it. `--deferred`, which takes no
  other flag, lists instead each unsettled deferred finding of a done item,
  by its name and title; it reads the items of the checkout it runs in,
  committed or not, as `new --from` and `drop` do, so it lists what they
  would settle there.
- **Starting is one atomic step, and the push is the claim.** `start` writes
  the commit that sets `building` first, with no branch, and then creates
  branch `<id>` pointing at it. Git creates a branch atomically, so a branch
  named for an item always holds its building record, and an interrupted
  `start` leaves no branch. The commit is built on the base's tip through a
  temporary index (`read-tree`, `update-index`, `write-tree`,
  `commit-tree`), so no checkout changes, and the branch is created with
  `git update-ref refs/heads/<branch> <commit> ""`, which fails when the
  branch exists. Where a remote is configured, `start` then pushes the
  branch, without force, before it reports success. A remote that already
  has the branch refuses the push: `start` deletes the local branch it
  made, leaves the checkout as it was, fetches that branch and names the
  thread its record gives. A later `start` in that clone is the take-over,
  from the remote-tracking branch. A remote that cannot be reached refuses
  `start` the same way, and says so. Only after the push does the checkout
  switch to the branch, except in the base's checkout (above). Debug builds
  can be held between the two steps through
  `ATELIER_TEST_HOLD_BEFORE_BRANCH`, which the tests of an interrupted start
  and of two starting threads use. `start --new` and `start --mission`
  start the record they create the same way, in one commit holding only
  it; `start --backlog` makes its branch at the base's tip with no commit.
- **Taking over** switches the checkout to the branch, creating the local
  branch from the remote-tracking one where only that exists, appends the
  caller to `implementers`, commits `item.md` alone, leaving the status as
  it is, and pushes where a remote is configured. A branch checked out in
  another worktree cannot be switched to; `start` then changes nothing and
  says to run it there. The holder running `start` again only switches.
- **`submit` commits the item's directory** and nothing else in the
  checkout, on an item's branch. On a mission's or a backlog branch it
  commits nothing but a planner taking the plan over, and refuses while a
  record the branch changes is uncommitted, naming the paths.
- **A question and its answer are written where the implementer works.**
  `escalate` runs on the item's branch, and `decide` only in the checkout
  on it, refused anywhere else, naming the checkout that has the branch.
  Each writes `item.md` there and commits nothing, so `submit` commits the
  question and its answer with the item; an approved item's implementer
  commits them before `merge`, which refuses anything uncommitted. A
  refused `escalate` or `decide` writes nothing.
- **An item's review is written where the implementer works.** `review` runs
  only in the checkout on the item's branch, and refuses anywhere else,
  naming it. It creates the round's file, or appends the caller to its
  `reviewers`. `verdict` writes the verdict into the file and the new status
  into `item.md`. Neither commits: the reviewer commits the review file and
  `item.md` with Git, in one commit, and the pre-commit hook checks the
  status change. The implementer's `merge` pushes it.
- **A plan review is written where its reviewer works.** The reviewer works
  in a checkout of its own at the commit `submit` pushed to the mission's
  branch, detached or on that branch. `review <mission>` reviews the plan as
  that checkout's HEAD holds it: the brief reads the mission, its documents
  and its items from the commit the review records, and names in one line
  the paths of the plan that the checkout or its index changes from that
  commit, which the review does not cover. A mission no commit holds is
  refused. It creates the round's file under `missions/<id>/reviews/`, or
  appends the caller to its `reviewers`, and `verdict` in a checkout on no
  item's branch writes the verdict of the one plan review there that the
  caller holds without a verdict. Neither commits. The reviewer commits the
  file and pushes it to the mission's branch, and the brief gives the
  commands, including a `git pull --rebase` for when the other reviewer
  pushed first. Review files have distinct names, so two reviewers' commits
  never conflict. Blindness comes from the order of the steps: each forms
  its view at the submitted commit before it publishes, and a reviewer who
  goes looking with `show` or a fetch can read the other's pushed verdict,
  which is accepted ([Decisions](#decisions)).
- **A refused commit changes nothing.** When `git commit` fails, as when a
  hook refuses it, `start`, the take-over and `submit` put the files they
  wrote and the index under the paths they staged back as they were,
  removing a file no commit held, so the act can be run again.
- **The forge's remote is the one remote.** Where a forge is configured,
  `start`, `submit` and `merge` push to and read from its `remote`, and the
  reader reads that remote's tracking branches. With no forge, nothing is
  pushed and only local refs are read; the claim is the local branch, and
  threads in one clone and its worktrees are coordinated as before.
- Records can be read at any commit, without a checkout, through
  `atelier-records`' `CommitReader`: one `git cat-file --batch` process
  serves every read, and it tells a path the commit lacks, an entry that is
  not a regular file and a failing Git apart. The Desk reads this way, and
  so does anything that compares a record now with the same record at the
  commit of an earlier act. A program that links `atelier-app`, such as the
  Desk, opens a `Repo` by path; `atelier-app` reads no environment variable
  to find the directory, the caller or a skip's reason, which its caller
  gives it.

## Merge

`merge` runs in the checkout on the branch it merges, of any of the three
kinds, and tells the kind from the branch's name. What it needs before it
merges is the kind's row of the act table: an item's approval or
`--trivial`, a plan's two approvals, or a record-only change's passing
`atelier check`. The base receives the merge one of two ways. With a forge,
the forge makes the merge commit when it merges the pull request, and
`merge` never pushes the base. Without one, `merge` makes the merge commit
and moves the local base, as before.

In order:

1. With `--trivial "<why>"` on an item: unless the head already holds that
   reason, write it into the item's `trivial` field and commit `item.md`
   alone on the branch. The head then differs from the one before only
   inside the item's directory, so a CI run on the earlier commit still
   counts for it.
2. Read everything, without a lock: first the head, which every later step
   is checked against, then the branch's records, and `config.toml` from the
   base's tip. Check the act's conditions, as in step 5. Where a forge is
   configured, also refuse a branch that conflicts with the base, since CI
   does not run on a pull request that conflicts.
3. Where a forge is configured: push the branch when the forge's copy of it
   is not the head, since the forge merges the head of the pull request and
   that must be the commit checked here; this is how a reviewer's verdict
   commit, made in the implementer's checkout, reaches the forge. Open the
   pull request when the branch has none. Where the forge names a CI
   workflow, wait for CI: ask the forge every 15 seconds, for up to 9
   minutes. A run counts when it ran on the head, or on an earlier commit
   from which the head differs only inside the item's directory. While no
   run that counts has finished, or the latest finished one failed and a
   newer one is running, keep asking; a question that fails counts as not
   finished, and one still unanswered at the bound is ended. Refuse as soon
   as CI fails; at the bound, stop with exit status 3 and say what the last
   answer said, or, when none came, why the first question failed. The wait
   holds no lock and writes nothing.
4. Take the merge lock, then read everything again, as in step 2. Nothing
   read before the lock is trusted, and a checkout now on another branch
   is refused, so `merge` merges only the branch it waited for.
5. Check the act's conditions from the table. On an item: the approval
   covers the head, or `--trivial` was given on a building item with no
   review file; no Decisions entry is unanswered; neither the item nor its
   mission is dropped, as the reader finds them. On a mission's branch: two
   reviews of the current round from threads not in `planners` approve, and
   the plan has not changed since the commit each reviewed, compared by
   section of `mission.md`, by file for the other documents, and by item
   while the plan is proposed; the mission is not dropped on its branch. On
   a backlog branch: every path it changes is under `.atelier/`, and it
   adds no mission and changes no mission's plan. On any branch: no tracked
   file has uncommitted changes and no file in a changed record's directory
   is untracked, ignored or not; and no record the branch brings to the base
   sets an item to a started status except the item the branch is named
   for, since a started item on the base reads as done (below). The role is
   told from the checkout, as every command tells it; the rest is judged on
   the branch as the head holds it. Without a forge, where the base is
   checked out, that checkout must have no uncommitted changes either, since
   merging moves it.
6. Build the merged tree with `git merge-tree`; a conflict refuses, naming
   the files. Where the forge names a CI workflow, ask once more, without
   waiting: a passing run must still count for the head as it now is. A
   question unanswered after a minute is ended, and `merge` stops with exit
   status 3, releasing the lock. Write a merge commit object on the merged
   tree, base tip first and branch tip second, whose message ends with the
   trailer `Atelier-Merge: <branch>`. Nothing points at it yet.
7. Check out that commit in a temporary worktree in the Git directory and
   run each merge check there with `sh -c`. A failure refuses, printing the
   command, its exit status and the last 20 lines of its output. When none
   failed, the output says, before the line that reports the merge, how
   many checks ran and that each passed, or that `config.toml` names none.
   A failing check `ATELIER_SKIP` passed has its skip line instead, and no
   line says the checks passed. On a record-only branch, the checks are
   `atelier check` on the merged tree, whatever `config.toml` lists.
8. Confirm the branch tip, the base tip and the checkouts are what steps 4
   to 7 read; otherwise write nothing and say to run it again. Then:
   - **With a forge**, ask it to merge the pull request as a merge commit,
     only at the head checked, with the message of step 6: for GitHub,
     `gh pr merge <number> --merge --match-head-commit <head> --subject
     <subject> --body <body>`. The commit object of step 6 is discarded;
     the forge makes its own merge commit of the same head onto its base,
     with that message, trailers included. When the forge
     merges, fetch the base, so this clone reads the merge, and fast-forward
     the local base where it is checked out with no changes. When the
     forge refuses because a review or a check its branch protection
     requires is missing, stop with exit status 3, change nothing, and say
     which is missing, as the forge reports it (for GitHub, the pull
     request's `reviewDecision` and `mergeStateStatus`). `merge` does not
     wait for a person: a required human review can take a day, and the
     command is run again once it is given. When the forge refuses because
     the head or the base moved, or the branch conflicts, refuse and say to
     run it again.
   - **Without a forge**, move the base to the merge commit: a fast-forward
     of the base's checkout where one has it checked out, else `git
     update-ref` from the tip read. Nothing is pushed.

`merge` has no flag but `--trivial`, and the bound, the time between
questions and the minute under the lock are constants; debug builds let
tests shorten them through `ATELIER_TEST_CI_WAIT`, the minute to no more
than the bound. The 9 minutes sit under Claude Code's 10-minute limit
on a command in the foreground; a harness with a shorter limit kills `merge`
during the wait, which leaves nothing behind. Run again, it picks up where
the facts now stand: on a branch already merged it says so, and says
nothing of merge checks. Its exit status is 0 when it merged or had nothing
left to do, 1 for a refusal, 2 for an error, and 3 when CI has not finished
by the bound or the forge waits on a required review or check.

**Between the checks and the forge's merge.** The checks run on the merged
tree as the base stood when they ran. The forge merges onto the base as it
stands when asked, and another clone can merge in between, which no lock on
this host prevents. `--match-head-commit` keeps the branch's side fixed. A
conflict is refused by the forge. A clash between two changes that each
merge cleanly is not seen by this run's checks; CI on the base after the
merge sees it, and a protection that requires a branch to be up to date
with its base makes the forge refuse instead, which `merge` reports with
exit status 3 and the `git merge` of the base that catches up. atelier-t5am
measures this and says in its Report what else catches it.

**A protected base that requires CI on the head.** The forge then waits for
a run on the exact head it merges. The commit holding a reviewer's verdict
is that head, and it is pushed only by step 3, so the forge waits for one
more CI run after approval than `merge`'s own rule asks for. `merge` reports
that as a required check missing, with exit status 3. A records-only fast
path in CI keeps that run short only where the branch changes nothing else;
for a branch with code it is a full run. That is the price of a protected
base, and where the base is not protected nothing waits for it.

### How `done` is recorded

A merged item is done, and nothing writes `done` at the merge. A started
item's record reaches the base only in the merge of its own branch: the
pre-commit hook refuses a record change on the base, and step 5 refuses any
other branch that brings an item in a started status. So the reader reads a
base's copy in `building`, `in_review`, `changes_requested` or `approved` as
`done` (rule 1 of [Finding a record across
branches](#finding-a-record-across-branches)), with the forge or without it,
and `atelier show` in a fresh clone says done once the merge is fetched.
`done` stays a stored status, written by `close` on an item never started and
by every merge before this change, and read as it always was. `log` finds an
item's merge where its base copy first holds a started status (see [The
log](#the-log)), and `trivial` is on the branch before the merge.

Turned down: a last commit on the branch, setting `done`, made by `merge`
after its checks and pushed before it asks the forge. Under a protection
that requires CI on the head, every merge would wait for one more run on
that commit; the branch would say done before it had merged, and would stay
wrong when the forge then refused; and it is a change to the record after
the approval that no reviewer sees. The merge commit's message, which the
forge lets `merge` write, could carry `done` as a trailer, but a trailer is
not a record and would be one more place to read status from.

The branch is kept; it is merged, so the base's record is the item's record
from then on, and the forge may delete it.

## The log

`atelier log` prints the history of acts. Nothing stores it. It is derived
from Git history, by finding the commits where a record changed in one of
the ways below. Each is one line, and each carries the commit it was found
in. `atelier-app` returns the same history as data (`history`, in
`crates/atelier-app/src/log.rs`), which is how the Desk reads it, and the
command only formats it (`crates/atelier-cli/src/history.rs`).

`atelier log` prints everything; `atelier log <item>` that item's acts;
`atelier log <mission>` the acts on the mission, its plan reviews and its
items, an item being the mission's when its record on the base's tip, or
else on its branch, names it. `--since <time>` keeps the acts committed at
or after an RFC 3339 time or a `YYYY-MM-DD` date, which is that day's start
in UTC. An ID no record has on the base's tip or on a branch named for it is
refused as `show` refuses it.

| Line | Found where | Thread shown |
| --- | --- | --- |
| start, take-over | An item's `implementers` gains an entry | That entry |
| submit | An item's status becomes `in_review` | The item's holder |
| review, take-over | A review file appears, or its `reviewers` gains an entry | That entry |
| verdict | A review's verdict is set, on an item or a mission | The review's holder |
| escalate | A Decisions entry appears | The item's holder |
| decide | A Decisions entry gains its answer | The thread stored with the answer |
| merge | On the base's first-parent line, an item's copy first holds a started status, or becomes `done` in an Atelier merge made before the branch model | The item's holder |
| settle | A finding gains `Taken` or `Dropped` | The thread in that line |
| lead | A mission's `lead` is set or changes | That entry |
| ready | An item's status becomes `ready`; on a mission in the old form, its status | None; the name given with `--by` |
| drop, close | A status becomes `dropped`, or `done` outside a merge | None |

A skip is shown on the line of the act whose commit carries its trailer,
which is every act where Atelier makes the commit. Those are `start`,
`submit` and `merge`, whose trailers are in the merge commit's message
whether the forge makes it or `merge` does, and a take-over of an
implementer, which `start` commits. Acts the agent commits carry no
trailer. The other lines of that
commit, such as the `review` line of a verdict's commit, do not repeat it.

### A line

```
2026-11-11T16:18:29Z 0a772ac96 atelier-6w8d verdict thread:r1 3-kvll approve; in_review -> approved
```

Each line holds, in this order: the commit's committer time in UTC, to the
second; the commit, as Git abbreviates it; the record's ID; the act; the
thread the table gives, or `-` where it gives none; then what changed, and
last each skip as `skip <rule>: <reason>`. The first five are separated by
single spaces and hold no space. What changed and the skips are the rest of
the line, in parts separated by `; `, with any line break in them made a
space. A thread's identity is printed whole, as the record holds it, so a
Codex subagent reads `<parent>/<child>`.

A status move reads `<from> -> <to>`, each spelled as a record spells it,
and `new` stands for the status of a record the commit added. What changed
holds:

| Act | What changed |
| --- | --- |
| start | The status move, when the commit made the item `building` |
| take-over, of an implementer | `from <former implementer>`, then the status move as for start |
| submit | The status move, as `building -> in_review` |
| review | The review's name, as `1-c27c` |
| take-over, of a reviewer | The review's name, then `from <former reviewer>` |
| verdict | The review's name and `approve` or `changes`, then the item's status move when the commit made it `approved` or `changes_requested` |
| escalate | The finding it is about, when there is one, then the question as its heading reads, with its date |
| decide | The outcome, `defer` or `no action`, when there is one, then the answer |
| merge | The status move, as `approved -> done`, then `trivial: <why>` for a trivial merge, and the merge commit's branch |
| settle | `<finding> taken into <item>`, or `<finding> dropped: <reason>` |
| lead | `from <former lead>`, when the lead changed |
| ready | The status move, then `by <name>` |
| drop | The status move, then the reason the record's `Dropped:` line gives |
| close | The status move |

### The walk

One `git log` lists every commit that changed a file under
`.atelier/items` or `.atelier/missions`, from the base's tip and the tip of
every unmerged branch the reader reads, local and remote-tracking,
parents before children, with its
parents, committer time, trailers, and the old and new blob of each record
it changed. The repository's one `git cat-file --batch` process reads those
versions. An act is found by comparing a record's two versions across one
commit, so no record's history is read on its own, and `log` starts the
same few Git processes however many records and commits there are. Git
lists each commit once, so each act is printed once.

- **Both sides of every merge.** Given paths, `git log` simplifies history:
  it follows only the side a merge matches on those paths, and the other
  side's commits are lost once no branch names them. `log` passes
  `--full-history`, so it follows both. An Atelier merge never matches
  either side, since it marks its item done, so the loss shows on a plain
  merge whose branch's record changes were undone; the scenario test of
  that case fails without the flag.
- **Merges.** A commit with one parent is compared with that parent. A merge
  commit on the base's first-parent line that first brings an item's copy
  to the base in a started status, or, before the branch model, an Atelier
  merge that set it `done`, gives only that item's `merge` line, from the
  item on the merge's second parent against the item in the merge: what its
  branch brought is found on the second-parent side. The forge's merge and
  `merge`'s own are read alike, and a merge someone made by hand on the
  forge is read the same way. A merge of a mission's or a backlog branch
  gives no line of its own: the acts its branch carried, such as a `ready`
  or a `drop`, are found on the second-parent side. Any other merge commit
  gives no line.
- **The cutover's conversion commit is read like any other.** Each record
  it added shows its acts at that commit's time, as `new -> <status>` and
  every thread its record names. Nothing special-cases it; a reader who
  wants the time after it uses `--since`.
- **A version that does not read.** History can hold a record that was
  invalid at one commit, or not UTF-8. `log` passes that version over,
  compares the versions on each side of it, and says on standard error
  which commit and path it passed over. `git log --parents` gives each
  commit the nearest listed commits as its parents, so what a commit passed
  over is carried to the next commit on its line of history: the same
  broken blob after two different versions stands each time for the one
  before it there.
- **Time and order.** A line's time is its commit's committer time. Lines
  are ordered by it, lines of one commit keep the order of the table above,
  and commits of one time keep Git's order, parents first.

In astromech on 2026-11-11, `atelier log` from a debug build printed 88
lines in 0.06 to 0.08 seconds, starting six Git processes.

What `log` does not show: `new`, `set` and edits to a record's text, which
Git's own log has, so an item's time as a draft is read from its
`created_at` and not from `log`; the executing thread of `ready`, `drop`
and `close`, which no record holds; and a skip in a Git hook or on a command
that makes no commit, which leaves nothing in Git.

`log` sees commits, not commands. Acts on different records committed
together appear as separate lines with one time. Two acts on one record in
one commit appear as their net change: an item made ready and closed in the
same commit shows only as closed. An act undone before it was committed does
not appear.

The walk covers the base's history, the second-parent side of each merge,
and the tips of unmerged branches, so an item's acts stay
visible after its branch is merged and deleted. The old tool's history
reader for the debrief followed first parents only and lost them.

`log` adds nothing up. Time in each status from `ready` on, rounds per item,
roles taken over, escalations and skips are counts over its lines, and the
gardener's procedure, not the binary, says which to take.

## Hooks and the bypass

`atelier setup` installs three Git hooks into the clone's hooks directory,
which all its worktrees share. Each is a one-line script that runs
`atelier check --hook <name>` with Git's arguments and input, so the rules
live in the binary (`atelier-cli`'s `check.rs`). A hook is Atelier's when it
is that script byte for byte and executable; run again, `setup` changes
nothing and says so.

- **A hook the clone already has** is kept: `setup` moves it to
  `<name>.local` and says so, a symbolic link moved as a link. Once its own
  rule passes, `check --hook <name>` runs `<name>.local` as Git would: only
  if it is executable, through the shell when it has no `#!` line, and with
  the same arguments and input. For pre-commit and pre-push a failure of the
  kept hook refuses the commit or push; for post-checkout it is printed and
  the checkout still succeeds. A `setup` stopped between moving a hook and writing its own leaves no hook
  and the kept one in place, and run again writes Atelier's.
- **Where `<name>.local` is taken** as well, by a hook that is not
  Atelier's, `setup` refuses and changes no file, and says to put both into
  `<name>.local` and remove `<name>`.
- **Where `core.hooksPath` points Git** outside the Git directory the
  worktrees share, as to a directory of each worktree's own or one shared
  with other clones, `setup` refuses and changes nothing, and names the
  shared directory to copy the hooks into and point `core.hooksPath` at.

The cutover's rollback restores a clone's hooks directory from a copy, so
`setup` has no undo of its own.

| Hook | Rule |
| --- | --- |
| pre-commit | Every staged record parses and validates: a `mission.md`, an `item.md`, a review file or `config.toml`. Other files under `.atelier/`, such as a mission's documents and an item's artifacts, are not examined. Where a record's status differs from the one in `HEAD`, the act table lists that change. A commit made on the base branch that adds, changes or deletes any file under `.atelier/` is refused, naming `start --backlog <slug>` and `start --mission`. The conditions of an act are the command's to check, not the hook's. |
| pre-push | The base receives only merges. With a forge configured, any push that moves the base is refused, naming `atelier merge`, since the forge makes every merge. Without one, each new commit on the base's first-parent line must be an Atelier merge: two parents, and the `Atelier-Merge:` trailer that `merge` writes in its message's last paragraph. What a merge brings in on its second parent is not examined. |
| post-checkout | None. On a branch switch it prints what `status` prints. |

`atelier check` reads every record in the checkout, tracked or not and
ignored or not, as the commands do, one file at a time
(`atelier-records`' `load`), so one that does not parse leaves the rest
read and validated. It prints each problem with its file and field, exits 1
when it found one, and lists the record changes not committed yet, those
`status` lists and `config.toml`, whatever it found. A review
whose item or mission is missing or does not parse is a problem of the
review's own. It needs no readable config
and no base branch, so it reports a broken `config.toml` rather than
stopping at it.

- **pre-commit** reads the index Git gives it, through the commit reader's
  `staged`, and validates every record there against the others. The
  problems it refuses are those of the records the commit adds or changes
  from `HEAD`; a record deleted, or a commit's other records, are not
  judged. A record renamed is judged at its new path. With no `HEAD` yet,
  no status is compared. The commit is on the base when the checkout's
  `HEAD` is the branch `config.toml` names as the base; a commit in a
  detached checkout is not, since it reaches the base only through a
  branch.
- **pre-push** reads the base's name from the checkout's `config.toml` and
  judges each pushed line whose remote ref is the base, whatever the local
  ref; a deletion adds nothing. The new commits are those on the local
  commit's first-parent line not in the history of the remote's old commit.
  When the remote has no base yet, or this clone lacks its commit after a
  force push, every commit on the line is new, so the first push of a
  repository's history is refused, and passes only as a skip. The rule it
  replaced let a commit that changed only `.atelier/missions/` and
  `.atelier/items/` through, for planning committed on the base, and
  refused one that set an item `done` while its branch existed; both went
  with planning on the base.
- **post-checkout** ends in success whatever it finds, a panic included:
  when `status` fails, as on a record that does not parse, it prints why.

A refusal is printed on stderr with exit 1, inside the Git command's own
output. It names each rule broken, each file or commit (its short ID and
subject) that breaks it, and the way out. A record that does not parse is
listed apart, as no rule; in a push, the way out is
to rewrite the commit that broke it, since every new commit is read. A
push from a checkout whose `config.toml` does not read is refused that
way too, since the config names the base.

`ATELIER_SKIP`, set to a reason, lets every rule one command or hook meets
pass. A rule is a refusal that depends on the state of the work or on who
asks. The variable is the whole interface: no flag, no config, no list of
rule names.

- **One place.** Every refusal of a rule goes through `atelier-app`'s
  `Skip::refuse`, given the rules it breaks, each said in one line, and its
  text, which ends with the way out. A refusal is data (`Refused`): its
  rules, its text and its way out. Without a reason the command's refusal
  ends with the skip, after the way out. With one, each rule passes: the command
  prints `Skipped with ATELIER_SKIP: <rule>: <reason>` for each and goes
  on, and where Atelier makes the commit (`start` and a take-over, `submit`
  and `merge`, whose message the forge's merge commit carries), `git
  interpret-trailers` adds a trailer
  `Atelier-Skip: <rule>: <reason>` for each to its message, after `merge`'s
  `Atelier-Merge:`. Whitespace in the reason and in each rule is made one
  line. A skip in a hook, or on a command that makes no commit, writes
  nothing; it shows in the transcript as the variable on the command, which
  is where the scorecard counts it. Atelier's own `git commit` and `git
  push` pass on what the hooks printed, so a rule a hook skips there is
  printed too. The reason and the rules passed are one value per
  invocation, a `Skip`, which its caller makes: the command once per
  process, from `ATELIER_SKIP`, and a program that links `atelier-app` once
  per act it calls. It needs no repository, since `check` and the hooks
  pass rules before one is opened, and it outlives any one, since `merge`
  opens several and prints its skips after the last. Whatever opens a
  repository, makes a commit or prints the outcome is given it, so a rule
  passed reaches that invocation's trailers and output and no other's.
- **All or nothing.** The skip passes an act's refusals, and is offered,
  only when every one is a rule: `acts::refusals` gives them all, where
  `check` gives the first. Otherwise the first is refused as before, with
  no skip offered.
- **Rules:** a role, a status and a holder of the act table, and its
  conditions; a merge check that fails; CI that failed, and CI whose run
  has not finished, which a skip passes without waiting (the "not yet" at
  the bound is no refusal and offers no skip); a record-writing command run
  on a kind of branch the act table does not give for its change, and
  settling a finding off a backlog branch and off an item the caller
  implements; `set` on a record that is not a draft or ready; `ready` on a
  record that is not a draft; `lead` on a mission that is not agreed; a
  record that reads and does not validate, in what a writing command
  writes, in `check` and in the pre-commit hook; a status changed as no act
  on that kind of record (an item or a mission, `acts::changes`) changes
  it; a record change committed on the base; a push that moves the base
  with a forge configured, or, without one, brings a commit that is not an
  Atelier merge; a plan without its two approvals; and `merge`
  with uncommitted changes in the branch's checkout or in the base's. Changes a skip lets stay are left in
  their checkout, and `merge` confirms them unchanged,
  index entries, modes and contents, and a submodule's commit and its own
  uncommitted changes, with its other inputs.
- **Not rules:** a record that does not read; a backlog branch that changes
  a path outside `.atelier/`, which is code and needs an item and its
  review, since no change outside `.atelier/` merges unreviewed; an ID, a mission or a finding
  that does not exist, and a finding settled already; a missing or
  malformed argument; a command run where it has nothing to act on: off the
  item's branch it writes, with no review file of the round to record a
  verdict in, with no question to decide, or `review` of a plan on an
  item's branch, where `verdict` is the item's; `start` where branch `<id>`
  exists, and a take-over where it does not; a merge that conflicts, and inputs that
  moved while `merge` ran; a Git or forge command that failed; `setup`
  where a hook it would keep is taken, where `core.hooksPath` points
  outside the clone, or where a harness's settings do not read; and the
  clone's own kept hook. With a reason set, each of these ends with a line
  saying it cannot be skipped; without one, it does not mention the skip.
- **A blank reason**, empty or only whitespace, is refused by every command
  and hook with a line saying a reason is needed. The post-checkout hook
  prints that line and still succeeds.

`status` begins with one line, naming `atelier setup`, when the directory Git runs
the clone's hooks from has none of Atelier's.

## The forge

The forge code, `forge.rs` in `atelier-cli`, was copied from the old
`atelier-app` with one change to its interface: it takes the forge section
of the config, whose CI workflow is optional, where the old one took
`workflow_policy::ForgeConfig` with a required workflow name. Its behaviour
stayed, and its contract tests came with it. The merged-tree checkout, the
merge commit and the push lived in the old `atelier-cli`'s `merge.rs` and
`merge_push.rs`, bound to `WorkflowPolicy` and `RecordIndex`. They were
copied and put on the config and item types, and their contract tests were
rewritten against the new command.

One more thing was kept, because it is the kind a rewrite loses by
accident. The old merge protected itself against the world moving while its
checks ran. `merge` holds an exclusive lock on `atelier-merge.lock` in the
Git directory every worktree of the clone shares, from the read after its
wait for CI to its end, so a second merge waits for the first and then reads
the base it left. The wait itself holds no lock, so a merge waiting for CI
does not hold up another item's merge, and what it read before the lock is
read again under it. `merge` is the only command that takes the lock, since
it is the only one that merges. The lock orders merges in one clone; with a
forge, merges from different clones are ordered by the forge, and step 8 of
[Merge](#merge) says what that leaves open.
After the checks, and before it writes anything, it confirms that the branch
tip and the base tip are the ones the checks ran on and that neither the
work checkout nor the base's checkout has changed, and it refuses if any
has. The lock and that confirmation were copied from the old `merge.rs`,
`mutation_lock` and `committed_snapshot.rs`, with tests that move the base,
move the branch, change the checkout and run two merges at once. The parts
of that code that guarded against tampering with file modes and index flags
were not carried over. A merge killed during its checks leaves its temporary
worktree behind; the next merge, holding the lock, removes it.

`forge.rs` and the merge code live in `atelier-cli`, with the Git calls
only they make; `atelier-app` runs no forge command and no merge.

Nothing else from the old merge was carried over: not the gate code, the
polling in `merge_wait.rs`, nor `merge_sync.rs`. The wait for CI was written
new. `submit` pushes the branch and opens its pull request with the same
code as `merge`.

The forge's part grows by three calls under the branch model: `merge` asks
it to merge a pull request at a given head (`gh pr merge --merge
--match-head-commit`), reads why it will not yet (`gh pr view --json
reviewDecision,mergeStateStatus`), and `start` pushes a new branch without
force as the claim. Pull request reviews and comments are still not read:
a required human review is something the forge reports as missing, not a
verdict Atelier records.

## Site

The tool has no site, and no Node step in its build or release. The old
tool's site went with its workspace at the switch; [Tried and
removed](#tried-and-removed) says why.

The human's pages are the Desk, a separate binary and web app (mission
atelier-z4cr; see [Decisions](#decisions) and the Desk's
[product page](product/desk.md) and [API](architecture/desk.md)). Its
crate, `atelier-desk`, and its app, `desk/`, are built in this workspace,
and the app has a CI job of its own. It reads records through
`atelier-records` and `atelier-app`, at any commit, so it can load a
mission's directory and items as they stood at the merge of its plan and
compare them with how they stand now. It reads the live repositories, the
base and every branch the convention names, as `list` and `show` do, and
fetches on a timer of its own.

## Transition

The installed binary is one per host and the installed skill is one per
host, so the process changes for every repository on a host at once. The
move to this tool is one step per host.

**On this host, on 2026-11-11.** At the time Sumeet named, the switch
(atelier-u1ko) merged: it deleted the old workspace and moved `next/` to the
root as version 0.5.0. Within four minutes one run of
`scripts/cutover/cutover.sh` converted atelier, astromech, demiurge,
life-on-rails and cosplay-studio, installed the binary and the skill,
stopped the old site's service, pushed each converted base and closed the
cutover item, atelier-wd1i, the first act the new tool recorded. It stopped
nowhere, and no rollback was needed. In each repository `--verify` matched
a fresh conversion of the recorded base, `atelier check` passed, and the
counts were those the conversion printed: 133 items and 7 missions in
atelier, 240 and 4 in astromech, 19 and 1 in demiurge, 47 and 3 in
life-on-rails, and 37 and 7 in cosplay-studio. The first item under the new
tool, atelier-2mlh, then made CI check the records with the tree's binary
only.

What is still to come:

- **The work host adopts.** It does not convert: its remotes are already
  converted, and until it adopts it must not fetch a repository that has
  been. It adopts with `scripts/cutover/cutover.sh adopt`, run from a fresh
  clone of atelier's converted master made for the purpose, which is where
  it gets the reviewed script, the tool's source and the skill without
  moving any checkout it works in. In order: the preflight, with each base
  allowed to be behind its remote and never ahead; the binary installed
  from that clone; the skill installed by the command the settings give,
  since hosts install it differently; the old site's service, where the
  host has it, stopped and disabled; each clone's base moved to its
  remote's converted tip, its main worktree put on the base and `atelier
  setup` run, as on this host; and the acceptance checks without CI's, with
  the count of items and missions read by `atelier check` and printed. It
  converts, pushes and closes nothing. `rollback.sh` undoes it from its
  journal and pushes nothing: hooks, refs (remote-tracking refs included)
  and worktrees back to what was recorded, the fallback release
  reinstalled, and the skill and the service as they were. The page for the
  day is [scripts/cutover/README.md](../scripts/cutover/README.md#another-host-adopts).
- **`scripts/cutover/` is deleted** once the work host has adopted. Nothing
  else is left to clean up, since the old workspace went whole at the
  switch.

## Testing

- One scenario test per lifecycle path, driving the real binary in a scratch
  repository: the plain path; changes requested and fixed over two rounds; a
  rebuttal withdrawn and one kept; an escalation decided as defer; a trivial
  merge; an item with no files; a mission from `start --mission` through two
  plan reviews and its merge to closed; an implementer taken over in
  `approved`; a reviewer taken over mid-round; a former implementer refused
  as reviewer and as decider, and a planner refused as plan reviewer; two
  clones of one remote starting one item at once; a started item dropped on
  its branch from another clone, then refused at merge, in both orders; a
  record-only branch merged on `check`, and one carrying code refused.
- The protected base (atelier-t5am): against a local remote whose hook
  refuses every push to the base, the item loop, a mission's plan and a
  backlog branch all merge through the forge's contract-test double; the
  same script with no forge configured merges by moving the local base.
- Unit tests on the act table: every act from every status.
- `log` (`crates/atelier-cli/tests/log.rs`): the plain path prints its acts
  in order with the right threads, and the same lines after the item's
  branch is merged and deleted; a take-over shows both implementers, and a
  review taken over both reviewers; an escalation and its answer; a deferred
  finding settled on a gardener's branch and merged; a mission's plan
  reviews, its merge, `lead` and `close`, with its item dropped; an item
  merged by the forge, whose base copy stays `approved`, shown as merged; a skip on its
  act's line; `--since`; two acts on one record in one commit as their net
  change; a version that does not parse passed over; a branch's acts after
  a plain merge that matches the base, which fails without
  `--full-history`.
- `new --from` on a deferred finding of a done item writes `Taken:` with the
  thread, and `list --deferred` no longer shows it; settling it again is
  refused; settling a finding of an item that is not done is refused.
- Hook scenarios in a scratch repository: a commit of an invalid record
  refused; a commit of a status change the act table does not allow refused;
  a commit on the base that changes a record refused; with a forge, any
  push that moves the base refused; without one, a push of the base holding
  a plain commit refused, whatever it touches, and one whose new commit is
  an Atelier merge of a branch with ordinary code commits allowed; a push
  of any named branch
  allowed whatever it holds; the refused push allowed with `ATELIER_SKIP`
  set, and the skip printed; a branch switch printing `status`, and one to
  a branch holding a record that does not parse succeeding.
- `ATELIER_SKIP`: each kind of rule passed, its line printed, and its
  trailer in the commit of `start`, `submit` and `merge`; a mission set
  from `dropped` back to `open` by hand refused, and one closed from `open`
  allowed; a blank reason refused by the commands and the hooks;
  and each kind of refusal that is not a rule refused with the variable
  set, saying it cannot be skipped.
- `merge` with CI still running: it waits, holding no lock, and merges as
  soon as CI passes or refuses as soon as it fails; at the bound it stops
  with its "not yet" exit status and changes nothing, also when the forge
  fails or never answers, and names the run the last answer named when a
  later question is cut off. A question under the lock that never answers
  is ended within the bound. A run that passed on the commit `submit` pushed
  still counts after `verdict approve`; one before a change outside the
  item's directory does not. A merge killed during the wait leaves
  everything as it was, and another item's merge completes meanwhile.
  Changes made during the wait are judged under the lock: a new base, a new
  merge check, a branch gone stale.
- A fresh-agent scenario, run by the eval harness (`next-loop` in
  `evals/agent-factory/`): an agent given only "run `atelier status`" takes
  a ready item to done. It is run with Claude Opus and with a Codex model.

## Tried and removed

Each of these was built and taken out. Do not propose one again without new
evidence.

| What | Why it went |
| --- | --- |
| A SQLite cache of records | Slower warm (250 to 310 ms) than parsing every record (62 ms for 973 issues) |
| Transition actions with side effects | A rules engine nobody could predict; removed in the October rebuild |
| A review provider, review rooms, Forgejo pull requests | Reviews belong in the repository |
| Evidence gates | They checked that a receipt existed, not that it was true |
| A configurable workflow file | Never tuned per project; each lifecycle change meant editing every repository |
| A hard limit on lines of code | The October limit of 20,000 held for a day, and a limit rewards shaving lines |
| Sessions as records | Attribution is a field on the record that the act writes |
| Records on a separate branch | Records with the code are the reason the tool exists |
| A plan as its own branch | Removed in October 2026 because grooming touches many items and planning branches collided. Restored on 2026-11-13 (atelier-to83), with what was learned: the collisions came from long-lived branches that carried grooming across the whole backlog, not from a plan having a branch. A mission's branch now lives only until two plan reviews approve it, grooming goes on short `backlog/` branches, two branches that touch one record meet as a merge conflict, and `atelier check` on the merged tree catches two that are each valid and contradict each other. Planning on the base, which replaced it, needed direct pushes that a protected base refuses and kept unagreed drafts on the base |
| `land`, a check an agent had to run | Agents forgot it, or hid its refusal behind a pipe; a pre-push hook runs without being remembered and fails the push itself |
| Waiting for CI inside `merge`, for as long as `--wait` and `--timeout` said | A command that sleeps for minutes fights the harness, and one network timeout failed it. It came back on 2026-11-10 in another form: bounded at 9 minutes, under the harness's limit, with no flag, holding no lock, and counting a failed question to the forge as not finished. The thread that runs `merge` is a delegated child that nobody wakes when CI finishes |

Mission atelier-to83 considered these, or planned them and took them out
of its plan before any was built (2026-11-12 and 2026-11-13):

| What | Why not |
| --- | --- |
| A mission integration branch: items merge into the mission's branch, which merges once at close | It fixes additions mid-mission and gives one human gate per mission, but it is the long-lived feature branch: late conflicts, late delivery and a large final merge. Raised and dropped several times, last as atelier-w0su. Items merge to the base one at a time, and a change that cannot land in pieces is hidden in the code by its mission's plan, as the 0.5 rewrite was in `next/` |
| Records in a database, with the Desk as the service | Consistent with itself, not with the code, which is still on branches. Every checkout, sandbox, eval clone and host would need to reach it, a pull request could no longer carry its record, and mission documents would stay in Git anyway, leaving both |
| A branch per running mission as the lead's claim | It solved an exclusion nobody needs: nothing that reads `lead` needs it to be a lock |
| Approval read from GitHub's reviews | GitHub cannot tell Sumeet from an agent on one account, and a second account is against its terms. Dropped on 2026-11-10 for this reason, with the whole idea of plans as pull requests, which did not depend on it |
| Small record changes direct on the base, plans through pull requests | The base would still need direct pushes, so it fails "only a merge writes the base" |
| A branch per plan verdict, which the tool gathered onto the mission's branch | It enforced by machinery what the order of the steps already gives: each reviewer forms its view at the submitted commit before it publishes |
| `verdict` committing and pushing the plan review itself, retrying the push | The same, and it put Git back into a command; the reviewer commits and pushes with Git |
| `merge` writing `done` in a last commit on the branch, before the forge merges | See [How `done` is recorded](#how-done-is-recorded) |
| A record-writing command in the base's checkout making a worktree for the change | The tool would own a path and its removal, which the harness already owns; it refuses instead (see [The base's own checkout](#the-bases-own-checkout)) |

The redesign also removed these from the old tool, as it stood at 0.4.0:

| Removed | Why |
| --- | --- |
| `workflow.yaml` and the workflow crate | The workflows are identical in all five repositories; every edit followed a tool change |
| Evidence records and commands | No gate reads them; the project's own audit rated 57% busywork |
| The light path and base exemptions | `start --new` and `merge --trivial` cover them with one path |
| Issue types, parents, loose ends, validations, decision records | An item either has files to merge or it does not; the rest are labels |
| `bundle`, `migrate`, `doctor`, `prune` | Unused by the skill, one-off, or not yet needed |
| `land` | A check an agent had to remember to run, and whose refusal a pipe once hid; a pre-push hook runs without being remembered and fails the push itself |
| `merge`'s `--push`, `--wait`, `--timeout`, `--sync`, `--check` and `--json` | Pushing follows from the config; the wait for CI has a fixed bound and needs no flag; catching up is `git merge`; what merge would check is in `status` |
| `brief` and `debrief` commands | The Desk shows them; until it has its mission page, a mission is read with `atelier show <mission>` |
| `serve` and the site crate | The human's pages are a product of their own, the Desk (see Decisions) |
| The nine `work` views and nine `review` subcommands | `status`, `list`, `review` and `verdict` replace them |
| Strategy files in `docs/strategies/` | A mission's documents live in its directory |
| ADRs | One design record, edited in place, with a list of what was tried and removed |

A lead or a planner lost none of the old tool's nine `work` views; they
became `show`, `list` and `status`:

| Old view | Replaced by |
| --- | --- |
| `work mission <id>` | `show <mission>`: status, lead, Outcome, criteria and Result, items by status with holder and blockers, questions, plan reviews |
| `work missions` | `list`, which heads each open mission's items with its status, whether it is in progress, and its lead; `status` on the base lists the missions in progress |
| `work missions --all` | `list --done` for finished missions and items; `list` heads the open ones, and lists items in no mission |
| `work ready` | `list --ready`, and "Ready to start" in `status` on the base |
| `work blocked` | `list --blocked`, which names each blocker that is not done |
| `work human` | `list --needs-human`, and "Waiting on a human" in `status`; open decisions are the unanswered questions in `show <mission>` |
| `work decisions <id>` | The questions in `show <mission>`, unanswered first; an item's Decisions section is in `show <item>`. Decision records and notes are gone |
| `work active` | `status`, which lists what the caller holds and what other threads hold, read from their branches; `list` shows each started item with its holder |
| `work all` | `list` |
| `work parent <id>` | None: parents are gone, and a mission groups its items |

## Decisions

Sumeet made each of these in conversation on 2026-11-09, except those dated
otherwise. Those dated 2026-11-13 are mission atelier-to83's, made with him
on 2026-11-12 and 2026-11-13; its `design.md` gives the longer reasoning. A
line that names an ADR is a decision of that ADR that still governs,
carried here when the ADRs were deleted on 2026-11-11; Git keeps the ADR.

| Decision | Reason |
| --- | --- |
| Records stay in the working tree of the branch that does the work | It is the tool's core value; a host database would be simpler than any halfway design |
| Status is a stored field | Every stage change is an act by a known role |
| The implementer merges its own approved item | The reviewer is already the unbiased judge; a third agent adds a relay |
| The harness starts agents, by whoever holds the status | T3 Code and Claude Code manage subagents well and change fast |
| Review and readiness are separate facts | Ready says an item should start now; a review says independent agents examined a change. Since 2026-11-13 only an item is made ready, and a mission's plan reviews are its gate |
| Planning commits went on the base (2026-10-15, atelier-3mjv), until 2026-11-13 | Reversed by "Only a merge writes the base" below |
| The workflow file goes | See Tried and removed |
| No backward compatibility | The design should be the best one, not the nearest one |
| Proof is the configured merge checks plus manual validation with artifacts | Work has no CI; routine receipts were busywork |
| The line count is an indication, not a limit | See Tried and removed |
| The ADRs are replaced by one design record | They carry history that agents read as current |
| The tool is rewritten in full, in a fresh workspace | A gradual rewrite has no benefit once the record layout changes under every command |
| A view of what changed since his go-ahead is the Desk's to derive | It is a derived view; this work supplies the acts with their commits and records readable at any commit |
| Hooks guard, commands act | A rule in a command applies only when the agent remembers the command |
| The tool gives facts and the skill does the arithmetic: `log` and `list`, and no scorecard command | A fixed report in the binary would change every time the measures do, as the debrief did |
| The gardener is a role, and its needs shape the records now | A bug's cause and a deferred finding's settlement are cheap in the first format and costly to add later |
| Every rule can be skipped on purpose, with a reason | Atelier's rules are not hard blockers, and a bypass should be intended and leave a trace |
| `submit` pushes the branch and opens its pull request where a forge is configured (2026-11-10) | CI then runs during the review; it was nearly all of the wait between approval and merge, a median of 5.5 minutes on 25 items |
| `merge` waits for CI, for up to 9 minutes, with no flag (2026-11-10, reversing "`merge` never waits") | The thread that runs `merge` is almost always a delegated child, which T3 neither lets watch a pull request nor wakes reliably, so "not yet, run me again" left approved items with nobody driving them |
| His go-ahead on an item stays `ready --by`; a mission has none since 2026-11-13 | A machine account for agents would let GitHub carry it, and his employer's sign-on rules make one too hard to get |
| The tracker is for agents and the Desk is for the human: two products sharing a vocabulary and the core, which reads records, computes views and performs acts for both. `atelier` has no site. The Desk is its own binary, `atelier-desk`, and web app, `desk/`, one service per host, outside the `atelier` release (ADR 0032, 2026-11-11) | A page that mirrors the tracker's records asks the human to read records; he needs to make a few decisions well, across every repository, at a desk or on a phone |
| The Desk acts only through the core: it calls the same function the command calls, naming the person acting, and gets the result as data. It runs no `atelier` binary, reads no command output and writes no record itself. What it needs that the core lacks is added to the core (ADR 0032) | A server outside the core spent most of its code re-deriving what the core knows, by parsing output and grepping records |
| Records in Git are the only source of truth for work. No check, merge or hook consults the Desk, its database or T3 Code; the guards decide from Git and the records, and from CI for a commit where a forge is configured. Dependence runs one way: `atelier-desk` on the core, never the reverse (ADR 0030, 2026-11-08; ADR 0032) | Records with the code are the reason the tool exists, and a guard that read anything else could disagree with them |
| One SQLite database per host holds only what Git never held, and each kind has one writer: `atelier` writes which thread, model and role ran each command, each command its own row from the environment its session gave it, never through an MCP server; a feed that runs daily, unattended, writes measurements and findings about how work went; the Desk writes what the human has seen. It holds no copy of a record, and agents read it through a command, never from the file (ADR 0030, decision 7; ADR 0032, decision 6) | Nothing else records who ran each command, so models cannot be compared by role; a copy of a record would be a second truth |
| Live pages read three things: records through the core, T3 Code through its MCP server with a scoped token, and the database. Nothing reads T3 Code's own database for a live page. Only the feed reads anything else, such as transcripts, through one adapter per source, and what it stores says where it came from. Conversations stay in T3 Code: the Desk starts a session and links to it, and is not a chat client (ADR 0032, decisions 7 and 8) | T3 Code gives a process outside a thread a supported way in through its MCP server; its own database is not an interface, and a stored measurement must say where it came from to be trusted |
| A forge carries branches and reports CI; review stays in the repository. No forge state feeds a check but a CI result for a commit, and pull request reviews, comments and issues are not read. GitHub is the one forge, run through `gh` and `git` with their own credentials, and another is added only when a repository moves to one (ADR 0031, 2026-11-08) | The routine of push, pull request, CI and merge was run by hand on every change; reviews and findings must stay records the guards can read from Git |
| Output is dense Markdown for every reader, with no JSON or YAML mode for agents. An agent piping a command's output through `head`, `grep` or `sed` shows a missing view, which is filed rather than met with a machine format (ADR 0022, 2026-10-17) | Agents read Markdown, labels and symbols fluently; a second format is a second interface to keep true |
| Repository instructions live in `AGENTS.md`, which a harness loads, and Claude Code repositories import it with `@AGENTS.md` in `CLAUDE.md`. Commands print state and the next command, not instructions an agent has already loaded (ADR 0022) | A `CLAUDE.md` without the import once kept `AGENTS.md` from loading, and static text in output is read on every call |
| Agent Factory's one copy is `skills/agent-factory` in this repository, with its evals in `evals/agent-factory`, installed with `npx skills add spadaval/atelier --skill agent-factory -g`. Nothing under `crates/` reads the skill's files, and a command change that makes the skill wrong updates it in the same commit. The skill steps aside outside an Atelier repository (ADR 0029, 2026-11-08) | A command and the guidance that describes it landed in separate repositories and drifted; a second copy of the skill diverged |
| A mission's success criteria are optional, and any criterion must be something the mission's own work can make true and an agent can check when it closes. Work that waits on calendar time, outside adoption or a measurement after shipping is its own item outside the mission (ADR 0024, 2026-11-04) | Criteria that restated items or waited on events stalled missions, and each stall asked Sumeet the same question again |
| A new rule or record kind names the kind of defect it would have caught in real agent sessions, and a rule that only patches another rule's edge case is a reason to remove the first rule (ADR 0028, 2026-11-07) | When qualities conflict, fewer concepts win over finer guarantees; a rule added for another rule's edge case is how the old machinery grew |
| One installed binary serves every repository on a host, so a later change to the record format keeps reading the old form until every repository on the host has moved. The redesign's change of layout was the one exception, made by converting every repository at once (ADR 0028) | A format change that one repository has not taken stops the binary there |
| Only a merge writes the base (2026-11-13, atelier-to83) | A protected branch refuses anything else, and most repositories protect theirs; requiring the opposite would be backwards. Content (the plan, findings, the Report) is written, reviewed and tied to code at a commit; coordination state (who holds what, what is in flight) is derived from the base and the named branches, not committed to the base, which is what made `.atelier/` behave like a database |
| Branch names are `<item-id>`, `<mission-id>` and `backlog/<slug>`, with one optional prefix in `config.toml` that applies to all three (2026-11-13) | The names are the index where there is no forge, the claim, and the line between published work and scratch. One prefix covers a host's own naming rule; a free pattern could not be read back into an ID |
| Three commands act on Git; the rest write files (2026-11-13) | `start` makes and claims a branch of any of the three kinds, `submit` publishes it, and `merge` merges it. `new`, `set`, `ready`, `drop`, `close`, `lead`, `review` and `verdict` write the file and stop; the agent commits and pushes, and the hooks validate. Sumeet said on 2026-11-10 that the tool leaned too far toward wrapping Git, and the plan had drifted the same way. `start` never switches the base's own checkout, which has cost us before, and the pre-commit hook refuses a record change committed on the base |
| A branch merges on what it contains (2026-11-13) | Code needs an independent review; a new mission or a changed plan needs two approving plan reviews; records alone need `atelier check`. A branch counts as records alone only when every path it changes is under `.atelier/`. A plan is compared by section of `mission.md`, not by file, and until it has merged its items are part of it |
| The trunk, not an integration branch (2026-11-13) | Each item merges to the base on its own and leaves it working, so a mission half done on the base is an accurate picture |
| A mission's stored status is `open`, `done` or `dropped` (2026-11-13) | Proposed is never stored: it is `open` read from a branch that has not merged. A mission dropped on its branch is dropped, and `merge` refuses it. An open mission on the base is agreed and may be led and closed. An old record's `ready` reads as `open`, and an old `draft` reads as proposed until the rollout moves it to a branch or its plan merges |
| A mission records its planners (2026-11-13) | `start --mission` binds the calling thread, as `start` binds an implementer, and a later thread takes the plan over by `submit`. It is how `merge` knows a plan review is independent |
| A review is a file the reviewer commits (2026-11-13) | `review` creates the file for the commit the checkout is at and prints the brief; `verdict` writes the verdict into it; the reviewer commits and, for a plan, pushes to the mission's branch. Blind means a reviewer forms its view without the other's, and each starts in its own checkout at the submitted commit. A reviewer who goes looking, with `show` or a fetch, can read the other's verdict; that is accepted on Sumeet's direction of 2026-11-13, because the tool's checks are nudges for trusted agents, and hiding it cost a rule in three commands. Plan review 7-dkmp of atelier-to83 kept a finding against this, and it stands as a recorded disagreement |
| A started item's branch is pushed at `start` where a remote is configured (2026-11-13) | That push is the claim across clones. The loser is refused, told the holder, and its local branch removed, so nothing in that clone shows the item as held there; a later `start` there is a take-over. With no remote the local ref is the claim |
| Stopping a mission is dropping it, and a drop counts once published (2026-11-13) | `drop` writes the record and stops nothing until it is merged, for an agreed mission by the record-only path, or pushed with its pull request closed, for a proposed plan. `status` and `show` say when a drop is not yet published, and the Desk's stop carries it through. From then `start`, a take-over and an item's `merge` refuse, naming the stop |
| A started item is dropped on its own branch (2026-11-13) | Its record changes only there, so that is where a drop takes effect: once pushed, its `submit` and `merge` are refused. The base's copy follows by the record-only path, and `status` says so until it has. A drop of an item already merged is refused |
| A mission has no draft status (2026-11-13) | A plan on its branch is a draft by where it is. An idea for a mission is a draft item on the base, so the base stays the one list of what might be done and branches stay short-lived |
| A mission has no go-ahead (2026-11-13) | It protected direction, and direction is given when the mission starts, in a conversation with Sumeet or a proposal he agreed; the Why says which. The plan reviews do the checking, and found nine problems in atelier-g961 that neither he nor the planner had seen. He keeps a veto at any time, made real by the Desk telling him when a plan is proposed and when it merges |
| The two plan reviews are enforced by `merge` (2026-11-13) | They are now the only gate, and until then the rule lived only in the skill |
| Items keep `draft` and `ready` (2026-11-13) | They record intent: agreed to exist, or agreed to start now. Readying a mission never readied its items, so a lead can hold later work until earlier work has shown what it shows |
| `lead` stays a field on the mission, as attribution (2026-11-13) | The Desk shows who leads. It reaches the base by the record-only path once per mission. It is not a lock, and a second lead loses at merge |
| `escalate` and `decide` are removed (2026-11-13; atelier-xbog does it) | They served two jobs. Advice needs no record: the implementer asks the thread that started it, as Claude's advisor tool lets a working model ask a stronger one and carry on. A disputed finding already has a path: a rebuttal in Responses, settled by a fresh reviewer. Nothing has used either command since the cutover |
| A disagreement is found, not enforced (2026-11-13) | The records hold it already: a `rebutted` Response with the next round's `Now: kept` or `Now: withdrawn`, and a plan-review round whose reviewers differ. `list --disputed` prints them, `show <mission>` gives a lead its mission's, and the gardener reads them first. Nothing blocks on them. A lead steps in with the lever it owns: it moves the disputed work out of the item's Scope into an item of its own |
| A lead's notes are ordinary record changes (2026-11-13) | Two or three per mission, in `notes.md`, through the record-only path |
| Record-only changes take about a minute (2026-11-13) | A tracker in Git cannot be instant, and does not need to be. Each repository's CI must give records a fast path, which is a line of the readiness checklist |
| With a forge, a merge finishes as the forge's merge of the pull request (2026-11-13) | So it works on a protected base and with human reviewers, where `merge` stops with status 3 until they approve. With no forge, `merge` moves the base locally |
| A merged item is done; nothing writes `done` at the merge (2026-11-13) | A started item's record reaches the base only through its own merge, so its base copy in a started status means merged. Chosen in atelier-ogpg, under the Authority atelier-to83 gives its lead, over a last commit setting `done`; see [How `done` is recorded](#how-done-is-recorded) |
| A record-writing command refuses in the base's own checkout, and `start` there makes the branch without switching (2026-11-13) | Files written there could be committed nowhere and would block other threads; see [The base's own checkout](#the-bases-own-checkout) |
| `ready.at` and `created_at` are read from Git (2026-11-13) | The commit holds the time, and `atelier log` already reads times there. A mission's record drops both; an item's keeps them until an item of its own removes them, since none of atelier-to83's items changes the item's format |

Three points of the design went beyond what he had confirmed when it was
written, and were put to him with it:

- **One cutover per host.** He agreed to convert repositories one at a time
  with the binary reading both layouts. The skill is installed once per host,
  so the process would change for every repository at once anyway, and
  reading both layouts would keep the old code alive through the whole build.
- **The conversion commits are pushed directly.** Neither tool can gate the
  commit that replaces one record layout with the other: the old one cannot
  read what it produces, and the new one cannot read what it starts from.
  They are a reviewed script's output, checked mechanically.
- **Deferred findings create no items.** He agreed that a deferred finding
  becomes a draft item at merge. This design leaves it in its review file
  and lists it with `list --deferred`, because the backlog already holds four
  piles of such items that nobody reads.

## Open questions

None of these changes the shape of the work. Each is settled by the item that
reaches it.

- Whether T3's pull request watch wakes a thread when a repository marks no
  check as required.
- Where video artifacts go when a project needs them.
