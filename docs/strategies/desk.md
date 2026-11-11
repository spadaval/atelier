---
status: draft
last_changed: "2026-11-11: planning review fixes: acts run where the new commands run and the Desk makes the commit the command asks for, reads take work branches, the mission finishes on converted clones, the database and feed are named"
---

# Why

Sumeet has no tool of his own. The site inside `atelier serve` mirrors the
tracker's records, and a spike that rebuilt it (`spike/mission-control`)
needed five redirections, each for the same reason: a page that lists records
asks him to read records. He needs to decide which mission is worth running,
whether its plan is right, and whether its result is good. The redesign
removes the `brief` and `debrief` commands on the promise that the site shows
them, and the cutover (atelier-qr5s) now deletes the site instead of porting
it, so after the cutover nothing shows them.
[ADR 0032](../adr/0032-the-desk-is-the-humans-product.md) makes the human's
tool a product of its own, the Desk, and
[its intent](../product/desk.md) says what it is for.

# Outcome

On this host, a service reached over the tailnet shows Atelier repositories
on the redesigned records: converted scratch clones of all five until this
host's cutover, and the live repositories after it. A mission has one page with what it is
for, its documents, its items and what happened to them, and the agents
working on it. An inbox lists what waits on Sumeet. From a page he can say go
on a mission, answer an open question, and start a planning session with an
agent. Each act is made by the `atelier` command, under his name, where
that command writes, and saying go ends in the one commit the command asks
its caller to make. `atelier` itself has no site.

# Success Criteria

- desk-matches-cli: with the Desk running as a service on this host, serving
  converted scratch clones of all five repositories and loaded in a headless
  browser over the tailnet hostname, every open mission in every clone has a
  page that renders without an error and shows the title, status and
  per-status item counts that the new `atelier show <mission>` prints in
  that clone, and the inbox lists the same items as `atelier list
  --needs-human` there. Shown by a validation item filed with the last
  increment. It does not need the cutover to have run.
- act-is-a-record: in a converted scratch clone, saying go on a draft
  mission from its page leaves the same record change as the new `atelier
  ready <mission> --by <name>` run in a second clone of the same commit, in
  one commit, and the page then shows the mission ready. Shown by a test in
  the Desk's suite.

# Sizing

- Complexity: high. A new binary, a web app in a second language, a client
  of T3 Code's MCP server, and a path that writes records.
- Value: high. After the cutover it is the only place a brief or debrief can
  be read, and "where are we?" is asked in nearly every session.
- Risk: medium. Reads can be wrong and show it: the first criterion checks
  them against the CLI. An act writes a record, but only by running a
  reviewed `atelier` command in a scratch worktree, and a wrong act is one
  commit to revert.

It is worth its cost because the cheaper path, porting the old site at the
cutover, spends that time on pages Sumeet has already rejected, and doing
nothing leaves him reading `atelier show` for every mission.

# Shape

- **Decisions are in ADR 0032.** Two products, one direction of dependency,
  acts only through `atelier`, T3 Code through its MCP server.
- **The spike is the design reference, not the code base.** Its pages,
  layout and styles are carried over. Its Node server is not: each route is
  rebuilt on the core.
- **Missions first.** The mission page is built before anything else,
  because it is what the cutover leaves missing. Items show inside a
  mission. A page that only lists records is not finished.
- **No database in this mission.** Nothing here needs one. The Desk's
  database is a SQLite file, and a daily run feeds it (ADR 0032, Decision
  6); both come in the next mission, with trends, findings and "since you
  last looked".
- **When qualities conflict:** agreeing with the CLI beats showing more; a
  page the human can act from beats a complete one.

# Design

## Components

| Part | Where | Owns |
| --- | --- | --- |
| `atelier-desk` | `next/crates/atelier-desk/`, moved to `crates/` by the switch | The HTTP server, the API's response types, the T3 Code client, and running acts |
| The app | `desk/` at the repository root | Pages, components and styles; its unit and browser tests |
| The core | `atelier-app` (`view.rs`), `atelier-records` (`at_commit.rs`) | Reading records at a commit and computing views. The Desk adds views here when one is missing; it never re-derives one. |

`atelier-desk` depends on `atelier-app` and `atelier-records`. No other
crate depends on it, and a test in the workspace fails if one does.

## Data flow

1. **Reads.** For each repository under its roots, the server captures the
   base branch's commit and the tip of each local branch named for an item,
   loads records at those commits with `CommitReader`, and asks
   `atelier-app` for views. A started item's status, holder, reviews and
   open question live on its branch, so the server takes each item from
   where the core's own rule takes it (`view::entries`, as `status`, `list`
   and `show` do on the base) and never from the base alone. Responses are
   cached by the set of commits they were read from. It reads no working
   tree.
2. **Threads.** At start the server mints a read-only token (`t3 auth
   session issue --subject mcp-client --scope orchestration:read`) and calls
   T3 Code's MCP server over HTTP. It polls `t3_thread_list` filtered to
   live and unsettled threads, and reads one thread's runs on demand. A
   thread is tied to an item by the thread IDs the records hold
   (`redesign-architecture.md`, The log), not by its title.
