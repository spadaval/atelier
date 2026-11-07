---
created_at: "2026-11-07T05:01:01.226766008+00:00"
id: "atelier-l457"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: master
  work_branch: atelier-l457
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-md0v"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-mmpv"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Adopt and upgrade Atelier without hand repair"
updated_at: "2026-11-07T10:31:03.146735599+00:00"
---

## Goal

Life-on-rails hit about ten refusals between `atelier init` and its first merge, and cosplay-studio needed a hand-written migrator after four dormant months. Make a new repository merge with the starter policy as written, make the first steps after `init` obvious and correct, and make every old record form on this host readable or migratable.

## Done when

- [x] `atelier init` picks the base branch in this order: origin's HEAD branch, else whichever one of `main` and `master` exists, else the current (or unborn) branch, writes it as `base_branch`, and prints its choice. The lifecycle scenario (crates/atelier-cli/tests/lifecycle_cost.rs's steps) merges with no policy edits from a `git init` on `master` and from one on `main`.
- [x] `atelier init` in a directory that is not a Git repository refuses with a message saying to run `git init` first. With an unborn HEAD (no commits yet), `init`, `status` and `check` work, and `land` and the Stop hook report and exit successfully. Tests for each.
- [x] `atelier init` ends by printing what it wrote and the next steps, which match how the install lands today: review the generated policy, commit it on the base (the install commit is exempt from review, as it predates the gates), then create the first issue. The lifecycle scenario's setup follows those printed steps.
- [x] Old record forms: on this host issue and evidence records are all schema 2, workflow 3, config 1, and review records 1 and 2; review records at version 1 stay as written (approvals are append-only). The only old forms still read are separate disposition events (astromech, cosplay-studio, demiurge) and After merge lines, migrated by `migrate dispositions` and `migrate after-merge`. When `atelier check` meets either, it prints a note naming the migration and still passes.
- [x] cosplay-studio's `atelier doctor` failure ("review needs YAML front matter", which the installed binary shows too) is diagnosed. If Atelier is wrong, it is fixed with a test that reproduces it, and `doctor` passes there; if a record is wrong, the record fix is added to atelier-ml3r's Done when.
- [x] An untracked file does not block `merge` or `land`, in the work checkout or a separate base checkout, unless the merge would overwrite it at that path; that case is refused before anything changes. Tests for both.
- [x] Removed, and listed in the implementer report: the fixed `base_branch: main` in the starter policy; untracked paths in `merge_checkout_dirty` (committed_snapshot.rs `require_paths_clean`, merge.rs). `land --install` stays: Sumeet chose it in atelier-bibd, and this item leaves it alone.

## Scope

`init`, the starter policy, `check`'s version notes, `merge` and `land`'s checkout-cleanliness checks, `land` on an unborn HEAD, `doctor` for cosplay-studio's records, tests, docs/product. Must not change: refusing uncommitted changes to tracked files, which could make merge or evidence lie about what was reviewed. The skill's procedures/install.md is atelier-ml3r's.

## Context

docs/strategies/simpler-lifecycle.md (Shape: Small repositories work out of the box), ADR 0028 decision 7. The draft-mission items atelier-0vp3 (base detection), atelier-6tmk (no Git, unborn HEAD) and atelier-t3ko (init next steps) cover the same ground; whether they are superseded is part of Sumeet's decision on atelier-mzpn. This item does not take 0vp3's `--base` override or t3ko's full list of created paths, re-run message, `-q` and `setup claude` step. atelier-md0v (no recorded base; report-only `land` on work branches) has landed. atelier-4nsl (light path) runs in parallel and also changes `merge.rs`; keep edits there focused.

## Report

