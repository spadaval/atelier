# CLI Surface Tiers

Atelier's public CLI presents the agent-native workflow first. Inherited
Chainlink utilities are not kept as command aliases once their replacement path
is documented; deleting old surfaces and their command code is preferred over
compatibility shims. This repository is a WIP product with few users, so
compatibility with obsolete command shapes, statuses, and output contracts is
usually harmful. Do not add staged deprecations, aliases, fallback readers, or
old-output shims unless a human explicitly asks for them for a specific
transition.

Per [ADR 0006](../adr/0006-agent-guidance-ownership-boundary.md), this file
and Atelier help own the repository's tactical operator guidance. Agent Factory
coordinates agents, but it should route repo-specific command choice,
workflow-recovery, readiness, and completion detail to Atelier-owned command and
doc surfaces instead of duplicating that contract.

Static docs describe the command surface and ownership boundaries. They do not
decide the next lifecycle action for a specific work item. Operators and agents
use `atelier status`, record detail, issue status <objective-id>, and transition option
output to decide what to do next in the current checkout.

The command audit may document target replacement forms before they are
implemented. The workflow-first list below names only command paths that exist
in the current CLI surface.

## Workflow-First Core

Workflow-first commands are stable enough to appear in `atelier --help` and are
the repo-owned surfaces Agent Factory may route operators toward. The command
output, not this static list, decides the next process step for a concrete
issue or mission:

- `atelier init`
- `atelier man [worker|reviewer|validator|manager|admin]`
- `atelier status`
- `atelier issue ...`
- `atelier issue create "..." --issue-type mission`
- `atelier issue show <objective-id>`
- `atelier issue status <objective-id>`
- `atelier issue link/unlink <objective-id> <issue-id> --role advances`
- `atelier issue block/unblock <objective-id> <issue-id>`
- `atelier search <query>`
- `atelier bundle preview/apply`
- `atelier evidence record/show/attach/list`
- `atelier review open/link/status/show/merge/comments/comment/approve/request-changes/resolve`
- `atelier forgejo roles check/provision`
- `atelier history`
- `atelier issue note <id> "..."`
- `atelier maintenance ...`
- `atelier lint`

## Command Categories

Atelier commands fall into four product categories. The category determines
where the command may appear, which operator should reach for it, and whether it
may be cited as ordinary workflow proof.

| Category | Definition | Examples | Excluded non-examples |
| --- | --- | --- | --- |
| Normal workflow | Product-facing commands used to orient, select work, mutate canonical records, record proof, inspect completion status, and check ordinary committed-state health. They may appear in root help, role guides, ready-work actions, issue status <objective-id>, and Agent Factory workflow guidance. | `status`, `issue show`, `issue transition --options`, `issue status <objective-id>`, `evidence record`, `lint`, plus lifecycle or review primitives when command output routes there | `doctor`, `export`, `rebuild`, `workflow check`, `diagnostics slow`, `import-beads`, destructive `maintenance delete` |
| Admin maintenance | Visible but specialized commands for setup, explicit local-state repair, explicit pruning, destructive record surgery, or manual owner-branch recovery. They may appear in admin guidance or targeted recovery output, but not as the default worker/reviewer loop. | `init`, `doctor`, `doctor --fix`, `prune`, `prune --apply`, `maintenance delete ... --force`, `branch status`, `branch merge` | `issue status <objective-id>`, hidden `workflow check`, hidden `diagnostics slow` |
| Hidden debug diagnostics | Callable implementation probes for raw workflow-policy detail, local telemetry, deterministic rendering, or projection debugging. They stay out of root help and ordinary role loops. Targeted diagnostics, tests, or migration notes may name them. | hidden `workflow check`, hidden `diagnostics slow`, hidden/advanced `export --check`, hidden/advanced `rebuild` when used as a projection probe | `lint`, `doctor`, `issue status <objective-id>`, `issue transition --options` |
| Temporary migration | Transitional surfaces that exist only to move inherited state or prove deterministic renderers while the Markdown-first store stabilizes. They must name their sunset or follow-up owner and must not become new workflow requirements. | `init --import-beads`, hidden/manual `import-beads`, hidden/admin `export` for deterministic renderer testing during migration | backup `import`, `export --format json|markdown`, routine handoff checks |

Tracked Markdown under `.atelier/` is authoritative. Local SQLite projection
state, runtime tables, locks, diagnostics, and cache files are repairable
checkout state. Normal commands should refresh or mark projections stale safely
when possible. `atelier doctor` and `atelier doctor --fix` are admin repair
surfaces for ignored runtime/cache/projection state and must not edit tracked
canonical Markdown. If an `export`-style deterministic renderer is retained, it
is hidden/admin migration or test infrastructure, not a normal health, handoff,
validation, or completion command.

`atelier init` is core tracker setup only. It creates `.atelier/` records,
`.atelier/config.toml`, `.atelier/workflow.yaml`, local runtime storage, and
root ignore rules. It does not install editor or assistant hooks. When
`--import-beads` is supplied, it imports the standard repo-local Beads backup
from `.beads/issues.manual.jsonl`; otherwise it may mention that migration
input was detected but must not silently convert it. Its default next steps
must not route a fresh checkout directly to issue creation before `atelier
lint` confirms the committed tracker and workflow configuration are valid.

