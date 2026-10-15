---
schema: atelier.review
schema_version: 1
id: atelier-h6fz
title: 'crates/atelier-app/src/lint.rs:157-188 the original one-dangling-edge case is fixed (3,005 issues: 14.21 s at d1cb7d90 -> 0.07 s release; valid 0.06 s), but cost is still one full RecordIndex rebuild per locally suspect record plus one per non-local error, so O(errors x issues). Scenario: 3,005 issues each with a blocks edge to a missing issue (e.g. a record file deleted that many issues reference) takes 10.34 s release; 1,502 task-owns-child errors take 3.90 s. All errors are reported. Not a regression (old code was about 14 s on both). Suggested follow-up: accumulate errors in one RecordIndex validation pass.'
target: atelier-1ioo
identity: independent-reviewer-2-2026-10-15
commit: e5a1d2f46211ec26f4c6d5f78c7bb461f65bda0b
created_at: 2026-10-15T19:20:58.879591792Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-app/src/lint.rs:157-188 the original one-dangling-edge case is fixed (3,005 issues: 14.21 s at d1cb7d90 -> 0.07 s release; valid 0.06 s), but cost is still one full RecordIndex rebuild per locally suspect record plus one per non-local error, so O(errors x issues). Scenario: 3,005 issues each with a blocks edge to a missing issue (e.g. a record file deleted that many issues reference) takes 10.34 s release; 1,502 task-owns-child errors take 3.90 s. All errors are reported. Not a regression (old code was about 14 s on both). Suggested follow-up: accumulate errors in one RecordIndex validation pass.
