---
schema: atelier.item
schema_version: 3
id: atelier-ob9u
title: Name branches by convention and commit every record change on a branch, never on the base
status: in_review
mission: atelier-to83
labels:
- redesign
priority: medium
blocked_by:
- atelier-ogpg
- atelier-xbog
implementers:
- thread:delegated-task:command%3Amcp%3A93b1b51f-2095-4d1f-af23-677aa6ba570d%3Adelegate-task%3A8c7f6fe9-cf71-4c03-ac83-d9beea29070e
ready:
  by: 'sumeet, on 2026-11-13: execute atelier-to83, relayed by the planner thread'
  at: 2026-11-13T22:37:19.473562932Z
trivial: null
caused_by: null
created_at: 2026-11-13T06:14:32.623183548Z
---

## Goal

Stop record changes landing on the base, and draw one line between the tool
and Git. Today `new`, `set`, `ready`, `drop`, `close` and `lead` change
records in the base's checkout and four of them commit there. After this item
a command that changes a record writes the file and nothing else, the agent
commits and pushes with Git, the hooks validate, and only three commands act
on Git: `start` makes and claims a branch, `submit` publishes it, and `merge`
merges it.

## Done when

- [x] `new`, `set`, `ready`, `drop`, `close`, `lead`, `review` and `verdict` write record files in the checkout they run in and make no commit, no branch and no push. Each prints the paths it wrote. Tests show `git status` after each has changes and `git log` has none.
- [x] `start` is the only command that makes a branch, and it makes all three kinds: `start <item>` as today; `start --mission "<title>"`, a new mission's record on a branch named for it; and `start --backlog <slug>`, a `backlog/<slug>` branch for record changes tied to neither. It commits the one record change that is the claim, where there is one.
- [x] `config.toml` takes one optional branch prefix. With it set to `x/`, every kind of branch carries it: `x/<item-id>`, `x/<mission-id>` and `x/backlog/<slug>`; unset, branches are named as today. A branch name is read back into its record's ID by one function, tested both ways.
- [x] `start` never switches the base's own checkout to another branch: there it does what the design settled, and a test shows the base's checkout still has the base out afterwards.
- [x] The pre-commit hook refuses a commit on the base that changes a record, and says to run `start --backlog <slug>` or `start --mission`. `ATELIER_SKIP` passes it with a reason, as for any rule.
- [x] Where a remote is configured, `start` pushes the new branch before it reports success. When the remote already has it, `start` is refused, names the thread its record gives, and removes the local branch it made, leaving the checkout as it was; a later `start` in that clone takes the item over from the remote's branch. When the remote cannot be reached, `start` is refused and says so. With no remote it behaves as today. A test starts one item from two clones of one remote at once: exactly one succeeds, and `atelier list` in the other, after a fetch, shows the item held by the winner.
- [x] `submit` refuses while a record the branch changes is uncommitted, naming the paths, then pushes and opens the pull request. On a branch that changes only records, `merge` merges once `atelier check` passes on the merged tree, with no review file asked for. A branch counts as records only when every path it changes against the base is under `.atelier/`; `merge` refuses any other on this path and names the paths, with a test that a `backlog/` branch carrying a change under `crates/` is refused. A branch that adds a mission or changes a mission's plan is refused here and left to atelier-2vdq.
- [x] The act table has rows for `submit` and `merge` of a `backlog/` branch: any thread, in a checkout on that branch, with no status to come from; and they are not the implementer's acts on an item.
- [x] `drop` of a started item is written on the item's own branch, by any thread in a checkout of that branch, and committed and pushed with Git; from that push the item's `submit` and `merge` are refused. The base's copy is dropped by the ordinary record-only path, and until it is, `status` and `list` say the drop has not reached the base. `drop` of an item already merged is refused. Tests run both orders from two clones, drop then merge and merge then drop, and show the base's copy dropped after the item's branch is deleted.
- [x] `status` names every record change in the checkout that is not committed, and every commit on the branch that is not pushed, with the Git command for each.
- [x] `AGENTS.md` no longer says a planning record may be committed on the base; the rest of that file waits for atelier-j5lr.
- [x] The act table in `docs/design.md` and each command's `--help` match what the commands now do.

## Scope

In: `crates/atelier-core` (config), `crates/atelier-app` and `crates/atelier-cli` for the named commands and `start`, their tests, and the one sentence of `AGENTS.md`.

Out: reading records from branches (atelier-dciw); missions' reviews and merge gate (atelier-2vdq); how a merge finishes with a forge and the pre-push rule (atelier-t5am).

## Context

`start` already creates the item's branch and `submit` already opens a pull request; this item gives the planning commands the same shape. Memory of this repository: `start` run in the base's checkout switched it, which is why the last line but two exists.

## Report

Record changes no longer land on the base. `new`, `set`, `ready`, `drop`,
`close`, `lead`, `review` and `verdict` write their files in the checkout
they run in, print the paths and the `git add`/`git commit` line, and commit
nothing. Each refuses, as a rule, in the base's own checkout, and the
planning ones refuse on an item's branch. Only `start`, `submit` and
`merge` act on Git.

What changed:

