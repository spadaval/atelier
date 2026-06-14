---
created_at: "2026-06-12T04:53:56.961262790+00:00"
id: "atelier-5i4h"
issue_type: "task"
labels:
- "assignee:root"
- "evidence"
- "markdown"
- "migration"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-n1ys"
  - kind: "issue"
    id: "atelier-uibk"
  children: []
  attachments:
  - kind: "evidence"
    id: "atelier-lboe"
    role: "validates"
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-12T05:12:27.609461536+00:00"
status: "done"
title: "Remove issue evidence_required frontmatter field"
updated_at: "2026-06-12T05:12:27.609461536+00:00"
---

## Description

Remove the issue-level YAML `evidence_required` field from canonical issue
records. Evidence requirements should live in the Markdown Evidence section, and
actual proof should live in attached `atelier evidence` records.
- Canonical issue frontmatter no longer requires or writes
  `evidence_required`.
- Issue parsing rejects or migrates legacy `evidence_required` according to the
  section-contract migration policy.
- Issue rendering does not emit `evidence_required`.
- Existing `evidence_required` content, if any, is preserved by moving it into
  the Markdown Evidence section or by reporting it for human migration when it
  cannot be converted safely.
- Documentation describes Evidence as body-authored proof expectations plus
  attached evidence records, not YAML issue metadata.
- Parser/rendering tests prove issue records round-trip without
  `evidence_required`.
- Migration tests cover empty `evidence_required: []` and non-empty legacy
  values.
- Repository migration removes `evidence_required` from issue Markdown after the
  schema change lands.
- Run `atelier export --check` and `atelier lint`.
This is separate from evidence records themselves. The `atelier evidence`
record type and evidence attachments remain part of the product; only the issue
frontmatter requirement list is being removed.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
