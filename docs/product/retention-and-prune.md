# Retention And Prune Policy

Atelier keeps the active `.atelier/` tree useful for current coordination. It
does not keep every historical record live forever. Git history is the long-term
audit log; current canonical records are the working set an operator needs for
active missions, open work, recent terminal work, and proof that still blocks a
current decision.

`atelier prune` is the explicit maintenance surface for shrinking accumulated
state. It is dry-run by default. Apply mode may remove only classes whose
eligibility, protection rules, and audit output are implemented and reported in
the dry run.

## Product Goals

- Make cleanup inspectable before it is destructive.
- Protect active work, current proof obligations, dirty worktrees, and unmerged
  Git state by default.
- Reduce active record count without creating compatibility aliases, hidden
  fallback commands, or a second tracker archive format operators must manage by
  hand.
- Rely on the prune diff, commit message, attached evidence, and Git history
  for canonical recovery instead of keeping a second live archive index.

## Command Contract

`atelier prune` reports candidates and protected records. It must not delete
anything.

`atelier prune --apply` removes eligible candidates from implemented cleanup
classes. Apply output must name each affected class, removed count, skipped
count, recovery command shape, and any failure. If a class is not implemented
yet, output must keep reporting it as deferred rather than silently ignoring it.

Canonical record pruning requires a clean tracked checkout and a healthy tracker
state. If canonical Markdown is invalid, the projection is stale in a way the
command cannot refresh, or tracked files are dirty, apply mode must stop before
removing canonical records. Local diagnostics cleanup may still run when it can
do so without reading or mutating tracked records.

Quiet output may collapse paths and record titles to counts and stable tokens,
but normal output must give enough IDs or paths to inspect the candidate set.

## Retention Classes

| Class | Default policy | Apply behavior |
| --- | --- | --- |
| Local diagnostics logs | Retain command diagnostics for 30 UTC days unless the operator supplies a command override. | Delete expired ignored diagnostics log files. |
| Ignored runtime, cache, and projection artifacts | Keep rebuildable local state while it is current or locked by a running command. Orphaned temp files, stale cache entries, and corrupt rebuildable projections are disposable. | Delete only ignored local files that are not locked and are safe to rebuild; print `doctor --fix` guidance when cleanup affects projection state. |
| Canonical issue, mission, epic, evidence, review, and activity records | Keep active, blocked, review-bound, recently terminal, or proof-relevant records in the active tree. Terminal records become candidates after their retention window and after all current references are closed. | Remove eligible terminal records from the active tree. Recovery uses Git history for the removed path or ID. |
| Evidence payload references | Preserve metadata while any retained record depends on it. External payload deletion is out of scope for v1. | Remove only evidence metadata records pruned with their sole terminal dependents; do not delete external payloads. |
| Native review rooms | Keep while the branch owner is open, under review, or has unmerged branch state. | Prune only with the terminal branch owner after review completion and branch integration are proven. |
| Git branches | Keep base, current, protected, unmerged, unpushed, or owner-active branches. | Delete only non-current owner branches whose owner records are terminal and whose commits are already integrated according to branch policy. |
| Git worktrees | Keep current, dirty, locked, owner-active, or unmerged worktrees. | Remove only clean non-current worktrees whose owner records are terminal and whose branch state is integrated or separately preserved. |

The canonical record retention window defaults to 7 days from the later of the
record's terminal transition time and its latest activity sidecar. Projects may
override it in tracked `.atelier/config.toml`:

```toml
[prune]
canonical_retention_days = 7
```

The `atelier prune --retention-days <days>` flag is an explicit command override
for the current run. Diagnostics keep their separate 30-day default when the
flag is omitted.

## Protection Rules

A record is protected when any of these are true:

- it is not in a terminal workflow status;
- it is a mission or epic with open child work, open blockers, pending review,
  pending validation, or incomplete required proof;
- it is linked from a non-terminal record through a blocker, parent, child,
  validation, review, or evidence relationship;
- it is evidence attached to any retained record;
- it has activity newer than the retention cutoff;
- it was changed in the current Git branch and has not been committed;
- pruning it would leave an unsupported relationship, missing evidence link, or
  lint failure in retained canonical state.

Git branches and worktrees are additionally protected when they are the current
checkout, match the configured base branch, contain commits not integrated into
the configured base/review target, have a dirty worktree, or are associated with
active workflow state.

## Git-History Recovery

Canonical pruning does not write a tracked prune manifest. The active tree
shrinks, and the Git commit that carries the deletion is the audit boundary.
Operators recover detail from Git history:

```text
git log --all -- .atelier/issues/<id>.md
git show <commit>:.atelier/issues/<id>.md
```

`atelier prune` output must print the removed path and this recovery shape for
canonical records. A future convenience command may wrap the Git lookup, but v1
must not keep a live `.atelier/prune/` index just to make lookup faster.

## Hard Deletion

`atelier prune` is allowed to remove eligible records from the active tree
because Git history preserves the audit trail. It must not delete protected
records by force.

Exceptional destructive surgery stays under explicit maintenance commands, not
routine pruning. Operators who need to delete a protected or malformed record
must use the destructive maintenance surface with force/confirmation semantics
and then run `atelier lint` plus the recovery commands it names.
