---
created_at: "2026-10-16T23:43:33.771473753+00:00"
id: "atelier-kops"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Owned branches trust a local base that may hold unchecked commits"
updated_at: "2026-10-16T23:43:33.771473753+00:00"
---

## Goal

land on an owned branch trusts history up to its branch base, the local base tip it started from. The local base can hold commits no land has checked, so a branch started from an unchecked local commit gets a passing land. The base's own land against its upstream catches this before a push, so nothing unchecked reaches the upstream; this closes the gap earlier.

## Done when

On an owned branch, land trusts history only up to the merge base of the branch base with the base's upstream (all claims checked without an upstream), with a test that starts work from an unpushed local base commit carrying failing proof.

## Context

Found in review of atelier-37uq (finding atelier-0c86), which made unowned branches trust only the upstream.
