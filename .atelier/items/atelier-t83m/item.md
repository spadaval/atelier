---
schema: atelier.item
schema_version: 3
id: atelier-t83m
title: Permit unchanged base records in a catch-up merge without weakening lifecycle guards
status: ready
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready:
  by: parent executing the requested guard-compliant tool repair
  at: 2026-11-15T06:11:41.633267875Z
trivial: null
caused_by: null
created_at: 2026-11-15T06:11:18.091461148Z
---

## Goal

Let an approved item commit a legitimate catch-up merge from its configured base when imported lifecycle records are unchanged from that base. Keep the pre-commit guard against newly authored unauthorized transitions.

## Done when

- [ ] Reproduce the installed v0.6.0 refusal in a disposable Git fixture with the actual pre-commit hook: a different item's ready-to-approved transition already exists on the base, and a conflict-resolved catch-up imports that record byte-for-byte but is rejected as a new act. Retain the failing baseline result.
- [ ] Implement the smallest verified-base import rule. Establish merge and configured-base identity from trusted committed Git state, validate the complete staged records as before, and accept only demonstrably unchanged imports. Do not exempt all merge commits, arbitrary parents, edited lifecycle records or staged configuration changes.
- [ ] Add positive and adversarial Git/hook regressions for unchanged imports, forged/edited imports, non-base or ambiguous parents, malformed records, ordinary unauthorized transitions, and changes on the base branch. Preserve normal review/merge checks and pre-push policy; use no guard skip or commit-plumbing bypass to claim success.
- [ ] Document the precise supported conflict-resolution route and its remaining refusals. Run required Rust checks and obtain independent review of the exact correction and any installed-v0.6.0-compatible candidate before use on another repository. Provide a pinned build/reference for the blocked integration without modifying that repository, its shared hooks or the global installed binary.

## Scope

The pre-commit guard compares staged lifecycle status against HEAD even when Git is committing a catch-up merge from the configured base. An imported ready-to-approved record can therefore be rejected despite matching the base exactly. Normal merge conflict resolution cannot finish without the currently disallowed skip. Current upstream and v0.6.0 contain this comparison; a consumer has a reviewed, fully resolved tree whose only difference from its base is its own item evidence, but cannot create the catch-up commit.

Repair the guard and its regression coverage, not the consumer's lifecycle data. Do not weaken author/review authority, globally bypass hooks, use ATELIER_SKIP, manually advance a consumer base, change its runtime, or publish private consumer records. Use synthetic record names and fixture histories in public artifacts. Consumer application is a separate, explicitly authorized step after exact-code review. A temporary reviewed build may be supplied for that step; no cargo install, shared hook replacement or global binary update is authorized here.

The consumer currently runs v0.6.0. If master differs, identify compatibility precisely and provide a minimal reviewed v0.6.0 patch/build or a tested compatible route, rather than silently requiring unrelated tool upgrades. The parent will coordinate independent review and the consumer handoff. Do the implementation yourself without implementation/research subagents; send the exact submitted head and printed normal reviewer task to the parent.

## Validation

Follow repository checks and forge workflow. Test the real hooks in owned temporary repositories and prove both acceptance and refusal with no skip environment. Preserve standard formatting, lint and relevant Rust tests; CI and normal independent review still gate integration. Keep all consumer source, indices, refs, hooks, installations and private records unchanged during preparation/review.

## Report

Planning only. Filed to unblock a verified base-import conflict under an explicit no-skip constraint. No correction, candidate authorization, consumer action or hook bypass is claimed.
