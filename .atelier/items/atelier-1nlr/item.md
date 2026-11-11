---
schema: atelier.item
schema_version: 3
id: atelier-1nlr
title: Make land refuse a plain git merge that brings unreviewed code onto the base
status: draft
mission: null
labels:
- review
- bug
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-08T22:36:30.936692751Z
---

## Goal

`atelier land` is the last gate before pushing the base, and it refuses a direct code commit on master. A plain `git merge --no-ff` of an unreviewed code branch should be refused the same way.

## Done when

- [ ] A test reproduces it: on a scratch repository, a plain `git merge --no-ff` of an unreviewed branch that changes code makes `atelier land` refuse, naming the merge.
- [ ] A merge made by `atelier merge` still lands.

## Context

Reported by the atelier-dkul implementer on 2026-11-08, not yet reproduced by anyone else. On a scratch clone, a plain `git merge --no-ff` of the unreviewed skill import into master passed `atelier land`, which reported "2 unpushed record-only commits outside review", while a direct code commit on master was refused. The import used `--allow-unrelated-histories`, which may matter.
