---
created_at: "2026-06-23T15:22:49.263355626+00:00"
id: "atelier-ycj9"
issue_type: "feature"
labels:
- "admin"
- "cli"
- "help"
- "human-output"
- "review"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-3js3"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Refresh role-guide, review, and admin command output"
updated_at: "2026-06-23T21:05:33.035541958+00:00"
---

## Description

Bring lower-frequency human surfaces onto the same output grammar after the
issue and proof surfaces define the pattern.

## Outcome

- `man` role guides use current objective terminology, short ranked command
  lists, and interactive styling only where helpful.
- `atelier man admin` provides substantially better guidance for admin-type
  agents: workflow/config ownership, fixed mission/epic domain rules, where to
  find the validator/action reference, how to inspect live transition policy,
  and when to use `lint`, `doctor`, hidden workflow diagnostics, branch
  recovery, prune, or maintenance commands.
- Man pages do not become the validator/action encyclopedia; they point admin
  agents to `docs/product/workflow-configuration.md`,
  `docs/product/work-model.md`, `atelier issue transition <id> --options`, and
  `atelier lint` for the correct level of detail.
- `review` output makes review authority and state obvious before provider
  detail.
- Admin and recovery surfaces such as `branch`, `doctor`, `lint`, `prune`, and
  `maintenance` use shared footers, bounded lists, and status styling without
  becoming routine worker guidance.
- Help and error output remove or quarantine stale flags and provider plumbing
  that no longer represent the current workflow contract.
- Retired, hidden, and admin-only command concepts such as `graph`, `repair`,
  `start`, `export`, and raw workflow diagnostics are not presented as normal
  worker/reviewer paths.
- Role guides and help text present the restored `mission` namespace only as a
  read-only mission report/discovery surface. Mission creation, linking,
  lifecycle transitions, and closeout remain documented under issue/workflow
  commands.
- Removed-command errors provide replacement guidance when there is a single
  current replacement, and otherwise fail clearly without implying an obsolete
  alias exists.

## Evidence

- Before/after transcripts cover `man worker`, `man manager`, one review status
  or comments surface, and at least one admin or recovery command.
- Before/after transcripts cover `man admin` showing the improved admin-agent
  workflow guidance and references to the canonical workflow docs and live
  inspection commands.
- Before/after transcripts or focused tests cover stale flag cleanup, retired
  command guidance, hidden/admin command framing, and colorless output for
  these surfaces.
- `target/debug/atelier lint`, focused CLI tests, and `git diff --check` pass.
