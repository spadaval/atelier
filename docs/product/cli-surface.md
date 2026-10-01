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
use `atelier status`, `atelier work`, `atelier issue show <id>`, and
`atelier issue transition <id>` output to decide what to do next in the current
checkout.

The command audit records the target replacement forms before every old command
path has been removed. This file names the product contract the CLI is moving
toward; implementation remnants must either be hidden/admin-only or deleted
without compatibility aliases once their replacement owner is in place.

## Workflow-First Core

Workflow-first commands are stable enough to appear in normal help and are the
repo-owned surfaces Agent Factory may route operators toward. The command
output, not this static list, decides the next process step for a concrete issue
or mission:

- `atelier init`
- `atelier status`
- `atelier work ...`
- `atelier issue ...`
- `atelier check [id]`
- `atelier issue create "..." --issue-type mission`
- `atelier issue show <objective-id>`
- `atelier issue link/unlink <objective-id> <issue-id> --role advances`
- `atelier bundle preview/apply`
- `atelier evidence record/show/list`
- `atelier issue note <id> "..."`
- `atelier check`
- `atelier prune`

The target normal surface deliberately excludes the old mission namespace, the
old objective-status command, blocker/table shortcuts, root search, normal
branch repair, and
destructive maintenance. Those names are implementation debt during the
migration, not compatibility contracts.

## Command Categories

Atelier commands fall into four product categories. The category determines
where the command may appear, which operator should reach for it, and whether it
may be cited as ordinary workflow proof.

| Category | Definition | Examples | Excluded non-examples |
| --- | --- | --- | --- |
| Normal workflow | Product-facing commands used to orient, select work, inspect objective/detail state, mutate canonical records explicitly, record proof, preview/apply graph bundles, and check ordinary committed-state health. They may appear in root help, role guides, ready-work actions, and Agent Factory workflow guidance. | `status`, `work`, `issue show`, `issue transition <id>`, `evidence record`, `bundle preview`, `check` | Removed/deprecated surfaces, admin repair commands, and hidden diagnostics. |
| Admin maintenance | Specialized commands for setup, explicit pruning, or manual recovery when normal workflow output routes there. They may appear in admin guidance or targeted recovery output, but not as the default worker/reviewer loop. | `init`, `prune`, `prune --apply` | Removed objective-status and mission-status surfaces and hidden workflow diagnostics. |

Tracked Markdown under `.atelier/` is authoritative and is the only tracker
state. Every command parses the record files it needs; there is no cache. A
command fails, naming the file, when a record file it loads is invalid;
`atelier check` reports every invalid record file at once.

`atelier init` is core tracker setup only. It creates `.atelier/` records,
`.atelier/config.toml`, `.atelier/workflow.yaml`, local runtime storage, and
root ignore rules. It does not install editor or assistant hooks. Its default
next steps
must not route a fresh checkout directly to issue creation before
`atelier check` confirms the committed tracker and workflow configuration are
valid.

`atelier status` is the root checkout signpost. It summarizes the current-work
set, active objective context when visible from committed records, ready work
count, tracker freshness, and the next work or health drill-down commands. It
points operators to scoped `issue show`, `issue transition`, and `work`
dashboards for objective completion status.

`atelier --help` should expose the normal operator path first: orient, inspect
objectives and issues, inspect workflow options, manage blockers, record proof,
inspect history, and validate committed records. Raw workflow diagnostics are
not normal root-help entries; targeted error messages may name them when they are relevant. Arbitrary destructive record deletion is not an Atelier command;
supported cleanup is owned by `prune`, and canonical recovery uses Git history.

## Visible Help Contract

Every help-visible command group needs one job, one default information budget,
one quiet-output contract, and a named drill-down path. Quiet mode stays terse:
IDs, counts, paths, status tokens, and pass/fail tokens only.

### Workflow-First Surfaces

