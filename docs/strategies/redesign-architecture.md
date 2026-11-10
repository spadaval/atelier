---
status: draft
last_changed: "2026-11-10: Records brought up to date with the record store atelier-mal9 built"
---

# Atelier redesign: architecture specification

This is a target design. Today's architecture is in `docs/architecture/`. The
product behaviour this serves is in
[redesign-product.md](redesign-product.md). When the redesign is finished this
document, edited to match the code, becomes the single design record that
replaces the ADRs.

## The rule for this work

The aim is better code, not more code. The tool is rewritten in full, in a
fresh workspace beside the old one. Nothing new depends on anything old. One
reviewed change then deletes the old workspace and puts the new one in its
place, and the cutover converts the records. No compatibility layer is
built: no binary reads both record layouts.

A piece of the old code is copied across only when its behaviour is wanted
as it is. The item that copies it names it, trims what the new tool does not
call, and it is reviewed as new code. Everything else is written new.

Non-test Rust is reported at the end of each mission as an indication. It is
not a limit, and no item is shaped to hit a number.

## Why a full rewrite, and what makes it safe

This is the tool's third rebuild, and 31% of the Rust ever written here
survives, so the choice needs its reasons stated.

- The record layout changes under every command at once, so the command
  layer cannot be replaced piece by piece.
- What would have stayed in place is small, and it is bound to types that go
  away: the forge code takes the workflow policy's config, and the merge
  code takes the workflow policy and the record index.
- A fresh workspace cannot grow a dependency on the old one by accident, and
  the switch becomes deleting one directory.

What is different from the earlier rebuilds: the scope is two reviewed
specifications, fixed before any code; the old tool keeps tracking the work
until the new one is rehearsed on every repository; the scorecard has a
baseline to judge the result against; and the list of what was tried and
removed is in this document.

## Where the code is today

48,988 non-test lines of Rust in six crates under `crates/`, and 59,627
lines of tests, of which 50,438 are CLI integration tests. None of it is
edited by this work. It is deleted whole at the switch.

| Area | Lines today | In the new workspace |
| --- | --- | --- |
| Record parsing, validation and index | 7,930 | Written new. Front-matter splitting is copied with one change, so that it keeps a record's body byte for byte where today it strips a newline at each end. ID generation is copied from `record_id.rs`. |
| Merge, gates, freshness, light path | 6,190 | Written new. Copied and put on the new types: the merged-tree checkout, the merge commit and the push from `merge.rs` and `merge_push.rs`, and the protection around them described under The forge. |
| Issue and work views and output | 5,304 | Written new, as `status`, `show` and `list` |
| Missions, brief, debrief, criteria | 4,753 | Only what the site's pages need, at the site step |
| Review and findings | 4,669 | Written new, as `review` and `verdict` over one file per review |
| Site, Rust and web | 4,926 | Copied at the site step, with its loader written new |
| Workflow policy | 3,418 | Not carried over; a short config section replaces it |
| Status, land and hooks | 2,438 | Written new. The harness hook installer is copied. |
| Prune and history | 2,410 | Not carried over |
| Evidence | 1,931 | Not carried over |
| Bundle | 1,665 | Not carried over |
| Forge and CI (`atelier-app/src/forge.rs`) | 1,202 | `forge.rs`, 382 lines, is copied with its contract tests, and takes the new config |
| Git and shell adapters (`commands/git.rs`, `git_freshness.rs`) | 3,501 | Copied: the Git calls the new commands use, and the object reader, which reads files at a pinned commit through one `git cat-file --batch` process and reports a missing path, a non-regular file and a failed subprocess. The record loader on top of it is written new; today's is bound to types that go. |
| Migrate | 883 | Not carried over |
| Setup | 821 | The merging of hook settings into a harness's config is copied; what it installs is new, and it gains Git hooks and Codex hooks |

No old test is ported, except the contract tests that come with a copied
piece. The new workspace has its own.

## Workspace and crates

