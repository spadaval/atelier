---
schema: atelier.item
schema_version: 3
id: atelier-v958
title: Show on the Desk what is proposed and in motion and who is doing it, from the live repositories
status: in_review
mission: atelier-to83
labels:
- redesign
priority: medium
blocked_by:
- atelier-dciw
- atelier-2vdq
- atelier-xbog
implementers:
- thread:delegated-task:command%3Amcp%3A93b1b51f-2095-4d1f-af23-677aa6ba570d%3Adelegate-task%3Ae23abf7c-959e-4c22-b50c-482d044d065d
ready:
  by: 'sumeet, on 2026-11-13: execute atelier-to83, relayed by the planner thread'
  at: 2026-11-14T01:30:23.044847409Z
trivial: null
caused_by: null
created_at: 2026-11-13T06:14:32.720158409Z
---

## Goal

Make the Desk the place Sumeet sees everything, since nothing asks for his
go-ahead any more. It reads scratch clones frozen on 2026-11-11, shows only
what is on the base, and its "say go" has no meaning under this mission. This
item takes in atelier-qxmr.

## Done when

- [x] The installed service's roots are the five live repositories, and for one open mission in each the page's title, status and item counts match `atelier show <mission>` there.
- [x] The Desk fetches each repository's remote on a timer the config sets, and each page says when it last did.
- [x] A page lists what is proposed in each repository: plans and record changes on branches, with the branch, its pull request where there is one, and its current round's verdicts. A plan dropped on its branch is not listed as proposed.
- [x] A page lists work in motion: each started item with its status, read from its branch, and the thread on it, with whether T3 says that thread is alive; and each mission in progress with its lead.
- [x] Sumeet is told when a plan is proposed and when it merges, by the means `docs/product/desk.md` settles, and the page links the plan.
- [x] The decide page and its act are gone, with the code that finds a checkout to write an answer into. A page lists what `atelier list --disputed` gives, with each side's model where T3 knows the thread.
- [x] "Say go" is gone. A mission's page has "stop" for a proposed plan, which drops the mission with his reason as atelier-2vdq defines a stop, and the act is the same record `atelier drop` writes, through one function in `atelier-app`. The Desk carries that stop through itself: it writes the drop in a worktree of its own, commits it, pushes the branch and closes its pull request with `gh pr close`, and the plan is shown as stopped once the drop is pushed and the pull request closed. An open mission's page offers no stop act in this item (atelier-yclc carries that one through): it shows the commands that stop it, and a stop someone has written shows as pending until its record-only branch has merged, and then as stopped. The interface test covers the proposed plan's stop and the open mission's pending and stopped views.
- [x] The Desk's writes go through a checkout of the branch they change and never touch a served repository's own checkout.
- [x] `docs/product/desk.md` and `docs/architecture/desk.md` describe all of this, and `pnpm test` and `pnpm e2e` pass.

## Scope

In: `desk/`, `crates/atelier-desk`, the Desk's two documents, the installed service's configuration on this host.

Out: a finder (atelier-ic22); a mission's history page (atelier-hj6b); any read the reader of atelier-dciw does not give, which is filed against that item.

## Context

`crates/atelier-desk/src/config.rs` holds `roots`. `desk/README.md` describes the checks `desk-matches-cli` and `act-is-a-record`, which this item keeps passing against the live roots.

How Sumeet is told of a plan is open, and this item settles it in `docs/product/desk.md`. Read `.atelier/missions/atelier-to83/notes.md` first: it names a candidate he agreed to as a separate item (atelier-plr9) and one means he turned down (T3's Browser panel).

## Report

The Desk now serves the five live repositories under `~/projects/`, fetches
each one's remote every five minutes, and shows what is proposed, what is in
motion and who holds it, and what two models disagreed on. Sumeet can stop a
proposed plan from its page: the Desk writes the drop through the same
function `atelier drop` calls, pushes the plan's branch and closes its pull
request. "Say go" and the answer form are gone.

The lead changed two things during the work (2026-11-13, in answer to my
questions):

- **The stop of an open mission moved to atelier-yclc.** Publishing it needs
  `atelier submit` and `atelier merge`, which live in `atelier-cli`, and ADR
  0032 says the Desk runs no `atelier` binary. The lead would not grant an
  exception, so atelier-yclc moves submit and merge into the core first. I
  rewrote the "Say go" Done when line to match. Here an open mission's page
  shows the commands that stop it, and it shows someone else's stop as
  pending until its branch merges, then as stopped.
- **Notifications go out by mail, and stay off until Sumeet turns them on.**
  The Desk runs a command from `[notify]` in its config. On this host that
  command is `desk/service/notify-mail`, which mails him from his own Gmail
  account to itself through `gog`. Mail from his account on a timer is his
  decision, so the installed config has the table commented out. To turn it
  on, uncomment these two lines at the end of
  `~/.config/atelier-desk/config.toml` and restart the service:

  ```toml
  [notify]
  command = ["~/.local/share/atelier-desk/notify-mail", "personal"]
  ```

  The inbox lists the news ("Lately") either way.

### What changed

- `atelier-app`: a mission's `drop` moved here from the CLI as
  `plan::drop_mission`, with `plan::agreed` and the two result texts. The
  `atelier drop` command calls it, and so does the Desk's stop, so both
  write the same record. The Desk's old `decide` is removed (`decide.rs`,
  `Act::Decide`, `Role::Decider`, the conditions `ReviewedFinding` and
  `NoQuestion`, the say functions and `MissionView::questions`), since its
  own doc comment said this item removes it. The binary still reads old
  Decisions sections.
