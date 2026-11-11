---
created_at: "2026-11-11T06:46:00.431925577+00:00"
id: "atelier-mg2j"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "review"
    id: "atelier-ear4"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T08:56:15.818353730+00:00"
status: "superseded"
title: "Low: next/crates/atelier-desk/src/main.rs:34 only SIGINT reaches threads.close(); SIGTERM takes the default termination…"
updated_at: "2026-11-11T08:56:15.818353730+00:00"
---

## Goal

Deferred from finding atelier-ear4 on atelier-e05n @df7c496b, by `atelier review defer`.

Low: next/crates/atelier-desk/src/main.rs:34 only SIGINT reaches threads.close(); SIGTERM takes the default termination path. A fresh subprocess probe using a stand-in t3 command produced SIGINT exit=0, issue_count=1, revoke_count=1, versus SIGTERM exit=-15, issue_count=1, revoke_count=0. Thus the new comment and docs/architecture/desk.md claim that stopping revokes the session is broader than the implementation, and a normal service stop leaves its read-only session until the one-hour expiry. When integrating the service, handle SIGTERM through the same cleanup path (or configure its stop signal and narrow the documentation), and exercise both signals. Deferred because service integration is a later surface, SIGINT cleanup and the required recorded live revocation work, and the abandoned credential is read-only and expires within one hour.

## Done when

Done when was not specified.

## Report

Settled by atelier-va99 (pull request #139): atelier-desk stops on SIGTERM as on SIGINT and revokes its T3 Code token, with the test tests/signals.rs.
