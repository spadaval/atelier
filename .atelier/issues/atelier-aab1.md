---
created_at: "2026-11-09T00:45:59.633639914+00:00"
id: "atelier-aab1"
issue_type: "spike"
labels:
- "agent-attribution"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-bnek"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Find how atelier serve can read T3 thread status, models and token usage"
updated_at: "2026-11-11T02:06:10.652212169+00:00"
---

## Goal

Find how a process outside a T3 thread, such as `atelier serve`, can read T3 Code's thread status, each thread's provider, model and effort, the mapping from a provider session ID to its thread, and per-request token usage. The live-status and metrics work depends on the answer.

## Done when

- [ ] The Report says, for each of thread status, model and effort, session-to-thread mapping and token usage, which T3 interface gives it (its MCP HTTP server, another API, or none), how a caller authenticates, and an example request and response run on this host.
- [ ] Where no interface gives it, the Report proposes the smallest upstream change to T3 Code that would, and names the T3 files involved (MCP toolkits live in `apps/server/src/mcp/toolkits/` in `~/projects/t3code`).
- [ ] The Report says whether T3's thread links (`t3-thread://` URLs) open the thread on Sumeet's phone and on the web client.

## Scope

Read-only investigation. Do not depend on T3's SQLite files or `usage-scan-cache-*.json`; they are internal and change format.

## Context

T3 already scans Claude, Codex and Grok transcripts for usage and keeps a price table (`~/.t3/userdata/usage-model-rates.json`). `t3_thread_list` returns status, model, provider and pull requests per thread, but not its branch.

## Notes

- 2026-11-10 [note] 2026-11-10 probe from the deck spike thread, run on this host against T3 0.1.9-nightly.20261110.4478; not this item's Report. An outside process gets in with a scoped token from 't3 auth session issue --subject mcp-client --scope orchestration:read --ttl <time>' and calls the MCP server at http://127.0.0.1:3773/mcp with it as a bearer token. Without '--subject mcp-client' the server answers 401 invalid_mcp_credential; --help does not say so. Given by MCP: every thread across projects with status, provider, model, parent and pull requests (t3_thread_list with allProjects and includeSubagents; 2.7 s per 100, 0.4 s when filtered by statuses); pending requests and run history with the executed model (t3_thread_read); tool calls, reads and file changes (t3_thread_read view=activity); a link per thread (app.t3.codes URL); starting and messaging a thread (t3_thread_launch, t3_thread_send; not exercised). A read-only token was refused on t3_thread_launch with capability_denied. Not given by any MCP result: token usage, context size, reasoning effort, and the provider session id to thread mapping. Still open from Done when: whether the thread links open on the phone. The 't3' on PATH is 0.1.6 and prints migration warnings into its --json output; call the MCP server directly. Both probe sessions were revoked.
