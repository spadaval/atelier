# Sessions and Hooks

```text
atelier status
atelier land
atelier land <owner-or-child-id>
atelier land --install
atelier land --push
atelier setup claude
atelier setup claude --binary <path-to-atelier>
```

`status` is the one orientation view (ADR 0022). It reads the checkout's
branch, owner, and dirty paths, the ready, needs-a-human, and active counts,
current missions, and the other local work branches. On an owner's branch it
adds the owner's record body, integration base, mission and strategy path,
scoped work, the current decisions bearing on the owner in brief (see
[decision rows](human-cli-output.md#decision-rows)), the 8 newest reviews, and the 8 newest evidence receipts with
their freshness; without an owner it lists up to 5 ready items under the ready
count, and an `Unassigned work:` count with that work grouped by label under
an `Unassigned Work` heading, as `work missions` groups it (see
[mission views](issue-inventory-and-mission-overview.md)), but only the first
5 groups, with a count of the rest that names `atelier work missions`, to keep
the hook context bounded; a group's own count of the rest names `atelier work
missions --all`. The other local work branches follow the owner's or the
unassigned sections, and the `Next` footer is always last. With no
open mission, the missions line names the most recently closed mission and
its close date. It creates no session record and prints state and next actions, not
instructions. `prime` is a deprecated alias that prints the same view after a
deprecation line, kept while adopters' installed hooks still call it.

A session's repository instructions come from AGENTS.md, which the harness
loads, not from `status`. Claude Code reads AGENTS.md only when the repository
has no `CLAUDE.md`, or through an `@AGENTS.md` import in it, so `atelier setup
claude` warns, with that fix, when AGENTS.md exists and the repository has a
`CLAUDE.md` or `.claude/CLAUDE.md` but neither imports `@AGENTS.md`.

`land` checks a handoff without writing records, committing, switching, or
merging. On a named branch other than the base, as HEAD's committed policy
names it, where work is in progress, it
reports what is open and refuses nothing: each failed check is a line with its
recovery step, the verdict reads `land: work in progress — N checks open (…)`,
and it exits 0, since nothing there reaches the base without `merge`'s gates
(ADR 0028 decision 5). On the base, and on a detached HEAD, a failed plain
command exits nonzero with the failed invariant and recovery step, since that
is where unreviewed code would reach the base. `atelier doctor` refuses any
failure wherever it runs.

Canonical records and policy must match committed HEAD, including ignored
canonical paths and files hidden by Git index flags. Active work must have a
recorded owner branch. An active owner may hand off unfinished code without
approval or full scope completion; report those changes honestly. Its done
child claims still need their applicable completion checks. A terminal owner
requires a clean checkout and the full merge gates. An explicit child ID
resolves its owner; otherwise ownership is inferred from the current branch.

A failed precondition is one more failed check, and land still runs every gate
it can: `land_records_uncommitted` names every tracker record that differs from
HEAD (the first five, then a count) as modified, disposition changed, new,
deleted or staged, with
`git add .atelier && git commit -m "…"`, except that a committed approval or
trivial exemption, or a review record the base already holds, that was
deleted, or changed in anything but a finding's disposition, is restored with
`git restore --source=HEAD --staged --worktree -- <path>`, since those are
append-only apart from dispositions (ADR 0025); a correction to a finding the
base does not hold is committed (see [correcting review
records](reviews-and-merge.md#correcting-review-records)). The gates read
HEAD's committed records instead. A rewrite already committed
refuses with `review_append_only`, whose fix leaves that commit out with
`git rebase --autostash --onto <commit>~1 <commit>`, named by the commit so it
holds after the other fixes commit, and printed first; below the tip it
rewrites the later commits (atelier-v4rd, atelier-61il). When that commit is a
merge, the line also says the rebase drops the commits it merged in, to be
merged or redone if they should stay (atelier-zegg). A rebase, merge,
cherry-pick or revert stopped on a conflict is reported before anything else,
as `land_rebase_in_progress`, `land_merge_in_progress`,
`land_cherry_pick_in_progress` or `land_revert_in_progress` (`merge_*` from
`merge`), with `git rebase --abort`, `git merge --abort`, `git cherry-pick
--abort` or `git revert --abort` (atelier-vlhn, atelier-zegg); `land_completion_dirty` names a done owner's uncommitted
paths, to commit (`git add -A && git commit -m "…"`) or set aside (`git stash
push --include-untracked`); an invalid committed policy is reported once, as
its `workflow_*` code, which also lets the gates run (atelier-kiry,
atelier-grsi, atelier-7ts2, atelier-y0en). The base is the policy's
`base_branch`; a `branch_base` an owner record carries is not read. `land <id>`
away from the owner's branch reports `land_branch_mismatch: owned on <branch>, not
on <here>` and `git switch <branch> && atelier land`, finding the branch from
the local branches' own records when this checkout's do not name it; a done
owner whose branch is gone says to land the checkout as it is; a mission says
missions have no branch (`land_mission`, with `atelier work mission <id>`); and
an item no branch owns names `atelier start <id>` (atelier-ns04). An unowned
branch, or active work, whose item a local branch owns is pointed at that
branch with `git switch <branch> && atelier land` (atelier-9a6e). A
precondition that keeps the gates from running prints after the ones found
before it, with a `Not checked:` line naming the gates it skipped.
When the base has commits the branch does not, `land` checks the work against
the point where it left the base and adds a note that `atelier merge` lands
the branch on it with a merge commit, with `atelier merge --sync` to bring the
base in first when they conflict (atelier-qz9x, atelier-jnrh). Once the
owner's approval is current, a passing `land` names `atelier merge <id>` as
the next step; work in progress hands off without it.

A branch that no item owns, such as a feature branch, a PR branch, a branch
from before adoption, or a detached HEAD, lands with a note when it changes no
tracker files (anything under `.atelier/`) since its merge base with the base
branch. A path counts only if it changed relative to every local and
remote-tracking copy of the base, so a stale local base does not blame the
branch for upstream records. If the branch does change tracker files, or
shares no history with the base, `land` reports `land_work_untraceable`,
names the changed paths (the first five, then a count), and points to
`atelier start`. A branch that installs Atelier says so: while no copy of the
base has a workflow policy, `land --install` passes with a note on how to land
it (see [installing Atelier](workflow-configuration.md#installing-atelier)), and
plain `land` reports `land_install_unconfirmed` and points to the flag.
`land` does not infer an install from the branch's own policy, which the branch
can edit (atelier-bibd). `--install` is refused on an owner's branch, on the
base, and when the base already has a policy. On a named branch these are work
in progress, reported with exit 0 and without blocking the Stop hook; on a
detached HEAD they refuse.

`land` ends with a verdict line, the last line of its combined output (see
[verdict line](human-cli-output.md#verdict-line)): `land: passed`; `land: work
in progress — N checks open (…)` on a work branch, with exit 0; or `land:
refused — N checks (…)` with a non-zero exit. The Stop hook's block reason
is the refusal as `land` prints it, ending with that verdict, with no
sentence of its own, so each failure carries its one recovery step; its
message on a repeated stop is one sentence, then the same refusal
(atelier-467y, atelier-ft88). `land --push` runs the same
checks and, when they pass or report work in progress on a branch, pushes the
checked HEAD to the current branch's own configured upstream
(`branch.<name>.remote` and `.merge`) through git, then ends `land: passed,
pushed to <remote>/<branch>` or `land: work in progress — N checks open (…),
pushed to <remote>/<branch>`. A refusal pushes
nothing and ends `land: refused — 1 check (<code>), nothing pushed`; a branch
with no upstream or a detached HEAD refuses (`land_push_no_upstream`,
`land_push_detached`), as does an upstream on remote `.` or on a branch of
another name, such as a work branch created from `origin/main`
(`land_push_upstream_mismatch`; fix it with `git push -u <remote> <branch>`).
It also refuses when the base branch has an upstream that does not resolve to
a remote-tracking branch, because its remote branch is gone or its remote is a
URL or path, so unreviewed base commits could not be checked
(`land_push_base_upstream_unresolved`; fix it with `git fetch --prune` and
`git branch -u <remote>/<base> <base>`).
What is pushed is the commit land checked: if HEAD moved during the checks
nothing is pushed (`land_state_changed`). A push git rejects exits non-zero with
git's message and `land: refused — 1 check (land_push_failed), nothing
pushed`. `--push` does not combine with
`--hook`. Prefer it to `atelier land && git push`, and never push behind a
pipe: a pipe's exit status is its last command's (atelier-ecah).

When the local base branch is ahead of its upstream, `land` (and `land
--push`) prints a warning, before any push, naming the unpushed base commits
the current owner did not make: up to five, as `<hash> <subject> (<author>)`,
then a count. A commit is the owner's when its message names the owner or an
issue in the owner's scope, as the start, merge and planning commits for
owned work do; git authorship cannot tell sessions apart, since agents
share one identity. With no owner, every unpushed base commit is named. The
warning does not refuse: pushing the base can be meant, and it publishes
those commits. A base with no upstream is not checked here (atelier-mvzi).

On the base branch itself, `land` checks unpushed base commits: only records
and mission strategies may be committed there without review (see
[planning changes on the base](reviews-and-merge.md#planning-changes-on-the-base)).

Landing also checks completed claims elsewhere in the checkout that a change
since a trusted point can affect. The trusted point is the merge base of the
base's upstream with the branch base on a work branch, or with HEAD otherwise:
the local base can hold commits no land has checked, so it is not trusted.
Without an upstream, every claim is checked. Every change that reaches the
base had its own claims checked when it landed, so earlier history holds by
induction. A claim is checked when its record changed; when a changed issue
record reaches it through blocked, parent, or child links, directly or along a
chain, before or after the change; when it is a mission that such a record
advances (the chain stops at the mission, so adding or editing one issue of a
mission does not re-check the mission's other done work, atelier-ss25); when
a review or evidence record
about it was added, removed, or retargeted; when the workflow policy at HEAD
or on the base differs from the trusted point; or, under a policy that gates
on `lint.none_blocking`, which reads every record, when any record changed.
Review history is checked from the same point for edits to approvals and
exemptions, and to review records the base already held.
`atelier doctor` checks every completed claim and the whole review history,
for history that reached the base without the gates, such as a direct push
(atelier-37uq).

Checked integrated claims retain the acceptance that was valid when completion
was claimed. Historical acceptance requires immutable review history: an
approval counts only as first committed, and its identity is recorded, not
compared with the work's authors (atelier-md0v). Without an owner, a claim
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

Installed commands are `atelier status --hook claude` and
`atelier land --hook claude`, with the selected executable. Rerunning setup
replaces an earlier managed `prime --hook claude` handler. The adapters read
JSON stdin and use its `cwd` to find the checkout. SessionStart returns context
through `hookSpecificOutput.additionalContext`. A Stop that `land` refuses
returns `decision: "block"` and a reason at exit 0. When `stop_hook_active`
signals a repeated failure, it emits a warning and allows stopping to avoid a
loop; plain `land` continues to fail until the invariant is fixed.

The Stop hook blocks only for a refusal on the base or a detached HEAD, where
`land` itself refuses (ADR 0028 decision 5). Each case below returns a
`systemMessage` at exit 0 instead of a block:

- A payload the adapter cannot use, such as input that is not JSON, another
  event's payload, or a missing or unusable `cwd`, is the hook's
  misconfiguration, not a landing failure: `Atelier hook misconfigured:
  <error>; check .claude/settings.json`. SessionStart returns the same text
  as its context (atelier-ft88).
- A Stop run from a directory outside any Atelier repository says so,
  `Atelier: <dir> is not in an Atelier repository, so there is nothing to
  land.`
- On a named branch other than the base, whatever land reports is work in
  progress: the message is `Atelier: work in progress; land reports it and
  refuses nothing on a work branch.`, then land's report ending with its
  verdict. A missing review, an open finding, uncommitted records or code and
  any other open check report the same way.
- On the base branch, when every failed check is `land_records_uncommitted`,
  the message says the base has uncommitted records and does not block, then
  gives land's refusal, which names the records and the commit that commits
  them (atelier-mp6v). Any other failure on the base blocks.

A record whose `schema_version` is higher than the installed binary reads was
written by a newer Atelier. Every command, `status` and its SessionStart
context, `land` and the Stop hook say so in one line instead of a parse error,
`<path> has schema_version N, and this atelier reads M, so records need a newer
atelier`, whose one step upgrades to the latest release with the installer
one-liner from the [README](../../README.md#install), or from Atelier's source
checkout runs `cargo install --path crates/atelier-cli --locked`. Like any record that fails
to parse, it carries no code, and `land` ends `land: refused`, or `land: work
in progress` on a work branch. A newer
workflow policy fails `workflow_config_schema_newer` with the same step,
said once: a command that falls back to the last valid policy notes the
fallback without also saying to edit the policy. A
lower version is an invalid record, as before (atelier-u4ql). These protocol fields
follow the [official Claude Code hooks reference](https://code.claude.com/docs/en/hooks).

Hooks assist handoff; their warning escape does not prove completion. Actual
session acceptance needs a dogfooded session as well as adapter tests.

## Other branches

`status` shows labeled summaries of committed local work-branch scopes, while
the current checkout uses live records. It does not fetch remote branches or
overlay another branch's records onto the current graph. Incompatible or
unavailable snapshots are reported separately. Every compatible owner branch
remains visible, including branches sharing a tracker tree. A branch's done
claim is not base completion: a done owner's row says `not merged` until the
base branch's own copy of the record is done, and `merged into <base>` after.
A row that is not merged also shows its approval state, commits behind the
base, whether proof is at its head, and whether it merges cleanly into the
current base or which files conflict; a trailing line names waiting branches
that change the same files. The row format is in
[human-cli-output.md](human-cli-output.md#branches).

The same summaries, read once per command, mark ready work that is in flight
elsewhere: in `status`'s ready items and unassigned work, in `work ready`
and `work all`, and in the Ready Work of `work mission` and `work parent`, an
item that owns another local branch gets a `↳` line under its row with its
state there, `<status> on <branch>`, plus `, not merged` or `, merged into
<base>` for a done claim. The item stays listed and classified from the
current checkout's records. When an owner's branch cannot be read, its row
says `↳ state unknown on <branch>: <reason>`; when the branch's owner cannot be
found either, the page names the branch instead, and if no summaries can be
read at all, the page says so.