`atelier man [<role>]` is the role-specific guide surface. It filters the
existing product command surface for the operator's job without creating
role-prefixed command namespaces. Valid roles are `worker`, `reviewer`,
`validator`, `manager`, and `admin`. `manager` is the broad CLI role class for work
coordination; Agent Factory may still use `orchestrator` for a specific agent
type inside that class, but `orchestrator` is not a `man` role alias. With no
role, `atelier man` lists the valid roles. Worker, reviewer, validator, and
manager guides require valid tracker/runtime state and fail fast with recovery guidance when
state is unavailable. The admin guide degrades gracefully before initialization
or when local state is broken.

`atelier status` is the root checkout signpost. It summarizes the current-work
set, active objective context when visible from committed records, ready work
count, tracker freshness, and the next work or health drill-down commands. It
does not replace `issue status <objective-id>`; it points operators to the
scoped status surface that owns completion status.

`atelier --help` should expose the normal operator path first: orient, inspect
objectives and issues, inspect workflow options, manage blockers, record proof,
inspect history, and validate committed records. Low-level state diagnostics
such as doctor/export/rebuild, predecessor imports, raw workflow diagnostics,
and command telemetry are not normal root-help entries; admin guidance,
targeted error messages, and migration docs may name them when they are
relevant.
Destructive maintenance may remain visible as an explicit danger-zone command.

## Visible Help Contract

Every help-visible command group needs one job, one default information budget,
one quiet-output contract, and a named drill-down path. Quiet mode stays terse:
IDs, counts, paths, status tokens, and pass/fail tokens only.

### Workflow-First Surfaces

| Surface | Job | Default output | Quiet output | Drill-down path |
| --- | --- | --- | --- | --- |
| `init` | Create tracker scaffolding in a repo that does not have Atelier yet. | Created or reused paths plus workflow setup, optional Beads migration detection, and verification commands before issue creation. | Created path(s) and a success token. | `lint`, `man admin`, `status`, inspect `.atelier/config.toml` and `.atelier/workflow.yaml`. |
| `man` | Show role-specific operating guidance for worker, reviewer, validator, manager, or admin. | Role list or a role guide with current state, ranked commands, normal loop, and commands not usually for that role. | Quiet mode is ignored because `man` is human guidance, not a composition API. | `status`, `issue status <objective-id>`, `issue list --ready`, role-specific commands, or `man admin` when repair is needed. |
| `status` | Root orientation for the current checkout. | Current-work set with configured active roles, active objective context when visible, ready count, tracker freshness, and next work commands. It names admin repair only when local state is degraded. | IDs, counts, and freshness token only. | `issue status <objective-id>`, `issue show <id>`, `issue list --ready`, and admin repair guidance only for degraded local state. |
| `issue` | Create, list, show, update, transition, note, inspect type-aware status, and manage issue-owned blockers and links. | Queue or detail views using the shared human-output grammar; detail reads name the canonical Markdown path and next commands. Transition output owns lifecycle routing for the current issue. Blocker mutations name the blocked issue and blocker issue, blocker inspection stays under `issue blocked`, type-aware objective status owns mission-shaped health and terminal readiness, link mutations name the source, target, and role, and note entry appends activity without field mutation. | IDs, status tokens, changed fields, blocker IDs, relationship roles, and canonical paths. | `issue show <id>`, `issue status <objective-id>`, `issue note <id> "..."`, `issue transition <id> --options`, `issue list --blocked`, `issue blocked [<id>]`, edit the Markdown record, `history --issue <id>`. |
| `search` | Search record text when the operator does not know the exact ID yet. | Bounded queue grouped by readiness or priority when useful, with the search query echoed. | Matching IDs only. | `issue show <id>`, `issue status <objective-id>`, `history`. |
| `mission` | Transitional namespace for existing mission records until objective issue replacements land. | Existing mission reads and mutations remain only long enough to migrate behavior to type-aware issue commands. The target state removes this root namespace rather than aliasing it. | IDs, counts, lifecycle tokens, completion-status token, and close reason. | `issue show <objective-id>`, `issue status <objective-id>`, `issue transition <objective-id> --options`, `history --issue <objective-id>`. |
| `bundle` | Preview and apply one-shot graph bundles from files. Use this for bulk mission, epic, issue, relationship, and evidence creation instead of shell loops over individual mutation commands. | `preview` prints deterministic non-mutating validation output; `apply` requires `--yes` and prints created IDs, relationship counts, and recovery guidance when needed. | Created IDs, counts, and pass/fail tokens. | `issue show <id>`, `issue show <objective-id> <id>`, `evidence show <id>`, `lint`. |
| `evidence` | Record and inspect proof records. | `record` is the default proof-capture workflow; `show` and `list` inspect existing evidence; output names target, kind, result, and reusable IDs. | Evidence IDs, target IDs, result tokens, and stored command status only. | `evidence show <id>`, `history --issue <id>`, `issue show <id>`. |
| `review` | Manage the configured review artifact for issue or epic work. | `open`, `status`, `show`, `merge`, `comments`, `comment`, `approve`, `request-changes`, and `resolve` operate on the configured review mode. Mutating commands use explicit `--role` or infer role from the owner issue status. `merge` enforces review safety but never changes Atelier workflow status. Normal lifecycle routing comes from issue transition/status output. | Issue ID, review ID/number or URL, role source, merge/review/comment status tokens only. | `issue show <id>`, `issue transition <id> --options`, `issue status <objective-id>`, configured review artifact. |
| `forgejo` | Configure and verify Forgejo integration when the repository configuration selects Forgejo-backed review artifacts. | Role and integration commands report configured account state, permission checks, and remediation text. They do not decide whether a workflow step needs a review artifact. | Role names, pass/fail tokens, and remediation text only. | `.atelier/config.toml`, `review`, `issue transition <id> --options`, configured provider UI. |
| `history` | Inspect canonical repo, mission, issue, or epic activity. | Newest-first bounded activity feed with scope and filter context echoed. | Event counts, scoped IDs, and timestamps only. | Broaden or narrow with `--mission`, `--issue`, `--epic`, `--event-kind`, `--actor`, or `--since`; return to `issue show` or `issue show <objective-id>` for current state. |
### Specialized But Visible Surfaces

