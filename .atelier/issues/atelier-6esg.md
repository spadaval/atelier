---
created_at: "2026-10-17T01:10:57.968962482+00:00"
id: "atelier-6esg"
issue_type: "feature"
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
closed_at: "2026-11-07T21:04:24.972698957+00:00"
status: "superseded"
title: "Warn when a bundle evidence commit is not in the repository"
updated_at: "2026-11-07T21:04:24.972698957+00:00"
---

## Goal

The shipped fixture docs/spec/bundle/fixtures/valid-bundle.json gives its evidence `commit: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa`. Apply accepts it and `atelier check` passes. The schema says such a receipt cannot count as proof, but nothing tells the author.

## Done when

- Preview and apply print `⚠ evidence.bundle-contract: commit aaaaaaaa is not in this repository; it cannot count as proof` and still succeed.
- A test asserts the warning.

## Report

A receipt proves nothing since atelier-md0v, so a warning that one "cannot count as proof" warns about a gate that no longer exists (decision atelier-mzpn).
