---
created_at: "2026-11-09T00:46:00.111340216+00:00"
id: "atelier-73vx"
issue_type: "feature"
labels:
- "agent-attribution"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-00hl"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-7yy9"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-aab1"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-bnek"
    type: "advances"
  - kind: "issue"
    id: "atelier-zpmb"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Report work, review outcomes and token cost by model and role"
updated_at: "2026-11-09T01:04:29.653404246+00:00"
---

## Goal

Compare models by role: a metrics page on the site reports, per provider, model and role, the items worked, review rounds before approval, `fix_now` findings by severity, findings a reviewer raised that were closed as `no_action` or `wont_fix`, findings raised later against approved commits, CI failures and merge refusals, harness failures, and token cost per merged item.

## Done when

- [ ] Provisional: before building the page, the attribution rule is checked on at least three real threads (an implementer, a fresh reviewer, and a reused orchestrator thread): the Report gives each thread's token total, the share attributed to each item and role, and the unattributed share, and the rule is changed until a fresh reviewer's reading tokens land on the item it reviewed.
- [ ] Provisional: the page shows the measures per model within each role, with counts beside every rate.
- [ ] Provisional: a one-time backfill parses existing free-text reviewer identities and matches T3 history to items by branch and time; the Report says how many records it attributed and how many it could not.

## Unknowns

- Where usage comes from (a T3 interface or scanning transcripts); settled by spike atelier-aab1.
- The attribution rule. Starting point: a thread's tokens before its first recorded item go to that first item and its role; after that, each recorded command opens a window that runs to the next; native subagents count toward their parent; `delegate_task` children count by their own rows, else toward the parent's item (planning review finding atelier-b9wq). Make ready after aab1, atelier-7yy9 and atelier-00hl land. May split into usage ingestion, backfill and the page.