The new tool lives in `next/`, a Cargo workspace of its own with its own
lock file, built to `next/target/`. It has the crates the old one has, less
`atelier-workflow`, under the same names:

| Crate | Owns |
| --- | --- |
| `atelier-core` | Domain values: Mission, Item, Review, Finding, Status, Thread, Config |
| `atelier-records` | Reading and writing the three record kinds and `config.toml`, from a checkout or from a commit; IDs; validation |
| `atelier-app` | Use cases: the status view, each lifecycle act, the merge checks, cross-branch reads, the history of acts, the site's views |
| `atelier-site` | `atelier serve` |
| `atelier-cli` | Arguments, dispatch, Git and shell adapters, output |

The crates sit at `next/crates/<name>/`, so the switch can move them to the
root `crates/` without changing a path inside them. The dependency direction
is `cli` to `site` to `app` to `records` to `core`.

Nothing in the new workspace builds from the old one, and CI proves it the
direct way: the job that builds and tests `next/` deletes the root `crates/`
directory first. If anything under `next/` reads the old code, by a path
dependency, a module path, an include, a symlink or a build script, that job
fails. No list of the ways to reach the old code has to be kept complete.

The rule is about what the build reads, not about text: a document or a
comment in `next/` may name the old code a piece came from.

CI builds and tests both workspaces. `scripts/ci-jobs.sh` says which a
change needs, the new one when a path under `next/` changes and the old one
when a path under `crates/` does, and both when the toolchain, `.cargo/` or
the CI workflow does, and `.github/workflows/ci.yml` gains the
jobs that build and test `next/`. `next/target/` is ignored by Git, and
`scripts/count_non_test_rust.py` can count one workspace at a time.

The new binary is `next/target/debug/atelier`. It is called "the new binary"
below, and it is installed nowhere until the cutover.

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
mission: atelier-icv4   # optional
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

A Decisions entry is a `### <date>: <question>` heading under Decisions.
Its lines may name a finding (`Finding: 2-m2bn.1`); once answered it has
`Answer by <thread>: <answer>`, and then, for a finding, `Outcome: defer` or
`Outcome: no action`. An `Outcome:` without a `Finding:` and an answer is
refused.

### `missions/<id>/mission.md`

```yaml
schema: atelier.mission
schema_version: 3
id: atelier-xxxx
title: ...
status: draft           # draft ready done dropped
labels: []
lead: <thread>          # set by lead
ready: {by: sumeet, at: <time>}
created_at: <time>
```

Sections: Why, Outcome, Success criteria (optional), Scope, Authority,
Result. Any other file in the directory is a planning document and has no
required shape.

### `reviews/<round>-<suffix>.md`, under an item or a mission

```yaml
schema: atelier.review
schema_version: 3
round: 2
commit: <sha reviewed>
reviewers: [<thread>]   # review appends; the last entry holds the review
model: gpt-6.1-sol
verdict: null           # approve or changes, set by verdict
created_at: <time>
```

The suffix is four random base-36 characters, and the front matter's
`round` is the file name's. Creating a review picks a suffix no file of the
item has, so blind reviews of one round never share a name. Sections:
Summary, then `## Finding <k>: <title>`, numbered without repeats, whose
first line is `Recommendation: fix now | defer | no action`. A finding's
name is its review's file stem and its number, as in `2-m2bn.1`. A finding
carried from an earlier round also has `Was: <finding name>`, naming a
finding of an earlier round, and exactly one of `Now: fixed`,
`Now: withdrawn`, `Now: kept`, or `Decided: defer | no action`, which the
tool writes from the Decisions entry and the reviewer cannot change. Every
finding name a record gives, in `Was:`, in Responses or in a Decisions
entry's `Finding:`, must be a finding of the item's reviews.

A fix-now finding is open until a later round carries it as fixed,
withdrawn or decided, or a Decisions entry naming it has an outcome.
Carried as kept, it moves: the carrying finding is then the open one, and
must say `Recommendation: fix now`. A
carrying finding that says fixed, withdrawn or decided is not itself open.
A review whose verdict is approve while one of its own findings is open is
refused.

