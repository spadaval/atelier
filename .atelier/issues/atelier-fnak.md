---
created_at: "2026-10-17T09:36:27.970145080+00:00"
id: "atelier-fnak"
issue_type: "bug"
labels:
- "cli-output"
workflow_branch:
  branch_base: master
  work_branch: bug/atelier-fnak
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
closed_at: "2026-10-17T09:43:02.623702922+00:00"
status: "done"
title: "Keep --quiet stdout empty when start or issue transition refuses"
updated_at: "2026-10-17T09:43:02.623702922+00:00"
---

## Goal

Mission validation (evidence atelier-pnha) found that `atelier -q start <id>` and `atelier -q issue transition <id> start` (or `close` without `--reason`) print the whole "Issue Transition … Validators … Blockers" report on stdout when the transition is refused. The Quiet Tokens rule in `docs/product/human-cli-output.md` says a refusal leaves stdout empty, with the explanation on stderr and a non-zero exit. `quiet_tokens.rs` only checks refusals for `check`, `land` and `merge`.

## Done when

- Every command's refusal under `-q` leaves stdout empty and explains itself on stderr with a non-zero exit.
- `quiet_tokens.rs` checks a refusal for every command that can refuse, including `start` and `issue transition`.

## Close Reason

Quiet start and issue transition refusals no longer print the transition report on stdout; the error on stderr names each blocker. A survey found no other command that writes stdout before refusing. quiet_tokens.rs now refuses every leaf command under -q and fails on any command without a checked refusal.
