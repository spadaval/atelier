---
status: draft
last_changed: "2026-11-09: revised after planning review round 2 (Astra, Grok)"
---

# Atelier redesign: architecture specification

This is a target design. Today's architecture is in `docs/architecture/`. The
product behaviour this serves is in
[redesign-product.md](redesign-product.md). When the redesign is finished this
document, edited to match the code, becomes the single design record that
replaces the ADRs.

## The rule for this work

The aim is better code, not more code. The new command layer is built beside
the old one. One reviewed change then switches the binary to it and deletes
the old one, and the cutover converts the records. No compatibility layer is
built: no binary reads both record layouts. Library
code below the commands is adapted to the new types where it is kept.
Non-test Rust is reported at the end of each mission as an indication. It is
not a limit, and no item is shaped to hit a number.

## Where the code is today

48,988 non-test lines of Rust in six crates, and 59,627 lines of tests, of
which 50,438 are CLI integration tests.

| Area | Lines today | What happens to it |
| --- | --- | --- |
| Record parsing, validation and index | 7,930 | Reshaped: three record kinds in directories, one schema version |
| Merge, gates, freshness, light path | 6,190 | The merged-tree checkout, the merge commit and the push are extracted onto the new types; the gate code (`gate_report`, `review_gate`, the light path, base exemptions) is deleted, not moved |
| Issue and work views and output | 5,304 | Replaced by `status`, `show` and `list` |
| Missions, brief, debrief, criteria | 4,753 | Kept only as far as the site's pages need |
| Review and findings | 4,669 | Replaced by `review` and `verdict` over one file per review |
| Site, Rust and web | 4,926 | Kept; converted to the new records |
| Workflow policy | 3,418 | Deleted; a short config section replaces it |
| Status, land and hooks | 2,438 | Reshaped around thread and status |
| Prune and history | 2,410 | Deleted |
| Evidence | 1,931 | Deleted |
| Bundle | 1,665 | Deleted |
| Forge and CI (`atelier-app/src/forge.rs`) | 1,202 | Kept, with its config type replaced: it takes `workflow_policy::ForgeConfig` today, with a required workflow name |
| Migrate | 883 | Deleted |
| Setup | 821 | Kept; gains Codex hooks |

Tests go with the code they test. An integration test for a deleted command
is deleted, not ported. Tests of extracted code are rewritten against the new
types.

## Crates

| Crate | Owns |
| --- | --- |
| `atelier-core` | Domain values: Mission, Item, Review, Finding, Status, Thread, Config |
| `atelier-records` | Reading and writing the three record kinds and `config.toml`; IDs; validation |
| `atelier-app` | Use cases: the status view, each lifecycle act, the merge checks, cross-branch reads, the site's views |
| `atelier-site` | `atelier serve` |
| `atelier-cli` | Arguments, dispatch, Git and shell adapters, output |

`atelier-workflow` is deleted at the cutover. The dependency direction stays
`cli` to `site` to `app` to `records` to `core`.

## Records

All records are Markdown with YAML front matter, `schema_version: 3`.

### `items/<id>/item.md`

```yaml
schema: atelier.item
schema_version: 3
id: atelier-06jc
title: Put open findings in full in the review brief
status: building        # draft ready building in_review changes_requested approved done dropped
mission: atelier-icv4   # optional
labels: [review]
priority: high
blocked_by: []
implementers: [<thread>]  # start appends; the last entry holds the item
ready: {by: sumeet, at: <time>}
trivial: null           # or the reason given to merge --trivial
created_at: <time>
```

Sections: Goal, Done when, Scope, Report, Decisions. Report has Validation
and Responses subsections when they apply. Responses holds one line per open
fix-now finding: `2-m2bn.1: fixed in <commit>` or
`2-m2bn.1: rebutted: <fact>`.

A Decisions entry is a dated question and, once answered, the answer and the
deciding thread. An entry may name a finding (`Finding: 2-m2bn.1`), and its
answer may then carry `Outcome: defer` or `Outcome: no action`.

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

