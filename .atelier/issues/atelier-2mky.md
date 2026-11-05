---
created_at: "2026-10-17T01:44:17.972310338+00:00"
id: "atelier-2mky"
issue_type: "mission"
labels:
- "cli-output"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-4t5l"
    type: "advances"
  - kind: "issue"
    id: "atelier-fb1i"
    type: "advances"
  - kind: "issue"
    id: "atelier-kset"
    type: "advances"
  - kind: "issue"
    id: "atelier-sfd9"
    type: "advances"
  - kind: "issue"
    id: "atelier-w4j2"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "draft"
title: "Make adopting and maintaining Atelier in a repository say what it did and what to do next"
updated_at: "2026-11-05T06:03:04.102700+00:00"
---

## Outcome

Someone adopting Atelier, or keeping a repository's tracker healthy, gets a working setup from `init` and `setup claude`, or a clear refusal that names the one step to take. `check` and `prune` say what they examined, report each problem once against the record to fix, and end with the exact next step. The workflow policy has a readable view, and the README says how to install a pinned release.

## Background

Regrouped on 2026-11-05 at the human's direction: missions are organised by outcome, not by the command surface or the audit that found their items. This mission is now repository adoption and upkeep. Its bundle items moved to atelier-uf97 with the Markdown input question. atelier-gg0m (hooks run outside a repository) and atelier-hzuk (installing a pinned release) joined it. The former S1-S8 restated their items and were dropped under ADR 0024.

Split out of atelier-o3lm on 2026-10-16 at the human's direction. Its items come from that mission's six-surface CLI audit; see atelier-o3lm's Background for the method and the transcript scripts. The output contract (atelier-o3lm S1, atelier-i2qq) and ADR 0022 (atelier-bw0b) govern how output looks. atelier-o3lm's Notes leave the human an open question: a Markdown input format for bundles. atelier-rz2l (record import) may answer part of it. On 2026-10-22 the human merged atelier-nn9y (init, setup, prune and the policy view) into this mission; its S1-S4 are S5-S8 here.