A finding is deferred when its recommendation is defer or a decision
deferred it (`Decided: defer`). A deferred finding is unsettled until its
text gains
`Taken: <item> by <thread>`, written by `new --from`, or
`Dropped by <thread>: <reason>`, written by `drop`; `<item>` is an item
ID, and a finding with both lines is refused. Only a finding of a done
item can be settled, and a settled one is refused. The line is written in
the checkout the command runs in, like any record write: on the base, or on
the branch of a gardener's run, where the staleness rule counts it as a
change outside the run's own item, so it is reviewed and reaches the base in
the run's merge. Two branches that settle one finding differently conflict
when the second merges, and Git reports it. These are the only writes to a
review after its verdict, and the only writes to a done item.

`caused_by` is checked for form only. The item that caused a bug may have
left the tree at the cutover, and then its ID or the commit is still worth
recording.

### `config.toml`

```toml
schema_version = 3
slug = "atelier"
base_branch = "master"

[[merge_check]]
run = "cargo fmt -- --check"

[forge]
provider = "github"
remote = "origin"
ci_workflow = "ci.yml"   # optional; without it nothing waits for CI
```

`merge` reads this file from the base branch's tip. `slug` is lowercase
letters, digits and single inner hyphens. `merge` always makes a merge
commit, so there is no merge strategy to set. `provider` has one value,
`github`. `merge_check` and `forge` may be left out. An unknown key is refused with its name.

### IDs

Missions and items keep today's form, `<slug>-<4 characters>`. `show`
accepts either kind.

## Status

Status is a stored field. One table in `atelier-app` lists every act: the
command, the role that may run it, the statuses it may run from, the status
it sets, and its conditions. Each command and the `status` view read that
table, so what `status` says a thing needs is what the command will check.

| Act | Role | From | To | Conditions |
| --- | --- | --- | --- | --- |
| `ready` (item) | Planner | `draft` | `ready` | none |
| `ready` (mission) | Planner | `draft` | `ready` | none; changes no item |
| `start` | Implementer | `ready` | `building` | blockers done; its mission, if any, is ready; no branch `<id>` exists; the base's record is not dropped |
| `start`, taking over | Implementer | `building`, `in_review`, `changes_requested`, `approved` | unchanged | branch `<id>` exists |
| `start --new` | Implementer | none | `building` | none; the item is created on its new branch |
| `submit` | Implementer | `building`, `changes_requested`, `approved` | `in_review` | Done when ticked; Report written; each open fix-now finding has a line in Responses or a decision; no unanswered question |
| `review` | Reviewer | `in_review` | unchanged | the caller is not in the item's `implementers` |
| `verdict changes` | Reviewer | `in_review` | `changes_requested` | the file has an open fix-now finding |
| `verdict approve` | Reviewer | `in_review` | `approved` | the file has no open fix-now finding |
| `verdict` on a mission review | Reviewer | any | unchanged | as above, for the file |
| `escalate` | Implementer | `building`, `changes_requested`, `approved` | unchanged | none |
| `decide` | Decider | any started status | unchanged | an unanswered question exists; the caller is not in the item's `implementers`, nor in the `reviewers` of the review that raised the finding |
| `merge` | Implementer | `approved`, or `building` with `--trivial` | `done` | the merge checks; the base's record is not dropped |
| `close` (item) | Any | `draft`, `ready` | `done` | a result text; no branch `<id>` exists |
| `close` (mission) | Lead or planner | `ready` | `done` | every item done or dropped; Result written |
| `drop` | Planner | any but `done` | `dropped` | a reason; written on the base, whatever the item's branch holds |
| `new --from`, `drop` on a finding | Planner on the base, or an implementer on its branch | a deferred finding not yet settled | unchanged | the finding's item is done |
| `lead` | Lead | a `ready` mission | unchanged | none |

Each act writes the status in the commit the act already makes: `submit`
commits the Report, `verdict` commits the review file, `merge` writes `done`
in the merge commit. No act makes a commit only to change a status.