The suffix is four random characters. Sections: Summary, then
`## Finding <k>: <title>` with a first line
`Recommendation: fix now | defer | no action`. A finding's name is its
review's file stem and its number, as in `2-m2bn.1`. A finding carried from
an earlier round also has `Was: <finding name>` and one of `Now: fixed`,
`Now: withdrawn`, `Now: kept`, or `Decided: defer | no action`, which the
tool writes from the Decisions entry and the reviewer cannot change.

A fix-now finding is open until a later review marks it fixed or withdrawn,
or a decision settles it.

### `config.toml`

```toml
schema_version = 3
slug = "atelier"
base_branch = "master"
merge_strategy = "merge_commit"

[[merge_check]]
run = "cargo fmt -- --check"

[forge]
provider = "github"
remote = "origin"
ci_workflow = "ci.yml"   # optional; without it nothing waits for CI
```

`merge` reads this file from the base branch's tip.

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

Hooks give orientation at session start: `atelier setup claude` exists, and
`atelier setup codex` is added, since Codex 0.167 has hooks. Where no hook
runs, a project's `AGENTS.md` says to run `atelier status` first.

## Where records are read and written

- **A started item is found by its branch.** An item is started when a local
  branch named for its ID exists and is not merged into the base. Its record
  is then read from that branch's tip, wherever the reader stands. Once the
  branch is merged, the base's record is the item's record. Nothing is copied
  between branches.
- **A terminal status on the base wins.** If the base's record is `dropped`
  or `done`, that is the item's status whatever its branch holds. This is how
  a started item is cancelled.
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
- A command on a work branch reads and writes that branch's `.atelier/`.
- `land` on the base applies one rule to each unpushed commit: it is a merge
  made by `atelier merge`, or it touches only paths under
  `.atelier/missions/` and `.atelier/items/`. A commit on the base may not
  set `done` on an item whose branch exists.
- A plan reviewer works in its own detached worktree so two reviewers stay
  blind to each other. Review files have distinct names, so the planner
  cherry-picks both onto the base without a conflict.

## Merge

1. Read `config.toml` from the base tip.
2. Check the approval covers the head, or that `--trivial` was given on an
   item with no review.
3. Check no Decisions entry is unanswered.
4. Build the merged tree in a temporary checkout and run each merge check.
5. Where a forge is configured: push the branch and reuse or open its pull
   request. Where it names a CI workflow, wait for that workflow on the head.
6. Make one merge commit on the base that sets the item `done`.
7. Where a forge is configured: push the base and confirm the pull request
   merged.

`atelier-app/src/forge.rs` is kept, with one change to its interface: it
takes the forge section of the new config, whose CI workflow is optional,
where today it takes `workflow_policy::ForgeConfig` with a required workflow
name. Its behaviour and its contract tests stay. The merged-tree checkout,
the merge commit and the push live today in `atelier-cli`'s `merge.rs`,
`merge_push.rs` and `merge_wait.rs`, bound to `WorkflowPolicy` and
`RecordIndex`. They are extracted onto the config and item types, and their
contract tests are rewritten against the new command. Nothing else from
today's merge is kept.

## Site

The site is converted to the new records in mission 3, just before the old
commands are deleted: its loader reads the three record kinds, its responses
drop what the new model does not have, and its fixtures are regenerated in
the new layout. The new reader is served by `atelier-next serve`, the web
tests move to it, and `serve` is removed from the old binary in the tree. The
web typecheck, unit and Playwright jobs must pass in the change that does it.
How much of the brief and debrief view code survives is settled when that
step is planned, by reading the site's code.

The installed binary keeps serving the old-layout repositories until the
cutover, because nothing rebuilds it.

## Transition

The installed binary is one per host and the installed skill is one per host,
so the process changes for every repository on a host at once. The transition
is one step per host, and it has four parts. The first three are ordinary
items under the old tool. The fourth is a runbook.

**1. Build beside (mission 2).** The installed binary tracks all work,
including the redesign's own, and the repository's records stay in the old
layout. New commands live in new modules of the existing crates and are
reached through a second binary target, `atelier-next`. The old binary, its
tests, CI's `atelier check` step and `scripts/smoke-repos.sh` are untouched.

