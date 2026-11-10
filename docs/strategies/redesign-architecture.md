---
status: draft
last_changed: "2026-11-09: first draft from Sumeet's direction in conversation"
---

# Atelier redesign: architecture specification

This is a target design. Today's architecture is in `docs/architecture/`. The
product behaviour this serves is in
[redesign-product.md](redesign-product.md). When the redesign is finished this
document, edited to match the code, becomes the single design record that
replaces the ADRs.

## The rule for this work

The aim is better code, not more code. The new command layer is built beside
the old one and replaces it in one swap, which deletes the old code and its
tests. No compatibility layer is built: no binary reads both record layouts.
Library code below the commands is reshaped in place. Non-test Rust is
reported at the end of each mission as an indication. It is not a limit, and
no item is shaped to hit a number.

## Where the code is today

48,988 non-test lines of Rust in six crates, and 59,627 lines of tests, of
which 50,438 are CLI integration tests.

| Area | Lines today | What happens to it |
| --- | --- | --- |
| Record parsing, validation and index | 7,930 | Reshaped: three record kinds in directories, one schema version |
| Merge, gates, freshness, light path | 6,190 | Reduced to the three checks in the product spec; the light path and base exemptions go |
| Issue and work views and output | 5,304 | Replaced by `status`, `show` and `list` |
| Missions, brief, debrief, criteria | 4,753 | Kept only as the views the site serves |
| Review and findings | 4,669 | Replaced by `review` and `verdict` over one file per review |
| Site, Rust and web | 4,926 | Kept; reads the new layout |
| Workflow policy | 3,418 | Deleted; a short config section replaces it |
| Status, land and hooks | 2,438 | Reshaped around thread and status |
| Prune and history | 2,410 | Deleted |
| Evidence | 1,931 | Deleted |
| Bundle | 1,665 | Deleted |
| Forge and CI | 1,202 | Kept |
| Migrate | 883 | Deleted |
| Setup | 821 | Kept; gains Codex hooks |

Tests go with the code they test. An integration test for a deleted command
is deleted, not ported.

## Crates

| Crate | Owns |
| --- | --- |
| `atelier-core` | Domain values: Mission, Item, Review, Finding, Status, Thread, Config |
| `atelier-records` | Reading and writing the three record kinds and `config.toml`; IDs; validation |
| `atelier-app` | Use cases: the status view, each lifecycle act, the merge checks, cross-branch reads, the site's views |
| `atelier-site` | `atelier serve` |
| `atelier-cli` | Arguments, dispatch, Git and shell adapters, output |

