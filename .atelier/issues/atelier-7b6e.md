---
created_at: "2026-10-16T21:55:56.381402258+00:00"
id: "atelier-7b6e"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-7b6e
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-as7i"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
  - kind: "review"
    id: "atelier-83w3"
    type: "discovered_from"
  - kind: "review"
    id: "atelier-pew5"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-22T23:08:53.320764670+00:00"
status: "done"
title: "Batch the per-review history walks in land with exact answers"
updated_at: "2026-10-22T23:08:53.320764670+00:00"
---

## Goal

After atelier-v5tt, land on this repository spends about 2.2 s (of 5 s) in roughly 260 path-limited git log walks, two or three per retained review event: introduced_query and the two scope_queries in review_gate.rs. Answer them from fewer walks.

## Done when

The walk count does not grow with review records, answers are identical to the per-path walks (parity tests over merges, criss-cross, deleted and re-added paths, cherry-picked review files, grafts and shallow clones), and land is measured before and after on this repository and the benchmark history fixture. The time between Git calls (in-process record reads and tree diffs, about 1.3 s of proof checks in the atelier-v5tt profile) is attributed and reduced or recorded as accepted. `ancestry_path` in git_freshness.rs walks forward from the pin over a parent-to-children map instead of repeating passes, which are quadratic when a long side branch is dated before its parents (review finding atelier-pew5), with a date-skewed parity case.

## Context

Split from atelier-v5tt on 2026-10-16.

## Unknowns

Path-limited history simplification and date ordering depend on the start commit and pathspec. A candidate: one --full-history walk per directory, used only when a path has exactly one adding commit (then the per-path walk must find it), falling back to the per-path walk otherwise. Ordering-dependent queries (first add among scope paths up to an event, first owner update with branch metadata) need a separate argument.

## Notes

- 2026-10-16 [note] 2026-10-16: atelier-37uq made land check only claims a change since a trusted point can affect, so land on this repository dropped from about 5.8 s to 0.33 s. These walks now cost mainly atelier doctor (about 4.5 s), which runs on demand, so this is no longer urgent.
- 2026-10-22 [note] From atelier-as7i: two history queries share the merge blind spot's neighbours. (1) introduced_query in crates/atelier-app/src/review_gate.rs uses --diff-filter=A, so a review record added in a commit that deletes a similar file is shown as a rename (R), not an add, and gets no introducing commit; adding --no-renames would fix it. (2) append_only_history's first-review query (same file, 'log --root --reverse --diff-filter=A ... -- .atelier/reviews/') has no --diff-merges=first-parent, so a repo whose first review record appears in a conflict-resolving merge would find no first commit, or a later one; it needs the same --diff-merges=first-parent --no-patch (and --no-renames) as introduced_query. Cover both in the merge parity tests.
- 2026-10-22 [note] From review of atelier-as7i (finding atelier-83w3): the as7i decision note and the comment at review_gate.rs:1084 wrongly say a clean merge stays simplified away under --diff-merges=first-parent; git lists both the side commit and the merge. Callers rely on --reverse putting the true introducer first; correct the note and document that contract when rewriting the queries.
- 2026-10-22 [decision] Answer every path-limited record history query (review introducer, scope start, owner updates, claim endpoint, first review event) by replaying Git's revision walk in process over a graph loaded with one rev-list and one diff-tree --stdin of every commit against each parent, limited to .atelier/issues/ and .atelier/reviews/ (path_log::RecordGraph); no per-path fallback
  Reason: It reproduces git log exactly for every query shape, including the ordering-dependent ones (first add up to an event, first owner update with branch metadata, newest-first claim walk), because it replays the same date-ordered queue with insertion-order ties, the same try_to_simplify_commit rules and the same first-parent diff filter. Parity tests compare all four shapes from every commit over merges, octopus, criss-cross, deleted and re-added paths, a path turned directory, cherry-picks, grafts, shallow clones and backward or equal dates, and a one-off run matched 964 sampled queries on this repository. The old git log arguments survive only as PathLog::git_args under cfg(test), the oracle.
  Rejected: The Unknowns candidate: one --full-history walk per directory used when a path has exactly one adding commit, else the per-path walk; it keeps the per-path walks as a production path and has no answer for the ordering-dependent queries
  Rejected: One git log -m --raw walk: its output does not say which parent an empty or non-empty merge diff belongs to
  Rejected: Writing changed-path Bloom filters (atelier-v5tt): a product decision, and the walks would still grow with review records
