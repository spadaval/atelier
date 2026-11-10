---
created_at: "2026-11-10T00:56:07.273072817+00:00"
id: "atelier-syyj"
issue_type: "task"
labels:
- "redesign"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-5cuc"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Add `merge` on the new records, with `--trivial` and no waiting"
updated_at: "2026-11-10T02:41:07.155326242+00:00"
---

## Goal

Finish the loop: an approved item reaches the base in one merge commit that marks it done, after three checks an agent could already see in `status`.

## Done when

- [ ] Scenario test, no forge: on an approved item, `atelier-next merge` makes one merge commit on the base in which the item's status is `done`.
- [ ] Staleness, each as its own test: after the reviewed commit, a change to a source file, to `.atelier/config.toml`, to another item's directory, or to this item's Goal, Done when or Scope makes `merge` change nothing and say what changed since round 1 and to run `submit`; `status` says the same. A change to this item's Report, or a new review file, does not block the merge.
- [ ] A configured merge check that fails on the merged tree stops the merge and prints its command and the end of its output. The config is read from the base's tip: a test removes the check from `config.toml` on the work branch, and it still runs.
- [ ] An unanswered entry in the item's Decisions section stops the merge and is named.
- [ ] `merge --trivial "<why>"` on a building item with ticked Done when lines and a Report merges with no review and stores the reason in `trivial`. It is refused on an item that has a review file.
- [ ] With a forge that names a CI workflow, against the existing test doubles for `gh`: the first run pushes the branch and opens the pull request; while CI has not finished on the head it prints the pull request, changes nothing else, and exits with a status of its own that is neither success nor a refusal. Run again once CI has passed, it merges and pushes the base. Run again after CI failed, it refuses and says so.
- [ ] `merge` accepts no flag but `--trivial`, and no code path sleeps or polls.
- [ ] With a forge that names no CI workflow: one run pushes the branch, opens the pull request, merges and pushes the base.
- [ ] The Merge section of `docs/strategies/redesign-architecture.md` matches what was built.

## Scope

`atelier-app/src/forge.rs` is kept with one change: it takes the new config's forge section, whose CI workflow is optional, where today it takes `workflow_policy::ForgeConfig` with a required workflow name; its own contract tests still pass. The merged-tree checkout, the merge commit and the push are extracted from `merge.rs`, `merge_push.rs` and `merge_wait.rs` onto the config and item types, as copies in the new modules; the old files stay for the old binary. Their contract tests are rewritten against the new command. Nothing else of today's merge is carried over: not `gate_report`, `review_gate`, the light path or base exemptions. The Git hooks and `ATELIER_SKIP` belong to step D; this item's checks need not honour the skip yet. Today's `merge_wait.rs` and `merge_sync.rs` are not carried over.

## Context

Design: `docs/strategies/redesign-architecture.md`, Merge and Status. Behaviour: `docs/strategies/redesign-product.md`, Merge.
