# CLI Surface

The primary interface is the `atelier` CLI and tracked Markdown. Read command
help for exact flags. Root help lists every command from its own description,
then the common commands in lifecycle order. Use `target/debug/atelier` to try changes; the installed
binary runs the Claude Code hooks (see [development setup](development-setup.md)).

| Surface | Purpose |
| --- | --- |
| `init` | Initialize repository records and policy. |
| `status` | Orient a session: checkout, owner, work, missions, and labeled branch summaries. |
| `work` | Ready, human, blocked, active, all, mission, parent, and mission overview views; bare `work` lists them. |
| `issue` | Create, list, show, update, transition, note, link, and unlink work. |
| `start` | Prepare the branch owner and start selected work. |
| `bundle` | Preview and apply one-shot graph input. |
| `evidence` | Record, show, list, and attach commit-bound proof. |
| `review` | Append approvals, findings, dispositions, and explicit trivial exemptions. |
| `merge` | Check base policy and fast-forward reviewed work, or merge it under `merge_commit`. |
| `land` | Check handoff invariants and committed records. |
| `doctor` | Check every done claim and the whole review history. |
| `setup` | Install supported harness hooks. |
| `check` | Validate canonical tracker records and workflow policy. |
| `prune` | Inspect or apply protected cleanup. |
| `migrate` | Apply one-shot record migrations, such as `migrate epics`. |

## Orientation and ordinary work

```text
atelier status
atelier work ready
atelier work human
atelier work blocked
atelier work missions
atelier work mission <mission-id>
atelier work parent <parent-id>
atelier issue show <id>... [--full]
atelier issue transition <id>
atelier start <id>
```

`issue list` is flat inventory, including objective records; `work` is operational
context. A mission is created with `issue create --issue-type mission`. Use a
parent issue (any work item) for direct implementation children, one level
deep, and `advances` for mission scope.
An `advances` link joins one mission to one non-mission issue. Every
`issue link` is stored only on the first record named, the one that declares
it, and the other record reads it from there, so linking new work to a shared
mission or blocker leaves that record unchanged (atelier-ihss). Either argument
order records the same `advances` link, and `--role blocks` is the blocker
declaring `blocked_by` from its side; the examples name the work first. Records
from before this change may store a link on both records or, for a blocker,
only in the blocker's `blocks`; they read the same, and `issue unlink` removes
a link wherever it is stored.
Creation presets (`--template`) and mission text generation flags
(`--constraint`, `--risk`, `--validation`) are retired. Author typed Markdown
with `--body`, and choose `--issue-type`, priority, and labels explicitly; Agent
Factory owns process record templates.

For example:

```text
atelier issue create "Implement the slice" --parent <parent-id>
atelier issue link <parent-id> <mission-id> --role advances
atelier issue link <blocked-id> <blocker-id> --role blocked_by
atelier issue note <id> "Handoff detail" --kind handoff
atelier issue transition <id> close --reason "Acceptance shown"
```

Transitions write records after read-only checks. `atelier start` prepares Git context; the `start` transition only changes the record.
Commit code and related tracker records yourself. Record focused proof and
independent review before `merge`; see [evidence](validation.md) and
[reviews and merge](reviews-and-merge.md). Done claims belong in committed work
branch records. Run `atelier land` before a session handoff and
`atelier merge <id> --check` to inspect completed-owner integration gates. Atelier never pushes or squashes, and commits only the merge commit that `merge` creates under `merge_strategy: merge_commit`.

## Review and session commands

`review record`, `approve`, `finding`, and `trivial` require an owner, explicit
`--identity`, and summary. `review disposition` appends a finding decision.
Use the [review guide](reviews-and-merge.md) for commit pins and acceptance rules.

`status` orients a session; `land [id] [--install]` checks committed records and
the claims that changed since the base; `doctor` checks every claim. `prime` is
a deprecated alias for `status`, kept for installed hooks.
`setup claude [--binary <path>]` installs the supported hooks. See
[sessions](sessions-and-hooks.md) for active-work handoff and hook behavior.

## Bulk input and maintenance

```text
atelier bundle preview <file>
atelier bundle apply <file> --yes
atelier check
atelier check <id>
atelier prune
atelier prune --apply
```

Bundle files are authored input, not durable exports. Preview is non-mutating;
apply validates the complete staged graph before installation. `check` diagnoses
records and policy; it does not repair a hidden cache. Prune is dry-run by default
and removes only eligible artifacts in apply mode. Commit canonical deletions.

Removed surfaces include provider review, review rooms, transition actions,
`work queue`, `history`, `man`, `rules`, telemetry, import/export, hidden branch
helpers, cache rebuild, and a TUI. Use ordinary Git for history and recovery.
There is no compatibility command layer.