- 2026-10-22 [decision] The addition queries (introducer, scope start, first review event) pass --no-renames, and append_only_history's first-review query gains --diff-merges=first-parent --no-patch
  Reason: From atelier-as7i. Rename detection pairs only paths inside the pathspec, so a single-file query such as introduced_query never saw an R and its answers are unchanged; the multi-file scope query and the .atelier/reviews/ directory query could, and now count such a record as added. Without merge diffs the first-review query could not see a review event that a conflict-resolving merge brought into being; the test additions_list_records_resolved_or_renamed_into_being shows the old query returning nothing there.
- 2026-10-22 [decision] Load the graph lazily, once per read phase, on the first record history query, and drop the parallel prefetch of git log walks
  Reason: On this repository the load costs about 0.3 s (diff-tree) and replaces 126 log walks; a land needing one or two queries pays it instead of a 50 ms walk, but the guard fixture's land median is unchanged (148 ms before, 145 ms after).
  Rejected: Indexing only the paths a land will ask about: the set depends on scope lookups made during evaluation, so the index would need extending or a fallback
- 2026-10-22 [decision] Accept the remaining time between Git calls in land for now and propose answering ancestry from the loaded graph as a follow-up
  Reason: After this change land here takes about 1.4 s: about 0.15 s before the first history query, 0.3 s loading the graph, and about 0.8 s across about 310 serial ancestry and edge calls from the approval checks (176 merge-base --is-ancestor, 74 rev-list --parents ranges, 45 rev-list --not, 11 first-parent lookups), about 0.35 s of it inside Git and the rest process start-up and in-process tree diffs and record reads between them. Those calls are in approval logic that atelier-r28p and atelier-s2ei are changing, outside this item's history-query layer.

## Close Reason

Land's review and claim history queries are now answered in process from one `rev-list` and one `diff-tree` per read phase, whatever the number of review records. Answers match the per-path `git log` walks in parity tests over merges, criss-cross, deleted and re-added paths, cherry-picked review files, grafts, shallow clones and skewed dates. `ancestry_path` now walks forward over a children map and has a date-skewed parity case. Land on this repository went from 3.9 s to 1.4 s (median). On the benchmark history fixture it went from 3.5 s to 1.0 s. The remaining time between Git calls is attributed and accepted (evidence atelier-u6xx; decision notes).

### Deviations
- Besides introduced_query and scope_queries, the claim endpoint walk (`claim_endpoint` in evidence_freshness.rs) also moved to the new graph, because the removed prefetch ran it. `claimed_before_required` in review_gate.rs still runs its own `git log`, since only a missing required field reaches it.
- The prefetch call in `evaluate_scope` and `ReadContext::prefetch_histories` were deleted. Nothing else used them.
- `git_input` moved out of the git_freshness tests so production code can feed Git's stdin. It now writes through the SIGPIPE-safe `Requests` socket.
- The time between Git calls is accepted rather than reduced. The rest of it comes from about 310 serial `merge-base`/`rev-list` calls in the approval checks, which are outside this item's layer.
- Benchmark evidence atelier-u6xx says this repository was measured "at c5833bba". The runs were actually at 98f4ffb5. c5833bba adds only two issue notes.

### Decisions
See the four decision notes on atelier-7b6e:
- in-process replay of Git's walk, with no per-path fallback
- `--no-renames` and merge diffs for the addition queries
- lazy graph load and removal of the prefetch
- accepting the remaining gaps

The atelier-as7i note now carries the atelier-83w3 correction.

### Surprises
- Every `--diff-merges` option turns off history simplification, as `-m` does. That makes the addition queries full-history walks, and it is why the clean merge is listed (atelier-83w3).
- `git log -m --raw` output does not say which parent each diff belongs to. The graph is therefore read with `diff-tree --stdin`, one input line per commit and parent pair.
- The atelier-as7i note expected rename detection to hide additions in introduced_query. It cannot there: Git pairs renames only inside the pathspec, so single-file queries never show R. Only the multi-file scope query and the directory query could.

### Open items
- Proposed follow-up, not filed: answer the approval checks' ancestry questions (`merge-base --is-ancestor`, `rev-list` ranges, `--not` sets) from the loaded graph. That would remove most of the remaining 0.8 s.
- Proposed follow-up, not filed: the introducer is the first commit after `--reverse`. If commit dates are skewed so that a clean merge is dated before the branch commit that added a review record, the merge is taken as the introducer. This behaviour existed before and the replay keeps it exactly.