A new repository now merges with the starter policy as written. `atelier init` refuses outside Git (`init_no_git`, naming `git init`), picks the base branch (origin's HEAD branch, else the one of `main` and `master` that exists, else the current or unborn branch), writes it as `base_branch`, prints `Base branch: <branch> (from <rule>)`, and ends with the install's steps: review the generated policy, commit it on the base with `git add .atelier .gitignore && git commit -m "Install Atelier"` (it predates the gates, so it needs no review), then create the first issue. The lifecycle scenario runs from `git init` on `main` and on `master`, runs init's printed commit command, and merges with no policy edit. With no commits yet, `status` shows `<branch> (no commits yet)` and puts the install commit first in its footer, `check` passes, and `land`, `doctor` and the Stop hook say there is nothing to land, name the commit and exit 0. `check` passes with a note naming `atelier migrate dispositions --apply` for separate disposition events and `atelier migrate after-merge` for open After merge lines. cosplay-studio's `doctor` failure was Atelier's: the append-only check of the whole review history parsed the pre-Markdown `.atelier/reviews/<id>.yaml` files that its record migration (288693f, 2026-11-05) deleted; it now judges only `.md` review records, and `doctor` passes there, so atelier-ml3r needs no record fix. Untracked files no longer hold up `merge` or `land` in the work checkout or a separate base checkout; one that integrating would overwrite is refused as `merge_untracked_overwritten` before anything changes.

Removed: the fixed `base_branch: main` in the starter policy (`STARTER_POLICY_YAML` became `starter_policy_yaml(base)`); untracked paths in `merge_checkout_dirty` (committed_snapshot.rs `require_paths_clean`, through `read_record_files` and `require_code_clean`, and merge.rs `clean`, which now reads tracked changes only); untracked paths in `land_completion_dirty` and land's `Unfinished checkout` line; untracked paths outside `.atelier/` in `start_checkout_dirty`. `land --install` is unchanged.

Survey (Done when 4), read with this binary on 2026-11-07: issues and evidence schema 2, workflow 3 and config 1 in all five repositories; reviews 1 and 2 (atelier 277/651, astromech 2623/22, cosplay-studio 300/0, demiurge 407/0, life-on-rails 2/2). Separate disposition events: astromech 1087, cosplay-studio 146, demiurge 167; open After merge lines: astromech 5. `check` and `doctor` pass in astromech, cosplay-studio, demiurge and life-on-rails.

### Deviations

- Scope widened by the orchestrator after review finding atelier-epcv: `atelier start` also leaves untracked files outside `.atelier/` in place, since the files merge now leaves would otherwise make the next `start` refuse. It still refuses uncommitted changes to tracked files and untracked records (`start_checkout_dirty`, now advising the stash that sets them aside), and refuses an untracked file the existing branch it switches to holds (`start_untracked_overwritten`) before anything changes. The overwrite check moved from merge.rs to commands/git.rs, shared by both.

### Decisions

- A detached HEAD that no rule settles is refused (`init_base_unknown`) before anything is written, rather than defaulting to `main`, since refusing before a write beats a policy that refuses later. "Whichever one of `main` and `master` exists" means exactly one local branch; with both, the current branch decides.
- The overwrite check is its own code, `merge_untracked_overwritten`, reported as a failed precondition with the gates, so `merge --check` shows it. It checks every tree integration moves the target checkout through (the work checkout: the base, then the merge commit; a base checkout: the merge commit), including a file where a directory goes and the reverse. Ignored files stay Git's business, as before.
- An untracked file under `.atelier/issues`, `reviews` or `evidence` still refuses: it is a record the gates read, not an untracked file.
- A passing owner `land` names untracked paths in an `Untracked:` line, so a forgotten `git add` is still seen, and still offers `atelier merge`.
- The advice for `merge_checkout_dirty` and `land_completion_dirty` changed from `git add -A && git commit` and `git stash push --include-untracked` to `git commit -am` and `git stash push`, since they now list tracked changes only and `git add -A` would commit the untracked files merge no longer minds.
- The open After merge lines are defined once, `after_merge::open_lines` in atelier-app, shared by `migrate after-merge` and `check`, so the note goes exactly when the migration has nothing left. Notes print on a passing, unquiet check; a scoped check counts the item's own.
- A re-run of `init` on an existing policy keeps the old footer; the re-run message is atelier-t3ko's.
- The test harnesses (cli_integration, smoke) now run `git init -b main` before `atelier init`.

### Surprises

- The cosplay-studio failure was not a record problem but a parse of files Atelier no longer reads, so the fix is in Atelier.
- About 80 tests ran `atelier init` outside a Git repository.
- astromech's policy has no `loose_end` type yet, so the note's `atelier migrate after-merge` refuses there with `migrate_no_loose_end` until atelier-ml3r updates that policy.

### Open items

- `merge --sync` no longer refuses untracked files up front; when merging the base in would overwrite one, Git's own refusal stops it before anything changes, reported as `sync_merge_failed` with Git's message.
