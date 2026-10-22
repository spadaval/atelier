---
created_at: "2026-10-22T23:23:56.029116881+00:00"
id: "atelier-j458"
issue_type: "bug"
labels:
- "land"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
  - kind: "review"
    id: "atelier-i0w4"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Load review history when a record path is not UTF-8"
updated_at: "2026-10-22T23:23:56.029116881+00:00"
---

## Goal

Since atelier-7b6e, land and doctor load the history of .atelier/issues and .atelier/reviews once (crates/atelier-app/src/path_log.rs:199-200). If any commit holds a non-UTF-8 file name under those directories, the load fails and land and doctor error outright, where the old git log walks kept working. Found in review of atelier-7b6e (finding atelier-i0w4).

## Done when

A history containing a non-UTF-8 path under .atelier/issues or .atelier/reviews loads, either keyed as bytes or skipping paths no record ID can match, and land's answers are unchanged. A test with such a path covers it.
