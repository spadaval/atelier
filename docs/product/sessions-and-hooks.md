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

Landing also checks completed claims elsewhere in the checkout. Unchanged
integrated claims retain proof and independent acceptance that were valid when
completion was claimed. Historical acceptance preserves the original work's
author scope and requires immutable review history.

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
claim is not base completion.
