---
created_at: "2026-11-10T00:56:08.329209632+00:00"
id: "atelier-pz1y"
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
    id: "atelier-fcb7"
    type: "advances"
  - kind: "issue"
    id: "atelier-jciw"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-jq24"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-yptz"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Add `check` and the three Git hook rules"
updated_at: "2026-11-10T21:30:50.051382946+00:00"
---

## Goal

Move Atelier's rules about Git actions to where Git acts. `check` validates every record, and with `--hook` it is what each of three Git hooks runs: a commit of a broken record, a status change no command could have made, and a push that puts unreviewed code on the base are each stopped by the Git command that would have done it.

## Done when

- [ ] `atelier check` validates every item, review, mission and `config.toml` in the checkout, prints each problem with its file and field, and exits non-zero when it found one. It also lists record changes not yet committed, which is information and does not change its exit status.
- [ ] `check --hook pre-commit` validates every staged `item.md`, `mission.md`, review file and `config.toml`, as staged and not as in the working tree. Other files under `.atelier/` are not examined: a test stages a mission document with no front matter and an image in an item's `artifacts/`, and the commit goes through. Where a staged record's status differs from the same record's in `HEAD`, the act table must list a row from the old status to the new one; the conditions of that row are not checked here. A test commits each kind of refused change (an invalid record, and a status change with no row) and one allowed change made by hand.
- [ ] `check --hook pre-push` reads what Git gives a pre-push hook. When the push updates the base branch, each new commit on the base's first-parent line must be a merge made by `atelier merge`, known by its `Atelier-Merge:` trailer, or touch only paths under `.atelier/missions/` and `.atelier/items/`. Commits a merge brought in on its second parent are not examined. A commit that is not such a merge may not set an item `done` while that item's branch exists. A push of any other branch is not examined.
- [ ] `check --hook post-checkout` prints what `status` prints when a branch was switched, and never fails the checkout.
- [ ] Scenario tests in a scratch repository, with hook scripts the test writes: a commit of an invalid record refused; a commit of a status change the act table does not allow refused; a push of the base holding a plain commit that touches code refused, and one that changes `.atelier/config.toml` directly refused; a push holding only planning commits allowed; a push whose new commit is an Atelier merge of a branch with ordinary code commits allowed; a push of an item's branch allowed whatever it holds; a branch switch printing `status`.
- [ ] Each refusal appears in the output of the Git command it stopped, says which rule and which file or commit, and names the way out. A record that does not parse is refused with a line saying it is not a rule and cannot be skipped.
- [ ] `docs/strategies/redesign-architecture.md` matches what was built.

## Scope

`check` and its three hook modes, in `next/`. Installing the hooks is atelier-ym3z, so these tests write the one-line hook scripts themselves. `ATELIER_SKIP` is atelier-zrss: until it lands the hooks have no bypass but Git's own `--no-verify`. There is no `land` command. Nothing is copied from the old workspace. No file under `crates/` is changed.

## Context

Design: `docs/strategies/redesign-architecture.md`, "Hooks and the bypass", "Where records are read and written" (the pre-push rule) and Testing. Behaviour: `docs/strategies/redesign-product.md`, "Hooks and the bypass" and Principles 6 and 9. The specification names the parser's inputs exactly so that a mission's documents and an item's artifacts are never parsed (planning finding atelier-1l4r). `atelier merge` already writes the `Atelier-Merge: <id>` trailer as the last line of its merge commit's message.

This is the first of step D's three items in `docs/strategies/redesign-2-core.md`.
