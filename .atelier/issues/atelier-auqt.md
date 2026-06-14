---
created_at: "2026-06-12T20:29:22.966441834+00:00"
id: "atelier-auqt"
issue_type: "validation"
labels:
- "cli"
- "command-surface"
- "validation"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-efpk"
  children: []
  attachments:
  - kind: "evidence"
    id: "atelier-826i"
    role: "validates"
  - kind: "evidence"
    id: "atelier-mz60"
    role: "validates"
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-12T21:37:04.161674575+00:00"
status: "done"
title: "Validate reduced issue command surface end to end"
updated_at: "2026-06-12T21:37:04.161674575+00:00"
---

## Description

Validate the reduced issue command surface after classification, folded helper
flows, and non-lifecycle command moves land.
- The command inventory from `atelier-exz1` maps to implemented help,
  replacement commands, removed-command behavior, and docs.
- Normal `atelier issue --help` exposes only lifecycle commands.
- Replacement workflows cover parented create, create-and-start, reopen,
  label/unlabel, blocked-list, dependency, link, hierarchy, search, activity,
  and destructive-maintenance behavior classified as retained.
- Obsolete command surfaces do not appear in Agent Factory guidance or normal
  next-action output.
- Any mismatch becomes a follow-up blocker before `atelier-efpk` closes.
- Command inventory audit with each row classified pass, fail, blocked,
  deferred, or not-applicable.
- Positive and negative CLI transcripts for representative retained, moved,
  hidden, and removed commands.
- Docs/help/Agent Factory parity check for the reduced issue surface.
- Focused CLI tests plus `atelier lint` and `atelier export --check`.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
