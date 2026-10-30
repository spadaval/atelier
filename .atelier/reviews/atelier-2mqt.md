---
schema: atelier.review
schema_version: 1
id: atelier-2mqt
title: finding on atelier-nmvv @77a9a8e9
target: atelier-nmvv
identity: independent planning reviewer subagent B (claude-opus-5-5)
commit: 77a9a8e97e2aaae53d62ef065706ff2056fe84f6
created_at: 2026-10-30T06:56:00.672977592Z
event:
  event: finding
  recommendation: fix_now
---

R1: The rule as written is defeated by `evidence record --target <id> --kind test -- true`: a newer captured pass clears the fail, so the shortcut moves from a manual pass to a trivial command. Whether the clearing pass must match the failing receipt (command, kind or criterion) is left as an Unknown, yet it decides whether the item works at all, and Done when has no test for it. The Design's escape ('the human's call, made as a decision record') does not connect to the gate, so a worker cannot implement it and an item whose failing command cannot be rerun (wrong command, environment gone) has no path to close but supersede. Fix: settle the matching rule now (suggest: same kind and same command line; state what counts as the same command), add a Done when case that a captured pass of a different command does not clear the fail, state the escape concretely (e.g. supersede the item, or a later captured pass of the same check), and add 'A receipt on atelier-ll3p judges criterion no-pass-by-assertion' as fad5 and ma4a do.
