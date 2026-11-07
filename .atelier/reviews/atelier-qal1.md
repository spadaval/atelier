---
schema: atelier.review
schema_version: 2
id: atelier-qal1
title: 'crates/atelier-app/src/lint.rs:187 graph_errors sets aside every record an error names, including the target of the edge, and records that are not locally suspect are never revalidated on their own, so a second independent error on a named record disappears (regression from d1cb7d90, breaks atelier-idae). Scenario: task E has children [C] (hierarchy error naming C and E) and C relates D with unconfigured type ''frobnicates''. ba7512b7 check reports 1 finding (hierarchy only); d1cb7d90 reports 2. Suggested fix: as for the ID-less error finding, check per-record errors without relying on set-aside, and test an error on a record that another error also names.'
target: atelier-1ioo
identity: independent-reviewer-2-2026-10-15
commit: e5a1d2f46211ec26f4c6d5f78c7bb461f65bda0b
created_at: 2026-10-15T19:20:58.842845694Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: rebuild-orchestrator
    commit: fdf8fd68061e5cfa8b6a9289b8830f0e50c9f637
    recorded_at: 2026-10-15T19:24:51.572049236Z
    resolution_commit: fdf8fd68061e5cfa8b6a9289b8830f0e50c9f637
    reason: Validation now collects every error in one linear pass; check reads that list. Fixed in fdf8fd68; test extended with the reviewer's scenarios.
    migrated_from: atelier-g5ex
---

crates/atelier-app/src/lint.rs:187 graph_errors sets aside every record an error names, including the target of the edge, and records that are not locally suspect are never revalidated on their own, so a second independent error on a named record disappears (regression from d1cb7d90, breaks atelier-idae). Scenario: task E has children [C] (hierarchy error naming C and E) and C relates D with unconfigured type 'frobnicates'. ba7512b7 check reports 1 finding (hierarchy only); d1cb7d90 reports 2. Suggested fix: as for the ID-less error finding, check per-record errors without relying on set-aside, and test an error on a record that another error also names.
