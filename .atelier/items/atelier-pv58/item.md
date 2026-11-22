---
schema: atelier.item
schema_version: 3
id: atelier-pv58
title: Move astromech's ATELIER_REV pin before astromech files its next item
status: open
mission: null
labels:
- needs-human
priority: high
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-22T18:33:36.796809892Z
---

## Goal

astromech's CI installs Atelier at a pinned commit: `ATELIER_REV:
24a00e836b7bd6cedbda3d6bd1b1ab2d60650896` (v0.7.0) in its
`.github/workflows/atelier.yml`, and runs `atelier check` on every pull
request and push to `main`. That commit's `Status` has no `open`. Since
atelier-v06d merged (3f97eaae8) and was installed on this host on
2026-11-22, `atelier new` writes `status: open`, so the first item filed
in astromech will fail its `atelier check` job. The other repositories'
workflows run no `atelier check`. astromech held no open item when this
was filed. The work belongs in astromech, which atelier-dh7a may not
write, so it waits for the human to send someone there.

## Done when

- [ ] astromech's `ATELIER_REV` names a commit of Atelier's `master` at or
  after 3f97eaae8, and its `atelier check` job passes on a pull request
  that holds an open item.

## Scope

astromech's `.github/workflows/atelier.yml`, changed by astromech's own
process. Move the pin in the same pull request as the first open record,
or before it. Nothing in this repository.
