---
created_at: "2026-11-09T00:46:00.111340216+00:00"
id: "atelier-73vx"
issue_type: "feature"
labels:
- "fleet-dashboard"
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
    id: "atelier-yfqu"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Report work, review outcomes and token cost by model and role"
updated_at: "2026-11-09T00:46:00.111340216+00:00"
---

## Goal

Compare models by role: a metrics page on the site reports, per provider, model and role, the items worked, review rounds before approval, `fix_now` findings by severity, findings a reviewer raised that were closed as `no_action` or `wont_fix`, findings raised later against approved commits, CI failures and merge refusals, harness failures, and token cost per merged item.

## Done when

- [ ] Provisional: the page shows those measures per model within each role, with counts beside every rate, from review records, the session database and token usage.
- [ ] Provisional: a one-time backfill parses existing free-text reviewer identities and matches T3 history to items by branch and time; the Report says how many records it attributed and how many it could not.
- [ ] Provisional: token cost assigns a thread's usage to an item and role by the windows between its recorded commands; native subagents count toward their parent, `delegate_task` children by their own commands, else toward the parent's item.

## Unknowns

- Where usage comes from (a T3 interface or scanning transcripts); settled by the T3 read spike. Make ready after the spike, the attribution item and the reviewer-identity item land. May split into usage ingestion, backfill and the page.