| Surface | Job | Default output | Quiet output | Drill-down path |
| --- | --- | --- | --- | --- |
| `branch` | Inspect, repair, or manually recover owner branches when workflow-owned lifecycle steps cannot complete automatically. | `status`, advanced repair, and manual branch-preparation forms name the owner record, branch, base branch, merge strategy, checkout, and recovery state. | Record IDs, branch names, paths, and branch-state tokens. | `status`, `issue show <id>`, `issue status <objective-id>`. |
| `prune` | Explicitly prune accumulated local artifacts and report cleanup classes whose retention contracts are not implemented yet. | `atelier prune` is dry-run by default; reports diagnostics log candidates, retention cutoff, protected/deferred classes, and the apply command. `atelier prune --apply` removes only eligible local diagnostics logs in the initial slice. | Candidate counts, removed paths, and failure tokens only. | `diagnostics slow`, local diagnostics directory, future retention contract issues for records, branches, and worktrees. |
| `maintenance` | Explicit destructive record surgery only. | Clear target and consequence summary before deletion, then confirmation of the deleted record. | Deleted ID and kind only. | `history`, `lint`, and Git inspection when recovery is needed. |
| `lint` | Validate canonical tracker records and committed workflow configuration. | Pass summary or named record, workflow config, and file errors with repair guidance. | Pass/fail token and offending IDs or paths only. | Edit the named record or workflow config and rerun `lint`; use admin repair only when local state is implicated. |
| `doctor` | Validate runtime, install, and derived-state health; repair ignored local state when `--fix` is supplied. | Named health checks, degraded-state reason, and repair guidance. With `--fix`, reports each ignored runtime/cache/projection repair and refuses to edit tracked `.atelier/` canonical records. | Pass/fail token and degraded check names only. | `lint`, edit named canonical records, `status`. |

Adjacent command placements that still need confirmation or cleanup are tracked
in the command audit:

- `rebuild`: hidden debug diagnostic or admin repair primitive only if
  `doctor --fix` delegates to it; not normal workflow.
- `workflow check`: hidden raw policy diagnostic; normal readiness uses
  `issue transition --options`, `lint`, and `issue status <objective-id>`.
- `diagnostics slow`: hidden local telemetry; not an automation contract.
- `prune`: visible admin maintenance; dry-run by default and applies only
  explicitly supported cleanup classes.
- `import-beads`: temporary migration surface; normal setup uses
  `init --import-beads`.
- `maintenance`: visible admin danger zone; never a routine next action.
- `branch`: advanced/manual owner-branch recovery. Routine branch guidance comes
  from status, issue detail, transition, and recovery output.
- `worktree`: removed visible workspace-management surface pending redesign.

Hidden advanced diagnostics probes may remain callable for local performance
analysis, but they are not visible root-help surfaces and must not appear as
normal mission, issue, blocker, evidence, or completion next actions. Slow-command
diagnostics remain hidden local performance analysis rather than a normal
operator workflow step.

## Operator Jobs

The public command surface is organized around jobs an operator performs under
time pressure:

- Orient: answer what is active, ready, blocked, stale, or unsafe to change.
  Owned by `atelier status`, `atelier issue show/list`, `atelier issue show`,
  `atelier issue table`, focused `atelier issue status <id>`, `atelier history`,
  and `atelier search`.
- Select and run work: choose a clear slice, inspect workflow state, leave notes
  and proof, and follow the lifecycle or recovery command Atelier prints for
  the current item. Owned by `atelier status`, `atelier issue ...`, `atelier
  evidence ...`, and record-specific note commands.
  Explicit `atelier branch ...` commands are advanced diagnostics or repair
  surfaces unless Atelier routes the operator there.
- Coordinate objective progress: see linked work by state, blockers, evidence
  gaps, completion status, and the next action for the objective. Target state
  is owned by `atelier issue show <objective-id>`, `atelier issue status
  <objective-id>`, `atelier issue update`, and issue link/block commands.
  Missions are mission-typed issue records, not a separate root command
  namespace.
- Manage relationships: record issue blockers and inspect cross-record impact
  when the next action depends on graph shape. Owned by issue blocker
  subcommands, issue link/unlink, issue detail/status, and evidence
  attachment.
- Check committed state: prove tracked records and workflow config are usable
  for handoff. Owned by `atelier lint`; local runtime repair is admin-only and
  should appear only when a command reports degraded local state. Low-level
  state diagnostics such as export/rebuild are not normal handoff commands.

