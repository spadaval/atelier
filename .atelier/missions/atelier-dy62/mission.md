---
schema: atelier.mission
schema_version: 3
id: atelier-dy62
title: Make the tracker one crate, atelier-core, that the CLI and the Desk call
status: done
labels:
- refactor
lead: d7c28b6a-ecd6-469e-8cb8-154984751526
ready:
  by: sumeet, in conversation on 2026-11-14
  at: 2026-11-14T06:21:12.860651075Z
created_at: 2026-11-14T05:56:49.622444154Z
---

## Why

Sumeet asked for this in conversation on 2026-11-14, after asking what an
"act" was and why the Desk could not perform most of them.

The tracker is one library with two programs on it: `atelier`, the command
for agents, and `atelier-desk`, the server of his web tool. The code does
not say so plainly. Three things are in the way.

- **The library is three crates for about 8,000 lines.** `atelier-core`
  holds the domain values (700 lines), `atelier-records` reads and writes
  the files (2,050) and `atelier-app` holds the use cases (5,400). Nothing
  uses one of them without the others. The split costs same-named files in
  different crates (`plan.rs` in two, `review.rs` in three), which gardener
  run atelier-qb4f found agents guessing wrong, and helpers that cannot be
  reached from the crate below.
- **Two names point at the wrong thing.** `docs/design.md` and the items of
  atelier-to83 say "the core" and mean `atelier-app`, while `atelier-core`
  is a different crate. `atelier-app` reads as a desktop app.
- **Git and the forge have no one home.** `git` is spawned from four files
  in three crates: `atelier-app/src/git.rs`, which is the helper;
  `atelier-records/src/at_commit.rs`, which sits below the helper and so
  cannot call it; and the Desk's `fetch.rs` and `stop.rs`. `gh` is spawned
  from `atelier-cli/src/forge.rs` and from the same two files of the Desk.
  The Desk's build script, `build.rs`, also runs `git` to stamp the commit
  it is built from. That runs at compile time, cannot call the crate it is
  building for, and is left as it is.

He also found "act" an odd word for a command that changes a record, and
agreed that "action" is the plain one.

## Outcome

The workspace has three crates. `atelier-core` is the tracker: the domain
values in a `model` module that does no I/O, the record files in `records`,
every `git` call a program makes when it runs in `git`, every `gh` call in
`forge`, the actions and the views. The module of the actions is `acts`
until atelier-nl43 renames it `actions`. `atelier-cli` parses arguments and prints. `atelier-desk` is the
server and the T3 Code client. "The core" in the documents and the crate
named `atelier-core` are the same thing.

No command's behaviour, output or exit status changes, no record format
changes, and no dependency new to the workspace is added. The one change a reader of the tool
sees is the word "action" where the help, the documents and the skill said
"act".

`docs/design.md` records two rules, so the next split is a decision and not
a drift:

- An action's doing moves from the command's file into the core when a
  second caller needs it, and not before.
- A new crate is made when a second program needs part of the core without
  the rest. Until then, a module.

## Success criteria

- three-crates: `cargo metadata --no-deps` lists exactly `atelier-core`,
  `atelier-cli` and `atelier-desk`, and `atelier-core` depends on neither
  of the others.
- nothing-changed: each of the three items leaves every test that passed
  on its own base passing, and changes no test's inputs or assertions. A
  test file's diff is the paths of crates and modules wherever they occur,
  the paths of fixtures that moved, the list of crate names in
  `crates/atelier-desk/tests/dependencies.rs`, and in atelier-nl43 the
  renamed identifiers and the word "act" in expected output.
- model-is-pure: no file under `crates/atelier-core/src/model/` names
  `std::fs`, `std::process` or a module of the crate outside `model`.
- one-git: outside `tests/` directories and build scripts
  (`build.rs`), `Command::new("git")` occurs in one source file of the
  workspace, and a `gh` program is spawned from one source file. If
  atelier-x5al has to stop at the Git half, as its Done when allows, the
  `gh` part is not met and the Result says so and why.
- one-word: `atelier --help`, `docs/product.md`, `docs/product/desk.md`,
  `docs/design.md`, `docs/architecture/desk.md` and `skills/agent-factory`
  say "action", and
  no "act" is left in them as the noun for a state-changing command.
