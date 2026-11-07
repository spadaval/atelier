---
created_at: "2026-11-07T05:01:01.226766008+00:00"
id: "atelier-l457"
issue_type: "feature"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-4nsl"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-md0v"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-mmpv"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Adopt and upgrade Atelier without hand repair"
updated_at: "2026-11-07T10:18:49.576212265+00:00"
---

## Goal

Life-on-rails hit about ten refusals between `atelier init` and its first merge, and cosplay-studio needed a hand-written migrator after four dormant months. Make a new repository merge with the starter policy as written, make the first steps after `init` obvious and correct, and make every old record form on this host readable or migratable.

## Done when

- [ ] `atelier init` picks the base branch in this order: origin's HEAD branch, else whichever one of `main` and `master` exists, else the current (or unborn) branch, writes it as `base_branch`, and prints its choice. The lifecycle scenario (crates/atelier-cli/tests/lifecycle_cost.rs's steps) merges with no policy edits from a `git init` on `master` and from one on `main`.
- [ ] `atelier init` in a directory that is not a Git repository refuses with a message saying to run `git init` first. With an unborn HEAD (no commits yet), `init`, `status` and `check` work, and `land` and the Stop hook report and exit successfully. Tests for each.
- [ ] `atelier init` ends by printing what it wrote and the next steps, which match how the install lands today: review the generated policy, commit it on the base (the install commit is exempt from review, as it predates the gates), then create the first issue. The lifecycle scenario's setup follows those printed steps.
- [ ] Old record forms: on this host issue and evidence records are all schema 2, workflow 3, config 1, and review records 1 and 2; review records at version 1 stay as written (approvals are append-only). The only old forms still read are separate disposition events (astromech, cosplay-studio, demiurge) and After merge lines, migrated by `migrate dispositions` and `migrate after-merge`. When `atelier check` meets either, it prints a note naming the migration and still passes.
- [ ] cosplay-studio's `atelier doctor` failure ("review needs YAML front matter", which the installed binary shows too) is diagnosed. If Atelier is wrong, it is fixed with a test that reproduces it, and `doctor` passes there; if a record is wrong, the record fix is added to atelier-ml3r's Done when.
- [ ] An untracked file does not block `merge` or `land`, in the work checkout or a separate base checkout, unless the merge would overwrite it at that path; that case is refused before anything changes. Tests for both.
- [ ] Removed, and listed in the implementer report: the fixed `base_branch: main` in the starter policy; untracked paths in `merge_checkout_dirty` (committed_snapshot.rs `require_paths_clean`, merge.rs). `land --install` stays: Sumeet chose it in atelier-bibd, and this item leaves it alone.

## Scope

`init`, the starter policy, `check`'s version notes, `merge` and `land`'s checkout-cleanliness checks, `land` on an unborn HEAD, `doctor` for cosplay-studio's records, tests, docs/product. Must not change: refusing uncommitted changes to tracked files, which could make merge or evidence lie about what was reviewed. The skill's procedures/install.md is atelier-ml3r's.

## Context

docs/strategies/simpler-lifecycle.md (Shape: Small repositories work out of the box), ADR 0028 decision 7. The draft-mission items atelier-0vp3 (base detection), atelier-6tmk (no Git, unborn HEAD) and atelier-t3ko (init next steps) cover the same ground; whether they are superseded is part of Sumeet's decision on atelier-mzpn. This item does not take 0vp3's `--base` override or t3ko's full list of created paths, re-run message, `-q` and `setup claude` step. atelier-md0v (no recorded base; report-only `land` on work branches) and atelier-4nsl (light path) land first.
