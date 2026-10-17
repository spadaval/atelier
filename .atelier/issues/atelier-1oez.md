---
created_at: "2026-10-17T01:10:57.969258244+00:00"
id: "atelier-1oez"
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
    id: "atelier-2mky"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Regenerate the bundle preview fixture and test it against real output"
updated_at: "2026-10-17T01:10:57.969258244+00:00"
---

## Goal

docs/spec/bundle/fixtures/dry-run-preview.txt lacks the `updated:` count line and the whole `Client References` section that `atelier bundle preview docs/spec/bundle/fixtures/valid-bundle.json` prints today. The schema calls the file "current human CLI output", so readers learn the wrong shape.

## Done when

- A test runs preview on valid-bundle.json in a fresh `atelier init` repository and compares the output to dry-run-preview.txt, so output changes must update the fixture.
- The fixture matches current output.

## Context

Land this after the preview-tree item in this mission, or regenerate the fixture again when that changes the output.
