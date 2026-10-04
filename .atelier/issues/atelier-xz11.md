---
created_at: "2026-10-01T21:19:17.757643119+00:00"
id: "atelier-xz11"
issue_type: "validation"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "blocked"
title: "Validate rebuild success criteria independently"
updated_at: "2026-10-03T07:41:18.031199469+00:00"
---

## Goal

Independently check the strategy's S1–S8 acceptance claims against the final integrated candidate.

## Done when

Each criterion has an explicit result and concrete evidence: gate and clean merge scenarios, freshness tests, release timing observations without a fixed latency budget, the command list and Rust source count, a dogfooded session, typed record lint and fixtures, and this tail tracker. Any failure or blocker is reported before a done claim.

## Scope

Read-only acceptance checks, evidence records, findings and their dispositions.

## Context

docs/strategies/atelier-rebuild.md, Success Criteria; docs/adr/0019-verified-repo-native-tracker.md.

## Notes

- 2026-10-03 [note] Scoped final functional proof at actual9ee/e72 PASS:317 captured calls/20 unchanged refusals/1 clean reviewed FF, independent6fd001ba; receipt oh02. Docs fixture49 exit observations PASS independent6b1997fe, receipt w7d9 (exit text, not all per-command raw; no actual Claude). Actual root Codex session prime/cleanland/dirtywp90 refusal/exact restore/cleanland PASS independentd44408e7, receipt qe7s; all16 captured outcomes/32 raw streams retained. Receipts explicitly pin actual9ee before this bookkeeping commit. Full587hooks/583default/count19930/exclusion revieweec5 remains separate current-code proof. S4 stays FAIL bdy0/95bu open and8ud1 blocks this validation. No full S1-S8/mission closeout or done claim; formal fresh Git230/bothhash/current regressions and last-retained-source acceptance remain pending.
- 2026-10-03 [note] Scoped native58 functional regression acceptance at actual3392/e72 PASS, receipt atelier-0ie2; independent /tmp/session-review-native47-plus58-validation.json SHA8d386d6d, root disposition1b06c40a. Original wholeFAIL native9agm remains unchanged: fixture lock initialization omitted before old twelfth baseline. Reviewed separate remaining47 initializes eight base locks through ordinary issue-list BEFORE baseline, with strict full runtime/index/native fences. Exact11+47=58 disjoint planned contexts/112 completed outcomes; all113 actualattempts retained. ActualGit2.30/bothhash coverage, G14-G16/hidden/aging/publicprune pass within namedplan; injectedrollback separately covered by existinghooks587. S4 bdy0 stillFAIL;8ud1 blocks acceptance. No allS1-S8/missiondone/mastermerge/install.
