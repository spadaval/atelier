---
status: draft
last_changed: "2026-11-11: amended at Sumeet's direction: no site crate and no `serve`; the Desk (ADR 0032) reads the core. Before that: brought up to date with what mission atelier-fcb7 built in next/, including Sumeet's decisions of 2026-11-10 that submit starts CI and merge waits for it; what the binary enforces of the Review section, and the fixes the validator's check found (atelier-8myy)"
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
| Missions, brief, debrief, criteria | 4,753 | Not carried over. The Desk adds the views its pages need (atelier-z4cr) |
| Review and findings | 4,669 | Written new, as `review` and `verdict` over one file per review |
| Site, Rust and web | 4,926 | Not carried over. Deleted with the old workspace; the Desk replaces it (ADR 0032) |
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
lock file, built to `next/target/`. It has four crates, those of the old
one less `atelier-workflow` and `atelier-site`, under the same names:

| Crate | Owns |
| --- | --- |
| `atelier-core` | Domain values: Mission, Item, Review, Finding, Status, Thread, Config |
| `atelier-records` | Reading and writing the three record kinds and `config.toml`, from a checkout or from a commit; IDs; validation |
| `atelier-app` | Use cases: the status view, each lifecycle act, the merge checks, cross-branch reads, the history of acts, the views the Desk reads; a repository opened by path (`Repo`) and the Git calls it and the acts here make; `ready` and `decide` as functions that return what they did or a refusal as data; the refusal and skip policy, and the wording of the refusals and results of the acts here |
| `atelier-cli` | Arguments, dispatch, the other commands' Git and shell calls, on `atelier-app`'s Git helpers, and the forge; output |

The crates sit at `next/crates/<name>/`, so the switch can move them to the
root `crates/` without changing a path inside them. The dependency direction
is `cli` to `app` to `records` to `core`. The Desk's crate, `atelier-desk`,
depends on `app` and `records`, and nothing depends on it.

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
status: draft           # draft ready done dropped
labels: []
lead: <thread>          # set by lead
ready: {by: sumeet, at: <time>}
created_at: <time>
```

The mission lives in a directory named for its `id`. Sections are `##`
headings: Why, Outcome, Success criteria (optional), Scope, Authority,
Result; Outcome is required. `drop` writes `Dropped: <reason>` first under
`## Result`, as on an item, and `MissionDir::dropped` reads it. Its plan
reviews are review files under `reviews/`, read and validated as an item's
are. Any other file in the directory is a planning document with no
required shape, and is not read.

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
the checkout the command runs in, like any record write: on the base, or on
the branch of a gardener's run, where the staleness rule counts it as a
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

Status is a stored field. One table in `atelier-app` (`acts::TABLE`) lists
every act: the command, the role that may run it, the statuses it may run
from, the status it sets, and its conditions. Each command and the `status`
view read that table, so what `status` says a thing needs is what the command
will check.

| Act | Role | From | To | Conditions |
| --- | --- | --- | --- | --- |
| `ready` (item) | Planner | `draft` | `ready` | none |
| `ready` (mission) | Planner | `draft` | `ready` | none; changes no item |
| `start` | Implementer | `ready` | `building` | blockers done; its mission, if any, is ready; no branch `<id>` exists; the base's record is not dropped |
| `start`, taking over | Implementer | `building`, `in_review`, `changes_requested`, `approved` | unchanged | branch `<id>` exists; its mission, if any, is not dropped |
| `start --new` | Implementer | none | `building` | none; the item is created on its new branch |
| `submit` | Implementer | `building`, `changes_requested`, `approved` | `in_review` | Done when ticked; Report written; each open fix-now finding has a line in Responses or an unanswered question about it; no unanswered question. After its commit, where a forge is configured, it pushes the branch and opens its pull request; a failure of either is a warning, and `merge` does it |
| `review` | Reviewer | `in_review` | unchanged | the caller is not in the item's `implementers` |
| `verdict changes` | Reviewer | `in_review` | `changes_requested` | the file validates; it has an open fix-now finding; it keeps what decisions settled, as the tool wrote it |
| `verdict approve` | Reviewer | `in_review` | `approved` | the file validates; no fix-now finding of the item is open, the file's or one it does not carry; it keeps what decisions settled |
| `verdict changes` on a plan review | Reviewer | any | unchanged | the file validates; it has an open fix-now finding; no finding has a `Decided:` line |
| `verdict approve` on a plan review | Reviewer | any | unchanged | the file validates; no fix-now finding is open among the file's own and those of earlier rounds; no finding has a `Decided:` line |
| `escalate` | Implementer | `building`, `changes_requested`, `approved` | unchanged | none |
| `decide` | Decider | any started status | unchanged | an unanswered question exists; the caller is not in the item's `implementers`, nor, for the oldest unanswered question when it names a finding, in the `reviewers` of the review that raised that finding or of any review whose finding it carries |
| `merge` | Implementer | `approved` | `done` | a review of the latest round approves it and still covers the head; no unanswered question; nothing uncommitted in tracked files or the item's directory; the base's record is not dropped |
| `merge --trivial` | Implementer | `building` | `done` | Done when ticked; Report written; no review file; no unanswered question; nothing uncommitted in tracked files or the item's directory |
| `close` (item) | Any | `draft`, `ready` | `done` | a result text; no branch `<id>` exists |
| `close` (mission) | Lead or planner | `ready` | `done` | every item done or dropped; Result written |
| `drop` | Planner | `draft`, `ready`, and an item's started statuses | `dropped` | a reason, on one line; written on the base, whatever the item's branch holds |
| `new --from`, `drop` on a finding | Planner on the base, or an implementer on its branch | `done`, the finding's item | unchanged | a review of the item has the finding; it is deferred; it is not settled |
| `lead` | Lead | a `ready` mission | unchanged | none |