Normal workflow commands speak in product terms: issue, mission, evidence,
blocker, proof, completion, and committed-state validity. Advanced
diagnostics may expose workflow policy names, projections, cache repair,
command telemetry, JSON summaries, or raw validator detail, but normal
operators should only run them when an admin repair path, targeted error,
assignment, or completion contract names them. JSON emitted by diagnostics
commands is an Atelier-maintenance interface for local telemetry and
performance analysis, not an automation contract for selecting work,
proving validation, deciding blockers, or closing missions. Destructive
maintenance is a third category: commands such as `atelier maintenance delete
issue <id> --force` exist for explicit record surgery and must not appear as
routine next actions.
Common read and orientation commands correct record-kind mistakes directly:
when an operator passes an existing mission or evidence ID to an issue command,
the error names the actual record kind and suggests the matching show command.
Historical plan or milestone IDs are deferred record concepts, not accepted v1
issue targets. Genuinely unknown IDs remain concise not-found errors without
implying a false record match.

Every command-consolidation proposal must pass a red-tape check before it is
implemented: the new shape must remove a real duplicate, reduce the chance of
choosing the wrong command, and shorten the path from question to observable
answer. A consolidation that merely moves ritual into a new umbrella command,
adds mandatory ceremony to ordinary work, or hides the domain next action behind
diagnostic jargon fails this check.

Mission-shaped objectives use the normal issue workflow statuses declared in
`.atelier/workflow.yaml`. Objective records use typed sections for intent,
constraints, risks, validation criteria, linked work, blockers, and closeout
notes. Mission focus is not renamed; it is removed as a separate state pointer.
Checkout orientation comes from `atelier status` and canonical active issue
records. Close reasons become transition notes on `issue transition
<objective-id> <close-transition> --reason "..."` when the configured
transition requires a reason.

Objective completion uses `atelier issue transition <objective-id>
<close-transition> --reason "..."`, which runs the configured validators before
it commits the lifecycle change and records the reason. `atelier issue update
<id> --status <done-status>` is not the ordinary completion path. Reopening by
direct status update does not run completion validators.

Issue mutation commands are migrating toward Markdown-direct writes through
RecordStore followed by projection refresh. Projection-backed query commands
such as issue list, ready queues, search, issue detail/status, lint, and
Mission Control views may use SQLite after freshness checks.
Issue creation and issue detail output print the canonical Markdown path under
`.atelier/issues/<id>.md` so large-field editing stays file-first. Human
footers point to editing that Markdown file, `atelier lint <id>`, and focused
drill-down commands rather than generic command dumps.
`atelier issue list --status <status>` filters by exact configured workflow
status only, with `all` as the only special token. Derived status categories
are a separate concept and use `atelier issue list --category <category>`.
Category values are the exact category names from `.atelier/workflow.yaml`
such as `todo`, `active`, `blocked`, `review`, `validation`, and `done`; status
tokens such as `in_progress` are not category aliases.
`atelier issue create` has one work-type decision. Use `--issue-type` for the
canonical type (`bug`, `completion`, `epic`, `feature`, `spike`, `task`, or
`validation`) or use a template preset whose default type is documented by the
template name (`bug` -> `bug`, `feature` -> `feature`, research/investigation ->
`spike`, audit -> `validation`, refactor/continuation -> `task`). Templates
also provide default labels, priorities, and body scaffolding, and those
defaults still apply when `--parent` or a matching explicit `--issue-type` is
supplied. Conflicting type choices fail with guidance rather than silently
creating a surprising record.

First-class mission, evidence, relationship, and work lifecycle
commands are now core as a staged implementation. Mission,
evidence, and issue lifecycle mutations write canonical
Markdown through RecordStore before refreshing the SQLite projection; local
projection repair is normally transparent or routed through `doctor --fix`.
`atelier bundle preview <file>` is the manager and orchestrator path for bulk
graph creation. It validates authored bundle JSON from a real file path and
prints a non-mutating deterministic preview. Use it before `atelier bundle
apply <file> --yes` when creating an objective plus many epics, issues,
relationships, or evidence links; do not script repeated `issue create`,
`issue link`, and `issue block` loops for that shape of work. `atelier
bundle apply` applies create-only v1 bundle resources from a file path after
the operator passes the command's required confirmation flag, creates record
graphs in canonical Markdown, normalizes issue dependency fields, writes
canonical relationship buckets, refreshes projection state after successful
canonical writes, and reports recovery detail if an unexpected apply failure leaves any
created IDs. `atelier issue show <objective-id>` is the rich objective detail
read: it summarizes evidence, prose planning/checkpoint references, and work
grouped by ready, blocked, done, and backlog state. `atelier issue status
<objective-id>` is the objective health surface for linked work, configured
validator failures, blockers, record health, docs/help drift, ignored-test
review, dirty worktree state, completion status, and next actions before any
separate projection or UI is required.

`atelier issue status` requires an ID for objective drill-down. Root
orientation and discovery belong to `atelier status` and explicit issue
browsing or inventory surfaces. With an ID, the command is scoped to that
objective. Default output is compact and answers: objective identity and state,
committed tracker health, work counts, selectable work, blocked work, open
blockers, missing proof required by configured validators, completion status,
and one or two state-specific next actions. Selectable-work rows name the issue
transition command, parent context, and whether proof is already attached;
blocked-work rows name the blocked issue, the blocker IDs, parent context, and
proof state. Verbose output keeps the same sections but expands the record
lists, validator detail, blocked work, and degraded health details enough for a
handoff transcript.

