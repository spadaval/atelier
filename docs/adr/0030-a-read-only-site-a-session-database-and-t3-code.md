# ADR 0030: A Read-Only Site, a Session Database and T3 Code

## Status

Decisions 2 to 5 are replaced, and Decision 6 widened, by
[ADR 0032](0032-the-desk-is-the-humans-product.md), accepted 2026-11-11.

Accepted (2026-11-08, human-directed: Sumeet decided atelier-7bs1 in a design
discussion; missions atelier-yfqu, `docs/strategies/fleet-dashboard.md`, and
atelier-bnek, `docs/strategies/agent-attribution.md`).

Scopes [ADR 0019](0019-verified-repo-native-tracker.md): its "no cache" and
the removed SQLite crate are about records, which still have no database or
cache. Scopes [ADR 0022](0022-status-orients-and-output-is-dense.md): the
CLI's views also serialize to JSON for the site, and still gain no JSON mode
for agents. Also scopes the Boundaries of
[PRODUCT_INTENT.md](../../PRODUCT_INTENT.md), which this ADR updates.

The decisions take effect as those missions' items merge: the site with
atelier-yfqu, the session database with atelier-bnek. Until then the docs
describe them ahead of the code, and the CLI's HTML brief and debrief pages
stayed until atelier-59fz removed them.

## Context

Sumeet reviews a mission twice, its brief before it starts and its debrief
after it ends. Each is an HTML file the CLI writes into whichever checkout ran it,
so it is in a different place each time and his phone cannot reach most of
them. Between those points he learns where a project stands only by asking an
agent, which he does in nearly every session. Nothing records which thread,
harness, model and role ran each command, so models cannot be compared by
role.

`PRODUCT_INTENT.md`'s Boundaries ruled out the fixes: a second UI, a daemon, a
SQLite cache. They were written against the machinery the rebuild removed (a
workflow engine, a review provider, a SQLite cache of records, a Mission
Control TUI), not as a judgment that a human should not read project state
directly. T3 Code, Atelier and Agent Factory are one stack on this host.

## Decision

1. **Records stay the only source of truth for work.** Tracker records are
   Markdown in Git. The guard decisions of `merge`, `land`, `check` and
   `doctor` consult only Git, plus the forge's CI result
   [ADR 0031](0031-a-forge-carries-branches-and-ci.md) allows. They never
   consult the site, the session database or T3 Code.

2. **The site reads commits and writes nothing.** `atelier serve` is a
   long-running server on this host that shows every Atelier repository here.
   What it says about missions and work comes from one captured commit of each
   repository's base branch. The one exception is the overlay of work in
   flight (branches, approvals and findings), which comes from each owned work
   branch's own records, read at a pinned commit of that branch. It reads no
   working tree and writes nothing to any repository. It may keep views it
   built in memory, keyed by the commits they were read from; that is not a
   record cache, since a commit never changes and nothing else reads it.

3. **The site is read-only until a later decision.** It has no controls. The
   human gives direction in conversation and agents record it with ordinary
   commands. Any action from the site needs a new decision.

4. **A web app over the CLI's own views.** The site is a web app over a
   read-only JSON API. Each response is a view the CLI's Markdown output is
   built from, serialized, with no logic of its own, so the site shows what
   the CLI computes and decides nothing about work. The API exists for the app. Agents keep reading
   the CLI's Markdown (ADR 0022), and nothing tells them to call the API.

5. **Humans read briefs and debriefs on the site.** An agent sends Sumeet the
   page for a mission's brief or debrief instead of writing an HTML file or
   publishing an artifact. Agents keep the Markdown forms of `brief` and
   `debrief`.

6. **A host-wide session database holds only what Git never held.** It may
   record which thread, harness, provider, model, effort and role ran each
   `atelier` command, and what is measured from those runs, such as token
   usage. It is the only copy of that data, not a cache. It may hold no copy or
   cache of records, statuses, reviews or evidence. Only the CLI's commands
   write it, and only `atelier serve` reads it.

7. **Attribution goes through the CLI.** Each command records its own row from
   the environment the session gave it, rather than through an MCP server.

8. **Atelier may depend on T3 Code.** The site and the attribution work may
   read T3 Code's state and link into its threads, and Atelier may ask T3 Code
   upstream for what it needs. Forking T3 Code is a last resort. The guards
   stay free of it (decision 1).

The site's design (crate, server, app, routes and build) is in decisions
atelier-c5vk and atelier-8p7u and the fleet-dashboard strategy; the session
database's is in the agent-attribution strategy.

## Consequences

- Atelier gains a long-running process, a web toolchain in development, CI
  and release, and a JSON contract between Rust and TypeScript.
- Losing or corrupting the session database loses attribution, never work:
  no guard reads it, and Git holds everything else.
- A stale or detached worktree cannot mislead the site, because the site reads
  commits. A page shows the base as of its captured commit, and uncommitted
  work never appears.
- The CLI's HTML pages, and the directory under `.atelier/runtime/` that held
  them, are removed (atelier-59fz).
- The installed binary serves every repository on the host, so a repository
  whose records are newer than it shows an error page until reinstall.

## Alternatives Considered

- **Keep the Boundaries.** Rejected: the human keeps reaching project state
  only through agents, and HTML debriefs stay inside whichever checkout
  generated them.
- **Static pages only.** `atelier site` would regenerate HTML after `land`, and
  `tailscale serve` would publish the folder. Rejected: no live thread status
  and no session database.
- **Record attribution through an MCP server.** Rejected: not every session
  runs one, while every command already inherits the session's environment.