3. **Acts.** The server runs the new `atelier`, the binary built from the
   same workspace commit as itself and named in its configuration; before
   the cutover that is not the installed one. It runs each command where
   that command is written to run, with no thread variable set, so the
   caller is `human`:
   - `ready <mission> --by <name>` in the checkout that has the base branch
     out. The command refuses anywhere else (`plan.rs`, `off_base`). It
     writes the record and does not commit: it prints the commit for its
     caller to make, `git add -- <paths> && git commit -m "<message>" --
     <paths>` (`output.rs`, `commit_line`). The server makes that commit,
     as an agent following the line would. Before it runs the command it
     refuses if the mission's directory has an uncommitted change in that
     checkout, so the commit holds only what `ready` wrote. Afterwards it
     compares the paths that changed under `.atelier/` with the paths in
     the printed line. If they match it commits exactly those paths with
     the printed message, which leaves every other staged or changed file
     in the checkout alone. If they do not match, or there is not exactly
     one such line, it commits nothing, shows the output, and gives the
     line to run by hand.
   - `decide <id> "<answer>"` in the checkout that has the item's branch
     out, which is where the implementer works (`questions.rs`, `decide`).
     The command writes the answer and the implementer's `submit` commits
     it, so the server commits nothing. Pages are read from commits, so
     until that commit exists the server shows the question as answered
     from the Desk and not yet committed, from its own memory of the act.
   The server finds the checkout from `git worktree list`. If no checkout
   has the branch out, it refuses and says so. It never makes a scratch
   worktree or moves a ref: the spike's server did both, for the old
   commands. A refused command's output is shown as it is.
4. **Sessions.** Starting a planning session calls `t3_thread_launch` with a
   second token that may operate, the repository's T3 project, and a first
   message naming the mission. The page then links to the thread.

## Interface

A JSON API under `/api`, with its types generated for the app by `ts-rs`, as
the old site's were. The first increment writes the route table and each
response's fields as a document before any code
(`docs/architecture/desk.md`), from the pages the spike has and the views
the new core has. Expected routes: repositories; a repository's missions;
one mission with its documents, items, plan reviews and history; one item;
the inbox; live threads; and one `POST` for acts.

## Dependencies

- `axum` and `tokio` for the server, as the old site used.
- `ts-rs` for the app's types, as the old site used. It is a dependency of
  the old workspace only, so this mission adds it to `next/`'s.
- The app keeps the spike's stack: Vite, React, react-router,
  react-markdown, lucide icons, plain CSS with tokens.
- No SQLite crate in this mission.

## Development

A change to the app never compiles Rust: Vite's dev server proxies `/api`
to a running `atelier-desk`. Prototype routes may be served from Vite's dev
server while a page is being tried. Nothing ships from them.

## Access

The service binds to the tailnet address and to loopback. It refuses a
request whose Host is not one it answers to, and an act without the app's
header or from another origin, as the spike does. There is no sign-in.

# Derisking

The riskiest assumption is that the new core's view layer can feed the
mission page. Today `view.rs` has three public functions, and the old site
had views for a board, an inbox, a brief and a debrief. The first increment
tests it on paper: it lists every field each page shows against the view
that gives it, and names each view the core lacks, before the server is
written. If the gap is large, adding views to `atelier-app` is the mission's
first code and is sized there.

The second is the cutover's timing. The Desk is built, installed and
validated against scratch clones converted by the rehearsed converter
(atelier-1sbw, atelier-nafc), so the mission needs no repository to be
converted and can close before the cutover runs. Pointing the service at the
live repositories is a loose end outside the mission (atelier-qxmr), which
waits on this host's cutover (atelier-wd1i). The cutover waits on nothing
here: its step E removes only the tracker's site and leaves the Desk's crate
and CI jobs alone.

Reading T3 Code is settled: the probe noted on atelier-aab1 ran the token
and the read calls on this host. Starting a thread was not exercised. The
increment that adds sessions proves it against the running T3 Code before
it is called done.

# Scope

- In: the `atelier-desk` crate; the app; the mission, item, inbox and agents
  pages; the acts `ready` and `decide`; starting a planning session;
  installing the service on this host over converted clones; the Desk's own
  CI jobs; `docs/architecture/desk.md`.
- Out: health, the feed and any database (the next mission); sessions for
  gardening and grooming; what changed since the human last looked;
  notifications; token and cost figures, which T3 Code's MCP server does not
  give; a release artifact or installer; sign-in; the old site, which the
  cutover deletes; pointing the service at the live repositories, which is
  the loose end atelier-qxmr.
- Target environment: this host, over the tailnet, at desktop and phone
  widths; the five repositories as converted scratch clones until the
  cutover, and live after it.

# Authority

The orchestrator may choose the API's routes and fields, the crate's module
layout, which views to add to `atelier-app` and how, and how pages are
arranged within the spike's design. It may add a dependency the Design does
not name when a step needs it and say why in the item.

It may not expose the service beyond the tailnet, add an act other than
`ready` and `decide`, add a database, read T3 Code's own database, change a
tracker command's behaviour, or change the specifications' Words, Lifecycle
or Review sections.

# Risks and Unknowns

- **The view gap is large.** Then the first code is views in `atelier-app`,
  and the orchestrator re-sizes the later increments at that boundary.
- **The core still moves.** atelier-fcb7's last item and the cutover edit
  `next/`. A breaking change shows as a failed build of `atelier-desk` in
  CI, which the change's author fixes with the change.
- **The gap after the cutover.** If the cutover runs before the mission
  page is live, Sumeet reads `atelier show <mission>` until it is.
- **An act needs a checkout.** `ready` needs some checkout with the base
  out, and `decide` one with the item's branch out. Where none exists the
  Desk refuses and names the command to run by hand.
- **The Desk reads a commit line the tool prints for people.** If the
  tracker rewords that line, saying go stops committing and shows the
  output instead; the act-is-a-record test fails first. A `ready` that
  commits on request would remove the reading, and is a change to the
  tracker that needs its own decision.
- **No sign-in.** Anyone on the tailnet can act as Sumeet. Accepted for one
  user on a private tailnet; a second user needs a decision first.
- **Whether T3 Code's thread links open on the phone** is the one question
  atelier-aab1 leaves open. If they do not, the Desk links to the host's own
  T3 Code address.
