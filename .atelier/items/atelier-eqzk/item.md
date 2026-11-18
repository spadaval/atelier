---
schema: atelier.item
schema_version: 3
id: atelier-eqzk
title: Stop test-extension.mjs leaving its temp folder behind in /tmp
status: draft
mission: null
labels: []
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-18T01:07:36.137201812Z
---

## Goal

`t3-extension/test-extension.mjs` makes a folder with
`fs.mkdtemp(os.tmpdir() + '/atelier-extension-test-')` and leaves it. On
2026-11-18 `/tmp` here held 17 of them, 436 MB together, and with review
clones they put the user over the `/tmp` quota, which broke command output
for an implementer of atelier-9f7b.

## Done when

- [ ] A run of `test-extension.mjs` removes its temp folder when it passes, when a check fails and when it is interrupted, with a way to keep it for debugging.
- [ ] The extension's other test scripts that make temp folders do the same.

## Scope

In: the test scripts under `t3-extension/`. Out: review clones other tools
leave in `/tmp`.