Two facts are checked live and never stored:

- **The approval covers the head.** Since the reviewed commit, nothing
  changed outside the item's own directory, and the item's Goal, Done when
  and Scope are unchanged. So a Report, a response or a review file never
  makes an approval stale, while code, `config.toml`, a mission's documents,
  another item or the item's own requirements do. If it is stale, `status`
  and `merge` say to run `submit`.
- **The merge checks pass on the head.** The configured commands, and CI
  where the forge names a workflow.

## Threads and roles

A thread's identity is `T3CODE_THREAD_ID` when it is set, else the harness's
session ID (`CLAUDE_CODE_SESSION_ID`, `CODEX_SESSION_ID`,
`GROK_SESSION_ID`), else `human`.

Entry commands store the identity in the record: `start` appends it to the
item's `implementers`, `review` to the review file's `reviewers`, and `lead`
sets the mission's `lead`. `decide` stores it with the answer. The last entry
of a list holds the role now. The earlier entries are why a thread that once
implemented an item can never review it.

There is no local state file. A command works out who is calling by looking
for the caller's identity in the records it can see: the item of the current
branch, an open review of that item, and missions on the base. A thread may
hold several things, and `status` lists each.

Harness hooks give orientation at session start: `atelier setup claude`
exists, and `atelier setup codex` is added, since Codex 0.167 has hooks. The
session-end hook runs `atelier check`, which also reports record changes
not yet committed. Where no harness hook runs, a
project's `AGENTS.md` says to run `atelier status` first, and the
post-checkout Git hook prints it on every branch switch.

## Where records are read and written

- **A started item is found by its branch.** An item is started when a local
  branch named for its ID exists and is not merged into the base. Its record
  is then read from that branch's tip, wherever the reader stands. Once the
  branch is merged, the base's record is the item's record. Nothing is copied
  between branches.
- **A terminal status on the base wins.** If the base's record is `dropped`
  or `done`, that is the item's status whatever its branch holds. This is how
  a started item is cancelled. Every command checks the base's record of its
  item first, including `status` and `merge` run on the item's own branch.
- **Who reads this way:** `status` on the base, `lead`, `list`, `show` and
  the site. The site already enumerates local branches for work in flight.
- **Starting is one atomic step.** `start` writes the commit that sets
  `building` first, with no branch, and then creates branch `<id>` pointing
  at it. Git creates a branch atomically, so a branch named for an item
  always holds its building record, and an interrupted `start` leaves no
  branch. When two threads in separate checkouts start the same item, one
  creates the branch and the other finds it and is told who holds it. Its
  `start` is then the take-over, which it runs again only if it means to.
- **One clone.** All of this coordinates the threads in one clone and its
  worktrees. Two clones on different hosts are not coordinated.
- Apart from that check, a command on a work branch reads and writes that
  branch's `.atelier/`.
- The pre-push hook applies one rule to each new commit on the base's
  first-parent line in a push of the base: it is a merge made by
  `atelier merge`, known by the trailer that command writes in its message,
  or it touches only paths under `.atelier/missions/` and `.atelier/items/`.
  What a merge brings in on its second parent is not examined. A commit
  that is not such a merge may not set `done` on an item whose branch
  exists.
- Records can be read at any commit, without a checkout. The site reads
  this way, and so does anything that compares a record now with the same
  record at the commit of an earlier act.
- A plan reviewer works in its own detached worktree so two reviewers stay
  blind to each other. Review files have distinct names, so the planner
  cherry-picks both onto the base without a conflict.

## Merge

1. Read `config.toml` from the base tip.
2. Check the approval covers the head, or that `--trivial` was given on an
   item with no review.
3. Check no Decisions entry is unanswered.
4. Build the merged tree in a temporary checkout and run each merge check.
5. Where a forge is configured: push the branch and open its pull request,
   if they are not there yet. Where it names a CI workflow: if that workflow
   has not finished on the head, print the pull request and stop with an
   exit status of its own, distinct from a refusal; if it finished and did
   not pass, refuse.