| Surface | Job | Default output | Quiet output | Drill-down path |
| --- | --- | --- | --- | --- |
| `init` | Create tracker scaffolding in a repo that does not have Atelier yet. | Created or reused paths plus workflow setup and verification commands before issue creation. | Created path(s) and a success token. | `check`, `status`, inspect `.atelier/config.toml` and `.atelier/workflow.yaml`. |
| `status` | Root orientation for the current checkout. | Current-work set, active objective context when visible, ready count, tracker freshness, and next work commands. It names admin repair only when local state is degraded. | IDs, counts, and freshness token only. | `work ready`, `issue show <id>`, and admin repair guidance only for degraded local state. |
| `work` | Show bounded operational multi-issue views. | Ready, blocked, active, scoped mission/epic dashboards, and the plural Mission Overview from the shared read pipeline. `work missions` is epic-first and collapsed; the legacy nested queue owns neither overview nor inventory. | The plural Mission Overview emits visible mission IDs only; other views emit their scoped bucket IDs. | `work mission <mission-id>`, `work epic <epic-id>`, `issue show <id>`, `issue transition <id>`, `work ready`, `work blocked`. |
| `issue` | Create, list, show, update, transition, note, and manage typed links. | `issue list` is a flat, all-status metadata inventory with one row per matching record; detail reads name the canonical Markdown path and next commands. Transition output owns lifecycle routing for the current issue. Objective health, blockers, linked work, and terminal-readiness summary belong in `issue show`; link mutations name the source, target, and role; note entry appends a dated bullet to the record's `## Notes` section. | Inventory IDs, status tokens, changed fields, relationship roles, and canonical paths. | `issue show <id>`, `issue note <id> "..."`, `issue transition <id>`, `work missions`, `work blocked`, edit the Markdown record. |
| `start` | Check out an issue's owner branch, record it on the owner, and apply the `start` transition. | Each Git operation run, the recorded branch and base, and the applied transition. | Same lines. | `issue show <id>`, `issue transition <id>`. |
| `bundle` | Preview and apply one-shot graph bundles from files. Use this for bulk mission, epic, issue, relationship, and evidence creation instead of shell loops over individual mutation commands. | `preview` prints deterministic non-mutating validation output; `apply` requires `--yes` and prints created IDs, relationship counts, and recovery guidance when needed. | Created IDs, counts, and pass/fail tokens. | `issue show <id>`, `issue show <objective-id> <id>`, `evidence show <id>`, `check`. |
| `evidence` | Record and inspect proof records. | `record` is the default proof-capture workflow; `show` and `list` inspect existing evidence; output names target, kind, result, and reusable IDs. | Evidence IDs, target IDs, result tokens, and stored command status only. | `evidence show <id>`, `issue show <id>`. |
### Specialized But Visible Surfaces

| Surface | Job | Default output | Quiet output | Drill-down path |
| --- | --- | --- | --- | --- |
| `prune` | Explicitly prune accumulated local, canonical, branch, and worktree artifacts according to [Retention And Prune Policy](retention-and-prune.md). | `atelier prune` requires an Atelier repository and is the dry run; it reports candidates, retention cutoffs, protected records, deferred cleanup classes, the apply command, and canonical Git-history recovery. `atelier prune --apply` removes only eligible classes whose class-specific safety checks pass. | Candidate counts, removed paths or IDs, protected/deferred counts, and failure tokens only. | Git history for removed canonical paths; Git inspection for branch/worktree state. |
| `check` | Validate canonical tracker records and committed workflow configuration, reporting every invalid record file. | Pass summary or every named record, workflow config, and file error. | Pass/fail token and offending IDs or paths only. | Edit the named record or workflow config and rerun `check`. |

Adjacent command placements that still need confirmation or cleanup are tracked
in the command audit:

- `prune`: visible admin maintenance; dry-run by default and applies only
  explicitly supported cleanup classes from
  [Retention And Prune Policy](retention-and-prune.md).
- `branch`: hidden advanced/manual owner-branch recovery. Routine branch guidance comes
  from status, issue detail, transition, and recovery output.
