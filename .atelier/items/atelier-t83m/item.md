---
schema: atelier.item
schema_version: 3
id: atelier-t83m
title: Permit unchanged base records in a catch-up merge without weakening lifecycle guards
status: approved
mission: null
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3Ab4fa2cab-3330-4c69-8257-673a9c07a7c0%3Adelegate-task%3A38b51059-4012-497c-b7ab-c6f0d002f223
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

- [x] Reproduce the installed v0.6.0 refusal in a disposable Git fixture with the actual pre-commit hook: a different item's ready-to-approved transition already exists on the base, and a conflict-resolved catch-up imports that record byte-for-byte but is rejected as a new act. Retain the failing baseline result.
- [x] Implement the smallest verified-base import rule. Establish merge and configured-base identity from trusted committed Git state, validate the complete staged records as before, and accept only demonstrably unchanged imports. Do not exempt all merge commits, arbitrary parents, edited lifecycle records or staged configuration changes.
- [x] Add positive and adversarial Git/hook regressions for unchanged imports, forged/edited imports, non-base or ambiguous parents, malformed records, ordinary unauthorized transitions, and changes on the base branch. Preserve normal review/merge checks and pre-push policy; use no guard skip or commit-plumbing bypass to claim success.
- [x] Document the precise supported conflict-resolution route and its remaining refusals. Run required Rust checks and obtain independent review of the exact correction and any installed-v0.6.0-compatible candidate before use on another repository. Provide a pinned build/reference for the blocked integration without modifying that repository, its shared hooks or the global installed binary.

## Scope

The pre-commit guard compares staged lifecycle status against HEAD even when Git is committing a catch-up merge from the configured base. An imported ready-to-approved record can therefore be rejected despite matching the base exactly. Normal merge conflict resolution cannot finish without the currently disallowed skip. Current upstream and v0.6.0 contain this comparison; a consumer has a reviewed, fully resolved tree whose only difference from its base is its own item evidence, but cannot create the catch-up commit.

Repair the guard and its regression coverage, not the consumer's lifecycle data. Do not weaken author/review authority, globally bypass hooks, use ATELIER_SKIP, manually advance a consumer base, change its runtime, or publish private consumer records. Use synthetic record names and fixture histories in public artifacts. Consumer application is a separate, explicitly authorized step after exact-code review. A temporary reviewed build may be supplied for that step; no cargo install, shared hook replacement or global binary update is authorized here.

The consumer currently runs v0.6.0. If master differs, identify compatibility precisely and provide a minimal reviewed v0.6.0 patch/build or a tested compatible route, rather than silently requiring unrelated tool upgrades. The parent will coordinate independent review and the consumer handoff. Do the implementation yourself without implementation/research subagents; send the exact submitted head and printed normal reviewer task to the parent.

## Validation

Follow repository checks and forge workflow. Test the real hooks in owned temporary repositories and prove both acceptance and refusal with no skip environment. Preserve standard formatting, lint and relevant Rust tests; CI and normal independent review still gate integration. Keep all consumer source, indices, refs, hooks, installations and private records unchanged during preparation/review.

## Report

The pre-commit guard now distinguishes an unchanged import from a newly
authored lifecycle transition. A single incoming merge parent must equal the
local configured base tip or the configured forge remote's base tracking tip.
The config authority comes from HEAD, and the staged config must equal HEAD.
Only byte-identical item/mission files skip the lifecycle comparison; staged
parsing, validation, base writes, review, merge and push guards still apply.

The installed v0.6.0 hook rejected the real conflict-resolution fixture with
ready-to-approved exactly as reported. The retained baseline and minimal
release-compatible patch are in artifacts/, with pinned source, patch hash,
binary reference and reproduction instructions in artifacts/compatibility.md.
The compatibility patch has the same helper and exception as current code,
with no crate consolidation or other upgrade required.

Validation: the complete CLI test suite passed; all five new real-hook tests
passed on current code and the patched v0.6.0 source. These include local and
configured remote base imports, linked worktrees, edited/forged records,
non-base/older/ambiguous parents, revision expressions, missing merge state,
staged and unstaged config redirection, invalid and malformed base records,
ordinary unauthorized transitions and writes on the base. Workspace Clippy
with all targets and warnings denied passed. Formatting, locked metadata,
whitespace and atelier check passed. The release-compatible source also passed
its hook tests, formatting and all-target Clippy.

Independent review is the next workflow step, coordinated by the parent as
specified in Scope. The final acceptance line is ticked for the prepared,
pinned candidate and the mandatory review-before-use constraint; it does not
claim that independent approval has already occurred. Neither candidate may
be used on another repository before exact-code review. No consumer state,
shared hook or global installation was modified. No bypass was used.

### Publication follow-up

Independent normal review 1-wzd0 approved the exact correction and pinned
v0.6.0 candidate without findings; committed as b9d690f8. Current-head CI
initially rejected the preserved patch's required blank unified-diff context
lines as trailing whitespace. An artifact-local Git attribute disables only
blank-at-eol checking for that exact patch. Its bytes, hash, reviewed source
and candidate binary remain unchanged. No lifecycle guard or CI command was
bypassed. The complete base-to-head whitespace check now passes.
