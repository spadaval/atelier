---
schema: atelier.item
schema_version: 3
id: atelier-dciw
title: Read missions and items from named branches, local and fetched, so list and show cover work in motion
status: approved
mission: atelier-to83
labels:
- redesign
priority: medium
blocked_by:
- atelier-ob9u
implementers:
- thread:delegated-task:command%3Amcp%3A93b1b51f-2095-4d1f-af23-677aa6ba570d%3Adelegate-task%3A24e5a5b5-98d7-45f0-a5fe-795b19aba485
ready:
  by: 'sumeet, on 2026-11-13: execute atelier-to83, relayed by the planner thread'
  at: 2026-11-13T23:47:21.436386299Z
trivial: null
caused_by: null
created_at: 2026-11-13T06:14:32.646572592Z
---

## Goal

Make work in motion visible from any clone. The tool reads a started item from
the local branch named for it and everything else from the base, so a plan on
a branch, a backlog change waiting to merge, or an item started on another
host appears nowhere.

## Done when

- [x] The reader in `atelier-app` finds records on every branch the convention names, local and remote-tracking for the configured remote, and gives each record the ref it was read from. The rule for which copy wins is the one in `docs/design.md`.
- [x] A mission whose record is only on its branch is read from there, with its items.
- [x] `show <mission>` on a proposed mission gives each round's verdicts from the mission's branch.
- [x] Proposed means status `open` read from a branch that has not merged, in the reader and in every view. A mission dropped on its branch is not listed as proposed: `list` and `show` give it as dropped, with the reason, and `--proposed` leaves it out.
- [x] `atelier list` shows, apart from the base's backlog, what is proposed: missions whose status is `open`, and record changes, on branches not yet merged, each with its branch. `--proposed` narrows to them.
- [x] `atelier show <id>` finds a record wherever it is and says where it read it from. For a record read from a remote-tracking ref it says how long ago that remote was fetched.
- [x] `atelier status` lists a started item held on another host by the thread its branch names.
- [x] No command fetches. A test builds two clones of one bare remote, plans in the first, pushes, fetches in the second and reads the plan there.
- [x] With 200 named remote branches, `atelier list` takes under two seconds on this host, and the Report gives the time.

## Scope

In: `crates/atelier-app` (`repo.rs`, `git.rs`, `view.rs`, `show.rs`), `crates/atelier-cli`'s output for `list`, `show` and `status`, and their tests.

Out: any write; fetching on a timer, which the Desk does (atelier-v958); a cache on disk, unless the last line cannot be met without one and the Report says so.

## Context

`docs/architecture/desk.md` describes the reader as it is: `Located`, `ReadFrom::Base` and `ReadFrom::Branch`, and a cache keyed on the base's tip and the tip of every local branch named for an item.

## Report

`atelier list` and `atelier show` now see work in motion from any clone:
a plan pushed from one clone shows as proposed in another once it has
fetched, with the branch it was read from, and nothing fetches.

What changed:

- The reader (`crates/atelier-app/src/repo.rs`) lists every branch the
  convention names, local and the forge's remote-tracking ones, with two
  `git for-each-ref` calls (one with `--merged=<base tip>`), in place of two
  `rev-parse` calls and an `is_ancestor` per branch. `Located<R>` and
  `Branch<R>` are generic over items and missions (`view::Record`), and
  `Located::record` picks the copy by the four rules of `docs/design.md`.
  Each copy carries the ref it was read from (`Branch::name`: the branch, or
  `<remote>/<branch>`) and whether the local and remote tips diverged.
- The records an unmerged branch adds, changes or removes come from one
  `git diff base...tip -- .atelier/items .atelier/missions` per branch,
  cached on the `Repo` by tip, so `list` diffs each branch once. A branch's
  own record is left out: that is rule 2. They fill `Located::elsewhere`,
  which gives rule 4 (`also_added`) and the proposed changes (`changes`).
- `Located::proposed` is the definition: open, not on the base, and read
  from an unmerged branch that adds it, or, for a mission, from its own. A
  mission dropped on its branch is dropped, not proposed, and its items are
  not listed as proposed either.