- `worktree`: removed visible workspace-management surface pending redesign.

Hidden advanced diagnostics probes are not visible root-help surfaces and must
not appear as normal mission, issue, blocker, evidence, or completion next
actions.

## Operator Jobs

The public command surface is organized around jobs an operator performs under
time pressure:

- Orient: answer what is active, ready, blocked, stale, or unsafe to change.
  Owned by `atelier status`, `atelier work ready`, bounded `atelier work`
  views, and `atelier issue show`; Git history of the record files is the
  audit trail.
- Select and run work: choose a clear slice, inspect workflow state, leave notes
  and proof, and follow the lifecycle or recovery command Atelier prints for
  the current item. Owned by `atelier status`, `atelier issue ...`, `atelier
  evidence ...`, and record-specific note commands.
  Explicit branch repair commands are advanced diagnostics or repair
  surfaces unless Atelier routes the operator there.
- Coordinate objective progress: see linked work by state, blockers, evidence
  gaps, completion status, and the next action for the objective. Target state
  is owned by `atelier issue show <objective-id>`, `atelier issue transition
  <objective-id>`, issue metadata mutation, and issue link/unlink commands.
  Missions are mission-typed issue records, not a separate root command
  namespace.
- Manage relationships: record issue linkers and inspect cross-record impact
  when the next action depends on graph shape. Owned by issue linker
  subcommands, issue link/unlink, issue detail/status, and evidence
  attachment.
- Check committed state: prove tracked records and workflow config are usable
  for handoff. Owned by `atelier check`.

Normal workflow commands speak in product terms: issue, mission, evidence,
blocker, proof, completion, and committed-state validity. Advanced
diagnostics may expose workflow policy names or raw validator
detail, but normal operators should only run them when an admin repair path,
targeted error, assignment, or completion contract names them. Supported cleanup
is constrained to `prune`; canonical record repair is reviewed and recoverable
through Git history rather than arbitrary CLI deletion.
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

Issue mutation commands write concrete Markdown records through `RecordStore`.
Query commands such as work buckets, issue inventory, issue detail, and lint
read the current record files on every invocation.
Issue creation and issue detail output print the canonical Markdown path under
`.atelier/issues/<id>.md` so large-field editing stays file-first. Human
footers point to editing that Markdown file, `atelier check <id>`, and focused
drill-down commands rather than generic command dumps.
`atelier issue list --status <status>` filters by exact configured workflow
status only, with `all` as the no-restriction default. Derived status
categories are a separate concept and use `atelier issue list --category
<category>`. Ready and blocked selection belongs to `atelier work ready` and
`atelier work blocked`, not inventory flags. The complete inventory and plural
Mission Overview contracts are in [Issue Inventory And Mission
Overview](issue-inventory-and-mission-overview.md).
Category values are the exact category names from `.atelier/workflow.yaml`,
such as `todo`, `active`, `blocked`, and `done`; status
tokens such as `in_progress` are not category aliases. `work queue` is removed
without an alias. Use `issue list` for metadata filters, `work ready`,
`work blocked`, `work active`, or `work all` for operational buckets, and
`work mission` or `work epic` for scoped work. Objective rollups use complete
app-layer scope facts; display limits apply after totals and recent work are
computed.
`atelier issue create` has one work-type decision. Use `--issue-type` for the
canonical type (`bug`, `completion`, `epic`, `feature`, `mission`, `spike`,
`task`, or `validation`) or use a template preset whose default type is
documented by the
template name (`bug` -> `bug`, `feature` -> `feature`, research/investigation ->
`spike`, audit -> `validation`, refactor/continuation -> `task`). Templates
also provide default labels, priorities, and body scaffolding, and those
defaults still apply when `--parent` or a matching explicit `--issue-type` is
supplied. Conflicting type choices fail with guidance rather than silently
creating a surprising record.
For mission creation, an unsectioned `--body` supplies Description prose. A
sectioned `--body` preserves authored `## Description`, `## Outcome`, optional
`## Evidence`, and optional `## Notes` content; when Description is omitted,
creation supplies the standard non-empty Description placeholder. A successful
create always writes a body that satisfies the canonical issue section
contract.