6. Make one merge commit on the base that sets the item `done`.
7. Where a forge is configured: push the base.

`merge` never waits and has no flag but `--trivial`. Run again, it picks up
where the facts now stand. Waiting is the harness's work.

## The log

`atelier log` prints the history of acts. Nothing stores it. It is derived
from Git history, by finding the commits where a record changed in one of
the ways below. Each is one line, and each carries the commit it was found
in. `atelier-app` returns the same history as data, which is how the site
reads it.

| Line | Found where | Thread shown |
| --- | --- | --- |
| start, take-over | An item's `implementers` gains an entry | That entry |
| submit | An item's status becomes `in_review` | The item's holder |
| review, take-over | A review file appears, or its `reviewers` gains an entry | That entry |
| verdict | A review's verdict is set, on an item or a mission | The review's holder |
| escalate | A Decisions entry appears | The item's holder |
| decide | A Decisions entry gains its answer | The thread stored with the answer |
| merge | An item's status becomes `done` in an Atelier merge | The item's holder |
| settle | A finding gains `Taken` or `Dropped` | The thread in that line |
| lead | A mission's `lead` is set or changes | That entry |
| ready | A status becomes `ready` | None; the name given with `--by` |
| drop, close | A status becomes `dropped`, or `done` outside a merge | None |

A skip is shown on the line of the act whose commit carries its trailer,
which is every act where Atelier makes the commit. Today those are `start`,
`submit`, `verdict` and `merge`.

What `log` does not show: `new`, `set` and edits to a record's text, which
Git's own log has, so an item's time as a draft is read from its
`created_at` and not from `log`; the executing thread of `ready`, `drop` and
`close`, which no record holds; and a skip in a Git hook or on a command
that makes no commit, which leaves nothing in Git.

`log` sees commits, not commands. Acts on different records committed
together appear as separate lines with one time. Two acts on one record in
one commit appear as their net change: an item made ready and closed in the
same commit shows only as closed. An act undone before it was committed does
not appear.

The walk covers the base's history, the second-parent side of each Atelier
merge, and the tips of started items' branches, so an item's acts stay
visible after its branch is merged and deleted. Today's history reader for
the debrief follows first parents only and would lose them; how to walk
both sides without reading each record's history separately is settled when
the command is planned.

`log` adds nothing up. Time in each status from `ready` on, rounds per item,
roles taken over, escalations and skips are counts over its lines, and the
gardener's procedure, not the binary, says which to take.

## Hooks and the bypass

`atelier setup` installs three Git hooks into the clone's hooks directory,
which all its worktrees share. Each is a one-line script that runs
`atelier check --hook <name>`, so the rules live in the binary.

| Hook | Rule |
| --- | --- |
| pre-commit | Every staged record parses and validates: a `mission.md`, an `item.md`, a review file or `config.toml`. Other files under `.atelier/`, such as a mission's documents and an item's artifacts, are not examined. Where a record's status differs from the one in `HEAD`, the act table lists that change. The conditions of an act are the command's to check, not the hook's. |
| pre-push | The base rule above, for each new commit when the push updates the base. |
| post-checkout | None. On a branch switch it prints what `status` prints. |

`ATELIER_SKIP`, when set to a non-empty reason, lets any rule of a hook or a
command pass. The tool prints the rule and the reason. Where Atelier makes
the commit, a trailer `Atelier-Skip: <rule>: <reason>` goes into its
message. A skip in a hook writes nothing; it shows in the transcript as the
variable on the Git command, which is where the scorecard counts it. A
record that does not parse is not a rule and cannot be skipped. `status`
reports a clone with no hooks.

## The forge

`crates/atelier-app/src/forge.rs` is copied into the new workspace with one
change to its interface: it takes the forge section of the new config, whose
CI workflow is optional, where today it takes `workflow_policy::ForgeConfig`
with a required workflow name. Its behaviour stays, and its contract tests
come with it. The merged-tree checkout, the merge commit and the push live
today in `atelier-cli`'s `merge.rs` and `merge_push.rs`, bound to
`WorkflowPolicy` and `RecordIndex`. They are copied and put on the config
and item types, and their contract tests are rewritten against the new
command.

