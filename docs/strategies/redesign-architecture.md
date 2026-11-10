---
status: draft
last_changed: "2026-11-10: drop and close; plan reviews of a mission with review and verdict; ready says whether the plan was reviewed (atelier-jq24)"
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
| `start`, taking over | Implementer | `building`, `in_review`, `changes_requested`, `approved` | unchanged | branch `<id>` exists |
| `start --new` | Implementer | none | `building` | none; the item is created on its new branch |
| `submit` | Implementer | `building`, `changes_requested`, `approved` | `in_review` | Done when ticked; Report written; each open fix-now finding has a line in Responses or a decision; no unanswered question. After its commit, where a forge is configured, it pushes the branch and opens its pull request; a failure of either is a warning, and `merge` does it |
| `review` | Reviewer | `in_review` | unchanged | the caller is not in the item's `implementers` |
| `verdict changes` | Reviewer | `in_review` | `changes_requested` | the file validates; it has an open fix-now finding |
| `verdict approve` | Reviewer | `in_review` | `approved` | the file validates; no fix-now finding of the item is open, the file's or one it does not carry |
| `verdict changes` on a plan review | Reviewer | any | unchanged | the file validates; it has an open fix-now finding |
| `verdict approve` on a plan review | Reviewer | any | unchanged | the file validates; no fix-now finding is open among the file's own and those of earlier rounds |
| `escalate` | Implementer | `building`, `changes_requested`, `approved` | unchanged | none |
| `decide` | Decider | any started status | unchanged | an unanswered question exists; the caller is not in the item's `implementers`, nor in the `reviewers` of the review that raised the finding |
| `merge` | Implementer | `approved` | `done` | a review of the latest round approves it and still covers the head; no unanswered question; nothing uncommitted in tracked files or the item's directory; the base's record is not dropped |
| `merge --trivial` | Implementer | `building` | `done` | Done when ticked; Report written; no review file; no unanswered question; nothing uncommitted in tracked files or the item's directory |
| `close` (item) | Any | `draft`, `ready` | `done` | a result text; no branch `<id>` exists |
| `close` (mission) | Lead or planner | `ready` | `done` | every item done or dropped; Result written |
| `drop` | Planner | `draft`, `ready`, and an item's started statuses | `dropped` | a reason, on one line; written on the base, whatever the item's branch holds |
| `new --from`, `drop` on a finding | Planner on the base, or an implementer on its branch | a deferred finding not yet settled | unchanged | the finding's item is done |
| `lead` | Lead | a `ready` mission | unchanged | none |

Each act writes the status in the commit the act already makes: `submit`
commits the Report, `verdict` commits the review file, `merge` writes `done`
in the merge commit. No act makes a commit only to change a status.

How the table is built:

- A row joins the table with its command. The table holds the rows of
  `ready` on an item and on a mission, `start`, `start --new`, `start`
  taking over, `submit`, `review`, both verdicts and `merge` with and
  without `--trivial`, `drop` (one row, for an item and a mission),
  `close` on an item and on a mission, and both verdicts on a plan review.
  A mission's row is checked with `acts::check_mission`, from the
  mission's status and, for `close`, the status of each item that names it,
  each found as `Located::record` finds it, and its Result; for a verdict,
  from the caller's plan review. `start --new` runs from no status, since
  it creates its item, and takes the status it sets from its row. `close`
  on an item and on a mission needs no role, like a planner's act.
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
  commands. `review` is the exception: an implementer, current or past, is
  refused by its condition, and is shown the brief. Then `check` refuses an
  act whose From does not hold the item's status, then one that needs a role
  the caller does not hold, then one with an unmet condition, and otherwise
  gives the status to set. An act that binds a role (`start`, `review`)
  needs nothing held; `submit` and `merge` need the caller to be the item's
  last implementer, and a verdict needs it to be the round's reviewer. No
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
  and `merge` say to run `submit`. `atelier-cli` gathers what this reads, as
  `acts::Checkout`: the paths that differ between the reviewed commit and
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

## Threads and roles

A thread's identity is `T3CODE_THREAD_ID` when it is set, else the harness's
session ID (`CLAUDE_CODE_SESSION_ID`, `CODEX_SESSION_ID`,
`GROK_SESSION_ID`), else `human`. A variable set to an empty value counts as
unset.

A harness's own subagent is not a new thread. Observed on 2026-11-10
(atelier-67nn): a Claude Code subagent started with its `Agent` tool runs with
its parent's `CLAUDE_CODE_SESSION_ID`, and under T3 Code with its parent's
`T3CODE_THREAD_ID`; a Codex 0.167 `spawn_agent` child shares its parent's
`CODEX_SESSION_ID` and has its own `CODEX_THREAD_ID`, which Atelier does not
read; it inherits its parent's environment, so under T3 Code it would carry
the parent's `T3CODE_THREAD_ID` too (inferred, not run there). Atelier sees
such a subagent as its parent, and refuses it as the reviewer of its
parent's item. Only a thread the harness starts as a thread, such as T3
Code's `delegate_task`, has an identity of its own. `submit`'s handoff and the
refusal of an author's `review` say so.

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
ready items, and the started items others hold; on the base branch it also
lists the record changes the checkout has not committed, and anywhere but
an item's branch it lists the items dropped on the base while their branch
is there, each with its reason. On the item's branch it says a dropped
item was dropped and why, as does the refusal of any act on it, and it
speaks to the round's reviewer as reviewer, naming its file, tells
the implementer of an item in review which thread holds the round's review,
or that none does yet, and tells the implementer of a building item whose
Goal or Done when lines are empty, as after `start --new`, to write them
first. That line is orientation, not a condition of `submit`. Missions join
with their commands.