First-class mission, evidence, relationship, and work lifecycle
commands are now core as a staged implementation. Mission,
evidence, and issue lifecycle mutations write durable Markdown through
`RecordStore`; the next command reads them directly.
`atelier bundle preview <file>` is the manager and orchestrator path for bulk
graph creation. It validates authored bundle JSON from a real file path and
prints a non-mutating deterministic preview. Use it before `atelier bundle
apply <file> --yes` when creating an objective plus many epics, issues,
relationships, or evidence links; do not script repeated `issue create`,
`issue link`, and `issue link` loops for that shape of work. `atelier
bundle apply` applies create-only v1 bundle resources from a file path after
the operator passes the command's required confirmation flag, creates record
graphs in canonical Markdown, normalizes issue dependency fields, writes
durable relationship buckets, validates the complete staged canonical graph
before installation, and holds an exclusive canonical transaction across the
snapshot and two-directory rollback-capable install so concurrent ordinary
writers apply afterward instead of being overwritten. It reports
post-commit cleanup trouble as a committed-success warning with a retained,
ignored backup or stage path; later applies refuse either recovery artifact
until the live tree is checked and the named real directory is removed.
`atelier issue show <objective-id>` is the rich
objective detail read: it summarizes evidence, prose planning/checkpoint
references, and work grouped by ready, blocked, done, and backlog state.
`atelier issue transition <objective-id>` owns live validator failures,
completion status, and executable lifecycle next actions.

Root orientation and discovery belong to `atelier status`, `atelier work`, and
explicit issue browsing or inventory surfaces. Objective detail output is compact
and answers: objective identity and state, committed tracker health, work
counts, selectable work, blocked work, open blockers, missing proof required by
configured validators, completion status,
and one or two state-specific next actions. Selectable-work rows name the issue
transition command, parent context, and whether proof is already attached;
blocked-work rows name the blocked issue, the blocker IDs, parent context, and
proof state. Verbose output keeps the same sections but expands the record
lists, validator detail, blocked work, and degraded health details enough for a
handoff transcript.

State-specific next actions are part of the command contract:

- `draft`: show missing readiness fields and point to record editing,
  `atelier check <mission-id>`, or `atelier issue transition <id> ready`.
- `ready`: show ready work and the command to transition an issue into active
  work or inspect issue show <objective-id> explicitly.
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

`atelier issue transition <id>` is the terminal-check drill-down. It remains
advisory orientation unless a workflow explicitly requires linked validation
work; parent judgment that can block completion belongs to linked validation
work with attached evidence and workflow approval. Raw workflow validator names
are diagnostic detail; normal completion output names the operator-facing
blocker class and the next domain command.
`atelier check` owns committed workflow/config validity, `work mission
<objective-id>` owns scoped mission coordination, and `issue transition
<objective-id>` owns executable mission publish-readiness inspection;
removed policy-debug commands do not replace them. Fast docs/help drift guards for
`AGENTS.md`, product command docs, visible root help, and
obsolete command-test references belong in `atelier check` or an explicitly
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
validation proof does not require manual transcript copy/paste. Command-backed
capture first binds and validates the repository association and target; an
invalid repository or target fails before the child runs. It then releases the
canonical association transaction while the child executes, so the child may
itself invoke normal or administrative Atelier commands. After the child exits,
capture reacquires the same logical repository association, refreshes and
revalidates the same canonical target identity (kind, ID, schema/version,
creation timestamp, and record type), and only then appends the evidence and activity under an
exclusive postflight transaction. An association swap, missing target, or
delete-and-recreate reuse of the same ID fails closed without writing proof;
same-record title, status, body, and update-time mutations plus canonical
subtree imports remain valid when the immutable identity is unchanged.
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

Transitions write records and run read-only checks only; `atelier start` is the
explicit command that switches branches, and it prints each Git operation it
runs.

