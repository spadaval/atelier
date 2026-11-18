---
schema: atelier.item
schema_version: 3
id: atelier-3zh2
title: Show a thread waiting on an approval in the Desk's Inbox and the sidebar's waiting count
status: draft
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-18T01:07:36.095109540Z
---

## Goal

A thread stopped on an approval needs the human, and neither the Desk's
Inbox nor the sidebar's count of missions waiting on a person shows it. Both
test `status == "waiting"` on the polled thread list
(`crates/atelier-desk/src/server.rs:672`, `server/views.rs:108` and `:162`),
and T3 reports such a thread as `running` with a pending-request count that
its thread list leaves out. The mission page already shows it, from each
thread's full read (atelier-9f7b).

## Done when

- [ ] A thread with a pending request appears among the Inbox's waiting threads and counts toward its mission's waiting count in the sidebar, with a test for each.
- [ ] The Report says which way was taken and why: T3 putting `pendingRequestCount` on `t3_thread_list`'s items, or the Desk reading each live thread in full.

## Scope

atelier-9f7b's Report names the fix at the cause: one line in T3's
`OrchestratorMcpService.ts`, after which the Desk reads the count in its
poll and the mission page's per-thread logic can go. That is a change to T3
Code, not to this repository, so who makes it and whether the Desk must work
without it is the human's call before this is readied.
