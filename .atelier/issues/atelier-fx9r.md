---
created_at: "2026-06-12T05:45:00+00:00"
id: "atelier-fx9r"
issue_type: "task"
labels:
- "diagnostics"
- "projection"
- "reliability"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-k9m8"
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-eprw"
    type: "related"
  - kind: "issue"
    id: "atelier-ncog"
    type: "related"
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Fix projection stale rebuild races and diagnostics"
updated_at: "2026-06-12T05:45:00+00:00"
---

## Description

Repair the stale projection path so agents can trust projection-backed commands
when canonical Markdown changes. Prior work added automatic rebuild for some
happy paths, but current behavior still emits unbounded low-level stale-source
lists, confuses invalid canonical records with rebuild races, and can surface
temporary rebuild files or missing projection artifacts as user-facing blockers.

## Outcome

- Projection-backed reads automatically rebuild when canonical tracker records
  are valid, even when many canonical files changed.
- Invalid canonical records fail with the real lintable record error, not a
  long list of stale index entries.
- Rebuild temporary files are never scanned or reported as unsupported canonical
  tracker records.
- Concurrent or near-concurrent rebuild/query activity cannot poison lint,
  query, export, or doctor output with `.state.db.*.rebuild-tmp` paths.
- Stale projection diagnostics are bounded, grouped, and action-oriented.
- Error text distinguishes stale projection, invalid canonical records, missing
  runtime projection artifacts, and rebuild failure.
- Mission/status/doctor surfaces summarize projection health without dumping
  hundreds of changed source paths.

## Evidence

- Add regression tests for many changed canonical files that prove output is
  bounded and automatic rebuild succeeds when canonical records validate.

- Add regression tests for invalid canonical Markdown that prove the primary
  error names the malformed record and the correct repair command.

- Add regression tests or a deterministic smoke test for rebuild/query
  interleaving that proves temporary rebuild files are ignored by canonical
  validation.

- Add tests proving `.state.db.*.rebuild-tmp` never appears in lint, query,
  export, or doctor diagnostics.

- Run focused projection freshness tests.

- Run `atelier lint`, `atelier doctor`, and `atelier export --check` after the
  canonical migration and projection repair issues land.

## Notes

Related prior work:

- `atelier-ncog` added transparent stale projection rebuild but did not fully
  close concurrency and diagnostics gaps.
- `atelier-eprw` covered invalid Markdown blocking stale projection reads, but
  current command behavior still produces confusing stale-index context around
  the real canonical error.
