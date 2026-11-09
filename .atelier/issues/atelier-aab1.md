---
created_at: "2026-11-09T00:45:59.633639914+00:00"
id: "atelier-aab1"
issue_type: "spike"
labels:
- "fleet-dashboard"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-yfqu"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Find how atelier serve can read T3 thread status, models and token usage"
updated_at: "2026-11-09T00:45:59.633639914+00:00"
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
