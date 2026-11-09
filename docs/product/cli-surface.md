# CLI Surface

The primary interface is the `atelier` CLI and tracked Markdown. Read command
help for exact flags. Root help lists every command from its own description,
then the common commands in lifecycle order and the order of steps for one
issue. Use `target/debug/atelier` to try changes; the installed
binary runs the Claude Code hooks (see [development setup](development-setup.md)).

| Surface | Purpose |
| --- | --- |
| `init` | Initialize repository records and policy. |
| `status` | Orient a session: checkout, owner, work, missions, and labeled branch summaries. |
| `work` | Ready, human, blocked, active, all, mission, decisions, parent, and mission overview views; bare `work` lists them. |
| `issue` | Create, list, show, update, transition, note, link, and unlink work. |
| `start` | Prepare the branch owner and start selected work. |
| `bundle` | Preview and apply one-shot graph input. |
| `evidence` | Record, show, list, and attach optional commit-bound receipts; no gate reads them. |
| `review` | Print the reviewer's brief; record approvals, findings and explicit trivial exemptions; set findings' dispositions. |
| `merge` | Evaluate the merge gates and integrate reviewed work with a merge commit that marks it done; `--check` evaluates them without merging; with a forge in the policy, `--push` pushes the branch, ensures a pull request, merges and publishes the base, and `--wait` waits for CI. |
| `land` | Run at every handoff: judge committed records, what changed since the base and the commits about to be pushed; report work in progress on a branch, refuse on the base. |
| `doctor` | Judge every done claim and the whole review history, and warn when atelier or the Agent Factory skill is out of date; see [update check](sessions-and-hooks.md#update-check). |
| `brief` | Review a mission's plan as Markdown or a static HTML page; see [mission reviews](mission-reviews.md). |
| `debrief` | Review a mission's outcome as Markdown or a static HTML page; see [mission reviews](mission-reviews.md). |
| `setup` | Install supported harness hooks, or link the installed Agent Factory skill to a checkout (`--link`, `--unlink`). |
| `check` | Validate canonical tracker records and workflow policy. |
| `prune` | Inspect or apply protected cleanup. |
| `migrate` | Apply one-shot record migrations: `migrate epics`, `migrate dispositions`, `migrate after-merge`. |

## Orientation and ordinary work

```text
atelier status
atelier work ready
atelier work human
atelier work blocked
atelier work missions
atelier work mission <mission-id>
atelier work decisions <mission-id>
atelier work parent <parent-id> [--all]
atelier issue show <id>... [--full]
atelier issue transition <id>
atelier start <id> [--take-over]
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
atelier issue create "Noticed while working" --discovered-from <issue-review-or-evidence-id>
atelier issue link <parent-id> <mission-id> --role advances
atelier issue link <blocked-id> <blocker-id> --role blocked_by
atelier issue note <id> "Handoff detail" --kind handoff
atelier issue note <id> "Chosen approach" --kind decision --reason "Why" [--rejected "Alternative"]
atelier issue transition <id> supersede --reason "Why it is obsolete"
atelier issue transition <id> decline --reason "Why it will not be done"
```

Transitions write records after read-only validators pass. `atelier start` prepares Git context; the `start` transition only changes the record.
Commit code and related tracker records yourself. The order is: start,
implement, tick every Done when line and write the Report, `atelier review
request <id>`, fix and re-review, approve, `atelier merge <id>`, which marks
the work done; there is no close step for work ([work
model](work-model.md#completion)). A change no issue owns, such as a release,
docs or a mockup, skips the issue: commit it on a branch, then `atelier
review request`, fix and re-review, approve with `--branch <name>`, and
`atelier merge`, all with no ID ([the light
path](reviews-and-merge.md#a-change-with-no-issue-the-light-path)). Run focused proof and get independent
review before `merge`; see [evidence](validation.md) and [reviews and
merge](reviews-and-merge.md). Run `atelier land` before a session handoff and
`atelier merge <id> --check` to inspect integration gates. Atelier never squashes, and pushes only when asked: `land --push` pushes the current branch, and `merge --push` pushes the branch and the base to the [forge](workflow-configuration.md#forge) the policy names ([CI on a forge](reviews-and-merge.md#ci-on-a-forge)). It commits only the merge commit that `merge` creates, and the records a command wrote when asked with `--commit-records` (see [Committing records](#committing-records)).

## Review and session commands

`review record`, `approve`, `finding`, and `trivial` require an owner, explicit
`--identity`, and summary (`record`, `approve` and `finding` take `--branch
<name>` in place of the owner on the light path); a commit they take (`--commit`, `--base`) may be a
unique short ID, stored in full. `review disposition` sets a finding's
disposition, replacing any earlier one: a finding recommended defer,
no_action or wont_fix needs one, and a `fix_now` finding is resolved by the
approval after its fix ([dispositions](reviews-and-merge.md#dispositions)).
`review defer <id>` defers every undecided finding recommended defer on an
item, filing a backlog follow-up for each under one parent.
`review request <id>` prints the reviewer brief for an owner, and with no ID
for the checked-out branch's owner or, on a branch no issue owns, the branch
itself (see [requesting a review](reviews-and-merge.md#requesting-a-review)).
Use the [review guide](reviews-and-merge.md) for commit pins and acceptance rules.

`status` orients a session; `land [id] [--install]` judges committed records and
the claims that changed since the base, and on a work branch reports what is
open and exits 0 (`land: work in progress — N checks open (…)`, quiet `work in
progress — …`; the verdict counts open gates as checks); `doctor` judges
every claim. `prime` is a deprecated alias for `status`, kept for installed hooks.
`setup claude [--binary <path>]` installs the supported hooks, and `setup
--link [<checkout>]` and `setup --unlink` link and unlink the installed Agent
Factory skill. See [sessions](sessions-and-hooks.md) for active-work handoff,
hook behavior and the skill link.

## Committing records

A step usually writes several records (a finding, its disposition, evidence, a
transition). `review record|approve|finding|trivial|disposition|defer`, `evidence record`
and `issue transition <id> <name>` accept two flags that replace one hand-made
commit per record:

- `--commit-records` stages the records the command wrote and commits them
  with a generated subject such as `Record finding atelier-ab12`.
- `--stage-records` stages them without committing. The next command with
  `--commit-records` commits them together with its own records, as one commit
  whose subject is `Record <first> and N more` and whose body lists each.

Only the records the command wrote are staged and committed: Atelier takes them
from its own record writes, not from `git status`, so other staged or
unstaged changes are never added. If, before the command writes, the index
holds anything but records staged earlier by `--stage-records`, the command
refuses with `record_commit_index_not_clean` and writes nothing. The
staged group is remembered in `.atelier/runtime/staged-records.json`, which Git
ignores. A group ends when its records are no longer staged (a hand commit, a reset), and a
record of the group with unstaged edits refuses with
`record_commit_pending_modified`; each command stages only the records it wrote.
The group is remembered before the commit runs and forgotten after it succeeds,
so a rejecting hook leaves it for the next `--commit-records`. `review` keeps `--commit` for the pinned commit; the flags here are
separate. Code still belongs in its own commit; `land` and `merge` judge the
committed records either way.

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
records and policy; it does not repair a hidden cache. The two old record
forms Atelier still reads, separate disposition events and open After merge
lines, pass it with a note naming the migration that rewrites them
(`atelier migrate dispositions --apply`, `atelier migrate after-merge`).
Version 1 review records are read as written, since approvals stay as
committed (atelier-l457). `check` also warns, at the file and line, about
mechanical plan problems: an open work item whose Done when has no `- [ ]`
line, a `- [ ]` line with no text, and a Markdown link in an issue record or
in a strategy a mission links whose repository-relative target exists
neither beside that file nor from the repository root (URLs and anchors are
not checked). Warnings print whether or not the check passes, on stderr with
`-q`, and never change its exit code (atelier-edaj). Every command that
writes records refuses, before writing anything, records that would fail a
rule `check` applies that the records before them pass (`record_invalid`). Prune is dry-run by default
and removes only eligible artifacts in apply mode. Commit canonical deletions.

Removed surfaces include provider review, review rooms, transition actions,
`work queue`, `history`, `man`, `rules`, telemetry, import/export, hidden branch
helpers, cache rebuild, and a TUI. Use ordinary Git for history and recovery.
There is no compatibility command layer.