Each act writes the status in the commit the act already makes: `submit`
commits the Report, `verdict` commits the review file, `merge` writes `done`
in the merge commit. The planning acts that change a stored status,
`ready`, `drop` and `close`, commit it themselves, in a commit holding only
the records they wrote, since other threads read a status at the base's tip.

How the table is built:

- A row joins the table with its command. The table holds the rows of
  `ready` on an item and on a mission, `start`, `start --new`, `start`
  taking over, `submit`, `review`, both verdicts, `escalate`, `decide`
  and `merge` with and without `--trivial`, `lead`, `drop` (one row, for an
  item and a mission), `close` on an item and on a mission, both verdicts
  on a plan review, and settling a finding (`Act::Settle`), whose finding's
  conditions `acts::check_settle` checks after the row's. A mission's row is checked with `acts::check_mission`, from
  the mission's status and, for `close`, the status of each item that names
  it, each found as `Located::record` finds it, and its Result; for a
  verdict, from the caller's plan review. `start --new` runs from no
  status, since it creates its item, and takes the status it sets from its
  row. `close` on an item and on a mission needs no role, like a planner's
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
  thread holds the planner's role, so a planner's act (`ready`) needs
  nothing held and is never refused for a role the caller holds.
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
  unmet. The mission counts as ready only when its record at the base's tip
  says `ready`; a mission that is not there is unmet.
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
`review` and `decide` say which subagents may be used, and to start one with
fresh context, not as a fork.

That rule is the command's. A program that links `atelier-app`, such as the
Desk, names the caller itself when it opens a repository (`Repo::open`),
and that name is the identity the records get: an answer it writes for
`sumeet` reads `Answer by sumeet:`, where the command run by a person with
no harness variable set is `human`. `ready`'s `by`, the person who said go,
is a parameter of the function, which `ready --by` fills. A name the records
cannot write and read back as it is, one with a line break or a `: `, is
refused when the repository is opened, before anything is written.

Entry commands store the identity in the record: `start` appends it to the
item's `implementers`, `review` to the review file's `reviewers`, and `lead`
sets the mission's `lead`. `decide` stores it with the answer. The last entry
of a list holds the role now. The earlier entries are why a thread that once
implemented an item can never review it.

There is no local state file. A command works out who is calling by looking
for the caller's identity in the records it can see: the item of the current
branch, an open review of that item, and missions on the base. A thread may
hold several things, and `status` lists each. Built so far: on a branch named
for an item, `status` speaks to the item's holder as its implementer, to a
thread that reviewed an earlier round as done with it, and to anyone else as
a visitor, naming the holder; anywhere else it lists the
started items the caller holds, with `start <id>` to return to one, the
ready items `start` would accept now (`Entry::startable`, as `list --ready`
reads it), one line counting the other ready items by what stops them (a
blocker not done, a mission not ready, a branch of their name) that names
`list`, and the started items others hold. Holding nothing, it ends with
`start` on an item that can start, or says no ready item can start now and
names no `start`. On the base branch it also
lists the record changes the checkout has not committed. Away from an
item's branch it also lists the missions in progress and, apart from the
ready items, the items labelled `needs-human` that nobody has started, as
waiting on a human, and the items dropped on the base while their branch
is there, each with its reason. A thread that is the `lead` of a ready
mission at the base's tip is told so, with each of the mission's items by
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
item's mission, when the mission is ready and its lead may decide it, else
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
escalation is required after three rounds without an approval.
Missions join
with their commands.

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
refused with nothing changed. The merging is copied from the old `setup
claude`. Where no harness hook runs, a project's `AGENTS.md` says to run
`atelier status` first, and the post-checkout Git hook prints it on every
branch switch.

## Where records are read and written

- **A started item is found by its branch.** An item is started when a local
  branch named for its ID exists and is not merged into the base, meaning
  its tip is not in the history of the base's tip. Its record is then read
  from that branch's tip, wherever the reader stands, except on that branch,
  where it is read from the checkout so that uncommitted ticks count. Once
  the branch is merged, the base's record is the item's record. Nothing is
  copied between branches. A branch named for an item whose tip holds no
  record of it starts nothing, and blocks `start` until it is deleted.
