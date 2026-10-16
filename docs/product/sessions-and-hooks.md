# Sessions and Hooks

```text
atelier prime
atelier land
atelier land <owner-or-child-id>
atelier setup claude
atelier setup claude --binary <path-to-atelier>
```

`prime` reads fresh checkout, owner, mission, blockers, evidence, review, and
repository guidance. It creates no session record. `land` checks a handoff
without writing records, committing, switching, or merging; a failed plain
command exits nonzero with the failed invariant and recovery step.

Canonical records and policy must match committed HEAD, including ignored
canonical paths and files hidden by Git index flags. Active work must have a
recorded owner branch. An active owner may hand off unfinished code without
approval or full scope completion; report those changes honestly. Its done
child claims still need their applicable completion checks. A terminal owner
requires a clean checkout and the full merge gates. An explicit child ID
resolves its owner; otherwise ownership is inferred from the current branch.
When the base has commits the branch does not, `land` checks the work against
the point where it left the base and adds a note to merge the base in (`git
merge <base>`) and record fresh proof before `atelier merge`, which refuses
with `merge_diverged` until then (atelier-qz9x).

A branch that no item owns, such as a feature branch, a PR branch, a branch
from before adoption, or a detached HEAD, lands with a note when it changes no
tracker files (anything under `.atelier/`) since its merge base with the base
branch. A path counts only if it changed relative to every local and
remote-tracking copy of the base, so a stale local base does not blame the
branch for upstream records. If the branch does change tracker files, or
shares no history with the base, `land` refuses with `land_work_untraceable`,
names the changed paths (the first five, then a count), and points to
`atelier start`. While no copy of the base has a workflow policy, the branch is
installing Atelier, and `land` passes with a note on how to land it (see
[installing Atelier](workflow-configuration.md#installing-atelier)).

On the base branch itself, `land` checks unpushed base commits: only records
and mission strategies may be committed there without review (see
[planning changes on the base](reviews-and-merge.md#planning-changes-on-the-base)).

Landing also checks completed claims elsewhere in the checkout. Unchanged
integrated claims retain proof and independent acceptance that were valid when
completion was claimed. Historical acceptance preserves the original work's
author scope and requires immutable review history. Without an owner, a claim
counts as unchanged when any parent of HEAD already has it, so a merge tip, such
as a branch that merged the base in last, keeps the claims it brings in through
its other parents (atelier-kqt9).

## Claude Code hooks

Claude is the supported harness. Setup writes `.claude/settings.json` with
managed SessionStart and Stop command handlers, preserving unrelated settings
and handlers. Repeating setup is idempotent; invalid JSON fails before writing.
The default executable is `atelier` on PATH. An explicit `--binary` is resolved
to an absolute path and quoted; use it to try a local rebuilt binary while the
installed binary still has the old model. Commit shared settings deliberately
and restart Claude Code to load them. Existing disabled-hook settings are
preserved and reported.

Installed commands are `atelier prime --hook claude` and
`atelier land --hook claude`, with the selected executable. The adapters read
JSON stdin and use its `cwd` to find the checkout. SessionStart returns context
through `hookSpecificOutput.additionalContext`. A failed Stop returns
`decision: "block"` and a reason at exit 0. When `stop_hook_active` signals a
repeated failure, it emits a warning and allows stopping to avoid a loop; plain
`land` continues to fail until the invariant is fixed. These protocol fields
follow the [official Claude Code hooks reference](https://code.claude.com/docs/en/hooks).

Hooks assist handoff; their warning escape does not prove completion. Actual
session acceptance needs a dogfooded session as well as adapter tests.

## Other branches

`status` shows labeled summaries of committed local work-branch scopes, while
the current checkout uses live records. It does not fetch remote branches or
overlay another branch's records onto the current graph. Incompatible or
unavailable snapshots are reported separately. Every compatible owner branch
remains visible, including branches sharing a tracker tree. A branch's done
claim is not base completion: a done owner shows as `done on branch <branch>,
not merged into <base>` until the base branch's own copy of the record is
done.