- `start` makes all three kinds of branch: `start <id>` and `start --new`
  as before, `start --mission "<title>"` (a draft mission committed alone
  on a branch named for it) and `start --backlog <slug>` (a branch at the
  base's tip, no commit). Where a forge's remote is configured it pushes the
  branch before reporting success. A push the remote refuses because it has
  the branch deletes the local branch, fetches the remote's and names its
  holder; an unreachable remote is refused and says so. In the base's own
  checkout `start` makes the branch without switching and prints the
  `git worktree add` to run.
- `config.toml` takes `branch_prefix`, validated as ref components ending
  in `/`. `Config::named` in `atelier-core` is the one reading of a branch
  name back into an ID or a backlog slug; `Config::branch` inverts it.
  Every branch lookup (`Repo::locate`, `current`, `everything`, `new_id`,
  `log`, `show`, `review`, `merge`, `submit`, the pre-push hook) goes
  through it.
- The reader now also reads the forge's remote-tracking branch of an item:
  where it is ahead it wins, and where the two diverged a drop on either
  wins, as `docs/design.md` settles. That is what makes a pushed drop stop
  `submit` and `merge` in another clone after a fetch, and lets a take-over
  start from the remote's branch. Reading every named branch, missions and
  backlog branches included, is still atelier-dciw's.
- `submit` on a backlog or mission branch refuses while a record change is
  uncommitted, naming the paths, then pushes and opens the pull request.
  `merge` on one merges once the records of the merged tree validate, with
  no review file. It refuses, and no skip passes, any path outside
  `.atelier/` or `config.toml`. It refuses, as rules, a branch that adds a
  mission or changes a plan (the sections Why to Authority, or a file
  beside `mission.md` other than `notes.md` and the reviews; left to
  atelier-2vdq), and one that brings an item to the base in a started
  status. The config's `merge_check`s do not run on this path, since
  records alone need `atelier check` only.
- `drop` of a started item runs only in a checkout of its branch, by any
  thread, and prints how to carry the base's copy on a backlog branch;
  `drop` there writes the base's copy with the branch's reason. A drop of an
  item whose branch is in the base's history, local or the forge's
  remote-tracking base, is refused. `status` and `list` name each item
  dropped on its branch whose base copy is not dropped yet.
- The pre-commit hook refuses a commit on the base that adds, changes or
  deletes anything under `.atelier/`, naming `start --backlog` and
  `start --mission`; `ATELIER_SKIP` passes it.
- `status` lists every record change not committed with the commit line,
  and the branch's commits not on the remote with the push. On a dropped
  item's branch whose base copy is not dropped yet, it says so and names the
  backlog step.
- The act table has rows for `start --mission`, `start --backlog`, and
  `submit` and `merge` of a backlog branch: role Planner (no role checked),
  binding nobody, from no status. `docs/design.md`'s table and each
  command's `--help` say what the commands do now, with the parts that wait
  for atelier-2vdq marked.

Tests: `crates/atelier-cli/tests/branches.rs` is new. It starts one item
from two clones at once (one wins; the loser keeps no branch, names the
winner, lists it held after a fetch, and then takes it over), refuses
`start` against an unreachable remote, pushes each kind of branch, runs
`start` in the base's checkout, runs the item loop under `branch_prefix =
"x/"`, merges a backlog branch with no review, refuses one carrying
`crates/lib.rs`, a new mission, a started record or an invalid record, and
runs drop then merge and merge then drop from two clones, with the base's
copy dropped after the item's branch is deleted. One test runs every record
command and checks `git status` shows its change while `HEAD` and every
ref, local and remote, stay put. The hooks tests cover the base rule. The
existing tests now work in a checkout detached at `main`, since the base's
own checkout takes no writes, and commit what the commands print. 317
tests pass; `cargo fmt --check` and `cargo clippy --all-targets -D
warnings` are clean.

Things a reviewer should know:

- The Desk's "say go" calls `ready` in the base's checkout, so it is now
  refused there, as the CLI's is; the Desk's tests say so. atelier-v958
  gives the Desk its say-go on the branch model.
- Until atelier-2vdq, a mission's plan cannot reach the base by `merge`
  without `ATELIER_SKIP`: its branch adds a mission. The mission lifecycle
  test stands a plain Git merge in for it.
- A mission made by `start --mission` is `draft`, with no `planners`; both
  are atelier-2vdq's.
- `start` builds on the local base. In a clone whose local base lags the
  forge's, `git pull` first; reading the base from the remote-tracking ref
  is atelier-dciw's reader rule.
- `AGENTS.md`: besides the sentence in Merge Policy, the Work bullet said
  `ready`, `drop`, `close` and `lead` commit and to push `master`, which is
  now refused, so it says to plan on a backlog branch. The rest waits for
  atelier-j5lr.
- `Github::open` reads the remote's configured URL rather than `git remote
  get-url`, which applies `insteadOf`; the multi-clone tests send a GitHub
  URL to a local bare repository that way.
- A verdict or `ready` passed with `ATELIER_SKIP` writes no trailer, since
  it makes no commit, as the design says of commands that make none.

### Responses

- 1-cj3m.1: fixed in 43f940c82
- 1-cj3m.2: fixed in 43f940c82
- 1-cj3m.3: fixed in 43f940c82
- 1-cj3m.4: fixed in 43f940c82
