# ADR 0032: The Desk Is the Human's Product, Beside the Tracker

## Status

Proposed (2026-11-11, drafted at Sumeet's direction in a design discussion
over the dashboard spike on branch `spike/mission-control`; mission
atelier-z4cr, docs/strategies/desk.md). It takes effect when he approves
that mission's brief.

Replaces Decisions 2 to 5 of
[ADR 0030](0030-a-read-only-site-a-session-database-and-t3-code.md) and
widens its Decision 6. ADR 0030's Decisions 1, 7 and 8 stand.

## Context

ADR 0030 gave the human a read-only site inside `atelier`: pages over the
views the CLI computes. Using it, and then a spike that rebuilt it, showed
the same fault five times over. A page that mirrors the tracker's records
asks the human to read records. He needs to make a small number of decisions:
which mission is worth running, whether its plan is right, whether the ground
agents work on is sound, and whether the result is good.

The two users also differ in every way that shapes a tool. Agents run
missions unattended, one repository at a time, through commands and dense
Markdown. The human plans, tends and judges, across every repository on the
host, in conversation with an agent, at a desk or on a phone.

The spike also settled three questions of fact. T3 Code gives a process
outside a thread a supported way in: a scoped token from `t3 auth session
issue --subject mcp-client` and its MCP server, which lists threads, reads
their runs and tool calls, and starts and messages threads (atelier-aab1).
The redesigned core in `next/` already reads records from a commit and has a
view layer. And a server written outside the core spends most of its code
re-deriving what the core knows, by parsing command output and grepping
records.

## Decision

1. **Two products.** The tracker is for agents: the `atelier` command and
   its records. The Desk is for the human: where he plans missions, tends
   repositories and reads results. They share a vocabulary and the core,
   which reads records, computes views and performs acts for both.

2. **`atelier` has no site.** `atelier serve` and the `atelier-site` crate
   are deleted with the old workspace and are not rebuilt. The Desk shows a
   mission's brief and debrief. Until it does, `atelier show <mission>` is
   how the human reads a mission.

3. **The Desk is a separate build.** A binary crate, `atelier-desk`, links
   `atelier-app` and `atelier-records` and serves the Desk's API. A web app
   at `desk/` is its interface, and the binary carries the app's built
   files. It runs as one service per host, reached over the tailnet. It is
   not part of the `atelier` release or its installer until a second host
   needs it.

4. **One direction of dependency.** `atelier-desk` depends on the core
   (`atelier-app` and `atelier-records`). No tracker crate depends on `atelier-desk`, and no
   check, merge or hook consults the Desk or its database (ADR 0030,
   Decision 1).

5. **The Desk acts only through the core.** It offers the acts that are
   the human's, such as `ready` and `decide`. It performs each by calling
   the same function in the core that the `atelier` command calls, naming
   the person acting, and it gets the result as data. It runs no `atelier`
   binary and reads no command output. The Desk writes no record itself
   and holds no decision the records lack. It is not a record editor.

   Where the Desk needs something the core lacks, a view or an act, it is
   added to the core, where the command line uses it too. The Desk keeps no
   copy of the tracker's logic, and the command line gets no output format
   for the Desk's sake.

6. **One host-wide SQLite database for what Git never held.** ADR 0030's
   session database is also the Desk's, and it is one SQLite file on the
   host. It may hold which thread, model and role ran each command (written
   by `atelier`), measurements and findings about how work went (written by
   a feed that runs daily, unattended), and what the human has seen
   (written by the Desk). It holds no copy of a record. Agents read findings
   through a command, never from the file.

7. **T3 Code through its MCP server.** Live pages read three things: records
   through the core, T3 Code through its MCP server with a scoped token, and
   the database. Only the feed may read anything else, such as transcripts,
   through one adapter per source, and what it stores says where it came
   from. Nothing reads T3 Code's own database for a live page.

8. **Conversations stay in T3 Code.** Planning, gardening, grooming and
   spikes are sessions with an agent. The Desk starts a session and links to
   it. It is not a chat client.

## Consequences

- The cutover gets smaller: it deletes the site instead of porting it, and
  the `atelier` binary needs neither Node nor a web build. The Desk has its
  own build and its own CI jobs, which run only for changes that touch it.
- From the cutover until the Desk's mission page is live, the human has no
  brief or debrief page. He reads `atelier show <mission>` and the mission's
  files.
- The Desk can change daily without a tracker release, and a tracker user
  who wants no Desk installs none.
- The Desk is tied to the core. A change there can break the Desk's
  build, which is where such a break should show.
- The tracker changes for the Desk's sake: its acts move out of the
  command-line crate into the core, and `ready` commits its own write.
- A second service runs on the host, with a token that can read, and where
  the human starts a session, operate, T3 Code.

## Alternatives Considered

- **Keep the site inside `atelier serve`.** One install, but every interface
  change is a Rust build and a tracker release, and the tracker gains a
  dependency on T3 Code and a host database.
- **A Node service over `atelier serve`'s JSON.** The spike's shape. It
  iterates fastest and needs two services, and its server re-derives the
  core's knowledge from text.
- **The core as a native Node module or WebAssembly.** It keeps TypeScript
  on the server at the cost of a second build chain, and the core runs
  `git`, which WebAssembly cannot.
- **Port the old site at the cutover and build the Desk later.** It leaves
  no gap in brief and debrief pages, and it spends the cutover's time on
  code this decision deletes.
