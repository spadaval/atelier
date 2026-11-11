---
schema: atelier.review
schema_version: 2
id: atelier-ear4
title: finding on atelier-e05n @df7c496b
target: atelier-e05n
identity: reviewer-sol-r1
commit: df7c496b739bf612afb2dfa1fc1a1eb12235a773
created_at: 2026-11-11T06:45:05.477467731Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: thread:delegated-task:command%3Amcp%3Ad4040a34-c28b-4672-8918-87b1ffbc9fe0%3Adelegate-task%3A6b851a38-1d19-4a4b-9325-287b92cdc7eb
    commit: f5a481011699792638b39c2b7addd04074334ab5
    recorded_at: 2026-11-11T06:46:00.431925577Z
    follow_up: atelier-mg2j
    reason: Deferred to backlog follow-up atelier-mg2j under atelier-km0i.
---

Low: next/crates/atelier-desk/src/main.rs:34 only SIGINT reaches threads.close(); SIGTERM takes the default termination path. A fresh subprocess probe using a stand-in t3 command produced SIGINT exit=0, issue_count=1, revoke_count=1, versus SIGTERM exit=-15, issue_count=1, revoke_count=0. Thus the new comment and docs/architecture/desk.md claim that stopping revokes the session is broader than the implementation, and a normal service stop leaves its read-only session until the one-hour expiry. When integrating the service, handle SIGTERM through the same cleanup path (or configure its stop signal and narrow the documentation), and exercise both signals. Deferred because service integration is a later surface, SIGINT cleanup and the required recorded live revocation work, and the abandoned credential is read-only and expires within one hour.