- **The base's record is the one at the base branch's tip** (`base_branch`
  in the checkout's `config.toml`), read through the commit reader, never the
  base checkout's working tree. A planning edit counts once it is committed.
  `start` reads its item, the item's blockers and its mission this way.
  `ready` commits what it writes, so an item or mission it made ready is
  ready at the tip at once.
- **Planning is read and written in the checkout of the base.** `new`,
  `set`, `ready`, `drop` and `close` run only on the base branch, and are
  refused elsewhere, naming the checkout that has the base. `new --from`
  and `drop` on a finding also run on the branch of a started item whose
  last implementer is the caller; there they read and write that checkout
  the same way, and their output says the write reaches the base in that
  item's merge and makes an approval of it stale. They read the checkout's records, with an
  item's branch found as above, so uncommitted planning counts and a started
  item is still seen as started. Each validates the records it writes against
  every record of the checkout and writes nothing when one of them has a
  problem; the pre-commit hook validates them again as they are committed.
  `ready`, `drop` and `close`, which change a stored status that other
  threads act on at the base's tip, and `lead` (below), commit what they
  write themselves, with one shared piece of code: one commit holding only
  the paths they wrote, each as it stands in the checkout, which leaves
  every other staged, changed or untracked file as it was, and they say the
  commit they made. `new`, `set` and settling a finding commit nothing,
  since a draft is edited several times and a settlement on a gardener's
  branch is committed with its run; each prints the paths it wrote and ends
  with a `Then:` line giving the Git command that commits those paths
  alone. `new` picks an ID
  that no item or mission has in the checkout, at the base's tip, or as a
  local branch, and writes a draft with its section headings and nothing
  under them; an item's priority starts as `medium`.
- **`drop` and `close`.** `drop` writes the base's record, the one in the
  base's checkout, whatever the item's branch holds. An item that is only
  on its branch, as after `start --new`, gets its branch's front matter
  and its Goal, Done when and Scope written there, under which its Report
  holds the reason; its reviews, the rest of its Report and its Decisions
  stay on the branch, so nothing on the base names a review it lacks. Once that is committed, every command reads it first, so a
  started item's `submit`, `merge` and take-over are refused with its
  reason. Dropping a mission changes no item; `start` on one of its items
  is refused, saying the mission was dropped. `close` on an item with a
  branch is refused, naming `merge` as the way it ends. `close` on a
  mission counts its items as the planning commands find them: in the
  base's checkout, or on their branches.
- **`ready` on a mission says what review it had.** It prints whether the
  plan has a review, each review of the latest round with its verdict, and
  the paths of the plan (the mission's directory less its reviews, and the
  directories of the items in the mission now or at the commit reviewed)
  that differ in the checkout from each commit reviewed. It never refuses
  for any of that.
- **`set`'s flags.** `set <id>` takes `--title`, `--priority`,
  `--labels a,b`, `--mission <id>`, `--blocked-by a,b` and
  `--caused-by <item or commit>`, at least one. A list flag replaces the
  list, and an empty value clears a list or an optional field. A mission
  takes only `--title` and `--labels`; an item's other flags on a mission are
  refused. It changes a draft or ready record and is refused for a started
  item, naming its branch, and for a done or dropped one. `ready <id>...
  --by <name>` checks every named ID first and writes nothing when any is
  unknown or not a draft, naming each.
- **Who decides what.** `atelier-app` holds the rule, as `Located::record`:
  the base's record when it says done or dropped, else the unmerged branch's,
  else the base's. `atelier-app`'s `Repo` gathers what the rule reads: the
  base's tip, each local branch whose name is an item ID, whether it is
  merged, and its record. A program that links `atelier-app`, such as the
  Desk, opens a `Repo` by path; `atelier-app` reads no environment variable
  to find the directory, the caller or a skip's reason, which its caller
  gives it.
- **A terminal status on the base wins.** If the base's record is `dropped`
  or `done`, that is the item's status whatever its branch holds. This is how
  a started item is cancelled. Every command checks the base's record of its
  item first, including `status` and `merge` run on the item's own branch,
  and then does not read the branch's record, so one that does not parse
  cannot hide the drop. `check` still reports it.
- **Who reads this way:** `status` on the base, `lead`, `list`, `show` and
  the Desk. `show` reads a mission and its plan reviews at the base's tip.
  `lead` reads its items and the missions this way, so what it says can
  start now is what `start` will start, and reads and writes its own
  mission in the checkout of the base, as `set` does: it runs only on the
  base branch, refuses a mission that is not ready there, sets the caller
  as `lead`, and commits `mission.md` alone, since `status`, a question's
  handoff and `log` read the lead from the base's tip. A thread that already
  leads the mission is shown it and writes nothing.
- **`list`'s flags.** With none, every item that is not done or dropped,
  under its mission and then under its sorted labels, and every such
  mission, even with no item. `--ready` keeps items whose `start` has no
  unmet condition; `--blocked` items with a blocker that is not done;
  `--needs-human` items with that label; `--mission <id>` that mission's
  items; `--label <l>` items, and missions, with the label; `--mine` items
  the caller holds as implementer or as the round's reviewer, and missions
  it leads with their items; `--done` turns all of this to done and dropped
  items and missions. Flags narrow together; with `--ready`, `--blocked` or
  `--needs-human` a mission is shown only for an item kept under it.
  `--deferred`, which takes no other flag, lists instead each unsettled
  deferred finding of a done item, by its name and title; it reads the
  items of the checkout it runs in, committed or not, as `new --from` and
  `drop` do, so it lists what they would settle there.
  Every command also reads its item, and each blocker of a ready item, this
  way.
- **Starting is one atomic step.** `start` writes the commit that sets
  `building` first, with no branch, and then creates branch `<id>` pointing
  at it. Git creates a branch atomically, so a branch named for an item
  always holds its building record, and an interrupted `start` leaves no
  branch. When two threads in separate checkouts start the same item, one
  creates the branch and the other finds it and is told who holds it. Its
  `start` is then the take-over, which it runs again only if it means to.
  The commit is built on the base's tip through a temporary index
  (`read-tree`, `update-index`, `write-tree`, `commit-tree`), so no checkout
  changes, and the branch is created with `git update-ref refs/heads/<id>
  <commit> ""`, which fails when the branch exists. Only then does the
  checkout switch to it. Debug builds can be held between the two steps
  through `ATELIER_TEST_HOLD_BEFORE_BRANCH`, which the tests of an
  interrupted start and of two starting threads use. `start --new` starts
  the item it creates the same way, in one commit holding only its record.
- **Taking over** switches the checkout to the branch, appends the caller to
  `implementers` and commits `item.md` alone, leaving the status as it is. A
  branch checked out in another worktree cannot be switched to; `start`
  then changes nothing and says to run it there. The holder running `start`
  again only switches.
- **`submit` commits the item's directory** and nothing else in the
  checkout.
- **A question and its answer are written where the implementer works.**
  `escalate` runs on the item's branch, and `decide` only in the checkout
  on it, refused anywhere else, naming the checkout that has the branch.
  Each writes `item.md` there and commits nothing, so `submit` commits the
  question and its answer with the item; an approved item's implementer
  commits them before `merge`, which refuses anything uncommitted. A
  refused `escalate` or `decide` writes nothing.
- **A review is written where the implementer works.** `review` runs only
  in the checkout on the item's branch, and refuses anywhere else, naming
  it. It creates the round's file, or appends the caller to its
  `reviewers`, and commits nothing. `verdict` commits the review file and
  `item.md`, with its new status, in one commit, and nothing else in the
  checkout.
- **A refused commit changes nothing.** When `git commit` fails, as when a
  hook refuses it, `submit`, the take-over, `verdict`, `ready`, `drop`,
  `close` and `lead` put the files they wrote and the index under the paths
  they staged back as they were, removing a file no commit held, so the act
  can be run again.
- **One clone.** All of this coordinates the threads in one clone and its
  worktrees. Two clones on different hosts are not coordinated.
- Apart from that check, a command on a work branch reads and writes that
  branch's `.atelier/`.
- The pre-push hook applies one rule to each new commit on the base's
  first-parent line in a push of the base: it is a merge made by
  `atelier merge`, known by its two parents and by the `Atelier-Merge:`
  trailer that command writes in its message's last paragraph, or it changes from its
  first parent only paths under `.atelier/missions/` and `.atelier/items/`.
  What a merge brings in on its second parent is not examined. A commit
  that is not such a merge may not set `done` on an item while a local
  branch named for it exists.
- Records can be read at any commit, without a checkout, through
  `atelier-records`' `CommitReader`: one `git cat-file --batch` process
  serves every read, and it tells a path the commit lacks, an entry that is
  not a regular file and a failing Git apart. The Desk reads this way, and
  so does anything that compares a record now with the same record at the
  commit of an earlier act.
- **A plan review is written where its reviewer works.** `review <mission>`
  runs in any checkout not on an item's branch, where `verdict` would be
  the item's, and reviews the plan as that checkout's HEAD holds it: the
  brief reads the mission, its documents and its items from the commit the
  review records, and names in one line the paths of the plan that the
  checkout or its index changes from that commit, which the review does not
  cover. A
  mission no commit holds is refused, with the Git command that commits
  it. It creates the round's file under `missions/<id>/reviews/`, or
  appends the caller to its `reviewers`, and commits nothing. `verdict`
  in a checkout on no item's branch records the verdict of the one plan
  review there that the caller holds without a verdict, and commits that
  file alone, changing no status. A plan reviewer works in its own
  detached worktree so two reviewers stay blind to each other. Review
  files have distinct names, so the planner cherry-picks both commits onto
  the base without a conflict.

## Merge

`merge` runs in the checkout on the item's branch. In order:

1. Read everything, without a lock: first the head, which every later step
   is checked against, then the item, and `config.toml` from the base tip.
   Check the act's conditions, as in step 4. Where a forge is configured,
   also refuse a branch that conflicts with the base, since CI does not run
   on a pull request that conflicts.
2. Where a forge is configured: push the branch when the forge's copy of it
   is missing or its tip is not a commit whose CI run would count for the
   head, as after a push that failed at `submit`; otherwise push nothing, so
   no run in progress is cancelled. Open the pull request when the branch
   has none. Where the forge names a CI workflow, wait for CI: ask the forge
   every 15 seconds, for up to 9 minutes. While no run that counts has
   finished, or the latest finished one failed and a newer one is running,
   keep asking; a question that fails counts as not finished, and one still
   unanswered at the bound is ended. Refuse as soon as CI fails; at the
   bound, stop with exit status 3 and say what the last answer said, or,
   when none came, why the first question failed. The wait holds no lock
   and writes nothing.
3. Take the merge lock, then read everything again, as in step 1. Nothing
   read before the lock is trusted, and a checkout now on another branch
   is refused, so `merge` merges only the item it waited for.
4. Check the act's conditions from the table: the approval covers the
   head, or `--trivial "<why>"` was given on a building item with no
   review file; no Decisions entry is unanswered; no tracked file has
   uncommitted changes and no file in the item's directory is untracked,
   ignored or not.
   The role is told from the checkout, as every command tells it; the rest
   is judged on the item as the head holds it. Where the base is checked out, that checkout must
   have none either, since merging moves it.
5. Build the merged tree with `git merge-tree`; a conflict refuses, naming
   the files. Where the forge names a CI workflow, ask once more, without
   waiting: a passing run must still count for the head as it now is. A
   question unanswered after a minute is ended, and `merge` stops with exit
   status 3, releasing the lock.
   Write the merge commit object on the merged tree, base tip first and
   branch tip second, with the item's record set `done` (and `trivial` set
   to the reason). Nothing points at it yet.