- `atelier-desk`:
  - The config adds `repos` (explicit repositories), `fetch_seconds`,
    `state`, `url`, `gh` and `[notify]`.
  - `fetch.rs` fetches each repository on a timer and reads the forge's pull
    requests with `gh pr list`.
  - `news.rs` notices proposals and merges, keeps what it has seen in
    `<state>/news.json`, and runs the notify command.
  - `stop.rs` stops a proposed plan in the Desk's own clone at
    `<state>/work/<repo>`, never in a served checkout.
  - `read.rs` reads branches through the dciw reader and gives each mission
    a state: proposed, agreed, stop pending, stopped or done.
  - New routes: `/api/proposed`, `/api/motion` and `/api/disputed`.
    `/api/act` now takes only `stop`.
  - The T3 poll also reads recent ended threads, so a dispute can name each
    side's model.
- `desk/`:
  - New pages: Proposed, In motion and Disputed.
  - The mission page shows its state, the branch it was read from, its pull
    request, the verdicts of its last round and the stop control.
  - The inbox lists proposed plans and recent news in place of drafts and
    questions.
  - Every page that reads a repository says when it was last fetched.
- `desk/service/`:
  - `install.sh` writes the live repositories into a new config and installs
    `notify-mail`.
  - `make-clones.sh` and its test are removed: the clones are no longer
    served.
- `desk/live/`:
  - `desk-matches-cli` takes the repositories directly and checks proposed
    missions too.
  - `act-is-a-record` now checks the stop against `atelier drop`.
- Documents: `docs/product/desk.md` settles how Sumeet is told and how a
  stop works, and lists the pages. `docs/architecture/desk.md` describes
  every new field and route, fetching, the stop and being told. In
  `docs/design.md` I changed three lines that named the Desk's `decide` or
  said the stop goes "through the same functions".

Outside the item's Scope, so a reviewer should look at them: the move of
`drop` and the removal of `decide` in `atelier-app`, the one-line change to
the CLI's `drop`, and the three lines in `docs/design.md`.

### The installed service

The service on this host runs this branch's build, 932f1cc7. I installed it
with `desk/service/install.sh` from a clean checkout of this branch, because
the first Done when line asks for the installed service. Its config, backed
up first to `~/.cache/atelier-desk/config.toml.before-v958`, now names the
five live repositories, `fetch_seconds = 300`, the state directory and the
Desk's tailnet URL. Reinstall from master after the merge.

I have not run a real stop against a live repository. That would push to
GitHub and close a real pull request. So two things are untested under
systemd: the push over SSH, and `gh` being signed in.

### Validation

- `desk-matches-cli` against the installed service and the five live
  repositories, using the CLI built from this branch: "All 16 missions and 5
  inboxes match the CLI." That covers agreed, in-progress and proposed
  missions in all five, and each repository's needs-human list.
- `act-is-a-record ~/projects/atelier` passes: the Desk pushed one commit,
  and its message, path and 36-line record equal the CLI's. It also closed
  the pull request, and the page shows "Stopped". With `--differ` it fails
  as it should.
- `/api/repos` on the installed service shows each repository fetched from
  `origin` seconds after the start. `/api/proposed`, `/api/motion` and
  `/api/disputed` read all five repositories without error. In motion lists
  this item's thread as live.
- `cargo nextest run`: 350 passed, 1 skipped. `cargo clippy --all-targets
  -- -D warnings` and `cargo fmt -- --check` are clean.
- `crates/atelier-desk/tests/acts.rs` runs against a bare remote with a
  `gh` stand-in:
  - It stops a plan through `POST /api/act` and finds the drop pushed and
    the pull request closed.
  - The record equals `drop_mission`'s in another clone.
  - The served repository's HEAD, files and local branches are unchanged.
  - The page shows the plan stopped, or stop pending while the pull request
    is open.
  - An open mission's stop shows as pending until its backlog branch
    merges, then as stopped, and the Desk refuses to make it.
- `crates/atelier-desk/tests/desk.rs` covers the proposed, in-motion and
  disputed pages from local branches. `news.rs` has a unit test of proposed
  and merged news, with no repeat across a restart and the stub command's
  arguments. `config.rs` tests the new keys.
- `pnpm typecheck`, `pnpm test` (41 passed), `pnpm build`, and `pnpm e2e`
  (48 passed, desktop and phone). The interface tests cover the plan's
  stop, its refusal, Not yet, an open mission's commands, a pending stop,
  and each new page.
- `desk/service/notify-mail.test.sh` passes against a `gog` stand-in, and
  nothing was sent.
