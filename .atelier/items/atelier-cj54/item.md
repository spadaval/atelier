---
schema: atelier.item
schema_version: 3
id: atelier-cj54
title: Let issue update replace one body section
status: dropped
mission: atelier-uf97
labels:
- issue-commands
- feature
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T01:36:20.904463567Z
---

## Goal

`atelier issue update --body` replaces every typed section, so agents rewrite a single section by script. On 2026-10-16 there were about 20 Python `str.replace` or section-splice edits of `.atelier/issues/*.md`, changing a Done when, a Design section, a Report or one Scope line. One script split on the first `## Context` and duplicated sections, which a later command refused ("Duplicate issue body section 'Context'"). Editing records directly stays supported for bulk work; this item covers the one-section case.

## Done when

- [ ] `atelier issue update <id> --section "Done when"` replaces only that section, with the text from stdin or a file so Markdown never passes through shell quoting; the result is validated like `atelier check`, and Notes and the Report are kept.
- [ ] An unknown or disallowed section is refused, and the error names the allowed sections for the record's type.
- [ ] A done record is refused, as `--body` refuses it today.
- [ ] The confirmation follows atelier-7ebi. Tests cover a replace, an unknown section and a done record.

## Context

From the 2026-10-16 transcript retrospective. Pairs with atelier-sxcj (`--body-file`). Dropped in decision atelier-mzpn: the rule for editing a completion claim (atelier-jy9t), since done now means merged.

## Notes

- 2026-10-17 [note] Retro 2026-10-17: agents keep splicing record and strategy bodies with Python str.replace (e592938d three times). Still wanted; pairs with the new --body-file item.

## Links

Relationships of the old record that the new format has no field for.

- related: atelier-sxcj (Accept --body-file in issue create and issue update)

## Report

Dropped: `issue update --body` was removed in 0.5; a record's body is edited in its file, so one section is changed directly.
