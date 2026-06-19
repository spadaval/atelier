---
created_at: "2026-06-19T03:57:15.108318890+00:00"
id: "atelier-7v02"
issue_type: "epic"
labels:
- "config"
- "review"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-0jsk"
  - kind: "issue"
    id: "atelier-69g3"
  - kind: "issue"
    id: "atelier-8uys"
  - kind: "issue"
    id: "atelier-at7i"
  - kind: "issue"
    id: "atelier-kyi8"
  - kind: "issue"
    id: "atelier-oe7c"
  - kind: "issue"
    id: "atelier-onkp"
  - kind: "issue"
    id: "atelier-q199"
  - kind: "issue"
    id: "atelier-qdgh"
  - kind: "issue"
    id: "atelier-rb5b"
  - kind: "issue"
    id: "atelier-swxv"
  - kind: "issue"
    id: "atelier-unwz"
  - kind: "issue"
    id: "atelier-zwe9"
  children:
  - kind: "issue"
    id: "atelier-13yy"
  - kind: "issue"
    id: "atelier-9h5w"
  - kind: "issue"
    id: "atelier-tv53"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-19T04:27:32.744292788+00:00"
status: "done"
title: "Epic: Configure mutually exclusive review modes"
updated_at: "2026-06-19T04:27:32.744292788+00:00"
---

## Description

Add project configuration for mutually exclusive review modes and migrate
Forgejo settings into the provider-backed review namespace. This epic owns
configuration shape, validation, migration, and starter guidance.

## Outcome

- `.atelier/config.toml` supports `[review] mode = "room"` and provider-backed
  mode with `provider = "forgejo"`.
- Forgejo host, owner, repo, admin token environment, and role authorship move
  under nested review provider configuration.
- Configuration validation rejects missing modes, conflicting room/provider
  settings, room-only commands in provider mode, and provider-only commands in
  room mode with direct guidance.
- Starter workflow/docs show how to choose exactly one review mode.

## Evidence

- Config parser and migration tests cover room mode, provider mode, invalid
  mixed config, and migrated Forgejo config.
- CLI/config validation output demonstrates wrong-mode and missing-config
  guidance.
- `atelier lint atelier-7v02`, focused config tests, and `git diff --check`
  pass.
