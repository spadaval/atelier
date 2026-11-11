---
created_at: "2026-11-11T02:17:26.850729926+00:00"
id: "atelier-h3hl"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-94pv"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-psjw"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-z4cr"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Let Sumeet say go, answer a question and start a planning session from a page"
updated_at: "2026-11-11T02:17:26.850729926+00:00"
---

## Goal

Preliminary. The acts `ready` and `decide`, each performed by calling the core's own function for it (atelier-psjw), as the strategy's Design says under Acts, and a button that starts a planning session in T3 Code for a mission.

## Done when

- [ ] Provisional: the strategy's act-is-a-record criterion holds in the Desk's suite; the Desk runs no `atelier` binary and reads no command output for an act; saying go names the person acting and leaves an unrelated staged file in that checkout staged and uncommitted; `decide` run from a page writes the answer in the checkout that has the item's branch out, as `atelier decide` does there, and until the implementer's `submit` commits it the page shows the question as answered and not yet committed; a refused command's output is shown as it is; an act from another origin or without the app's header is refused.
- [ ] Provisional: starting a session is proved against the running T3 Code on this host, not a stand-in: one thread is created in a scratch project with `t3_thread_launch` under a token that may operate, its link opens, and the Report gives the thread's ID. The 2026-11-10 probe on atelier-aab1 only read; it never started a thread.
