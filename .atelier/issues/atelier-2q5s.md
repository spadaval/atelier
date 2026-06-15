---
created_at: "2026-06-15T05:11:26.860310564+00:00"
id: "atelier-2q5s"
issue_type: "epic"
labels:
- "app-layer"
- "cli"
- "rewrite"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-3kap"
  - kind: "issue"
    id: "atelier-4j3k"
  - kind: "issue"
    id: "atelier-fchz"
  children:
  - kind: "issue"
    id: "atelier-14z2"
  - kind: "issue"
    id: "atelier-4ren"
  - kind: "issue"
    id: "atelier-nyn0"
  - kind: "issue"
    id: "atelier-sclf"
  - kind: "issue"
    id: "atelier-vv2i"
  - kind: "issue"
    id: "atelier-zwna"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-15T18:54:10.767822733+00:00"
status: "done"
title: "Epic: Split application layer and CLI shell"
updated_at: "2026-06-15T18:54:10.767822733+00:00"
---

## Description

Separate command orchestration from Clap parsing by introducing an application layer and thinning the CLI shell. Preserve visible command intent, help/docs alignment, and the human-first output contract.

## Outcome

- `atelier-app` owns use-case orchestration through explicit request/outcome/view-model APIs and does not write directly to stdout or stderr.
- `atelier-cli` owns Clap definitions, parsing, tracing setup, telemetry recording, command identity, terminal rendering, exit-code handling, and thin delegation into `atelier-app`.
- Oversized command handlers are split by product job and view model ownership while keeping help-visible command jobs stable.
- No old command aliases, fallback shims, or public compatibility facades are introduced.

## Evidence

- Child issue proof shows application ports, request/outcome/view-model extraction, rendering ownership, and thin CLI shell wiring.
- Search transcript proves `atelier-app` use-case code does not call `println!` or `eprintln!`.
- CLI/help/docs parity checks cover visible workflow surfaces.
- Representative command transcripts for status, issue, mission, evidence, lint, doctor, and export check remain behaviorally stable.

## Notes

- Temporary adapters used while splitting `atelier-app` from `atelier-cli` must
  follow `docs/architecture/source-layout.md`: name the adapter marker, removal
  owner, removal condition, and proof that no public compatibility promise is
  being created.