**2. Rehearse (mission 2).** `scripts/cutover/convert_records.py` reads a
repository's old `.atelier/` directly and writes the whole new one: missions
and items from open records and the strategy files they link, and
`config.toml` from `workflow.yaml`, with no CI workflow where the repository
has no Actions. Done and superseded records leave the tree, and Git keeps
them; deferred findings on them are not carried over, and the backlog sort is
where any worth keeping become items. Run twice on the same input it writes
the same bytes, and `--verify` checks a converted tree against a fresh run.
The rehearsal converts a scratch clone of each of the five repositories and
runs `atelier-next check`, `status` and one full item on each.

**3. Prepare (mission 3).** Each of these is reviewed and merged under the
old tool:

- The skill is rewritten on master. It does not go live, because the live
  copy is a checkout that moves only when someone moves it.
- The site is converted, as above.
- The last release of the old tool, 0.4.1, is cut from master. It is the
  binary CI and the rollback use.
- The switch: the `atelier` binary target points at the new command layer,
  and the old command layer, its integration tests and
  `scripts/smoke-repos.sh` are deleted. CI's `check` step runs the 0.4.1
  release while the repository holds `.atelier/issues/` and the tree's
  binary once it does not. `AGENTS.md` says the installed binary stays the
  tracker and nobody installs from the tree. The new `AGENTS.md` text for
  each repository is added under `scripts/cutover/`, so it is reviewed here.

After the switch merges, the tree's binary cannot read this repository's
records, and only the installed binary tracks work. The cutover follows at
once.

**4. Cut over (mission 3).** One thread runs `scripts/cutover/cutover.sh`,
which stops at the first failure:

1. Confirm no item is in progress in any of the five repositories and no
   other thread is running in them.
2. In atelier, on master: run the converter and replace `AGENTS.md` with the
   reviewed text. Commit. Run `--verify`, and the tree's `atelier check`.
3. Install the binary from master, run `atelier setup` for each harness, and
   move the skill checkout forward.
4. Push master. CI now runs the tree's binary on the converted records.
5. In each of the other four repositories: convert, replace the process text
   in `AGENTS.md`, update any Atelier version its CI pins, commit, run
   `--verify` and `atelier check`, and push.
6. Close the cutover item with `atelier close`, the first act recorded by the
   new tool.

The conversion commits are pushed directly, by plain `git push`. They are the
output of a reviewed script, checked by `--verify`, and the reviewed
`AGENTS.md` text. This is the one time a commit that is not an Atelier merge
changes more than planning records on a base, and the runbook is the only
thing that does it.

**Rollback**, by where the run stopped:

| Stopped | Undo |
| --- | --- |
| Before step 3 | Reset the unpushed commit |
| After step 3 | Also reinstall 0.4.1, run its `setup`, and move the skill checkout back |
| After step 4 | Also revert the conversion commit on master and push |
| During step 5 | Also revert the conversion commit in each repository already pushed |

Each row is tried once on scratch clones before the real cutover, including
stopping straight after step 4 and half-way through step 5.

**After the cutover**, the first items under the new tool delete the library
code nothing reaches any more: the workflow crate, the old record types, the
gate code, evidence, bundle, migrate and prune, and CI's 0.4.1 branch.
`scripts/cutover/` is deleted after the work host converts.

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

Five points in this design go beyond what he confirmed, and his approval
should weigh them:

- **The loop is rebuilt as one unit beside the old one.** He agreed to
  replace commands one at a time, each deleting its predecessor. The record
  layout changes under every command at once, so a new `start` and an old
  `review` could not work on the same item.
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
- **No view of what changed from the plan.** He was told the result would
  show items added, dropped or redefined since his go-ahead. Both planning
  reviewers advised cutting it from this work, and Git history keeps what a
  later view would need.

## Open questions

None of these changes the shape of the work. Each is settled by the item that
reaches it.

- Which Codex hook event runs at session start.
- Where video artifacts go when a project needs them.
- The exact flags of `set` and `list`.
- How much of the brief and debrief view code the site keeps.
