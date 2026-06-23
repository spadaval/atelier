# Actual Agent Complaint Audit

This audit records command complaints found in Codex session logs from
2026-06-16 through 2026-06-22. It complements
[human output refresh](human-output-refresh.md): the earlier pass sampled current
output and predicted user complaints, while this pass records where agents
actually got confused, hit command correctness bugs, or called out a command
shape as wrong.

Scope:

- Logs scanned under `/root/.codex/sessions/2026/06/16` through
  `/root/.codex/sessions/2026/06/22`.
- Three read-only GPT 5.4 Mini explorer subagents split the date range.
- Local searches then narrowed evidence around `atelier` commands and complaint
  language.
- Pure performance complaints, especially bulk-update slowness, are excluded
  from the findings below.

## Complaint Themes

| Theme | Evidence | UX implication |
| --- | --- | --- |
| Status output had correctness bugs. | `atelier status` was reported as blocked by a workflow config error in `/root/.codex/sessions/2026/06/21/rollout-2026-06-21T12-25-38-019eeb00-7c44-71f1-996a-d256516264a8.jsonl:15`. Another agent saw `git status` show modified files while `atelier status` reported a clean checkout in `/root/.codex/sessions/2026/06/20/rollout-2026-06-20T15-21-06-019ee67a-c70b-7b43-9236-17eeabdb1b1a.jsonl:46`. | If current `status` output already shows Git state directly, verify that behavior with regression tests. If it can still disagree with Git or fail on valid config, fix the command behavior rather than designing around it. |
| Status-like views could show stale or incompatible data. | `mission status` appeared to use outdated display text after canonical Markdown edits in `/root/.codex/sessions/2026/06/16/rollout-2026-06-16T10-54-25-019ed0ed-2e07-7f20-8517-f4dca5ae8d94.jsonl:3220`. Another run noted the installed command rejected a newly valid canonical mission status in `/root/.codex/sessions/2026/06/18/rollout-2026-06-18T12-39-19-019edb99-efed-76e2-bc2a-7051a410efad.jsonl:577`. | The user-facing requirement is simple: status/detail commands should report current record state or fail with a clear public recovery path. Internal projection or binary details belong in verbose/admin diagnostics unless needed to explain the recovery. |
| Ready/selectable work views can hide blockers or real work. | An agent observed that parent-epic blockers did not stop child records from showing as selectable work in `/root/.codex/sessions/2026/06/18/rollout-2026-06-18T23-54-41-019ede04-4375-7641-bb03-a4a59d022403.jsonl:184`. Another found `issue status` only printed selectable leaf work in the Ready section in `/root/.codex/sessions/2026/06/20/rollout-2026-06-20T16-19-46-019ee6b0-7e7f-7d43-9aff-7d7e28025f3b.jsonl:394`. | Queue and objective views must distinguish workflow state, blocker state, and display role. Do not let visual grouping imply that a context row is next work, or that selectable leaf work is the whole ready set. |
| Lifecycle commands need one obvious path. | A June 22 audit called out duplicate lifecycle paths around `mission`, `start`, and workflow-backed `issue transition` in `/root/.codex/sessions/2026/06/22/rollout-2026-06-22T13-34-54-019ef066-4226-7f53-bdd8-dfba83a72459.jsonl:38` and `:42`. A June 20 review noted removed `mission start` behavior failed without `--switch` replacement guidance in `/root/.codex/sessions/2026/06/20/rollout-2026-06-20T16-19-46-019ee6b0-7e7f-7d43-9aff-7d7e28025f3b.jsonl:618`. | The refresh should prefer one transition-owned lifecycle surface and make removed-command errors point at the current command only when a replacement exists. |
| Help can be accurate and still not useful enough. | `issue --help` was described as accurate but not telling a worker which subset matters in `/root/.codex/sessions/2026/06/22/rollout-2026-06-22T13-34-54-019ef066-4226-7f53-bdd8-dfba83a72459.jsonl:22`. `search` help was called issue-text oriented even if the implementation covers broader records in the same log at `:24`. | Root help should stay concise, but role guides and command footers need intent-ranked next actions. Names and summaries must match actual scope. |
| Implementation-shaped nouns force operators to translate intent. | `graph` was called weak implementation language because operators start from an issue, mission, blocker, or proof question in `/root/.codex/sessions/2026/06/22/rollout-2026-06-22T13-34-54-019ef066-4226-7f53-bdd8-dfba83a72459.jsonl:39`. `repair` was called misleading because it overlaps with `doctor --fix` at `:51`. `export` was called misleading if visible because product docs frame export/rebuild as low-level mechanics at `:53`. | The formatter pass should not preserve abstract command concepts just because output can be made prettier. Some confusing names belong in retired, hidden, or admin-only surfaces. |
| Stale flags and provider plumbing weaken the public contract. | `forgejo roles provision` still advertised `--write-config` in `/root/.codex/sessions/2026/06/20/rollout-2026-06-20T17-10-07-019ee6de-968a-7ba0-9c09-9fd8ec02c433.jsonl:64`. | Help output must be audited with the same seriousness as command output. Stale options are UX bugs because they teach old ownership boundaries. |
| Output wording is brittle enough that tests and agents chase text instead of behavior. | A projection refactor run noted start/status failures were display-text failures, not command-success failures, in `/root/.codex/sessions/2026/06/16/rollout-2026-06-16T10-54-25-019ed0ed-2e07-7f20-8517-f4dca5ae8d94.jsonl:1719`. The same run found evidence output printed `Status`, not `Result`, in `:1362`. | The shared formatter should define stable semantic sections and labels. Tests should assert those semantic contracts, not incidental prose churn. |
| Docs/help drift diagnostics expose awkward machine-shaped rules. | A command-surface run found a table entry was human-readable but invisible to the drift checker because it lacked a backticked `atelier ...` span in `/root/.codex/sessions/2026/06/17/rollout-2026-06-17T17-44-51-019ed78b-4c8d-7780-b1f5-31db35f81772.jsonl:7393`. | Drift tooling should not force docs into unnatural shapes. Where tests need anchor syntax, document that contract or move the anchor to a dedicated inventory. |

## Refresh Guidance

Actual agent complaints line up with the predicted human-output issues, but add
three sharper requirements:

- Correctness bugs stay bugs. If `status` can disagree with Git, if status-like
  views can use stale data, or if a valid workflow config blocks orientation,
  the command behavior needs a regression fix rather than a prettier warning.
- Command names and help are part of the UX. Retired or hidden surfaces such as
  `graph`, `repair`, `mission`, `start`, and `export` should not be polished
  back into normal workflow by accident.
- Work-selection output must be semantically precise. It needs to separate
  workflow state, blocker state, display role, and next command so visible rows
  are not confused with next work.

The formatter work should therefore cover more than color and layout. It should
create a shared language for rows, blockers, display roles, summaries, and
drill-down commands, while command-specific code remains responsible for
correct state, current data, and valid recovery guidance.

## Out Of Scope

Performance complaints were intentionally set aside. The logs contain repeated
friction around slow reads, bulk operations, and projection-cache cost, but this
audit only records command correctness, UX, output, and guidance problems.
