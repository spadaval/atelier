# CLI Surface

The primary interface is the `atelier` CLI and tracked Markdown. Read command
help for exact flags. During the rebuild use `target/debug/atelier`; update the
installed binary only after the new model is stable.

| Surface | Purpose |
| --- | --- |
| `init` | Initialize repository records and policy. |
| `status` | Orient in the checkout and inspect labeled branch summaries. |
| `work` | Ready, blocked, active, all, and mission views. |
| `issue` | Create, list, show, update, transition, note, link, and unlink work. |
| `start` | Prepare the branch owner and start selected work. |
| `bundle` | Preview and apply one-shot graph input. |
| `evidence` | Record, show, list, and attach commit-bound proof. |
| `review` | Append approvals, findings, dispositions, and explicit trivial exemptions. |
| `merge` | Check base policy and fast-forward reviewed work. |
| `prime` | Restore session context. |
| `land` | Check handoff invariants and committed records. |
| `setup` | Install supported harness hooks. |
| `check` | Validate canonical tracker records and workflow policy. |
| `prune` | Inspect or apply protected cleanup. |

## Orientation and ordinary work

```text
atelier prime
atelier status
atelier work ready
atelier work human
atelier work blocked
atelier work missions
atelier work mission <mission-id>
atelier work epic <epic-id>
atelier issue show <id>
atelier issue transition <id>
atelier start <id>
```

`issue list` is flat inventory, including objective records; `work` is operational
context. A mission is created with `issue create --issue-type mission`. Use an
epic parent for direct implementation children and `advances` for mission scope.
An `advances` link joins one mission to one non-mission issue and is stored on
both records, so either argument order records the same link; the examples name
the work first.
Creation presets (`--template`) and mission text generation flags
(`--constraint`, `--risk`, `--validation`) are retired. Author typed Markdown
with `--body`, and choose `--issue-type`, priority, and labels explicitly; Agent
Factory owns process record templates.

For example:

```text
atelier issue create "Implement the slice" --parent <epic-id>
atelier issue link <epic-id> <mission-id> --role advances
atelier issue link <blocked-id> <blocker-id> --role blocked_by
atelier issue note <id> "Handoff detail" --kind handoff
atelier issue transition <id> close --reason "Acceptance shown"
```

Transitions write records after read-only checks. `atelier start` prepares Git context; the `start` transition only changes the record.
Commit code and related tracker records yourself. Record focused proof and
independent review before `merge`; see [evidence](validation.md) and
[reviews and merge](reviews-and-merge.md). Done claims belong in committed work
branch records. Run `atelier land` before a session handoff and
`atelier merge <id> --check` to inspect completed-owner integration gates. Atelier never commits, pushes, squashes, or creates merge commits.

## Review and session commands

`review record`, `approve`, `finding`, and `trivial` require an owner, explicit
`--identity`, and summary. `review disposition` appends a finding decision.
Use the [review guide](reviews-and-merge.md) for commit pins and acceptance rules.

`prime` restores context; `land [id]` checks committed records and applicable
claims. `setup claude [--binary <path>]` installs the supported hooks. See
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
`work queue`, `history`, `doctor`, `man`, `rules`, telemetry, import/export, hidden branch
helpers, cache rebuild, and a TUI. Use ordinary Git for history and recovery.
There is no compatibility command layer.
