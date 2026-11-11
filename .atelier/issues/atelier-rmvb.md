---
created_at: "2026-10-17T01:10:57.973536770+00:00"
id: "atelier-rmvb"
issue_type: "bug"
labels:
- "cli-output"
- "prune"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T03:55:56.699406184+00:00"
status: "declined"
title: "Make prune's output one line per record, grouped by the root that keeps it, ending with a step that works"
updated_at: "2026-11-11T03:55:56.699406184+00:00"
---

## Goal

`atelier prune` output is hard to read, and its last step can mislead. Checked 2026-11-07 on this repository (1,683 record candidates: 486 eligible, 1,203 protected):
- Each record takes three lines (the row, a `recover:` recipe with a `<commit>` placeholder, and `title:`), and the title is an evidence or review record's whole summary, so many rows are 1,000-character paragraphs (crates/atelier-cli/src/commands/prune.rs). Protected rows leave out the title, which is what helps decide whether a record matters.
- The header says `Candidates: 1683` with no eligible/protected split, and nothing says which retained root keeps a cluster or when it becomes prunable. A hard-coded `Deferred Cleanup Classes` section always prints `none` (atelier-rp1c, merged here).
- The dry run always ends `Next: atelier prune --apply`, even when nothing is eligible or the checkout is dirty, where apply then refuses (atelier-pxyz, merged here).
- `prune --apply` prints the same list and stops, with no total and no commit step, though docs/product/retention-and-prune.md says the deletions must be committed; the owner reason reads `owner … has active workflow state` for a todo issue (atelier-08rc, merged here).

## Done when

- [ ] Every candidate is one line, e.g. `eligible issue atelier-0349  done 2026-10-16  Make atelier land fast enough…`, with titles cut to their first line and truncated; protected rows include the title; the recovery recipe prints once in a footer. A test with a multi-paragraph evidence summary asserts one line per row.
- [ ] The header reads `Canonical records: 486 to remove, 1203 kept (cutoff <date>, 7 days)`, and kept records are grouped by the root that keeps them, e.g. `atelier-1ioo  … · closed 2026-10-15, within 7 days (prunable after 2026-10-22) · keeps 46: 3 issues, 31 reviews, 12 evidence`, with per-record lines only under `--full`. The Deferred Cleanup Classes section prints only when a class is deferred. A test covers grouping by a recent root.
- [ ] The dry run ends `Nothing to prune` when nothing is eligible, `⚠ apply needs a clean checkout: commit or stash N paths first` on a dirty checkout, and otherwise `Next: atelier prune --apply`; tests cover all three.
- [ ] Apply ends with `Removed 651 (512 issues, 90 evidence, 49 reviews). Next: git commit -am "Prune …"; undo before committing: git restore .atelier`, using the real counts, and the owner reason reads `owner atelier-bw0b is open (todo)`; a test asserts the summary and footer.

## Context

Applies the one-line rule from ADR 0022 and atelier-deyy to prune. docs/product/retention-and-prune.md (protection follows links in both directions; atelier-r7cd). Merged from four prune items in decision atelier-mzpn; low priority, outside any mission.

## Report

Dropped in the cutover sort (atelier-5zre), reason (a). It changes `prune`, which the What is removed table removes.
