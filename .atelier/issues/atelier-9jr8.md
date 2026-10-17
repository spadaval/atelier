---
created_at: "2026-10-17T01:10:57.980264087+00:00"
id: "atelier-9jr8"
issue_type: "bug"
labels:
- "cli-output"
- "land"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-rc8j"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Give each failed land check its item, reason and fix command"
updated_at: "2026-10-17T01:10:57.980264087+00:00"
---

## Goal

`land_invariants_unmet` prints `{target} {check}: {reason}` lines, then one generic `Resolve the named checks, commit the records, then rerun atelier land.` (crates/atelier-cli/src/commands/session.rs:535-566). Observed in a scratch clone:
```
atelier-jb2t blockers.none_open: open blockers: atelier-ihns
atelier-jb2t blockers.transitive_none_open: completion blockers: atelier-ihns
```
and
```
atelier-43m1 review.current: no current independent approval or explicit trivial exemption; authors: sumeet padavala, sumeet padavala <sumeet.padavala@gmail.com>, sumeet.padavala@gmail.com
```
- One cause produces two lines.
- The blocker's title and status are missing.
- The same author appears three times.
- The message does not say that trivial exemption atelier-36rk already targets atelier-43m1 but is not current.
- No line gives the command that fixes it.
- An unrelated behind-base note is appended after the generic line.

## Done when

- Each failed check prints one line with the item, the reason and a fix, for example:
  - `atelier-jb2t done but blocked by atelier-ihns [todo] Show the between-missions state… — close atelier-ihns or remove the link`
  - `atelier-43m1 needs review: no approval at a current commit (atelier-36rk pins 1620b534, not on this branch) — atelier review approve atelier-43m1 --identity <reviewer> "…"`
- A transitive failure that repeats a direct failure on the same item is merged into it.
- Authors are deduplicated by email.
- The behind-base note is printed separately from the failures.
- Tests cover blockers, review and findings.

## Context

The reasons come from `review_gate` (crates/atelier-app/src/review_gate.rs). The `merge --check` gate-reason item in this mission shares these reasons, so they should be changed once at the source.