One more thing is kept, because it is the kind a rewrite loses by accident.
Today's merge protects itself against the world moving while its checks run.
It takes the lock that serialises Atelier's writers. After the checks, and
before it writes anything, it confirms that the branch tip and the base tip
are the ones the checks ran on and that the checkout has not changed, and it
refuses if any has. The new `merge` does the same, with the lock and that
confirmation copied from `merge.rs`, `mutation_lock` and
`committed_snapshot.rs`, and with tests that move the base, move the branch
and run two merges at once. The parts of that code that guard against
tampering with file modes and index flags are not carried over.

Nothing else from today's merge is carried over: not the gate code, the
polling in `merge_wait.rs`, nor `merge_sync.rs`.

## Site

The site is rebuilt on the new records in mission 3, in the new workspace:
the site crate and its web app are copied across, the loader is written to
read the three record kinds, the responses drop what the new model does not
have, and the fixtures are generated in the new layout. The new binary's
`serve` serves it, and the web typecheck, unit and Playwright jobs run
against it. How much of the brief and debrief view code is copied is settled
when that step is planned, by reading the site's code.

The old workspace's site is not touched, and the installed binary keeps
serving the old-layout repositories until the cutover.

The site can derive more than this work builds. A view of what changed
between a mission's go-ahead and its result is the dashboard's to compute,
and the tool gives it what that needs: every act with the commit it happened
in, and records readable at any commit. So the site can load a mission's
directory and items as they stood at the commit of its `ready` act and
compare them with how they stand now.

## Transition

The installed binary is one per host and the installed skill is one per host,
so the process changes for every repository on a host at once. The transition
is one step per host, and it has four parts. The first three are ordinary
items under the old tool. The fourth is a runbook.

**1. Build beside (mission 2).** The installed binary tracks all work,
including the redesign's own, and the repository's records stay in the old
layout. The new tool is written in `next/`. The old workspace, its tests,
CI's `atelier check` step and `scripts/smoke-repos.sh` are untouched.

**2. Rehearse (mission 2).** `scripts/cutover/convert_records.py` reads a
repository's old `.atelier/` directly and writes the whole new one: missions
and items from open records and the strategy files they link, and
`config.toml` from `workflow.yaml`, with no CI workflow where the repository
has no Actions. Done and superseded records leave the tree, and Git keeps
them; deferred findings on them are not carried over, and the backlog sort is
where any worth keeping become items. Run twice on the same input it writes
the same bytes, and `--verify` checks a converted tree against a fresh run.
The rehearsal converts a scratch clone of each of the five repositories and
runs the new binary's `check` and `status` on each. It runs one full item in the
atelier clone and in one product clone; a full item runs the repository's
merge checks, and some of those are too heavy to run for all five here.

**3. Prepare (mission 3).** Each of these is reviewed and merged under the
old tool:

- The skill is rewritten in `skills/agent-factory-next/`, beside the live
  one. The live skill is a checkout that follows master, and the installed
  binary tells every session to move it forward when it falls behind, so the
  new text must not sit at the live path before the cutover.
- The fallback release is fixed. It is the binary CI and the rollback use,
  so it must check this repository's records and serve the site as the
  installed binary does. The installed binary is six commits past 0.4.0,
  and those commits changed only the wording of the review brief, so the
  0.4.0 release is tried first. A 0.4.1 is cut only if 0.4.0 fails that
  test.