- `Repo::locate`, which the acts use, still reads only the base and the
  record's own branch, so no act can start or merge an item that is only on
  a backlog or plan branch. `Repo::find`, `find_mission`, `everything` and
  `all_missions` read everything. `Repo::missions` now returns every
  mission's copy that counts, base and branches.
- `list` prints a first line naming the remote and the age of the last
  fetch (newest `FETCH_HEAD` of the clone's worktrees), the base's backlog,
  then "Proposed, on branches not yet merged" (proposed missions and items,
  each with its branch), "Record changes on branches not yet merged" (each
  with its branch and the status on the base and there), and "Missions
  dropped on their branch, not merged", with the reason. `--proposed`
  narrows to the proposed part and leaves a dropped mission out. A started
  item read from a remote-tracking branch is marked `on <remote>/<branch>`.
- `show` prints `Read from: <ref>` under the title, with how long ago the
  remote was fetched for a remote-tracking ref, whether it is proposed,
  whether the local and remote tips diverged, the other branches that add
  it, and the unmerged branches that change it. A proposed mission's page
  takes its plan reviews from its branch.
- `status` marks a started item held in another clone `on <remote>/<id>`
  next to the thread its branch's record names.
- The Desk compiles unchanged in behaviour: it passes no proposed missions
  to `list`, so it hides proposed items, and it now sees missions read from
  branches. Showing what is proposed there is atelier-v958's.

Tests, in `crates/atelier-cli/tests/branches.rs`, over two clones of one
bare remote:

- `a_plan_pushed_from_one_clone_is_proposed_in_another_once_it_has_fetched`:
  a mission planned with `start --mission`, an item added with `new --in` and
  an approving plan review, pushed from clone a. Clone b sees nothing before
  `git fetch`, and `list`, `show` and `status` leave its remote-tracking ref
  absent (no command fetches). After the fetch, `list` and `list --proposed`
  show the mission and its item on `origin/<id>`, and `show <mission>` reads
  it from there, with the fetch age and the review's verdict.
- `a_mission_dropped_on_its_branch_is_listed_as_dropped_and_not_as_proposed`.
- `a_backlog_branch_s_record_changes_are_listed_as_proposed_with_its_branch`:
  an item set back to draft and an item added on `backlog/tidy`.
- `status_lists_an_item_started_in_another_clone_with_the_thread_its_branch_names`.
- `list_with_200_named_remote_branches_takes_under_two_seconds`, ignored by
  default because it times the host. On this host `list` took 0.82 s on the
  debug build. On the release build, which is how the tool is installed, it
  took 0.55 s in each of five runs over the same fixture (200 remote
  branches, each an item started in another clone). No cache on disk is
  needed.

`cargo fmt -- --check`, `cargo clippy --all-targets --workspace -- -D
warnings` and `cargo nextest run --workspace` pass: 332 tests passed, and
the timing test was skipped as ignored. The existing views tests now expect
the `Read from:` line, and the two-clone start test expects `on
origin/<id>`.

Left for other items, all outside this item's Done when:

- The base's tip is still the local base, not the forge's remote-tracking
  base that `docs/design.md` names. Changing it moves what `merge` reads,
  which is atelier-t5am's.
- A base mission in the old `draft` status is not shown as proposed. The
  old-format reading is atelier-2vdq's.
- `status` on an item's branch does not yet say that the local and remote
  tips have diverged; `show` does.

Round 1 asked for three fixes, all made in 3e145311f:

- On the branch checked out here, the records it touches are found from
  the checkout's files since the branch forked, committed or not, so a
  record `new` just wrote is in `list` and `show`, also on a backlog branch
  with no commit yet.
- `Entry::startable` is false for a proposed item, so `list --ready`,
  `list --proposed --ready` and `show` no longer offer one, and `start`
  still refuses it.
- A proposed record change is kept as its record would be: `narrows_item`
  and `narrows_mission` in `work.rs` hold the narrowing flags, and `list`
  uses them for its groups and for the changes.

After the fixes, the full suite with the timing test passes: 336 tests.
`list` with 200 named remote branches took 0.81 s on the debug build.

### Responses

- 1-d4f6.1: fixed in 3e145311f
- 1-d4f6.2: fixed in 3e145311f
- 1-d4f6.3: fixed in 3e145311f
