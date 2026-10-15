---
schema: atelier.review
schema_version: 1
id: atelier-qal1
title: 'crates/atelier-app/src/lint.rs:187 graph_errors sets aside every record an error names, including the target of the edge, and records that are not locally suspect are never revalidated on their own, so a second independent error on a named record disappears (regression from d1cb7d90, breaks atelier-idae). Scenario: task E has children [C] (hierarchy error naming C and E) and C relates D with unconfigured type ''frobnicates''. ba7512b7 check reports 1 finding (hierarchy only); d1cb7d90 reports 2. Suggested fix: as for the ID-less error finding, check per-record errors without relying on set-aside, and test an error on a record that another error also names.'
target: atelier-1ioo
identity: independent-reviewer-2-2026-10-15
commit: e5a1d2f46211ec26f4c6d5f78c7bb461f65bda0b
created_at: 2026-10-15T19:20:58.842845694Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-app/src/lint.rs:187 graph_errors sets aside every record an error names, including the target of the edge, and records that are not locally suspect are never revalidated on their own, so a second independent error on a named record disappears (regression from d1cb7d90, breaks atelier-idae). Scenario: task E has children [C] (hierarchy error naming C and E) and C relates D with unconfigured type 'frobnicates'. ba7512b7 check reports 1 finding (hierarchy only); d1cb7d90 reports 2. Suggested fix: as for the ID-less error finding, check per-record errors without relying on set-aside, and test an error on a record that another error also names.