- The site is rebuilt, as above.
- The switch: `crates/`, the old workspace with all its tests, and
  `scripts/smoke-repos.sh` are deleted, and the tracked contents of `next/`
  move to the repository root, so the root workspace, the release build and
  `cargo install --path crates/atelier-cli` are all the new tool. The move
  also touches the root manifest and lock file, the release profile and
  metadata, CI's two sets of jobs and the web paths in the release workflow;
  the full list is made when the switch is planned. CI's
  `check` step runs the fallback release while the repository holds
  `.atelier/issues/` and the tree's binary once it does not. `AGENTS.md`
  says the installed binary stays the tracker and nobody installs from the
  tree. Added under `scripts/cutover/`, so that they are reviewed here: the
  runbook, the new `AGENTS.md` text for each repository, and a replacement
  for each workflow that runs Atelier in a repository's CI, installing it
  from this change's commit. Astromech's workflow installs Atelier from a
  commit today and runs `doctor` and `land`, which will not exist.

After the switch merges, the tree's binary cannot read this repository's
records, and only the installed binary tracks work. The cutover follows at
once.

**4. Cut over (mission 3).** One thread runs `scripts/cutover/cutover.sh`,
which stops at the first failure. The cutover item is never started: it stays
ready, the runbook runs on the base, and the item is closed at the end.

1. Confirm no item is in progress in any of the five repositories and no
   other thread is running in them. Record each repository's base commit,
   and copy each clone's Git hooks directory aside.
2. In atelier, on master: run the converter, replace `AGENTS.md` with the
   reviewed text, and move `skills/agent-factory-next/` to
   `skills/agent-factory/`. Commit. Run `--verify`, and the tree's
   `atelier check`.
3. Install the binary from master, run `atelier setup` for each harness and
   for Git hooks, move the skill checkout forward, and restart the
   `atelier-serve` service. Check the site answers for atelier.
4. Push master, with `ATELIER_SKIP="cutover conversion"`, since the hook
   just installed would refuse a commit that is not a merge and changes
   `AGENTS.md` and the skill. CI now runs the tree's binary on the converted
   records.
5. In each of the other four repositories: convert, replace the process text
   in `AGENTS.md` and any workflow that runs Atelier with the reviewed ones,
   commit, run `--verify` and `atelier check`, run `atelier setup` for Git
   hooks, and push with the same skip.
6. Close the cutover item with `atelier close`, the first act recorded by the
   new tool.

The conversion commits are pushed directly, by plain `git push`. They are the
output of a reviewed script, checked by `--verify`, and reviewed text moved
into place. This is the one time a commit that is not an Atelier merge
changes more than planning records on a base, and the runbook is the only
thing that does it.

**Rollback.** The runbook undoes its own work in this order:

1. If step 3 had started, put each clone's Git hooks directory back as the
   cutover's step 1 copied it. That removes the hooks the new `setup`
   installed and restores any hook the clone had before. This comes first:
   the new hooks would refuse the pushes below, and once the fallback is
   installed they would call commands it lacks.
2. Undo each repository from what step 1 of the cutover recorded:

   | A repository that is | Is undone by |
   | --- | --- |
   | Converted and not pushed, committed or not | Resetting it to its recorded base commit and cleaning its working tree |
   | Converted and pushed | Reverting its conversion commit and pushing |

3. If step 3 had started, reinstall the fallback release, run its `setup`, move
   the skill checkout back to the commit it was on, restart `atelier-serve`,
   and check the site answers.

The rollback is tried on scratch clones before the real cutover, stopping
after each of these: a conversion that fails `--verify` before its commit;
step 2's commit; part-way through step 3, with the binary installed and the
hooks not yet; step 3; step 4; and a second repository's conversion in step 5
that is committed and not pushed. One of the scratch clones has a hook of
its own beforehand, and the rehearsal checks it is back afterwards.

**After the cutover**, the fallback branch of CI's `check` step is removed.
Nothing else is left to clean up, since the old workspace went whole at the
switch. `scripts/cutover/` is deleted after the work host converts.

**The work host** converts later by the same runbook, run by Sumeet. Until it
does, it must not fetch a repository that has been converted.

## Testing

- One scenario test per lifecycle path, driving the real binary in a scratch
  repository: the plain path; changes requested and fixed over two rounds; a
  rebuttal withdrawn and one kept; an escalation decided as defer; a trivial
  merge; an item with no files; a mission from draft to closed; an
  implementer taken over in `approved`; a reviewer taken over mid-round; a
  former implementer refused as reviewer and as decider; two threads
  starting one item from separate checkouts; a started item dropped on the
  base, then refused at merge.