There is no target-state rule that one checkout may hold only one current issue.
Repeated starts of the same issue are harmless, and starting a second issue in
the same checkout should extend the current-work set when the workflow allows
it. `atelier status` and `atelier issue show <objective-id>` should render all
`in_progress` issues visible in the checkout rather than promote one hidden
"active issue". Use issue views to render all current work in the checkout, and
use the epic branch as the review boundary.

Different Git checkouts or branches may legitimately show different
current-work sets because each carries its own tracked `.atelier/` record copy.
Reconciliation happens through normal Git review and merge of canonical
Markdown records. Legacy root `abandon` and `repair` commands have been removed:
stopping work without a durable status change does not need a cleanup command,
and stale checkout recovery should use normal status inspection plus canonical
issue transitions rather than hidden active-pointer repair.
Root `atelier status`, `atelier work`, `atelier issue show <id>`, and
`atelier issue transition <id>` expose current-work orientation, so operators
should not need the removed work-status helper or any legacy work-start path for
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

## Binary Drift

Installed-binary drift is distinct from malformed records. Normal tracker work
uses the installed `atelier` command. When a command reports that canonical
records use a schema the binary does not understand, operators should rebuild
and use `target/debug/atelier` for local CLI changes or update the installed
binary before continuing. Use `cargo run -- ...` only when a one-off rebuild
plus execution is specifically useful. When canonical Markdown is malformed,
the repair path remains `atelier check`, editing the named record, and rerunning
`atelier check`; ordinary record syntax errors are not stale-binary drift.
Invalid record-file diagnostics keep the specific record path and parser detail
visible.

## Removed Behavior

The inherited command layer has been classified and removed from the public
command surface. The default classification for an inherited or duplicate
surface is `delete` unless it is in the core list above.

## Historical Compatibility Classification (Non-Normative)

| Shape | Disposition | Reason | Replacement or boundary |
| --- | --- | --- | --- |
| Evidence add predecessor | Remove | Splits manual proof capture away from the unified evidence workflow. | `atelier evidence record --target ... "summary"` |
| Evidence capture predecessor | Remove | Splits transcript capture away from the same proof workflow. | `atelier evidence record --target ... -- <command>` |
| `atelier evidence attach` | Keep with distinct purpose | Needed only when an existing evidence record is being mirrored or reused on another accountable target. | Do not teach as the normal first proof step. |
| Hidden `issue quick/subissue/search/relate/tree/tested` helpers | Remove | Replacement commands are clear enough; hidden callable aliases are rediscovery risk. | Public workflows use issue record commands, `work ready/blocked/active/all`, issue detail, record-specific notes, `evidence`, and `status`. |
| Root `abandon`, root `repair`, root `start`, hidden work-status helper, and any legacy work-start path | Remove or replace | Current work is the canonical `in_progress` issue set in the checkout, so hidden active-pointer cleanup is not a target-state workflow concept. Duplicate lifecycle paths obscure the workflow-backed status and issue-transition surfaces. | Docs and help teach status, issue detail, transition options, and work dashboards as next-step sources. |
| `mission` root namespace | Remove | Mission objectives are issue records, so a parallel root namespace duplicates lifecycle, linking, and status concepts. | `issue create --issue-type mission`, `issue show <objective-id>`, `work mission <objective-id>`, `issue link <objective-id> <issue-id> --role advances`, and `issue transition <objective-id> close --reason "..."`. |
| Flat issue aliases such as `create`, `show`, `list`, `ready`, `close`, `update`, `block`, `unblock`, `relate`, `related`, and `tree` | Remove | Duplicate verbs make the command surface harder to learn and easier to misroute. | `issue` owns issue lifecycle, blockers, links, detail, and objective status. |
| Generic link root | Remove | Relationship ownership belongs to record-specific commands, and the generic surface misrepresents mission support. | Mission work links use `issue link/unlink`; issue blockers and links use issue commands; evidence uses evidence commands; issue detail inspects impact. Attempts to run the removed root command fail with corrective guidance naming those record-specific homes. |
| Backup `import`, `import-beads`, and `export` | Remove | Predecessor and backup formats are not the target durable contract. | Tracked `.atelier/` records and Git history. |
| `cascade` and `falsify` | Remove | Relationship-specific verbs hide the broader graph model and encourage one-off command paths. | Issue detail/status, record-specific note commands, and lifecycle commands. |

