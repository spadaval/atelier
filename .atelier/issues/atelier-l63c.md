---
created_at: "2026-11-04T23:03:20.457889128+00:00"
id: "atelier-l63c"
issue_type: "mission"
labels:
- "cli-output"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "draft"
title: "Show what changed since the last session, and measure whether agents still reshape tracker output"
updated_at: "2026-11-04T23:03:20.457889128+00:00"
---

## Outcome

Session start and orchestrator wake-ups show what changed in the tracker: recent activity, a since-commit digest of a mission, planned versus built work, and docs that exist only on unmerged branches. In-flight marks stay cheap as branches multiply. Once atelier-4pzj has landed, transcripts show whether agents still reshape `status` and `work` output, and that count decides whether repositories get custom work views.

## Success criteria

- S1: Session start shows recent tracker activity. A since-commit digest of a mission exists for orchestrator wake-ups. In-flight marks are computed without reading each branch's whole tracker. `work missions` shows planned versus built work and backlog health. Docs that exist only on unmerged branches are shown (atelier-majk, atelier-avx0, atelier-3i7x, atelier-jbwi, atelier-kglz). Shown by session and CLI tests, and a timing recorded as evidence for atelier-3i7x.
- S2: The deprecated `prime` alias is removed once adopters' hooks stop calling it (atelier-jjvc). Shown by a CLI test and a note naming the adopter repositories checked.
- S3: Claude Code transcripts recorded after atelier-4pzj closes show `status` and `work` views piped through head, grep, sed or tail in under a third of runs. Shown by a dated note with the counts.
- S4: Repositories can define custom work views declaratively (atelier-e95f) only if S3's count shows agents still reshape output; otherwise atelier-e95f is superseded with the count as its reason. Shown by a CLI test, or by that supersede reason.

## Background

Split out of atelier-4pzj on 2026-11-04 at the human's direction, so that mission can close on its rendering work. These criteria were its S5, S6 and S7. S3 and S2 wait on calendar time (transcripts after atelier-4pzj lands; adopters updating their hooks), not on implementation. S3 carries atelier-1w8c's S6, whose before-measure was about four runs in five piping atelier output; atelier-o3lm's Background has the transcript scripts.