- Unit tests on the act table: every act from every status.
- `log` on the plain-path scenario prints its acts in order with the right
  threads, and still does after the item's branch is merged and deleted; on
  the take-over scenario it shows both holders.
- `new --from` on a deferred finding of a done item writes `Taken:` with the
  thread, and `list --deferred` no longer shows it; settling it again is
  refused; settling a finding of an item that is not done is refused.
- Hook scenarios in a scratch repository: a commit of an invalid record
  refused; a commit of a status change the act table does not allow refused;
  a push of the base holding a plain commit that touches code refused, and
  one holding only planning commits allowed; a push of the base whose new
  commit is an Atelier merge of a branch with ordinary code commits allowed;
  the refused push allowed with `ATELIER_SKIP` set, and the skip printed; a
  branch switch printing `status`.
- `merge` with CI still running: it stops with its "not yet" exit status and
  changes nothing; run again after CI passed, it merges; run again after CI
  failed, it refuses.
- A fresh-agent scenario, run by the eval harness: an agent given only "run
  `atelier status`" takes a ready item to done. It is run with Claude Opus
  and with a Codex model.

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
| A plan as its own branch | Grooming touches many items and planning branches collide |
| `land`, a check an agent had to run | Agents forgot it, or hid its refusal behind a pipe; a pre-push hook runs without being remembered and fails the push itself |
| Waiting for CI inside `merge` | A command that sleeps for minutes fights the harness, and one network timeout failed it |

## Decisions

Sumeet made each of these in conversation on 2026-11-09.

| Decision | Reason |
| --- | --- |
| Records stay in the working tree of the branch that does the work | It is the tool's core value; a host database would be simpler than any halfway design |
| Status is a stored field | Every stage change is an act by a known role |
| The implementer merges its own approved item | The reviewer is already the unbiased judge; a third agent adds a relay |
| The harness starts agents, by whoever holds the status | T3 Code and Claude Code manage subagents well and change fast |
| Review and readiness are separate facts | A mission may sit on the base as a draft, reviewed or not |
| Planning commits go on the base | Decided 2026-10-15 in atelier-3mjv, and kept |
| The workflow file goes | See Tried and removed |
| No backward compatibility | The design should be the best one, not the nearest one |
| Proof is the configured merge checks plus manual validation with artifacts | Work has no CI; routine receipts were busywork |
| The line count is an indication, not a limit | See Tried and removed |
| The ADRs are replaced by one design record | They carry history that agents read as current |
| The tool is rewritten in full, in a fresh workspace | A gradual rewrite has no benefit once the record layout changes under every command |
| A view of what changed since his go-ahead is the dashboard's to derive | It is a derived view; this work supplies the acts with their commits and records readable at any commit |
| Hooks guard, commands act | A rule in a command applies only when the agent remembers the command |
| The tool gives facts and the skill does the arithmetic: `log` and `list`, and no scorecard command | A fixed report in the binary would change every time the measures do, as the debrief did |
| The gardener is a role, and its needs shape the records now | A bug's cause and a deferred finding's settlement are cheap in the first format and costly to add later |
| Every rule can be skipped on purpose, with a reason | Atelier's rules are not hard blockers, and a bypass should be intended and leave a trace |
| `merge` never waits | Waiting belongs to the harness, which already wakes a thread when a pull request's checks finish |
| His go-ahead stays `ready --by` | A machine account for agents would let GitHub carry it, and his employer's sign-on rules make one too hard to get |

Three points in this design go beyond what he confirmed, and his approval
should weigh them:

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

- Which Codex hook event runs at session start.
- Whether T3's pull request watch wakes a thread when a repository marks no
  check as required.
- Where video artifacts go when a project needs them.
- The exact flags of `set` and `list`.
- How much of the brief and debrief view code the site keeps.
