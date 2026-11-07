---
schema: atelier.review
schema_version: 1
id: atelier-pyun
title: disposition on atelier-s66u @f7a9ab51
target: atelier-s66u
identity: Sumeet Padavala <sumeet.padavala@gmail.com>
commit: f7a9ab5166dde310f18c7ad345f1c3cd359ce11d
created_at: 2026-11-07T01:07:31.626403597Z
event:
  event: disposition
  finding: atelier-gf8h
  decision: fix_now
  supersedes: null
  resolution_commit: b388ad0bf3d3aa8cee5960e2de10f9a150a059fd
  follow_up: null
---

evidence list's verdict states a missing or stale pass without a warning unless the policy runs evidence.attached on the item's workflow; tested by evidence_list_marks_a_missing_pass_only_under_an_evidence_attached_gate.
