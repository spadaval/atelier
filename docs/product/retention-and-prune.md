# Retention and Prune

The active `.atelier/` tree is the working set, not an eternal archive. Terminal
records may leave it after retention and graph protection checks. Git history
preserves the audit trail; there is no second tracked archive format.

```text
atelier prune
atelier prune --apply
atelier prune --retention-days <days>
```

Prune is dry-run by default. Inspect candidates and protection reasons before
apply. The default canonical retention window is seven days; tracked config can
set `[prune] canonical_retention_days`. A command override affects that run.

Canonical issue and evidence cleanup protects open work, retained graph
references, required proof, recently closed records, uncommitted changes, and
paths with unmerged history. What pruning keeps follows atelier-r7cd (decided
by Sumeet 2026-10-15):

- Any record linked from, or linking to, a record that stays, which includes
  every record an open record links.
- Records with decision or research content: `decision` records, and any
  record with a Decision or Background section. These are the records most
  likely to be needed again, and Git history alone would bury them.

Protection follows links in both directions and repeats, so one kept decision
or Background record keeps every closed record connected to it, such as its
parent, children, related records, and their evidence and reviews. That is
what keeps prune from leaving dangling links.

A record on another branch can still link a record that prune removed. When
it reaches the checkout, `atelier check` (and every command, since the graph
no longer loads) reports the missing reference, names the commit that deleted
it, and shows the `git restore --source=<commit>^` command that brings it back
(it works from any directory). If the record is still committed and only
missing from the checkout, it suggests `git restore` instead. Alternatively,
delete the link from the linking issue by hand, or remove a review or
evidence record whose only target is gone. Apply requires a clean tracked checkout and valid
records. It must not leave missing links or an invalid retained graph.

Branch and worktree cleanup protects the current checkout, base branch, active
owners, dirty or locked worktrees, and unintegrated or unpushed commits. Output
names eligible, protected, removed, failed, and deferred classes. A deferred
class is never silently deleted. Reviews are append-only proof; do not delete
review records by hand to bypass gates. External payload deletion is outside
prune's scope.

Protected prune removes a terminal target and its eligible review cluster
together. Git preserves their audit trail. Retained review events stay immutable;
rewrites, partial review deletions, and edits later reverted fail landing.

Commit canonical record removals yourself. After a partial cleanup, inspect Git
and rerun the dry run; do not assume every candidate was removed. Recover a
record from Git history:

```text
git log --all -- .atelier/issues/<id>.md
git show <commit>:.atelier/issues/<id>.md
```

There is no arbitrary-record deletion or cache repair command. Repair malformed
canonical records through reviewed edits and rerun `atelier check`.