Harness hooks give orientation at session start: `atelier setup claude`
exists, and `atelier setup codex` is added, since Codex 0.167 has hooks. The
session-end hook runs `atelier check`, which also reports record changes
not yet committed. Where no harness hook runs, a
project's `AGENTS.md` says to run `atelier status` first, and the
post-checkout Git hook prints it on every branch switch.

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
  `start` reads its item, the item's blockers and its mission this way. When
  it is refused while the item or its mission is `ready` in the base's
  checkout and not at the tip, it says the change is not committed yet and
  prints the Git command that commits the checkout's planning.
- **Planning is read and written in the checkout of the base.** `new`,
  `set`, `ready`, `drop` and `close` run only on the base branch, and are
  refused elsewhere, naming the checkout that has the base. They read the checkout's records, with an
  item's branch found as above, so uncommitted planning counts and a started
  item is still seen as started. Each validates the records it writes against
  every record of the checkout and writes nothing when one of them has a
  problem; nothing else validates planning until the Git hooks exist. Each
  commits nothing, prints the paths it wrote, and ends with a `Then:` line
  giving the Git command that commits those paths alone. `new` picks an ID
  that no item or mission has in the checkout, at the base's tip, or as a
  local branch, and writes a draft with its section headings and nothing
  under them; an item's priority starts as `medium`.
- **`drop` and `close`.** `drop` writes the base's record, the one in the
  base's checkout, whatever the item's branch holds; an item that is only
  on its branch, as after `start --new`, gets its branch's record written
  there. Once that is committed, every command reads it first, so a
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
  else the base's. `atelier-cli` gathers what the rule reads: the base's tip,
  each local branch whose name is an item ID, whether it is merged, and its
  record. `atelier-app` runs no Git.
- **A terminal status on the base wins.** If the base's record is `dropped`
  or `done`, that is the item's status whatever its branch holds. This is how
  a started item is cancelled. Every command checks the base's record of its
  item first, including `status` and `merge` run on the item's own branch.
- **Who reads this way:** `status` on the base, `lead`, `list`, `show` and
  the site. The site already enumerates local branches for work in flight.
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
- **A review is written where the implementer works.** `review` runs only
  in the checkout on the item's branch, and refuses anywhere else, naming
  it. It creates the round's file, or appends the caller to its
  `reviewers`, and commits nothing. `verdict` commits the review file and
  `item.md`, with its new status, in one commit, and nothing else in the
  checkout.
- **A refused commit changes nothing.** When `git commit` fails, as when a
  hook refuses it, `submit`, the take-over and `verdict` put the files they
  wrote and the index under the paths they staged back as they were, so the
  act can be run again.
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
- Records can be read at any commit, without a checkout, through
  `atelier-records`' `CommitReader`: one `git cat-file --batch` process
  serves every read, and it tells a path the commit lacks, an entry that is
  not a regular file and a failing Git apart. The site reads this way, and
  so does anything that compares a record now with the same record at the
  commit of an earlier act.
- **A plan review is written where its reviewer works.** `review <mission>`
  runs in any checkout and reviews the plan as that checkout's HEAD holds
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
   bound, stop with exit status 3. The wait holds no lock and writes
   nothing.
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
   waiting: a passing run must still count for the head as it now is.
   Write the merge commit object on the merged tree, base tip first and
   branch tip second, with the item's record set `done` (and `trivial` set
   to the reason). Nothing points at it yet.
6. Check out that commit in a temporary worktree in the Git directory and
   run each merge check there with `sh -c`. A failure refuses, printing the
   command, its exit status and the last 20 lines of its output.
7. Confirm the branch tip, the base tip and both checkouts are what steps
   3 to 6 read; otherwise write nothing and say to run it again. Then move
   the base to the merge commit: a fast-forward of the base's checkout
   where one has it checked out, else `git update-ref` from the tip read.
8. Where a forge is configured: push the base.

`merge` has no flag but `--trivial`, and the bound and the time between
questions are constants; debug builds let tests shorten them through
`ATELIER_TEST_CI_WAIT`. The 9 minutes sit under Claude Code's 10-minute limit
on a command in the foreground; a harness with a shorter limit kills `merge`
during the wait, which leaves nothing behind. Run again, it picks up where
the facts now stand: on an item it already merged it only pushes the base
where a forge is configured, which is what a run that stopped after step 7
left undone. Its exit status is 0 when it merged or had nothing left to do,
1 for a refusal, 2 for an error and 3 when CI has not finished by the bound.

The merge commit's message ends with the trailer `Atelier-Merge: <id>`,
which the pre-push hook knows an Atelier merge by. The branch is kept; it
is merged, so the base's record is the item's record from then on.

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

`forge.rs` and the merge code live in `atelier-cli`, which owns the Git and
shell adapters; `atelier-app` runs neither.

Nothing else from today's merge is carried over: not the gate code, the
polling in `merge_wait.rs`, nor `merge_sync.rs`. The wait for CI is written
new. `submit` pushes the branch and opens its pull request with the same code
as `merge`.

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
- `merge` with CI still running: it waits, holding no lock, and merges as
  soon as CI passes or refuses as soon as it fails; at the bound it stops
  with its "not yet" exit status and changes nothing, also when the forge
  fails or never answers. A run that passed on the commit `submit` pushed
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
| A view of what changed since his go-ahead is the dashboard's to derive | It is a derived view; this work supplies the acts with their commits and records readable at any commit |
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

- Which Codex hook event runs at session start.
- Whether T3's pull request watch wakes a thread when a repository marks no
  check as required.
- Where video artifacts go when a project needs them.
- The exact flags of `list`.
- How much of the brief and debrief view code the site keeps.
