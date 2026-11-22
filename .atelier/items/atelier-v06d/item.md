---
schema: atelier.item
schema_version: 3
id: atelier-v06d
title: 'Make an unstarted item open: remove draft, ready and the ready command from the binary, the records and the documents'
status: in_review
mission: atelier-dh7a
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A27af7c87-abb4-4f3f-9cc7-730af698e6bc%3Adelegate-task%3Af016502d-77ea-4d2c-b468-d9958a9dc24c
ready:
  by: Sumeet, agreed plan of atelier-dh7a
  at: 2026-11-22T17:38:00.693469211Z
trivial: null
caused_by: null
created_at: 2026-11-19T15:25:03.017123820Z
---

## Goal

An item is born `draft` and needs `atelier ready` before `start` takes it.
The step records nothing a started status does not, and leads run it as a
ceremony (the mission's Why has the evidence). After this item an unstarted
item is `open`, `start` takes any open item whose blockers are met, and the
`ready` command is gone. Records written before still read, here and in the
five other repositories the installed binary serves, because nothing
converts them.

## Done when

- [x] `atelier new` writes `status: open`. Once the branch it was filed on
  has merged, `atelier start <id>` starts that item with no readying
  between, and a test holds it. A test also holds that an open item on an
  unmerged branch is still refused by `start`.
- [x] `atelier ready <id>` is refused for an item as it is for a mission,
  with a line that says items need no go-ahead, and it is in no help text,
  no `status` output and no row of the action table.
- [x] A record that says `draft` or `ready` reads as `open` everywhere the
  binary reads items, and is never rewritten by a command that does not
  otherwise change it. A test reads one of each. An old record's `ready.by`
  still prints wherever `atelier show` printed it.
- [x] An old mission's `draft` still reads as proposed: the action table
  no longer stands a proposed mission on an item's `draft` or an agreed
  one on an item's `ready` (`MissionStatus::status()` does today), and a
  test holds that a mission stored as `draft` cannot be led and that
  `start` refuses its items.
- [x] `atelier list` keeps a way to ask for open items that can start now,
  under a flag that does not name a removed status.
- [x] `atelier log` still prints an old record's ready line from history,
  and writes none for a new item.
- [x] `atelier check` passes on this repository's records unchanged, and
  `atelier list` and `atelier check` succeed in a checkout of astromech,
  life-on-rails, cosplay-studio, demiurge and worldcrafter; the Report
  gives the commands and their exit statuses.
- [x] Every fixture and case in `evals/agent-factory` sets up and grades
  under `test.sh` with no `atelier ready` and no draft step.
- [x] The skill and the documents say what the binary does: no file under
  `skills/agent-factory` tells an agent to run `atelier ready` or calls an
  item a draft; `procedures/garden.md` starts its run without readying it,
  files something broken as an open item, and loses the "waiting to
  start" number from its steps and from its page template, with nothing
  in its place; `docs/product.md`'s Item table, `docs/design.md`'s status line,
  action table and decision row, `docs/architecture/desk.md`, `AGENTS.md`
  and `scripts/cutover/agents/` match.
- [x] The Desk's app and the extension show and count open items at this
  merge: a loose open item and an open item in a mission, as the server
  now returns them, appear on the Desk's project page and mission page and
  on the extension's home and mission pages, and a test in each holds it.
  CI's `desk` and `extension` jobs pass. How those pages lay the items out
  is atelier-rme3's.
- [x] The Report gives the lines added and removed under `crates/`.

## Scope

`crates/`, the Desk's server included; `evals/agent-factory` fixtures and
cases; `skills/agent-factory`; the documents named above. In `desk/src` and
`t3-extension/src`, only what keeps open items shown and counted. Not `new` on an
item's branch (atelier-u6oy), not the instruction to file (atelier-8phx),
not priority. No status, field or check is added in place of what goes.
atelier-kw8u's items change `evals/agent-factory` too: merge the base in
before submitting.

## Report

An item now starts with no go-ahead. `new` writes `status: open`, `start`
takes any open item on the base whose blockers are done, and `atelier
ready` is refused with "Items need no go-ahead". Old records read without
conversion: `check` and `log` print byte-identically under the installed
binary and this one in all six repositories.

### Before this is installed: three programs and one CI pin

A binary from before refuses `open`, so these must move before any
repository holds an open record:

- **`atelier`, `atelier-desk` and the T3 extension on this host, together.**
  The mission plans only the binary's reinstall. I gave the installed binary
  one open item in a sandbox clone: `status` and `list` exit 2 for the whole
  repository and `check` exits 1. The running Desk was built on 2026-11-16
  on the same reader. An extension from before has no glyph for `open`, so
  by reading its code its project home fails on one (I did not run it).
- **astromech's CI.** Its `.github/workflows/atelier.yml` pins `ATELIER_REV`
  to v0.7.0 (24a00e836), whose `Status` has no `open`, so its `atelier
  check` job will fail on the first open record there. That needs a change
  in astromech, which this item may not write. The other repositories'
  workflows on their bases run no `atelier check`.

`docs/design.md` (Records, items) now says the first of these.

### What changed

- **Reading old records.** `Status::Open` has serde aliases for `draft` and
  `ready`. `Store::write_item` leaves a file that already reads as the item
  it is given, so a no-change `set` respells nothing. A real change writes
  `open` and keeps `ready.by`, which `show` still prints. Only `log` reads
  the old spelling.
- **Missions.** `MissionStatus::status()` and `of()` are gone.
  `actions::ON_MISSION` holds a mission's transitions in its own statuses,
  and the pre-commit hook judges each record by its kind.
- **`ready`.** It is a hidden command that only refuses, for any argument,
  and has no row in the table.
- **`list --startable`** replaces `list --ready`.
- **The clients.** Loose open items show and count in Next; Later holds
  proposed plans. An open item in a mission shows and counts on both
  mission pages.
- **The cutover kit.** `rehearse.sh` also named `ready` and `list --ready`,
  and no longer does.

### The lead's answer: `status` is capped at 10

With no ready mark, `status` would have listed every open item at each
session start. I asked the lead as advice. They chose my default: the 10
most urgent by priority and then ID, then how many more `atelier list
--startable` lists, with what a thread holds and a lead's own mission left
whole, in a commit of its own (a89f1fc9a). Its test is
`status_names_the_ten_most_urgent_that_can_start_and_a_leads_own_items_whole`.

| `atelier status`, bytes | installed | open, uncapped | open, capped |
| --- | --- | --- | --- |
| astromech (35, then 166 can start) | 9,713 | 19,947 | 6,817 |
| atelier on master (2, then 41) | 2,290 | 6,429 | 3,314 |
| life-on-rails (13, then 31) | 3,798 | 5,271 | 3,516 |
| cosplay-studio (0, then 7) | 850 | 1,549 | 1,549 |
| demiurge (0, then 8) | 687 | 1,573 | 1,573 |

### Lines under `crates/`

1,191 added and 1,097 removed. In `crates/*/src` it is 371 added and 447
removed, and `scripts/count_non_test_rust.py` goes from 24,409 to 24,333:
the binary shrinks by 76 lines. The tests are 820 added and 650 removed.

### Behaviour a reader may not expect

- Items that were held back as drafts can start, and a lead's handoff names
  each of them. `blocked_by` is now the only way to hold one back.
- `list --proposed --startable` now keeps a change to an item that can
  start, where a draft's was filtered out.
- `show` no longer prints "Not made ready yet".

### Left, for the lead

- The wire fields named `ready` in `/api/launch` and `/api/motion` keep
  their names, so this change breaks no reader of them. They hold open
  items that can start.
- For atelier-rme3: the Desk's item page still says "Ready: Not yet" for an
  item with no old mark, both glances say "Nothing is ready to start", and
  a proposed mission still borrows the `draft` glyph.
- `scripts/cutover/agents/` and `rehearse.sh` still describe the model from
  before branches (planning on the base, `escalate`, commands that commit).
  I changed only their `ready` lines. This wants an item of its own.
- `scripts/scorecard/draw_samples.py` dates a mission's start from its
  `ready` log line, which no mission has had since 2026-11-13.
- `~/projects/worldcrafter` is a symlink to demiurge, so the mission's "five
  others" are four repositories, and its nine draft missions are eight.

### Validation

Run on this branch after the last code commit (fc6c137a8).

`atelier` here is `target/debug/atelier` of this branch, run in each
repository's own checkout with `GIT_OPTIONAL_LOCKS=0`. Each checkout's refs
and `git status` hashed the same before and after.

| Checkout | `atelier check` | `atelier list` | `atelier log` | `atelier list --startable` |
| --- | --- | --- | --- | --- |
| atelier (this branch) | 0, identical | 0 | 0, identical (1,178 lines) | 0 |
| astromech | 0, identical | 0 | 0, identical (674 lines) | 0 |
| life-on-rails | 0, identical | 0 | 0, identical (314 lines) | 0 |
| cosplay-studio | 0, identical | 0 | 0, identical (38 lines) | 0 |
| demiurge | 0, identical | 0 | 0, identical (16 lines) | 0 |
| worldcrafter | 0, identical | 0 | 0, identical (16 lines) | 0 |

Each cell is the exit status; "identical" compares standard output and
standard error with the installed binary's. `list` differs as it should.

- **Old draft missions.** In clones of mirrors with no path back to the real
  repositories, each of the eight shows as proposed and `lead` of it exits
  1. `start` of up to three unstarted items of each was refused as proposed;
  cosplay-studio-ro6g has no such item.
- **The Desk on real records.** An `atelier-desk` and app built here served
  those clones, one with a newly filed open item. `desk-matches-cli` exits
  0: 16 missions and 4 inboxes match the CLI, and `/api/roadmap` and
  `/api/launch` return the new item.
- **Rust.** `cargo fmt -- --check`, `cargo clippy --all-targets --locked --
  -D warnings` and `cargo nextest run --locked` exit 0 (462 passed). The
  ignored 200-branch `list` timing test passes too.
- **Evals.** `evals/agent-factory/test.sh` exits 0, and no setup runs
  `atelier ready` or writes a draft or ready item.
- **Desk.** `pnpm typecheck`, `pnpm test`, `pnpm build` and `pnpm e2e` exit
  0.
- **Extension.** `pnpm typecheck`, `pnpm build`, `node run-tests.mjs` (7 of
  7) and `scripts/t3-extension-release.test.sh` exit 0. The last needed the
  working `pnpm` first on `PATH` on this host.
- **Scripts.** The cutover and scorecard unit tests exit 0. `rehearse.sh`
  was only syntax-checked.
- **Not run:** the eval cases with a model, and the installed extension.
  CI starts when `submit` opens the pull request, so the CI line above is
  ticked on the same jobs' commands run here, and `merge` waits for CI's
  own result.
