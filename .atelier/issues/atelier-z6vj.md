---
created_at: "2026-10-17T01:10:57.968659947+00:00"
id: "atelier-z6vj"
issue_type: "bug"
labels:
- "bundle"
- "cli-output"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-uf97"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T03:55:31.873423713+00:00"
status: "declined"
title: "Make bundle errors name the entry and drop the garbage tail"
updated_at: "2026-11-11T03:55:31.873423713+00:00"
---

## Goal

Bundle validation errors reuse record-parser messages that do not fit bundles:

- A body missing Done when: `Invalid body for a … Missing required issue body section 'Done when' for issue (unknown), section Done when, path .atelier/<bundle issue body>`.
- An unknown field (`unknown field 'descripton', expected one of ...`) does not say which entry it is in.
- An unknown resource lists `missions` among the expected values (`expected one of 'issues', 'missions', 'evidence'`) even though `resources.missions` is rejected.
- A bundle with no resources prints `Bundle preview is valid.` with all counts at zero.

## Done when

- [ ] The missing-section error reads `a: body is missing ## Done when`.
- [ ] Unknown-field errors name the client_ref, or the array index when there is none.
- [ ] The expected-resources list omits `missions`.
- [ ] An empty bundle fails with `Bundle has no records`.
- [ ] Tests cover each message.

## Context

crates/atelier-cli/src/commands/bundle.rs:38-58 (parse errors), :143 and :472 (body errors).

## Report

Dropped in the cutover sort (atelier-5zre), reason (a). It changes `bundle` error messages, and the What is removed table removes `bundle`.