`atelier-workflow` is deleted. The dependency direction stays `cli` to `site`
to `app` to `records` to `core`.

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
branch: atelier-06jc    # set by start; absent on an item with no files to change
implementer: <thread>   # set by start
ready: {by: sumeet, commit: <sha>, at: <time>}
trivial: null           # or the reason given to merge --trivial
created_at: <time>
```

Sections: Goal, Done when, Scope, Report, Decisions. Report has Validation
and Responses subsections when they apply. A Decisions entry is a dated
question, and its answer with the deciding thread once it has one.

### `missions/<id>/mission.md`

```yaml
schema: atelier.mission
schema_version: 3
id: atelier-xxxx
title: ...
status: draft           # draft ready running done dropped
labels: []
lead: <thread>          # set by lead
ready: {by: sumeet, commit: <sha>, at: <time>}
created_at: <time>
```

Sections: Why, Outcome, Success criteria (optional), Scope, Authority,
Result. Any other file in the directory is a planning document and has no
required shape.

### `reviews/<n>.md`, under an item or a mission

```yaml
schema: atelier.review
schema_version: 3
commit: <sha reviewed>
reviewer: <thread>
model: gpt-6.1-sol
verdict: null           # approve or changes, set by verdict
created_at: <time>
```

Sections: Summary, then `## Finding <k>: <title>` with a first line
`Recommendation: fix now | defer | no action` and, on a finding carried from
an earlier review, `Was: review <n> finding <k>` and `Now: fixed | withdrawn |
kept`.

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
ci_workflow = "ci.yml"
```

`merge` reads this file from the base branch's tip.

### IDs

Missions and items keep today's form, `<slug>-<4 characters>`. Reviews have
no ID. `show` accepts either kind.

## Status

Status is a stored field. One table in `atelier-app` lists every act: the
command, the role that may run it, the statuses it may run from, the status
it sets, and its conditions. Each command and the `status` view read that
table, so what `status` says a thing needs is what the command will check.

| Act | Role | From | To | Conditions |
| --- | --- | --- | --- | --- |
| `ready` | Planner | `draft` | `ready` | none |
| `start` | Implementer | `ready` | `building` | blockers done |
| `submit` | Implementer | `building`, `changes_requested`, `approved` | `in_review` | Done when ticked; Report written; each open fix-now finding has a fix commit or a response; no unanswered question |
| `verdict changes` | Reviewer | `in_review` | `changes_requested` | the file has a fix-now finding |
| `verdict approve` | Reviewer | `in_review` | `approved` | the file has no fix-now finding |
| `merge` | Implementer | `approved`, or `building` with `--trivial` | `done` | the merge checks |
| `close` | Any | an item with no branch, not done | `done` | a result text |
| `drop` | Planner | any but `done` | `dropped` | a reason |

Each act writes the status in the commit the act already makes: `submit`
commits the Report, `verdict` commits the review file, `merge` writes `done`
in the merge commit. No act makes a commit only to change a status.

Two facts are checked live and never stored:

- **The approval covers the head.** No path outside `.atelier/` changed
  between the reviewed commit and the head. Record changes never make an
  approval stale. If code changed, `status` and `merge` say to run `submit`.
- **The merge checks pass on the head.** The configured commands, and CI
  where a forge is named.

`ready` stores the commit it was set at. For a mission, the site's result view
compares the mission's directory and items at that commit with their final
state to show what changed from the plan. Nothing else reads the pin.

## Threads and roles

A thread's identity is `T3CODE_THREAD_ID` when set. T3 sets it in the shells
of Claude, Codex and its ACP providers. Otherwise it is the harness's session
ID (`CLAUDE_CODE_SESSION_ID`, `CODEX_SESSION_ID`, `GROK_SESSION_ID`).
Outside any harness it is absent, and entry commands record `human`.

Entry commands store the identity in the record: `start` in the item's
`implementer`, `review` in the review file's `reviewer`, `lead` in the
mission's `lead`. `decide` stores it with the answer.

There is no local state file. A command works out who is calling by looking
for the caller's identity in the records it can see: the item of the current
branch, an open review of that item, and running missions on the base. A
thread may hold several things, and `status` lists each.

Rebinding is allowed and visible. If a thread runs `start` on an item another
thread is building, the item passes to the caller and the command says which
thread held it. Threads die, and a successor must be able to continue.

Hooks give orientation at session start: `atelier setup claude` exists, and
`atelier setup codex` is added, since Codex 0.167 has hooks. Where no hook
runs, a project's `AGENTS.md` says to run `atelier status` first.

## Where records are read and written

- A command on a work branch reads and writes that branch's `.atelier/`.
- `status` on the base, `lead`, `list` and the site read each started item
  from the tip of its branch, by the `branch` field, as the site does today
  for work in flight. Nothing is copied between branches.
- `land` on the base applies one rule to each unpushed commit: it is a merge
  made by `atelier merge`, or it touches only paths under `.atelier/`. A
  commit on the base may not set `done` on an item that has a `branch`.
- A plan reviewer works in its own detached worktree so two reviewers stay
  blind to each other. The planner cherry-picks each review commit onto the
  base.

## Merge

1. Read `config.toml` from the base tip.
2. Check the approval covers the head, or that `--trivial` was given on an
   item with no review.
3. Check no Decisions entry is unanswered.
4. Build the merged tree in a temporary checkout and run each merge check.
5. Where a forge is configured: push the branch, reuse or open its pull
   request, wait for CI on the head.
6. Make one merge commit on the base that sets the item `done`.
7. Where a forge is configured: push the base and confirm the pull request
   merged.

Steps 4 to 7 are today's code and stay.

## Site

`atelier_app::commit_records::load` learns the new layout, and the JSON API
keeps its routes. The brief and debrief stay as site views and gain nothing
in this work except the comparison against the ready commit. The board reads
status directly, where today it classifies.

## Transition

The installed binary is one per host and the installed skill is one per host,
so the process changes for every repository on a host at once. The transition
is therefore one step per host, not one per repository.

- **While the core loop is built**, the installed 0.4.0 binary keeps tracking
  all work, including the redesign's own, and the repository's records stay
  in the old layout. The new commands live in new modules of the existing
  crates and are reached through a second binary target, `atelier-next`. The
  old binary, its tests, CI's `atelier check` step and
  `scripts/smoke-repos.sh` keep working untouched.
- **The swap** is mission 2's last item. It deletes the old modules, commands
  and tests, and renames `atelier-next` to `atelier`. From then until the
  cutover, CI's `atelier check` step pins the 0.4.0 release, since the tree's
  binary no longer reads the repository's own records, and
  `scripts/smoke-repos.sh` is removed.
- **The converter** is `scripts/convert_records.py`, a script that reads the
  old Markdown directly and writes the new layout. It is not a command and is
  deleted after the last host converts. It converts open missions and items,
  and the strategy files they link. Done and superseded records leave the
  tree; Git keeps them. A branch in flight is merged or abandoned before
  conversion, so no review is converted.
- **Rehearsal** converts a scratch clone of each of the five repositories and
  runs the new binary's `check`, `status` and one full item on each.
- **Cutover** on a host: confirm no work is in flight, convert each
  repository in one commit, install the new binary, move the skill checkout
  forward, reinstall the hooks. Rollback is reinstalling the 0.4.0 release
  and reverting the conversion commits.
- **The work host** converts later by the same runbook, run by Sumeet.

## Testing

- One scenario test per lifecycle path, driving the real binary in a scratch
  repository: the plain path, changes requested and fixed, a rebuttal, an
  escalation, a trivial merge, an item with no files, a mission from draft
  to closed.
- Unit tests on the act table: every act from every status.
- The site's fixture repositories and generated types stay as they are.
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

Three points in this design go beyond what he confirmed, and the planning
reviews and his approval should weigh them:

- **The loop is rebuilt as one unit beside the old one.** He agreed to
  replace commands one at a time, each deleting its predecessor. The record
  layout changes under every command at once, so a new `start` and an old
  `review` could not work on the same item. What is reshaped in place is the
  library code below the commands.

- **Deferred findings create no items.** He agreed that a deferred finding
  becomes a draft item at merge. This design leaves it in its review file
  and lists it with `list --deferred`, because the backlog already holds four
  piles of such items that nobody reads.
- **One cutover per host.** He agreed to convert repositories one at a time
  with the binary reading both layouts. The skill is installed once per host,
  so the process would change for every repository at once anyway, and
  reading both layouts would keep the old code alive through the whole build.

## Open questions

None of these changes the shape of the work. Each is settled by the item that
reaches it.

- Which Codex hook event runs at session start.
- Where video artifacts go when a project needs them.
- The exact flags of `set` and `list`.
