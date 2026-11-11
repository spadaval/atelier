---
schema: atelier.item
schema_version: 3
id: atelier-bp3s
title: Add a suite merge check to demiurge, cosplay-studio and life-on-rails
status: draft
mission: null
labels: []
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-06T23:39:07.679326401Z
---

## Goal

Give demiurge, cosplay-studio and life-on-rails a test of the merged tree. They have no CI (life-on-rails' Android builds run off-box, so its check may be limited to what runs on this host) and no suite merge check (life-on-rails has only a whitespace check); until atelier-s66u ships, the only such test is `merge --sync` rerunning recorded suite receipts, which atelier-s66u removes.

## Done when

- [ ] Each repository's `.atelier/workflow.yaml` has a `merge_checks` `shell.check` that runs its test suite on the tree that lands, under the repository's current merge strategy, and one `atelier merge` there shows the check running and a failing suite refusing the merge.

## Context

Pick this up when each repository next adopts an Atelier release containing atelier-s66u (mission atelier-n4mj). It is outside the mission because it is work in other repositories. Strategy: ~/projects/atelier/docs/strategies/lighter-workflow.md (Scope, Risks).

## Links

Relationships of the old record that the new format has no field for.

- related: atelier-s66u (Evidence records only what nothing else records)
