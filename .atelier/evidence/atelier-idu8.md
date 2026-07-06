---
created_at: "2026-07-06T21:16:26.442951530+00:00"
id: "atelier-idu8"
evidence_type: "test"
captured_at: "2026-07-06T21:16:26.431766860+00:00"
command: "sh -c 'set -eu; ! rg -n \"non-issue kinds are missions|first-class issue, mission|issue, mission, and evidence\" docs/architecture/markdown-first-record-store.md; rg -n \"active v1 non-issue kinds are evidence and review|Missions use the issue record contract|cache-source coverage therefore spans|issue, evidence, and review\" docs/architecture/markdown-first-record-store.md; rg -n \"pub const ISSUE_KIND|pub const FIRST_CLASS_RECORD_KINDS|kind: \\\"issue\\\"|kind: \\\"evidence\\\"|kind: \\\"review\\\"\" crates/atelier-records/src/record_kinds.rs'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-mska"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-mska"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "sh -c 'set -eu; ! rg -n \"non-issue kinds are missions|first-class issue, mission|issue, mission, and evidence\" docs/architecture/markdown-first-record-store.md; rg -n \"active v1 non-issue kinds are evidence and review|Missions use the issue record contract|cache-source coverage therefore spans|issue, evidence, and review\" docs/architecture/markdown-first-record-store.md; rg -n \"pub const ISSUE_KIND|pub const FIRST_CLASS_RECORD_KINDS|kind: \\\"issue\\\"|kind: \\\"evidence\\\"|kind: \\\"review\\\"\" crates/atelier-records/src/record_kinds.rs'"
updated_at: "2026-07-06T21:16:26.445319013+00:00"
---

## Summary

sh -c 'set -eu; ! rg -n "non-issue kinds are missions|first-class issue, mission|issue, mission, and evidence" docs/architecture/markdown-first-record-store.md; rg -n "active v1 non-issue kinds are evidence and review|Missions use the issue record contract|cache-source coverage therefore spans|issue, evidence, and review" docs/architecture/markdown-first-record-store.md; rg -n "pub const ISSUE_KIND|pub const FIRST_CLASS_RECORD_KINDS|kind: \"issue\"|kind: \"evidence\"|kind: \"review\"" crates/atelier-records/src/record_kinds.rs'

## Command

```console
sh -c 'set -eu; ! rg -n "non-issue kinds are missions|first-class issue, mission|issue, mission, and evidence" docs/architecture/markdown-first-record-store.md; rg -n "active v1 non-issue kinds are evidence and review|Missions use the issue record contract|cache-source coverage therefore spans|issue, evidence, and review" docs/architecture/markdown-first-record-store.md; rg -n "pub const ISSUE_KIND|pub const FIRST_CLASS_RECORD_KINDS|kind: \"issue\"|kind: \"evidence\"|kind: \"review\"" crates/atelier-records/src/record_kinds.rs'
```

Exit status: 0

## Stdout

Bytes: 577
Truncated: no

```text
70:lists. The active v1 non-issue kinds are evidence and review. Missions use the
331:- `RecordStore` owns first-class issue, evidence, and review record files;
333:  `issue_type: "mission"`. Rebuild and cache-source coverage therefore spans
334:  issue, evidence, and review. `RecordStore` must not absorb activity event
13:pub const ISSUE_KIND: RecordKindSpec = RecordKindSpec {
14:    kind: "issue",
21:pub const FIRST_CLASS_RECORD_KINDS: &[RecordKindSpec] = &[
23:        kind: "evidence",
30:        kind: "review",
41:        kind: "evidence",
48:        kind: "review",
```

## Stderr

Bytes: 0
Truncated: no

```text
```
