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
title: "Add `merge` on the new records, with `--trivial`"
updated_at: "2026-11-10T00:56:07.273072817+00:00"
---

## Goal

Finish the loop: an approved item reaches the base in one merge commit that marks it done, after three checks an agent could already see in `status`.

## Done when

- [ ] Scenario test, no forge: on an approved item, `atelier-next merge` makes one merge commit on the base in which the item's status is `done`.
- [ ] After a commit that changes a path outside `.atelier/` since the reviewed commit, `merge` changes nothing and says code changed since review 1 and to run `submit`; `status` says the same. A commit that changes only paths under `.atelier/` does not block the merge.
- [ ] A configured merge check that fails on the merged tree stops the merge and prints its command and the end of its output. The config is read from the base's tip: a test removes the check from `config.toml` on the work branch, and it still runs.
- [ ] An unanswered entry in the item's Decisions section stops the merge and is named.
- [ ] `merge --trivial "<why>"` on a building item with ticked Done when lines and a Report merges with no review and stores the reason in `trivial`. It is refused on an item that has a review file.
- [ ] With a forge configured, the existing forge and CI tests, moved to the new command, pass: it pushes the branch, reuses or opens the pull request, waits for CI on the head, merges, and pushes the base.
- [ ] The Merge section of `docs/strategies/redesign-architecture.md` matches what was built.

## Scope

The merged-tree checks and the forge and CI code are reused as they are; their behaviour does not change. `land` belongs to step D.

## Context

Design: `docs/strategies/redesign-architecture.md`, Merge and Status. Behaviour: `docs/strategies/redesign-product.md`, Merge.
