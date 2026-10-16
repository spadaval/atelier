---
created_at: "2026-10-16T21:52:40.300016659+00:00"
id: "atelier-tqru"
issue_type: "bug"
labels:
- "adoption"
workflow_branch:
  branch_base: master
  work_branch: bug/atelier-tqru
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T22:00:38.278365223+00:00"
status: "done"
title: "atelier check accepts duplicate YAML keys in records"
updated_at: "2026-10-16T22:00:38.278365223+00:00"
---

## Goal

A git merge resolved by hand can leave a relates entry with two `id:` keys under one `kind`/`type`. The YAML parser keeps the last key, so one relationship silently disappears, and `atelier check` reports "Lint passed".

## Done when

`atelier check` (and every record read) rejects duplicate keys in a record's front matter, naming the file and key; a test with a duplicated `id` in a relates entry fails the check.

## Context

Found 2026-10-16 in astromech: merging main into a feature branch produced `id: astromech-gtza` and `id: astromech-i4gx` in one entry of astromech-7q0z.md; an independent reviewer caught it, `atelier check` did not. Frequent base merges with several owners in flight make relates-list conflicts common.

## Close Reason

Every record read (and so atelier check) first parses front matter with a visitor that rejects a mapping repeating a key, at any depth, reporting 'Invalid YAML front matter in <path>: duplicate key <key>'. Test test_check_rejects_repeated_keys_in_record_front_matter covers a duplicated id in a relates entry and a repeated top-level key, and fails without the check; this repository's records pass.