State-specific next actions are part of the command contract:

- `draft`: show missing readiness fields and point to record editing,
  `atelier lint <mission-id>`, or `atelier issue update <id> --status ready`.
- `ready`: show ready work and the command to transition an issue into active
  work or inspect issue status <objective-id> explicitly.
- `active`: show active work, ready work, blockers, configured validator
  failures, and the next issue, evidence, or health command that advances the
  mission.
- `blocked`: show the open blocker records first and point to the specific
  blocker or dependent issue to resolve.
- `close-ready`: show the completion command only after linked work is closed,
  required proof is attached to accountable work, explicit validation or
  validation work has approved any parent-level judgment required by the mission,
  health gates are current, and the checkout is clean:
  `atelier issue transition <id> close --reason "..."`.
- `closed`: show the close reason, completion evidence or validation issue, and
  history/audit drill-down commands without suggesting new implementation work.

`atelier issue status <id> --verbose` is the mission terminal-check
drill-down. It remains advisory orientation unless a workflow explicitly
requires linked validation work; parent judgment that can block completion
belongs to linked validation work with attached evidence and workflow approval.
Raw workflow validator names are diagnostic detail; normal completion output
names the operator-facing blocker class and the next domain command.
`atelier lint` owns committed
workflow/config validity, `issue transition --options` owns issue-level
readiness inspection, and `issue status <objective-id>` owns mission completion inspection;
removed policy-debug commands do not replace them. Fast docs/help drift guards for
`AGENTS.md`, product command docs, visible root help, and
obsolete command-test references belong in `atelier lint` or an explicitly
named development-only diagnostic, not a required normal workflow command.
`atelier evidence record` is the normal evidence-recording surface. It records
manual summaries, command transcripts, audits, failed validations, deferred
proof, and artifact references as one operator workflow. The target is supplied
as one low-friction argument:

```text
atelier evidence record --target issue/<id> --kind validation "summary"
atelier evidence record --target issue/<id> --kind test -- <command>
```

The target syntax is `<kind>/<id>`. Version 1 accepts `issue/<id>` as the normal
accountable target. Direct `mission/<id>` targets are reserved for legacy
imports, migration notes, or explicit completion mirroring; normal mission and
epic readiness reads proof from linked accountable child issues. The command
mode preserves the old capture behavior by storing the command, exit status,
success flag, timestamp, result, and bounded stdout/stderr summaries so
validation proof does not require manual transcript copy/paste.
Evidence target links use the relation role `validates`; evidence classifications
such as `validation`, `test`, or `review` belong in `--kind`.

`atelier evidence attach` is not the default proof-capture workflow. Its only
distinct job is reusing an already-created evidence record on an additional
target, such as an explicit completion mirror or another accountable issue. New
help, next-action text, and Agent Factory guidance should teach
`atelier evidence record`; they should mention `evidence attach` only when the
operator is reusing existing proof instead of capturing it.
The predecessor evidence add/capture shapes split one operator job into two
verbs. New help and Agent Factory guidance should teach `evidence record`;
implementation may keep old entrypoints only as internal migration scaffolding
until the unified surface is shipped.
`atelier history` is the canonical project-history view. Repo-wide history and
scoped forms such as `atelier history --mission <id>`, `atelier history --issue
<id>`, and `atelier history --epic <id>` read canonical activity sidecars,
first-class records, evidence, and record links. History defaults to newest
first with bounded output, supports event kind, actor, time, and scope filters,
and separates canonical tracker history from local runtime diagnostics. Issue
and issue show <objective-id> surfaces may include compact recent activity or record context,
but they point to scoped history for full activity instead of expanding
unbounded logs.
Issue workflow commands mutate the canonical Markdown tracker copy. They must
not create a second durable active-work pointer in runtime state. Current-work
orientation is derived from the set of canonical `in_progress` issues in that
checkout. Static docs describe the command surface; status, issue detail,
transition, and recovery output decide which lifecycle command applies to a
specific work item.

Owner branch, base branch, and merge strategy are workflow/configuration
concerns. Commands that need branch context must report the computed owner,
target, and recovery path directly; operators should not infer branch action
from static command-surface prose.

Completion behavior must match the owner boundary and be recoverable. When a
tracker commit, merge, push, or required integration step fails, output must
name the failed step plus the repair or retry surface, without requiring the
operator to infer branch or review policy from static docs.

There is no target-state rule that one checkout may hold only one current issue.
Repeated starts of the same issue are harmless, and starting a second issue in
the same checkout should extend the current-work set when the workflow allows
it. `atelier status` and focused `atelier issue status <objective-id>` should render all
`in_progress` issues visible in the checkout rather than promote one hidden
"active issue". Use issue views and `issue status <objective-id>` to render all current work in the
checkout, and use the epic branch as the review boundary.