6. Check out that commit in a temporary worktree in the Git directory and
   run each merge check there with `sh -c`. A failure refuses, printing the
   command, its exit status and the last 20 lines of its output. When none
   failed, the output says, before the line that reports the merge, how
   many checks ran and that each passed, or that `config.toml` names none.
   A failing check `ATELIER_SKIP` passed has its skip line instead, and no
   line says the checks passed.
7. Confirm the branch tip, the base tip and both checkouts are what steps
   3 to 6 read; otherwise write nothing and say to run it again. Then move
   the base to the merge commit: a fast-forward of the base's checkout
   where one has it checked out, else `git update-ref` from the tip read.
8. Where a forge is configured: push the base.

`merge` has no flag but `--trivial`, and the bound, the time between
questions and the minute under the lock are constants; debug builds let
tests shorten them through `ATELIER_TEST_CI_WAIT`, the minute to no more
than the bound. The 9 minutes sit under Claude Code's 10-minute limit
on a command in the foreground; a harness with a shorter limit kills `merge`
during the wait, which leaves nothing behind. Run again, it picks up where
the facts now stand: on an item it already merged it only pushes the base
where a forge is configured, and says nothing of merge checks, which is what a run that stopped after step 7
left undone. Its exit status is 0 when it merged or had nothing left to do,
1 for a refusal, 2 for an error and 3 when CI has not finished by the bound.

