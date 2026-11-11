---
status: draft
last_changed: "2026-11-08: split out of fleet-dashboard.md after planning review"
---

# Shape

- ADR 0030 governs: the CLI only writes the session database
  (`~/.local/share/atelier/sessions.db`), only `atelier serve` reads it, and
  guard decisions never consult it.
- **Recording through the CLI**, not an MCP server: every command inherits the
  session's environment. Commands that name an item, reads included, write a
  row, so a reviewer that reads before writing still marks where its work
  began.
- **Harness facts** (spike of 2026-11-08): every harness T3 runs gets
  `T3CODE_HOME`, but only Claude Code and Codex get `T3CODE_THREAD_ID`; Grok
  does not. Each names its own session (`CLAUDE_CODE_SESSION_ID`,
  `CODEX_SESSION_ID`, `GROK_SESSION_ID`), and T3 links every one to its thread.
  Markers: `CLAUDE_CODE_ENTRYPOINT=t3-code` with `AI_AGENT`, `CODEX_VERSION`,
  `GROK_AGENT=1`. No harness gives the model; only Claude gives effort
  (`CLAUDE_EFFORT`). Rows record the session ID always and the thread ID when
  present; model, effort and Grok's thread come from T3.
- **Roles** come from an explicit `--as`/`ATELIER_ROLE` first, then the
  command. Delegators should set the role when starting a child.
- **Reviewer identity** gains structured `provider`, `model`, `effort` and
  `harness` fields beside the free-text identity; the shared installed binary
  must keep reading records without them.
- **Tokens.** T3 already scans Claude, Codex and Grok transcripts and keeps a
  price table. The attribution rule (lead-in to the first item, then windows
  between recorded commands, subagents to their parent, delegated children by
  their own rows) is validated on real threads before the metrics page is
  built. T3's SQLite and cache files are not an interface.
- **Metrics** compare models within one role only, with counts beside every
  rate.

# Risks and Unknowns

- Whether an outside process can read T3's thread status, models and usage is
  open until spike atelier-aab1 reports; the fallback is a small read API
  upstreamed to T3 Code, and a fork is the last resort.
- Token attribution may leave a large unattributed share; the attribution
  check measures it before anything is built on it.
- Upstream T3 Code changes fast (about 4,400 commits in the 30 days to
  2026-11-08).
