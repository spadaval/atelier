---
created_at: "2026-10-17T01:10:57.973844204+00:00"
id: "atelier-o3lm"
issue_type: "mission"
labels:
- "cli-output"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-5rs0"
    type: "advances"
  - kind: "issue"
    id: "atelier-6xxb"
    type: "advances"
  - kind: "issue"
    id: "atelier-7iyw"
    type: "advances"
  - kind: "issue"
    id: "atelier-7w1g"
    type: "advances"
  - kind: "issue"
    id: "atelier-91w1"
    type: "advances"
  - kind: "issue"
    id: "atelier-a76d"
    type: "advances"
  - kind: "issue"
    id: "atelier-bsyn"
    type: "advances"
  - kind: "issue"
    id: "atelier-e95f"
    type: "related"
  - kind: "issue"
    id: "atelier-i2qq"
    type: "advances"
  - kind: "issue"
    id: "atelier-jpok"
    type: "advances"
  - kind: "issue"
    id: "atelier-krf4"
    type: "advances"
  - kind: "issue"
    id: "atelier-ksx6"
    type: "advances"
  - kind: "issue"
    id: "atelier-od8b"
    type: "advances"
  - kind: "issue"
    id: "atelier-onwh"
    type: "advances"
  - kind: "issue"
    id: "atelier-pr4k"
    type: "advances"
  - kind: "issue"
    id: "atelier-qrwr"
    type: "advances"
  - kind: "issue"
    id: "atelier-teld"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Share one output contract across every Atelier command"
updated_at: "2026-10-17T05:23:01.143453389+00:00"
---

## Outcome

Every Atelier command follows one written output contract:
- one row grammar;
- one footer form (`Next`);
- counts in plain words;
- one documented `--quiet` token;
- a verdict line for commands that gate;
- a confirmation shape for commands that change records.

No command crashes when piped. Hints and errors name real IDs and share one form, and help documents every argument and flag. The surface missions split out of this one apply the contract to their own commands: gates, issue commands, views, bundle and check, and setup and prune.

## Success criteria

- S1: `docs/product/human-cli-output.md` defines the contract, and shared rendering helpers implement it (atelier-i2qq). Shown by rendering tests for the helpers.
- S2: Piping any command into `head` exits cleanly (atelier-5rs0). Shown by a test per command family.
- S3: `--quiet` prints one documented token on every command (atelier-pr4k). Next-step hints and errors name the real ID (atelier-6xxb). Not-found and invalid-value errors share one form, with short IDs (atelier-qrwr). Priorities accept P0-P3 and words alike (atelier-jpok). Shown by tests across commands.
- S4: Footers, counts, terms and timestamps are consistent across views (atelier-onwh, atelier-ksx6, atelier-krf4, atelier-91w1, atelier-7iyw). Shown by rendering tests.
- S5: Root help is generated and orders common commands by lifecycle. Subcommand help documents every argument and flag. `--log-level` and `--log-format` are implemented or removed (atelier-od8b, atelier-7w1g, atelier-a76d). Shown by help tests.

## Background

This mission was filed from a six-surface CLI audit on 2026-10-16. The audit covered work views; status, land, init and setup; issue commands and start; review, evidence and merge; bundle, check, prune and policy; and help, errors and cross-cutting consistency. It ran every command against this repository and against fixture clones, and mined Claude Code transcripts from this repository and astromech. It built on ADR 0022 (atelier-bw0b: status orients, output is dense Markdown and never JSON for agents, and routine piping means a view is missing) and on the human's review of `work missions` and `work mission`.

On 2026-10-16, at the human's direction, the 106 audit items were split by surface into separate missions. This mission keeps the cross-cutting contract and help. The audit's transcript-pipe measure moved to each surface mission, which measures its own commands. The transcript-analysis scripts live in `~/scratch/cli-audit/cc-scripts/` on the audit host, not in the repository; commit them before any mission measures its pipe rate.

## Notes

- 2026-10-17 [note] Filed from a six-surface CLI audit (99 items, all backlog). Depends on ADR 0022 (atelier-bw0b) being accepted, but that is deliberately not a blocked_by link: a link would edit bw0b's record on master and stale its pending approval on task/atelier-bw0b when master is merged in. Open question for the human: a spike on a Markdown bundle input format instead of JSON with escaped Markdown bodies.
- 2026-10-17 [note] Split by the human's direction (Sumeet, 2026-10-16: break up over-scoped missions). 106 items was one mission across 15 surfaces. This mission keeps the cross-cutting output contract and help (14 items). The surfaces moved to: atelier-rc8j (gates: land, merge, review, evidence), atelier-uf97 (issue commands and start), atelier-4pzj (status and work views), atelier-2mky (bundle and check), atelier-nn9y (init, setup, prune, policy). The retrospective's integration items moved to atelier-k8h8. The old S5 transcript measure is now measured per surface mission. Renderer blocked_by links were kept.