Different Git checkouts or branches may legitimately show different
current-work sets because each carries its own tracked `.atelier/` record copy.
Reconciliation happens through normal Git review and merge of canonical
Markdown records. Legacy root `abandon` and `repair` commands have been removed:
stopping work without a durable status change does not need a cleanup command,
and stale checkout recovery should use normal status inspection plus canonical
issue transitions rather than hidden active-pointer repair.
Root `atelier status`, focused `atelier issue status <id>`, and `atelier issue transition
<id> --options` expose current-work orientation, so operators should not need
the removed work-status helper or any legacy work-start path for
normal workflow.
The visible worktree helper surface is removed pending redesign. Branch helpers
inspect or repair owner branch state when workflow-owned lifecycle steps cannot
complete automatically.
Mission completion is ready only when all linked work is closed, mission blockers
are clear, required proof is attached to the accountable implementation,
review, validation, or validation work, configured validation work has
approved any parent-level judgment, linked issue records are parseable,
docs/help and ignored-test review gates are current, and the Git worktree is
clean.

## Cache Transparency

The local SQLite projection and cache are implementation details. Normal
operators should not need to know that they exist, refresh them manually, or
interpret projection freshness as a product concept. Core read and mutation
commands must transparently keep local projection state usable, and degraded
states must be reported as record or workflow repair problems rather than cache
maintenance chores.

Low-level debug and repair surfaces may expose cache mechanics when diagnosing
Atelier itself. Doctor/export/rebuild diagnostics and similar repair checks
must not appear as ordinary next actions from core workflow surfaces or root
help, and their output should make clear that they are diagnostic tools rather
than required user workflow. `atelier doctor --fix` is the admin explicit
repair path for ignored local runtime/cache/projection state and must not edit
tracked `.atelier/` canonical records. Hidden slow-command telemetry summarizes
local command performance for investigations; it is an advanced diagnostic, not
a normal mission or issue workflow step. Its JSON is stable for diagnostic
tooling and local analysis, but normal recipes must not parse it to decide
ready work, validation status, blockers, evidence coverage, or completion
status.

Installed-binary drift is distinct from malformed records. Normal tracker work
uses the installed `atelier` command. When a command reports that canonical
records use a schema the binary does not understand, operators should rebuild
and use `target/debug/atelier` for local CLI changes or update the installed
binary before continuing. Use `cargo run -- ...` only when a one-off rebuild
plus execution is specifically useful. When canonical Markdown is malformed,
the repair path remains `atelier lint`, editing the named record, and rerunning
`atelier lint`; ordinary record syntax errors are not stale-binary drift.
Stale projection and invalid canonical-record diagnostics should present one
ordered recovery path, preserve the original blocked command, and keep the
specific record path or parser detail visible so operators do not cycle through
export, rebuild, lint, and read commands blindly.

## Removed Behavior

The inherited command layer has been classified and removed from the public
command surface. The default classification for an inherited or duplicate
surface is `delete` unless it is in the core list above.

## Compatibility Classification

| Shape | Disposition | Reason | Replacement or boundary |
| --- | --- | --- | --- |
| Evidence add predecessor | Remove | Splits manual proof capture away from the unified evidence workflow. | `atelier evidence record --target ... "summary"` |
| Evidence capture predecessor | Remove | Splits transcript capture away from the same proof workflow. | `atelier evidence record --target ... -- <command>` |
| `atelier evidence attach` | Keep with distinct purpose | Needed only when an existing evidence record is being mirrored or reused on another accountable target. | Do not teach as the normal first proof step. |
| Export check diagnostic | Low-level diagnostic | Cache/projection state should be transparent; normal health and completion routes use `lint`, issue status <objective-id>/audit, and issue transition readiness. | Hidden/advanced diagnostic only; do not teach as a normal handoff command. |
| Export diagnostic | Low-level diagnostic | Deterministic repair/render mechanics are implementation details. | Hidden/advanced diagnostic only; `doctor --fix` owns admin explicit local repair. |
| Rebuild diagnostic | Low-level diagnostic | Projection rebuild is cache repair, not a product workflow. | Hidden/advanced diagnostic only; `doctor --fix` owns admin explicit local repair. |
| Hidden `issue quick/subissue/search/relate/tree/tested` helpers | Remove | Replacement commands are clear enough; hidden callable aliases are rediscovery risk. | Public workflows use issue record commands, root `search`, issue detail/status, record-specific notes, `evidence`, and `status`. |
| Root `abandon`, root `repair`, root `start`, hidden work-status helper, and any legacy work-start path | Remove or replace | Current work is the canonical `in_progress` issue set in the checkout, so hidden active-pointer cleanup is not a target-state workflow concept. Duplicate lifecycle paths obscure the workflow-backed status and issue-transition surfaces. | Docs and help teach status, issue detail, transition options, and issue status <objective-id> as next-step sources. |
| `mission` root namespace | Remove | Mission objectives are issue records, so a parallel root namespace duplicates lifecycle, linking, and status concepts. | `issue create --issue-type mission`, `issue show <objective-id>`, `issue status <objective-id>`, `issue link <objective-id> <issue-id> --role advances`, and `issue transition <objective-id> close --reason "..."`. |
| Flat issue aliases such as `create`, `show`, `list`, `ready`, `close`, `update`, `block`, `unblock`, `relate`, `related`, and `tree` | Remove | Duplicate verbs make the command surface harder to learn and easier to misroute. | `issue` owns issue lifecycle, blockers, links, detail, and objective status. |
| Generic link root | Remove | Relationship ownership belongs to record-specific commands, and the generic surface misrepresents mission support. | Mission work links use `issue link/unlink`; issue blockers and links use issue commands; evidence uses evidence commands; issue detail/status inspect impact. Attempts to run the removed root command fail with corrective guidance naming those record-specific homes. |
| Backup `import` plus `export --format json|markdown` | Remove | Backup-oriented predecessor formats are not the target durable contract. | `init --import-beads` and temporarily hidden/manual `import-beads` for migration. |
| `cascade` and `falsify` | Remove | Relationship-specific verbs hide the broader graph model and encourage one-off command paths. | Issue detail/status, record-specific note commands, and lifecycle commands. |