- the-rules-are-written: `docs/design.md`'s crate table has three rows and
  its Decisions hold the two rules of the Outcome.

## Scope

In: `crates/`, `Cargo.toml`, `Cargo.lock`, the Desk's app where it names an
act, `docs/design.md`, `docs/product.md`, `docs/product/desk.md`,
`docs/architecture/desk.md`,
`skills/agent-factory`, and any script or CI file that names a crate.

Out:

- Moving any action's doing out of `atelier-cli`: none has a second
  caller. The forge code is no action and has two callers, the command and
  the Desk, so atelier-x5al moves it.
- One file per command in `atelier-cli`, which gardener run atelier-qb4f
  proposed as P2 and Sumeet has not answered.
- Narrowing what the merged crate exports, or any other cleanup the merge
  makes possible.
- Any change of behaviour, of output other than the word "act", or of the
  record format.
- The four other repositories, the installed binary and a release.

Changed on 2026-11-14: Sumeet dropped atelier-yclc of atelier-to83, which
was to move `submit`, `merge` and the forge code into the core, and
atelier-x5al waited on it for the forge code. On his word atelier-x5al now
moves `forge.rs` itself and waits on nothing. `submit` and `merge` stay in
`atelier-cli`. The documents and the Desk's messages that still name
atelier-yclc are atelier-f3ax's to correct, outside this mission, except the
mission's own Decision in `docs/design.md`, which atelier-x5al corrects.

## Authority

The lead may choose the layout inside `atelier-core` other than the module
names `model`, `records`, `git` and `forge`, the names of
re-exports, and the order of atelier-x5al and atelier-nl43. It may not add
a crate or a dependency new to the workspace (`atelier-core` may take
`serde_json` and `tempfile`, which `forge.rs` uses and the workspace already has), change what any command does or prints beyond the
word "act", move code between `atelier-cli` and the core beyond what an
item names, or change the record format. atelier-nl43 is the item to drop
if the mission has to shrink: it is a rename of about 400 places and
nothing else rests on it. If it is dropped the module stays `acts`, and
the criterion one-word is not met and the Result says so.

## Result

All three items merged on 2026-11-14, each implemented by Claude Opus 5.5
and approved by GPT-6.1 Sol with no findings, and each ending with the same
362 Rust tests passing as its base.

- atelier-ni3d (519984c): `atelier-records` and `atelier-app` are modules of
  `atelier-core`. The workspace is `atelier-core`, `atelier-cli` and
  `atelier-desk`.
- atelier-nl43 (8fcc22c): "action" for "act" in the code, the help, the
  Desk's app and the four documents. The Desk's route is `POST /api/action`.
- atelier-x5al (6843c70): `forge.rs` moved into the core, and the record
  reader and the Desk run `git` and `gh` through the core's `git` and
  `forge` modules.

Every criterion is met, checked on 6843c70. three-crates: `cargo metadata`
lists the three, and the core depends on neither other. model-is-pure: no
file under `model/` names `std::fs`, `std::process` or the rest of the
crate. one-git: `git` is spawned from `crates/atelier-core/src/git.rs` and
`gh` from `crates/atelier-core/src/forge.rs`, and from nowhere else but the
Desk's build script. one-word and the-rules-are-written hold in the
documents. nothing-changed: each item's Report gives its test counts.

What changed from the plan: Sumeet dropped atelier-yclc of atelier-to83
while this ran, so atelier-x5al moved the forge code itself, after a third
and fourth round of plan review. `submit` and `merge` stay in `atelier-cli`.

Left open:

- atelier-f3ax (draft): the documents and two of the Desk's messages still
  name atelier-yclc.
- "Act" as the noun is still in `docs/architecture/quality/standards.md:23`
  and in a field of `scripts/scorecard/`, both outside atelier-nl43's scope.
- The Desk's Git calls now pass `--no-replace-objects`, as every other call
  does. atelier-x5al's Report says why that changes nothing it does.
- The installed `atelier` and the Desk's service are older builds and have
  none of this until they are rebuilt.
