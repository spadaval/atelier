---
created_at: "2026-11-09T02:02:04.388927238+00:00"
id: "atelier-ah82"
issue_type: "task"
labels:
- "forge"
workflow_branch:
  branch_base: master
  work_branch: atelier-ah82
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-1gb3"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Record ADR 0031 and document the forge, ci.passed and the one-command merge before building them"
updated_at: "2026-11-09T02:42:21.785184840+00:00"
---

## Goal

Write decision atelier-vmq2 down as ADR 0031 and describe the new interface in the product docs, so the forge policy keys, the `ci.passed` check and the merge flow are reviewed on paper before any code is written (strategy `docs/strategies/forge-delivery.md`, Shape).

## Done when

- [x] `docs/adr/0031-*.md` states the decision: a forge carries branches and reports CI; reviews, findings and approvals stay repository records (ADR 0019 Decision 6 stands); GitHub first, through `gh`; a pull request's CI run tested the branch head merged with the base as it was then (`refs/pull/<n>/merge`), which Atelier accepts as the CI gate, so a moved base needs no new run, and CI on the base after the push is the backstop for the combination nobody tested; a failing base CI run is shown, never a gate. It is marked as scoping ADR 0019 where 0019 removed the provider, and ADR 0019 links to it.
- [x] `PRODUCT_INTENT.md`'s Boundaries no longer read as ruling out a forge: Atelier is still not a review provider, and may push branches, open pull requests and read CI when a repository's policy names a forge.
- [x] `docs/product/workflow-configuration.md` gives the policy keys that name the forge and the trusted CI workflow, with an example, and says a policy without them uses no network.
- [x] `docs/product/reviews-and-merge.md` describes `ci.passed` (what it proves, when it passes, its refusals and their Next lines), the merged-tree rule's scope (local `shell.check`s prove the tree Atelier merges; `ci.passed` accepts a pull request run, which tested the head merged with an earlier base, and says so), and `merge --push` and `--wait` step by step, including the pull request and what happens when `gh` is missing or signed out.
- [x] No other current doc contradicts these (`git grep -n -i "merge-ci-passed\|review provider\|forge" docs PRODUCT_INTENT.md CONTEXT.md` shows each hit updated or historical).

## Scope

Docs and the ADR only; no code. Strategy files stay as they are.

## Context

The interface these docs set is what atelier-7tul builds. Name policy keys and flags here; the strategy's Authority lets the orchestrator choose them.

## Report

All five Done when lines hold. ADR 0031 (`docs/adr/0031-a-forge-carries-branches-and-ci.md`) records atelier-vmq2 and is marked as scoping ADR 0019, whose Status now links to it. `PRODUCT_INTENT.md` Boundaries keep "not a review provider" and allow a forge for branches, pull requests and CI when policy names one. `docs/product/workflow-configuration.md` has a new Forge section with the keys, an example and the no-network rule. `docs/product/reviews-and-merge.md` has a new "CI on a forge" section: what `ci.passed` proves, its two pass lines, a refusal table with Next lines, the base-run note, `merge --push` in eight steps and `--wait`; the merged-tree paragraph now says it covers the local `shell.check`s. The Done when grep (`git grep -n -i "merge-ci-passed\|review provider\|forge" docs PRODUCT_INTENT.md CONTEXT.md`) leaves only new text, strategies and historical ADRs; `docs/index.md:6` is ADR 0019's own history. "Atelier never pushes" in `cli-surface.md` and `repository-contribution.md` and the merged-tree claims in `validation.md` and `CONTEXT.md` were updated. `atelier check` and `git diff --check` pass. This is docs only, so there is no test; the proof is the grep and the checks.

### Deviations

- `ci.passed` is written `- ci.passed`, not `- ci.passed: {}`. The workflow parser writes paramless built-ins as plain strings (`- blockers.none_open`) and refuses params on every built-in but `shell.check` ("built-in validator '...' does not accept params"), so the string form matches the schema.
- The merge help text is clap doc comments in `crates/atelier-cli/src/main.rs` (`Commands::Merge`), so it is left for atelier-7tul.

### Decisions

Interface choices the brief left open, for atelier-7tul and later items to match:

- `forge.remote` defaults to `origin`; `forge.provider` and `forge.ci_workflow` are required. Policy errors: `workflow_config_invalid_forge` (malformed `forge`, naming the key) and `workflow_config_invalid_validator` (`ci.passed` without `forge`, or in transition validators).
- Only `pull_request` event runs count for `ci.passed`; `push` runs do not.
- `ci.passed` prints its pass on its own line instead of collapsing, and the merge commit message repeats it.
- Refusal codes: `forge_gh_missing`, `forge_gh_signed_out`, `forge_unreachable`, `ci_no_run`, `ci_pending`, `ci_failed`, `merge_push_no_forge`, `merge_push_branch_rejected`, `merge_push_base_diverged`, `merge_push_base_rejected`.
- `merge --push` pushes the branch and ensures the pull request even when a later gate refuses, so CI can start before approval; it fetches and fast-forwards the local base before merging; it pushes the base after land's base judgment; it never deletes the remote branch; an unmerged pull request after the push is a note with exit zero.
- `--wait` polls every 30 seconds, `--timeout <minutes>` defaults to 30, a run not yet started counts as pending for 5 minutes after a push; `--push` and `--wait` conflict with `--check`, `--json` and `--sync`.

### Open items

- An installed binary older than atelier-7tul refuses a policy with a top-level `forge` key (`workflow_config_unknown_field`), and this host shares one installed binary across repositories. atelier-7tul should decide whether `forge` needs a schema version bump and reinstall before this repository's policy uses it.
- `repository-contribution.md` says this repository's policy names no forge yet; the item that moves this repository's policy updates it, along with AGENTS.md.
