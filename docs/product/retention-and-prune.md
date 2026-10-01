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

Canonical issue and evidence cleanup protects non-terminal work, retained graph
references, required proof, recent terminal records, uncommitted changes, and
paths with unmerged history. Apply requires a clean tracked checkout and valid
records. It must not leave missing links or an invalid retained graph.

Branch and worktree cleanup protects the current checkout, base branch, active
owners, dirty or locked worktrees, and unintegrated or unpushed commits. Output
names eligible, protected, removed, failed, and deferred classes. A deferred
class is never silently deleted. Reviews are append-only proof; do not delete
review records by hand to bypass gates. External payload deletion is outside
prune's scope.

Commit canonical record removals yourself. After a partial cleanup, inspect Git
and rerun the dry run; do not assume every candidate was removed. Recover a
record from Git history:

```text
git log --all -- .atelier/issues/<id>.md
git show <commit>:.atelier/issues/<id>.md
```

There is no arbitrary-record deletion or cache repair command. Repair malformed
canonical records through reviewed edits and rerun `atelier check`.