Removed command surfaces:

- The `mission` root namespace; use type-aware `atelier issue ...` commands for
  mission objectives.
- The normal `work start` and `work status` group; use status, issue workflow
  detail, and transition output instead.
- Flat issue aliases such as `create`, `show`, `list`, `ready`, `close`,
  `update`, `block`, `unblock`, `relate`, `related`, and `tree`; use
  `atelier issue ...`.
- The legacy `work queue` dashboard; use metadata inventory or the focused
  operational work buckets.
- Flat timer aliases such as `stop`, and the `timer` group.
- Legacy groups `archive`, `milestone`, `daemon`, `cpitd`, `usage`,
  `agent`, `locks`, and `sync`.
- Backup `import`, `import-beads`, `init --import-beads`, and `export`.
- The retired graph impact and graph tree forms; use `issue show <id>` for
  downstream impact and `issue show <objective-id>` for objective work
  hierarchy.

When a removed or commonly retried command is rejected, the CLI lets Clap report
the path as unknown or invalid without a compatibility guidance shim. Examples
include `finish`, `current-work`, `issue new`, `work start`, `archive`,
and `timer`. Unsupported forms such as JSON mode fail as ordinary invalid arguments instead
of teaching a legacy workflow path.

## Low-Level Debug And Repair

The implementation may retain raw workflow diagnostics for development,
diagnostics, and test evidence. They are not part of the normal product
workflow or root help.

## Integration Or Experimental

Integration experiments must not define the product's default mental model.
The Claude Code integration surface is removed rather than supported as an
Atelier product feature. Future integrations should return as explicit product
proposals with their own operator value, not as hidden command groups.

Generic replacements should use domain language. Search is intentionally removed
from this cut; `atelier issue show <id>` owns issue-scoped downstream impact,
and `atelier issue show <objective-id>` owns objective hierarchy and work
health.
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
| Show mission intent, linked work, blockers, evidence, and completion state | mission ID | `atelier issue show <mission-id>` | Mission records are objective-shaped issues. Detail does not replace lifecycle transitions or proof records. |
| Show issue accountability, status, blockers, notes, and completion status | issue ID | `atelier issue show <issue-id>` or `atelier issue transition <issue-id>` | Issue commands accept issue IDs. Passing a mission or evidence ID should produce wrong-kind guidance to the matching show surface. Historical plan or milestone IDs are deferred records and should not be accepted as v1 issue targets. |
| Add or remove mission work | mission ID plus issue or epic ID | `atelier issue link <mission-id> <issue-id> --role advances` and `atelier issue unlink <mission-id> <issue-id>` | Mission work links use the `advances` relation. Do not use a generic link command. |
| Add or inspect blockers | issue IDs for issue linkers; mission ID plus issue ID for mission blockers | `atelier issue link <blocked-id> <blocker-id> --role blocked_by`, `atelier issue unlink <blocked-id> <blocker-id> --role blocked_by`, `atelier work blocked`, and `atelier issue show <id>` | Blocking is a relationship. Do not use top-level dependency commands. |
| Record new proof | issue target, normally `issue/<id>` | `atelier evidence record --target issue/<id> --kind validation "summary"` or `atelier evidence record --target issue/<id> --kind test -- <command>` | New proof starts with `evidence record`. Direct mission targets are reserved for legacy imports or explicit completion mirroring. |
| Reuse existing proof on another target | evidence ID plus issue target | `atelier evidence attach <evidence-id> issue <issue-id> --role validates` | Attachment reuses an existing evidence record. Evidence kind stays in `--kind`, while the relation role is `validates`. |
| Inspect issue impact or objective hierarchy | issue ID or objective issue ID | `atelier issue show <issue-id>` | Issue views inspect relationships. They do not create mission work links, blockers, notes, or evidence. |
| Add durable handoff context | issue or mission ID | `atelier issue note <issue-id> "..."` or `atelier issue note <mission-id> "..."` | Notes are contextual activity. They are not a substitute for required evidence on completion claims. |
| Reference execution plans | repository Markdown path or prose inside a mission, epic, issue, or evidence record | edit the accountable Markdown record or attach evidence that names the plan path | Plans are ordinary Markdown artifacts in v1. They are not `.atelier/plans/` records and do not replace issue linkers or mission work links. |
| Preview or apply a one-shot graph bundle | bundle file path | `atelier bundle preview <file>` and `atelier bundle apply <file>` | Bundle apply requires its explicit confirmation flag. Bundles create graph deltas from a temporary file and stop being relevant after canonical records are written. |
| Inspect or repair epic review branches | epic or issue ID | Hidden branch repair helpers outside normal root help | Branch helpers are advanced Git lifecycle surfaces. Routine workers follow `atelier status`, issue detail, and transition output for lifecycle routing. |
| Inspect first-class evidence records | evidence ID | `atelier evidence show <evidence-id>` | Evidence records are supporting artifacts. Issue commands should reject their IDs with corrective wrong-kind guidance. Plan and milestone records are deferred v1 concepts. |

