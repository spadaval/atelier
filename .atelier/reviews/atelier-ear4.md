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
---

Low: next/crates/atelier-desk/src/main.rs:34 only SIGINT reaches threads.close(); SIGTERM takes the default termination path. A fresh subprocess probe using a stand-in t3 command produced SIGINT exit=0, issue_count=1, revoke_count=1, versus SIGTERM exit=-15, issue_count=1, revoke_count=0. Thus the new comment and docs/architecture/desk.md claim that stopping revokes the session is broader than the implementation, and a normal service stop leaves its read-only session until the one-hour expiry. When integrating the service, handle SIGTERM through the same cleanup path (or configure its stop signal and narrow the documentation), and exercise both signals. Deferred because service integration is a later surface, SIGINT cleanup and the required recorded live revocation work, and the abandoned credential is read-only and expires within one hour.
