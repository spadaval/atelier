---
status: draft
last_changed: "2026-11-08: first draft, from Sumeet's side-chat design session"
---

# Why

Sumeet can only learn where his projects stand by asking an agent, and he asks
"where are we on this project?" in nearly every session. Documents written for
him, such as a mission's HTML debrief, land in `.atelier/runtime/reviews/`
inside whichever checkout generated them, where his phone cannot reach them and
he has to know which worktree to look in. Codex models now share the work with
Claude, and nothing records which model implemented, reviewed or planned what,
so error rates, review counts and token cost cannot be compared by model or
role. T3 Code, Atelier and Agent Factory are his whole stack, so Atelier may
lean on T3 Code wherever that helps.

# Outcome

A site on this host, reached over Tailscale, shows Sumeet every Atelier
repository here: an inbox of everything waiting on him, each with a one-click
link to what he needs to read; each repository's state (waiting on him, in
progress, up next, recently done); and each mission's strategy, items and
debrief. Every `atelier` command an agent runs inside a T3 thread records which
thread, harness, provider, model, effort and role did it, and review records
name their reviewer's provider and model in structured fields. The site shows
which agent is working on what, links each thread into T3, and reports by model
and role how much work was done, how it fared in review, and what it cost in
tokens.

# Success Criteria

- one-click-inbox: from a device on the tailnet, the inbox lists every open
  item across this host's Atelier repositories that waits on the human, and
  each entry opens the page he needs (the HTML debrief for a debrief read) in
  one click. Shown by an agent fetching the inbox and every linked page over
  the Tailscale hostname, plus Sumeet's read of it on his phone.
- who-did-what: for one real item implemented by a Claude model and reviewed by
  a Codex model after attribution lands, the item's page names each thread's
  provider, model, effort and role, links each thread into T3, and gives each
  role's token cost. Shown by an agent comparing the page with T3's thread
  records and the provider transcripts.

# Shape

- Decision atelier-7bs1 and ADR 0030 (written by the first
  item) change `PRODUCT_INTENT.md`'s Boundaries: Atelier may have a human UI
  and a long-running read-only server, may keep a host-wide session database,
  and may depend on T3 Code. Tracker records stay Markdown in Git and remain
  the only source of truth for work; `merge`, `land`, `check` and `doctor`
  never read the session database or T3.
- **Server.** `atelier serve` renders HTML on each request, read-only in this
  mission, run as a systemd user service and published with `tailscale serve`
  beside T3 Code's own published port. It reads each repository's records from
  its base branch ref, never from a checkout, so a stale or detached worktree
  cannot mislead it. It reuses the app layer's views and the debrief renderer;
  pages are server-rendered with no front-end framework. Repositories are
  found by scanning configured roots (default `~/projects`) for `.atelier/`
  and deduplicating worktrees by Git common directory.
- **Attribution.** The CLI, not an MCP server, records attribution: every
  command inherits the session's environment, including `T3CODE_THREAD_ID`,
  whatever the harness. Each command run in a T3 thread writes one row to
  `~/.local/share/atelier/sessions.db` (SQLite): time, repository, item,
  command, role derived from the command, T3 thread ID, provider session ID,
  harness, provider, model and effort where the environment gives them. The
  site fills missing provider and model from T3's thread records. A hook nudges
  an agent that commits code in a T3 thread with no started item.
- **Harness facts** (spike of 2026-11-08):
  every harness T3 runs gets `T3CODE_HOME` and `T3CODE_TAILSCALE_SERVE`, but
  only Claude Code and Codex get `T3CODE_THREAD_ID`; Grok does not. Each
  names its own session: `CLAUDE_CODE_SESSION_ID`, `CODEX_SESSION_ID`,
  `GROK_SESSION_ID`, and T3 links every one of them to its thread. Harness
  markers are `CLAUDE_CODE_ENTRYPOINT=t3-code` with `AI_AGENT`,
  `CODEX_VERSION`, and `GROK_AGENT=1`. No harness puts the model in the
  environment; only Claude gives effort (`CLAUDE_EFFORT`). So rows record the
  session ID always and the thread ID when present, and model and effort come
  from T3's thread records.
- **Reviewer identity.** Review records gain structured `provider`, `model`,
  `effort` and `harness` fields that the CLI fills from the environment. The
  free-text identity stays. The installed binary serves every repository, so
  records without the new fields keep reading.
- **Tokens and cost.** T3 Code already scans Claude, Codex and Grok transcripts
  for per-request usage and keeps a model price table. Attribution windows
  (from one recorded command to the next in a thread) assign a thread's tokens
  to an item and role. Native subagents count toward their parent; a
  `delegate_task` child counts by its own commands, else toward its parent's
  item. Whether usage comes from a T3 interface or from scanning transcripts
  is settled by the T3 read spike; T3's internal cache files are not an
  interface.
- **Metrics** compare models within one role only, and show counts beside every
  rate. A one-time backfill parses existing free-text reviewer identities and
  matches T3 history to items by branch and time.
- When qualities conflict: correct and current beats complete, read-only beats
  convenient, and plain server-rendered pages beat polish.

# Scope

In: `atelier serve` and its pages, the session database, attribution recording
and the hook, structured reviewer identity, reading T3 thread status and
usage, the metrics page, the backfill, ADR 0030 and the Boundaries rewrite.

Out: actions from the site (recording decisions, approving plans), which wait
for a later mission under ADR 0030; a dashboard inside T3 Code, which waits for
T3 extensions; forking T3 Code; CLI orientation views for agents (mission
atelier-4pzj), though its shared view fixes, such as atelier-om6l's Waiting on,
reach the site too.

Environment: this host only, which also runs the T3 Code server Sumeet's phone
depends on, so the server must stay light. Repositories: every Atelier
repository under `~/projects`.

# Authority

The orchestrator may change items, their order and design detail within ADR
0030 and this Shape. Changing the Outcome, a criterion, the decision to record
attribution through the CLI, or adding write actions to the site needs Sumeet.

# Risks and Unknowns

- Whether an outside process can read T3's thread status and usage (MCP HTTP
  server, its auth, or an API) is unknown; the T3 read spike settles it before
  the live-status and usage items are made ready. If it cannot, a small read
  API upstreamed to T3 Code is the fallback, and a fork is the last resort.
- The mission is large. If planning review judges it two missions, the seam is
  the site (inbox, overview, missions) versus attribution and metrics.
- Upstream T3 Code changes fast (about 4,400 commits in the 30 days to
  2026-11-08); anything that reads T3 should go through its documented tools
  or a small interface, never its database or cache files.