Removed command surfaces:

- The `mission` root namespace; use type-aware `atelier issue ...` commands for
  mission objectives.
- The normal `work start` and `work status` group; use status, issue workflow
  detail, and transition output instead.
- Flat issue aliases such as `create`, `show`, `list`, `ready`, `close`,
  `update`, `block`, `unblock`, `search`, `relate`, `related`, and `tree`; use
  `atelier issue ...`.
- Flat timer aliases such as `stop`, and the `timer` group.
- Legacy groups `archive`, `milestone`, `daemon`, `cpitd`, `usage`,
  `agent`, `locks`, and `sync`.
- Backup `import` and `export --format json|markdown`; use
  `init --import-beads` for the standard repo-local predecessor import.
  Standalone `import-beads` remains a hidden transitional command for explicit
  backup paths until migration cleanup removes it.
- The retired graph impact and graph tree forms; use `issue show <id>` for
  downstream impact and `issue status <objective-id>` for objective work
  hierarchy.

When a removed or commonly retried command is rejected, the CLI lets Clap report
the path as unknown or invalid without a compatibility guidance shim. Examples
include `finish`, `current-work`, `issue new`, `work start`, `archive`,
and `timer`. `workflow check` remains a hidden low-level diagnostic;
unsupported forms such as JSON mode fail as ordinary invalid arguments instead
of teaching a legacy workflow path.

## Low-Level Debug And Repair

The implementation may retain low-level export/rebuild commands, raw workflow
diagnostics, import helpers, and command diagnostics for development,
diagnostics, migration, and test evidence. They are not part of the normal
product workflow or root help. Public orientation commands such as
`atelier status`, `atelier lint`, and record-specific repair guidance should
absorb routine cache recovery, naming admin repair only when local state is
degraded.

Review commands are visible only for review artifacts. They may create,
inspect, merge or confirm merge state for, comment on, approve, request
changes, and resolve review artifacts owned by the configured review mode.
Static docs should not tell agents which review mode, provider, artifact shape,
or role applies to a work item. Atelier configuration and command output own
that routing. Mutating review commands must print whether their role was
explicit or inferred from status. Review commands must not start, close, or
otherwise transition Atelier issues.

Workflow transitions may run a configured review artifact open action.
That action is reached through the explicit issue transition that printed it,
not by treating `atelier review` as workflow authority. Review commands remain
the artifact inspection and artifact-action surface; transition/status output
remains the workflow routing surface.

Workflow validators read review state and return actionable pass/fail guidance
through transition/status surfaces. The local facts Atelier owns are the active
review field, configured review identity, expected source and target branches
when workflow policy needs them, merged state, review-complete state when a
configured transition asks for it, and unresolved review-comment counts for
operator guidance. The configured review system remains the authority for its
own external protection, approval, merge-method, and permission details.

## Integration Or Experimental

Integration experiments must not define the product's default mental model.
The Claude Code integration surface is removed rather than supported as an
Atelier product feature. Future integrations should return as explicit product
proposals with their own operator value, not as hidden command groups.

Generic replacements should use domain language. `atelier search <query>` owns
text search; `atelier issue show <id>` owns issue-scoped downstream impact; and
`atelier issue status <objective-id>` owns objective hierarchy and work health.
Impact follows mission work links, issue hierarchy, blocking relationships, and
any explicitly impact-bearing relationship types that remain after the generic
link root is removed. The inherited `cascade` and `falsify` commands are removed
so reassessment stays an explicit operator action through issue detail/status,
record-specific notes, and lifecycle commands instead of an assumption-specific
command path.

## Operator Command Map

Use the record kind to choose the command family first. Do not start from a
generic relationship verb.

