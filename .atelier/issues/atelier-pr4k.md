---
created_at: "2026-10-17T01:10:57.992053843+00:00"
id: "atelier-pr4k"
issue_type: "bug"
labels:
- "cli-output"
- "cross-cutting"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-o3lm"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Make --quiet print one documented token on every command"
updated_at: "2026-10-17T01:10:57.992053843+00:00"
---

## Goal

`-q` is global but behaves differently on each command:
- Ignored: `start` (8 lines), `issue show` (whole record), `issue transition <id>` inspect (whole report), `issue note` ("Added note to issue ..."), `issue update` (8 lines plus a footer), `check` ("Lint passed."), `prime` (7 lines), `init`.
- Inconsistent: `-q issue link` prints nothing, `-q issue transition X defer` prints `backlog`, and `land -q`/`doctor -q` print `passed`.
- Three shapes among the work views: `work -q` prints dashboard names one per line, `work missions -q` prints IDs, and `work mission X -q` prints `atelier-1w8c active=0 ready=2 human=0 context=0 blocked=3 done=0 backlog=2`. `status -q` prints `work=none current_missions=2 ready=6 human=1`, where `work` mixes `none` and a number.

Agents work around this: `start | grep Branch` 29 times and `start | tail -N` 43 times in transcripts to get the branch, and `issue show X | grep ^Status` / `sed -n 3p` to get a status. One agent tried `issue show $P -q`.

## Done when

- Every command honours `-q` with the token from the quiet table in `human-cli-output.md`. Proposed tokens:
  - `start` → branch name
  - `issue show` → `<id> <status>`
  - `issue transition <id>` (inspect) → allowed transition names
  - `issue note`, `issue update` and `issue link` → the ID
  - `check`, `land` and `doctor` → `passed`, or a non-zero exit
  - `init` → the slug
  - `prime` and `status` → `key=value` with numbers only
  - `work*` views → IDs, one per line, with counts only on `work mission` and documented
- A test runs `-q` on every subcommand against a fixture repo and checks each token.

## Context

`main.rs` does not pass `quiet` to start, merge, check, bundle, init, or review list/show. The table lives in the doc written by the output-contract item in this mission.
