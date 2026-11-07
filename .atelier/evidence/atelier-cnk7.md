---
created_at: "2026-11-07T00:49:42.621884743+00:00"
id: "atelier-cnk7"
evidence_type: "validation"
commit: "1c952fe14c67473bfc68e80017e417a46c0e5a82"
result: "pass"
captured_at: "2026-11-07T00:49:42.621467920+00:00"
agent_identity: "Claude Opus 5.5 (implementer)"
target:
  kind: "issue"
  id: "atelier-hwxm"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-hwxm"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Ran migrate dispositions --apply with this branch's binary on fresh clones of atelier, astromech, demiurge and cosplay-studio under ~/.cache/atelier-tmp (the original repositories untouched), committed the result, then atelier check and doctor. atelier: 577 findings folded, 580 events removed, doctor passed (253 done claims, whole review history). astromech: 1060 findings, 1077 events, doctor passed (352 claims). demiurge: 161 findings, 167 events, doctor passed (134 claims). cosplay-studio: 146 findings, 146 events, check passed; doctor fails with 'review needs YAML front matter' from legacy .yaml review files in its history (such as .atelier/reviews/atelier-8bym.yaml), which the installed 0.3.1+711abe6f binary also reports on an unmigrated clone, so it predates this change."
updated_at: "2026-11-07T00:49:42.622973559+00:00"
---

Ran migrate dispositions --apply with this branch's binary on fresh clones of atelier, astromech, demiurge and cosplay-studio under ~/.cache/atelier-tmp (the original repositories untouched), committed the result, then atelier check and doctor. atelier: 577 findings folded, 580 events removed, doctor passed (253 done claims, whole review history). astromech: 1060 findings, 1077 events, doctor passed (352 claims). demiurge: 161 findings, 167 events, doctor passed (134 claims). cosplay-studio: 146 findings, 146 events, check passed; doctor fails with 'review needs YAML front matter' from legacy .yaml review files in its history (such as .atelier/reviews/atelier-8bym.yaml), which the installed 0.3.1+711abe6f binary also reports on an unmigrated clone, so it predates this change.
