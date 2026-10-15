---
schema: atelier.review
schema_version: 1
id: atelier-jbu4
title: 'crates/atelier-app/src/lint.rs:115 graph_errors rebuilds the full RecordIndex once per issue (plus once more) whenever the graph is invalid, so atelier check (and scoped check <id>) is quadratic on the error path. Scenario: synthetic repo with 3,005 issues and one dangling blocks edge: release check 14.8 s (debug 68 s) vs about 0.1 s (debug 0.36 s) when valid; about 1.6 s extrapolated at the ADR-0019 reference scale of 973 issues. Valid graphs and mutations are unaffected. Suggested fix: make index graph validation accumulate errors in one pass, or check per-record edges against the id set without rebuilding the index; add a timing note to the S4 benchmark.'
target: atelier-1ioo
identity: independent-reviewer-2026-10-15
commit: d1cb7d908869c6ddadafcc69416ddf7aea42df75
created_at: 2026-10-15T19:13:37.153036747Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-app/src/lint.rs:115 graph_errors rebuilds the full RecordIndex once per issue (plus once more) whenever the graph is invalid, so atelier check (and scoped check <id>) is quadratic on the error path. Scenario: synthetic repo with 3,005 issues and one dangling blocks edge: release check 14.8 s (debug 68 s) vs about 0.1 s (debug 0.36 s) when valid; about 1.6 s extrapolated at the ADR-0019 reference scale of 973 issues. Valid graphs and mutations are unaffected. Suggested fix: make index graph validation accumulate errors in one pass, or check per-record edges against the id set without rebuilding the index; add a timing note to the S4 benchmark.