Mission-vs-issue example:

```text
atelier issue link atelier-hy2i atelier-4p7q
atelier issue link atelier-isd5 atelier-a625
atelier evidence record --target issue/atelier-isd5 --kind validation "operator command map checked against current help"
atelier issue show atelier-4p7q
atelier issue note atelier-isd5 "CLI surface examples checked against root help."
```

If an operator retries removed names such as workflow check/init, finish,
archive, timer, current-work, issue new, top-level dep, generic link,
import-beads, export, rebuild, doctor, or integrations during normal work, the
supported path is to stop and choose the record-specific command above. Workflow
diagnostics may still exist for development or targeted
diagnostics, but they are not the normal operator route for mission progress,
proof, blockers, or completion.

## Record-File Recovery

Tracked Markdown under `.atelier/` is the durable source of truth. Ignored
runtime files under `.atelier/runtime/` hold only locks. A command that writes
canonical Markdown has landed durable state when the tracked Markdown diff
exists and `atelier check` accepts the record.

Use this recovery order:

| Symptom | First command | Repair path |
| --- | --- | --- |
| Unsure whether committed tracker records are valid | `atelier check` | Edit the named `.atelier/` Markdown or workflow config, then rerun `atelier check`. |
| A command fails naming an invalid record file | `atelier check` | Fix every named tracked file, then rerun the command. |
| Checkout context is unclear after interrupted cleanup | `atelier status` then `git status --short --branch` | Reconcile canonical issue statuses through normal issue transitions or record edits. There is no separate active-pointer repair path in the target workflow. |
| Workspace isolation is needed for a risky or conflicting slice | Plain Git checkout/worktree commands outside Atelier | Keep the durable issue state in canonical `.atelier/` records and use `atelier status`/`issue show <objective-id>` inside the checkout. |
| Installed binary does not understand committed record shape | `cargo build` then `target/debug/atelier <command>` for local CLI changes | Update or rebuild the binary before diagnosing canonical records. |

Normal handoff and completion should cite `atelier check`, `atelier status`, `atelier
issue show <objective-id>`, and the specific command that was retried after repair.

## Removed Or Deferred Behavior

The daemon surface and changelog-on-close behavior are not part of the target
public workflow. Issue closure records close state, close time, and optional
reason in tracker state; it does not mutate `CHANGELOG.md`.

Canonical state is Markdown under tracked `.atelier/` records and checked with
`atelier check`. Commands parse the record files directly; there is no local
index to repair.