The merge commit's message ends with the trailer `Atelier-Merge: <id>`,
which the pre-push hook knows an Atelier merge by. The branch is kept; it
is merged, so the base's record is the item's record from then on.

## The log

`atelier log` prints the history of acts. Nothing stores it. It is derived
from Git history, by finding the commits where a record changed in one of
the ways below. Each is one line, and each carries the commit it was found
in. `atelier-app` returns the same history as data, which is how the Desk
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
`submit`, `verdict`, `merge`, `ready`, `drop`, `close` and `lead`.

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
| pre-commit | Every staged record parses and validates: a `mission.md`, an `item.md`, a review file or `config.toml`. Other files under `.atelier/`, such as a mission's documents and an item's artifacts, are not examined. Where a record's status differs from the one in `HEAD`, the act table lists that change. The conditions of an act are the command's to check, not the hook's. |
| pre-push | The base rule above, for each new commit when the push updates the base. |
| post-checkout | None. On a branch switch it prints what `status` prints. |

`atelier check` reads every record in the checkout, tracked or not and
ignored or not, as the commands do, one file at a time
(`atelier-records`' `load`), so one that does not parse leaves the rest
read and validated. It prints each problem with its file and field, exits 1
when it found one, and lists the record changes not committed yet, those
`status` on the base lists and `config.toml`, whatever it found. A review
whose item or mission is missing or does not parse is a problem of the
review's own. It needs no readable config
and no base branch, so it reports a broken `config.toml` rather than
stopping at it.

- **pre-commit** reads the index Git gives it, through the commit reader's
  `staged`, and validates every record there against the others. The
  problems it refuses are those of the records the commit adds or changes
  from `HEAD`; a record deleted, or a commit's other records, are not
  judged. A record renamed is judged at its new path. With no `HEAD` yet,
  no status is compared.
- **pre-push** reads the base's name from the checkout's `config.toml` and
  judges each pushed line whose remote ref is the base, whatever the local
  ref; a deletion adds nothing. The new commits are those on the local
  commit's first-parent line not in the history of the remote's old commit.
  When the remote has no base yet, or this clone lacks its commit after a
  force push, every commit on the line is new, so the first push of a
  repository's history is refused, and passes only as a skip.
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
  on, and where Atelier makes the commit (`start` and a take-over, `submit`,
  `verdict`, `merge`, `ready`, `drop`, `close` and `lead`), `git
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
  the bound is no refusal and offers no skip); a planning command run off
  the base, and settling a finding off the base and off an item the caller
  implements; `set` on a record that is not a draft or ready, which with a
  skip changes the base's record of a started item; `ready` on a record that
  is not a draft; `lead` on a mission that is not ready; a record that reads
  and does not validate, in what a planning command writes, in a verdict's
  review file, in `check` and in the pre-commit hook; a status changed as no
  act on that kind of record (an item or a mission, `acts::changes`)
  changes it; and a pushed commit that brings code to the base, or sets an
  item done, outside an Atelier merge; and `merge` with uncommitted changes in the
  item's checkout or in the base's. Changes a skip lets stay are left in
  their checkout, and `merge` confirms them unchanged,
  index entries, modes and contents, and a submodule's commit and its own
  uncommitted changes, with its other inputs.
- **Not rules:** a record that does not read; an ID, a mission or a finding
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
The new `merge` holds an exclusive lock on `atelier-merge.lock` in the Git
directory every worktree of the clone shares, from the read after its wait
for CI to its end, so a second merge waits for the first and then reads the
base it left. The wait itself holds no lock, so a merge waiting for CI does
not hold up another item's merge, and what it read before the lock is read
again under it. `merge` is the only command that takes the lock, since it is
the only one that writes the base.
After the checks, and before it writes anything, it confirms that the branch
tip and the base tip are the ones the checks ran on and that neither the
work checkout nor the base's checkout has changed, and it refuses if any
has. The lock and that confirmation are copied from `merge.rs`,
`mutation_lock` and `committed_snapshot.rs`, with tests that move the base,
move the branch, change the checkout and run two merges at once. The parts
of that code that guard against tampering with file modes and index flags
are not carried over. A merge killed during its checks leaves its temporary
worktree behind; the next merge, holding the lock, removes it.

`forge.rs` and the merge code live in `atelier-cli`, with the Git calls
only they make; `atelier-app` runs no forge command and no merge.

Nothing else from today's merge is carried over: not the gate code, the
polling in `merge_wait.rs`, nor `merge_sync.rs`. The wait for CI is written
new. `submit` pushes the branch and opens its pull request with the same code
as `merge`.

## Site

The tool has no site. The old workspace's site is deleted with it at the
switch, and the `atelier` binary built from the new workspace has none: no
`atelier-site` crate, no `serve` command, and no Node step in its build or
release. The workflows the switch puts in place have no job for the old
site's app. The installed binary keeps serving the old-layout repositories
until the cutover.

The human's pages are the Desk, a separate binary and web app
([ADR 0032](../adr/0032-the-desk-is-the-humans-product.md), mission
atelier-z4cr). Its crate, `atelier-desk`, and its app, `desk/`, are built
in this workspace with their own CI jobs, which the switch keeps. It reads
records through `atelier-records` and `atelier-app`, and what it needs from
this work is already here: every act
with the commit it happened in, and records readable at any commit. So it
can load a mission's directory and items as they stood at the commit of its
`ready` act and compare them with how they stand now.

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
where any worth keeping become items. A Done when with text and no checkbox
line, which the new tool would count as ticked, becomes one unticked line of
its prose, ahead of any fenced block or table, which follow unchanged. Run twice on the same input it writes
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
- The tracker's site is taken out, as above: the `atelier` binary built
  from the new workspace has no site and no `serve`. The Desk's crate and
  app, where they have been added, stay.
- The switch (atelier-u1ko): `crates/`, the old workspace with all its
  tests, and `scripts/smoke-repos.sh` are deleted, and the tracked contents
  of `next/` move to the repository root, so the root workspace, the release
  build and `cargo install --path crates/atelier-cli` are all the new tool,
  version 0.5.0. CI's `check` step runs the fallback release, fetched by its
  pinned archive and SHA-256, while the repository holds `.atelier/issues/`,
  and the tree's binary once it does not. `AGENTS.md` says the installed
  binary stays the tracker and nobody installs from the tree. Added under
  `scripts/cutover/`, so that they are reviewed here: the runbook and its
  rollback, the new `AGENTS.md` text for each repository, and the one
  workflow that runs Atelier in another repository's CI, astromech's, which
  installs Atelier from the commit of master the cutover converted and runs
  `atelier check`.

  What the move touched, besides the renames: the root `Cargo.toml` and
  `Cargo.lock` (the new workspace's, with its version and `dist` profile);
  `atelier-cli`'s manifest (the version, and `dist = true`, since the
  workspace does not publish) and its `--version`; the other tracker
  crates' manifests (the version); the Desk's types test (the path to
  `desk/src/api/` from its new place); `.github/workflows/ci.yml` (one set
  of Rust jobs, the two-way `check` step, the Desk's job as it was);
  `.github/workflows/release.yml` and `dist-workspace.toml` (no Node or
  pnpm); `scripts/ci-jobs.sh` and its test (the jobs `rust` and `desk`);
  `scripts/count_non_test_rust.py` (its help); `scripts/cutover/rehearse.sh`
  and the converter's test (the binary's path); `.gitignore`,
  `.gitattributes`, `AGENTS.md`, and the eval harness's README and the
  `next-*` scenarios (the binary's path). Deleted: `.cargo/`, `.config/`,
  `proptest-regressions/`, `.github/build-setup.yml`, the root
  `package.json` and `package-lock.json`, `scripts/smoke-repos.sh` and
  `scripts/bench_read_commands.py`.

After the switch merges, the tree's binary cannot read this repository's
records, and only the installed binary tracks work. The cutover follows at
once.

**4. Cut over (mission 3).** One thread runs `scripts/cutover/cutover.sh`
from the worktree that has atelier's master checked out
(`scripts/cutover/README.md` is the page for the day). The cutover item is
never started: it stays ready, the runbook runs on the base, and the item is
closed at the end. Everything outside the script that it touches comes from
one settings block (`common.sh`) or the environment, with this host's
values as defaults; the trials run the same script with every one pointed
at a sandbox.

The runbook keeps a journal in its state directory. Each entry is written
before the change it describes and marked done after: each conversion it
starts, with its directory and base commit; each commit; each push, with the
ref's old and new commits; each ref and checkout it moves and from where;
each hooks directory it copies; and what it installs, stops or disables. It
stops at the first failure, and says which step failed and what
`rollback.sh` will undo. Run again after a stop, it picks up from the
journal, and it settles an entry started and not marked done by looking at
the state itself: whether a push happened is read from the remote's tip,
never assumed.

1. **Preflight**, also a command of its own, `cutover.sh preflight`, so
   that it can be run before the switch merges. It stops unless, in each
   repository, every worktree is clean, the base is at its remote's tip, no
   item is in progress as the installed old binary reports it (in every
   worktree's records and the branches its `status` lists; before the
   switch has merged, atelier-u1ko may be), and no T3 thread other than
   the one running the script, or a thread waiting on it, has a run going
   in the repository or one of its worktrees. That last is one read-only,
   bounded query of T3's database; if it cannot be read, the preflight
   prints the hand check and continues only when told it was made. It lists
   each unmerged local branch, whose reviews the conversion does not carry,
   and each worktree not on its base. Then it records each base commit,
   what each worktree has checked out, a copy of each clone's hooks
   directory, the installed binary's version and hash, the skill
   checkout's commit and the service's state.
2. **Convert atelier**, in the worktree that has master: build the tree's
   binary in release (into the state directory), move aside ignored files
   under the paths the conversion writes (the old tool's runtime cache),
   run the converter, replace `AGENTS.md` with the reviewed text, move
   `skills/agent-factory-next/` to `skills/agent-factory/`, and run the
   tree's `setup claude` and `setup codex`, which write
   `.claude/settings.json` and `.codex/hooks.json` in the repository.
   Commit everything, and run `--verify` and the tree's `atelier check`.
3. **Install**: `cargo install --path crates/atelier-cli --locked`, reusing
   that build; `atelier setup` for the Git hooks and for each harness,
   which must change no file; the skill checkout to the conversion commit;
   `atelier-serve` stopped and disabled, since the new binary has no
   `serve`.
4. **Push master** with `ATELIER_SKIP="cutover conversion"` by plain `git
   push`, since the hook just installed refuses a commit that is not a
   merge and changes code.
5. **Each product repository**, in a fresh clone of its base: the
   converter; its `AGENTS.md` process section replaced (below); in
   astromech, `.github/workflows/atelier.yml` replaced, with atelier's
   conversion commit written into it, and no other workflow may run a
   removed command or install Atelier; the harness hook settings; commit;
   `--verify` and `atelier check`; push with the same skip.
6. **Each real clone onto the converted base**, atelier included, so that
   the directory agents start in is on new records. Fetch, and move the
   base to the pushed commit: a fast-forward in the worktree that has it
   checked out, else an update of the ref. A main worktree not on its base
   is switched to it, or, where another worktree has the base, detached at
   the base's tip; the branch it left is kept. `atelier setup` installs the
   Git hooks, keeping a clone's own hook as `<name>.local`. Other worktrees
   are not touched; each one on a branch that still has the old layout is
   printed, since `atelier` and the hooks refuse there until it takes the
   converted base, as the README says.
7. **Accept**, before anything is closed: in each main and base worktree,
   `atelier check` passes, `atelier status` runs, and the count of items and
   missions is the count the conversion printed; each base is at its
   remote's tip; `atelier --version` is the new tool; the skill checkout's
   `SKILL.md` is the new text; `atelier-serve` is stopped and disabled; and
   CI on atelier's master has passed on the conversion commit, its test
   job's `atelier check` step included, waited for with a bound (7 minutes;
   past it the runbook exits 3, nothing failed, and run again it waits on).
   A failure here is a stop like any other.
8. **Close** the cutover item with `atelier close`, the first act the new
   tool records, which makes its own commit, and push it. Past step 7 the
   cutover is not rolled back for a later doubt: a failed push here is
   retried by running the script again.

At the end it prints, for each repository, the conversion commit, the
count of items and missions, `--verify`'s result, the count of relative
links that still resolve and where each worktree was left, and how long
each step took.

A product repository's `AGENTS.md` keeps all of its own text but the
sections that name a removed Atelier command (from a heading of level one
or two to the next): the first becomes the reviewed process section and the
others go. The runbook stops when a repository has none, or more than the
number recorded for it. No product repository configures a forge, so their
process text says to push the base with `git push` after `atelier merge`,
which the pre-push hook checks.

The conversion commits are pushed directly, by plain `git push`. They are
the output of a reviewed script, checked by `--verify`, and reviewed text
moved into place. This is the one time a commit that is not an Atelier
merge changes more than planning records on a base, and the runbook is the
only thing that does it.

**Rollback.** `scripts/cutover/rollback.sh` undoes what the journal says
was started, and is safe to run twice:

1. Every hooks directory goes back from its copy, first, since the new hooks
   would refuse the pushes below and would call commands the fallback
   lacks.
2. For each repository the runbook converted, the remote's base tip, read
   with `git ls-remote`, decides. Still at the recorded base, nothing was
   published: a fresh conversion clone is deleted, and in atelier's master
   worktree the base is reset to the recorded commit and the paths the
   conversion writes are cleaned, leaving ignored files that were there.
   Holding the runbook's commits, each one not yet reverted is reverted,
   newest first, in a clone of the remote's tip, and the reverts are pushed,
   so a closing commit is undone before the conversion beneath it. Each
   revert names the commit it undoes in a `Cutover-Revert:` trailer, which
   is how a second run knows it is done. Holding anything else, the
   rollback stops for that repository, resets nothing there, and says what
   it found.
3. Each real clone's base goes to where its remote now is: the recorded
   commit, or the reverts' tip, whether the clone had taken the conversion
   or not; a commit the runbook made and never pushed is dropped. Each
   worktree is put back on what the journal says it had checked out, and
   the files set aside in step 2 of the cutover go back.
4. If the install had started: the fallback release, v0.4.0, is
   reinstalled from its pinned archive with its SHA-256 checked, its `setup
   claude` is run in each repository whose harness settings it wrote and
   must change no file, the skill checkout is moved back, and `atelier-serve`
   is enabled, started and checked to answer.

Restored means, checked the same way after every trial: where nothing of a
repository was pushed, its base is the recorded commit; where something was
pushed, its base holds one revert per pushed commit, its tracked tree is the
recorded commit's, and local and remote tips agree. In every repository
each worktree has checked out what it had, or the base's tip where it had
the base; its status, ignored files included, is as before, except for the
old tool's own runtime cache, which any old command writes; its hooks
directory is as copied; and the old binary's `check` passes on the old
records. The installed binary is the fallback, the skill checkout is at its
old commit, and the old site answers.

The rollback was tried in sandboxes (`scripts/cutover/trials.sh`) at every
point the cutover can stop: a conversion that fails before its commit (the
section rule matching nothing), `--verify` failing after step 2's commit,
after step 2, part-way through step 3 and after it, after step 4, a second
product repository committed and not pushed, part-way through step 6,
CI reported failed in step 7, after step 8's commit and after its push, a
push in step 4 the remote accepted and reported as failed, and a stop after
one product repository's push. One sandbox clone has a hook of its own,
which is back afterwards.

**After the cutover**, the fallback branch of CI's `check` step is removed.
Nothing else is left to clean up, since the old workspace went whole at the
switch. `scripts/cutover/` is deleted after the work host converts.

**The work host** does not convert: its remotes are already converted. It
adopts, with the same script's `adopt` command, run from a fresh clone of
atelier's converted master made for the purpose, which is where it gets the
reviewed script, the new tool's source and the new skill without moving any
checkout it works in. In order: the preflight, with each base allowed to be
behind its remote and never ahead; the new binary installed from that
clone; the skill installed by the command the settings give, since hosts
install it differently; `atelier-serve` stopped and disabled where the host
has it; step 6 for each repository; and step 7 without the CI check, with
the count of items and missions read by `atelier check` and printed. It
converts, pushes and closes nothing. `rollback.sh` undoes it from its
journal and pushes nothing: hooks, refs (remote-tracking refs included) and
worktrees back to what was recorded, the fallback reinstalled, and the skill
and the service as they were. Until it adopts, the work host must not fetch
a repository that has been converted.

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
  one changing `config.toml` refused; one holding only planning commits
  allowed; a push of the base whose new commit is an Atelier merge of a
  branch with ordinary code commits allowed; a push of an item's branch
  allowed whatever it holds; the refused push allowed with `ATELIER_SKIP`
  set, and the skip printed; a branch switch printing `status`, and one to
  a branch holding a record that does not parse succeeding.
- `ATELIER_SKIP`: each kind of rule passed, its line printed, and its
  trailer in the commit of `start`, `submit`, `verdict` and `merge`; a
  mission set from `draft` to `done` by hand refused, and one closed from
  `ready` allowed; a blank reason refused by the commands and the hooks;
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
| Waiting for CI inside `merge`, for as long as `--wait` and `--timeout` said | A command that sleeps for minutes fights the harness, and one network timeout failed it. It came back on 2026-11-10 in another form: bounded at 9 minutes, under the harness's limit, with no flag, holding no lock, and counting a failed question to the forge as not finished. The thread that runs `merge` is a delegated child that nobody wakes when CI finishes |

## Decisions

Sumeet made each of these in conversation on 2026-11-09, except the two
dated 2026-11-10.

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
| A view of what changed since his go-ahead is the Desk's to derive | It is a derived view; this work supplies the acts with their commits and records readable at any commit |
| Hooks guard, commands act | A rule in a command applies only when the agent remembers the command |
| The tool gives facts and the skill does the arithmetic: `log` and `list`, and no scorecard command | A fixed report in the binary would change every time the measures do, as the debrief did |
| The gardener is a role, and its needs shape the records now | A bug's cause and a deferred finding's settlement are cheap in the first format and costly to add later |
| Every rule can be skipped on purpose, with a reason | Atelier's rules are not hard blockers, and a bypass should be intended and leave a trace |
| `submit` pushes the branch and opens its pull request where a forge is configured (2026-11-10) | CI then runs during the review; it was nearly all of the wait between approval and merge, a median of 5.5 minutes on 25 items |
| `merge` waits for CI, for up to 9 minutes, with no flag (2026-11-10, reversing "`merge` never waits") | The thread that runs `merge` is almost always a delegated child, which T3 neither lets watch a pull request nor wakes reliably, so "not yet, run me again" left approved items with nobody driving them |
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

- Whether T3's pull request watch wakes a thread when a repository marks no
  check as required.
- Where video artifacts go when a project needs them.