| Need | Record kinds accepted | Supported command path | Boundary |
| --- | --- | --- | --- |
| Show mission intent, linked work, blockers, evidence, and completion state | mission ID | `atelier issue show <mission-id>` or `atelier issue status <mission-id>` | Mission reads own mission coordination. It does not replace issue detail or proof records. |
| Show issue accountability, status, blockers, notes, and completion status | issue ID | `atelier issue show <issue-id>` or `atelier issue transition <issue-id> --options` | Issue commands accept issue IDs. Passing a mission or evidence ID should produce wrong-kind guidance to the matching show surface. Historical plan or milestone IDs are deferred records and should not be accepted as v1 issue targets. |
| Add or remove mission work | mission ID plus issue or epic ID | `atelier issue link <mission-id> <issue-id> --role advances` and `atelier issue unlink <mission-id> <issue-id>` | Mission work links use the `advances` relation. Do not use a generic link command. |
| Add or inspect blockers | issue IDs for issue blockers; mission ID plus issue ID for mission blockers | `atelier issue block <blocked-id> <blocker-id>`, `atelier issue unblock <blocked-id> <blocker-id>`, `atelier issue blocked [<id>]`, or `atelier issue block <mission-id> <issue-id>` | Issue blockers and mission blockers are different relationships. Do not use top-level dependency commands. |
| Record new proof | issue target, normally `issue/<id>` | `atelier evidence record --target issue/<id> --kind validation "summary"` or `atelier evidence record --target issue/<id> --kind test -- <command>` | New proof starts with `evidence record`. Direct mission targets are reserved for legacy imports or explicit completion mirroring. |
| Reuse existing proof on another target | evidence ID plus issue target | `atelier evidence attach <evidence-id> issue <issue-id> --role validates` | Attachment reuses an existing evidence record. Evidence kind stays in `--kind`, while the relation role is `validates`. |
| Inspect issue impact or objective hierarchy | issue ID or objective issue ID | `atelier issue show <issue-id>` and `atelier issue status <objective-id>` | Issue views inspect relationships. They do not create mission work links, blockers, notes, or evidence. |
| Add durable handoff context | issue or mission ID | `atelier issue note <issue-id> "..."` or `atelier issue note <mission-id> "..."` | Notes are contextual activity. They are not a substitute for required evidence on completion claims. |
| Reference execution plans | repository Markdown path or prose inside a mission, epic, issue, or evidence record | edit the accountable Markdown record or attach evidence that names the plan path | Plans are ordinary Markdown artifacts in v1. They are not `.atelier/plans/` records and do not replace issue blockers or mission work links. |
| Preview or apply a one-shot graph bundle | bundle file path | `atelier bundle preview <file>` and `atelier bundle apply <file>` | Bundle apply requires its explicit confirmation flag. Bundles create graph deltas from a temporary file and stop being relevant after canonical records are written. |
| Inspect or repair epic review branches | epic or issue ID | `atelier branch status`, `atelier branch for-epic <epic-id>`, and `atelier branch merge <epic-id>` | Branch helpers are advanced Git lifecycle surfaces. Routine workers follow `atelier status`, issue detail, and transition output for lifecycle routing. |
| Inspect first-class evidence records | evidence ID | `atelier evidence show <evidence-id>` | Evidence records are supporting artifacts. Issue commands should reject their IDs with corrective wrong-kind guidance. Plan and milestone records are deferred v1 concepts. |

Mission-vs-issue example:

```text
atelier issue link atelier-hy2i atelier-4p7q
atelier issue block atelier-isd5 atelier-a625
atelier evidence record --target issue/atelier-isd5 --kind validation "operator command map checked against current help"
atelier issue status atelier-4p7q
atelier issue note atelier-isd5 "CLI surface examples checked against root help."
```

If an operator retries removed names such as workflow check/init, finish,
archive, timer, current-work, issue new, top-level dep, generic link,
import-beads, export, rebuild, or integrations during normal work, the supported
path is to stop and choose the record-specific command above. Low-level export,
rebuild, predecessor import, and workflow diagnostics may still exist for
development, migration, or targeted diagnostics, but they are not the normal
operator route for mission progress, proof, blockers, or completion.

## Canonical And Projection Recovery

Tracked Markdown under `.atelier/` is the durable source of truth. The local
SQLite projection, runtime state, locks, diagnostics, and cache files are
rebuildable checkout state. A command that writes canonical Markdown has landed
durable state when the tracked Markdown diff exists and `atelier lint` accepts
the record; a stale projection can block reads without invalidating the durable
write.

Use this recovery order:

| Symptom | First command | Repair path |
| --- | --- | --- |
| Unsure whether committed tracker records are valid | `atelier lint` | Edit the named `.atelier/` Markdown or workflow config, then rerun `atelier lint`. |
| Operator-facing command reports stale or missing derived state | Re-run the same command once after the automatic refresh path, then follow its named repair guidance if it still reports degraded local state. | Use `atelier doctor --fix` for ignored runtime/cache/projection repair. It must not edit tracked canonical records. |
| Canonical Markdown parse or schema error | `atelier lint <id-or-path>` | Fix the named tracked file. Do not treat parser failures as cache problems. |
| Checkout context is unclear after interrupted cleanup | `atelier status` then `git status --short --branch` | Reconcile canonical issue statuses through normal issue transitions or record edits. There is no separate active-pointer repair path in the target workflow. |
| Workspace isolation is needed for a risky or conflicting slice | Plain Git checkout/worktree commands outside Atelier | Keep the durable issue state in canonical `.atelier/` records and use `atelier status`/`issue status <objective-id>` inside the checkout. |
| Installed binary does not understand committed record shape | `cargo build` then `target/debug/atelier <command>` for local CLI changes | Update or rebuild the binary before diagnosing canonical records. |

Export and rebuild diagnostics are advanced implementation tools. Normal
handoff and completion should cite `atelier lint`, `atelier status`, `atelier
issue status <objective-id>`, and the specific command that was retried after repair.

## Removed Or Deferred Behavior

The daemon surface and changelog-on-close behavior are not part of the target
public workflow. Issue closure records close state, close time, and optional
reason in tracker state; it does not mutate `CHANGELOG.md`.

Canonical state is Markdown under tracked `.atelier/` records and checked with
lint and health gates. Local indexes are repaired transparently by normal
commands or explicitly by `doctor --fix`. `atelier export` and `atelier
rebuild` remain low-level diagnostics only; backup JSON/Markdown formats are no
longer command surfaces.
