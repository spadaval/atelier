# Command Surface Capability Inventory

This inventory reframes the CLI cleanup around capabilities rather than
commands. A command survives only when its capability cannot reasonably live in
another command.

This document does not decide whether Atelier needs a selector DSL. It records
where capability would go after cutting command surfaces, what syntax might be
enough, and where capability loss is a real risk.

## Decision Lens

For each capability:

- Preserve the user job, not the old command.
- Prefer one strong owner over several partial surfaces.
- Treat flags, modes, and selectors as implementation choices after ownership
  is clear.
- Do not replace command sprawl with unbounded flag sprawl.
- Do not remove a command until its real capability has a surviving home.

## Agent Factory Operating Jobs

This inventory also uses Agent Factory as evidence for what agents actually try
to do. Agent Factory is not supposed to be a repository command manual; it
routes agents to Atelier's executable surfaces for tactical truth. That makes
the command surface responsible for preserving these jobs clearly.

Source: `.agents/skills/agent-factory/SKILL.md` and its procedure references.

Agent Factory's recurring jobs are:

- orient in the repository and find the tracker;
- inspect current work, objective scope, readiness, blockers, and checkout
  state;
- create, split, sequence, and repair durable work;
- assign implementation, migration, docs, audit, review, and validation slices;
- inspect an assigned issue before editing;
- prove work with reproducible evidence;
- review diffs and proof claims;
- validate scenario or completion claims;
- perform demolition/reconnect migrations without preserving obsolete paths;
- report handoff state, dirty state, evidence IDs, blockers, and follow-up
  recommendations.

Those jobs imply capabilities that must survive the command cuts. They do not
require preserving the current command names.

Current Agent Factory guidance still names some surfaces proposed for removal,
especially `atelier man <role>` and focused `atelier issue status
<objective-id>`. If those surfaces are removed or folded, Agent Factory
guidance must be updated in the same migration slice so it routes agents to the
new surviving surfaces.

| Agent Factory job | Required CLI capability | Proposed owner |
| --- | --- | --- |
| Find current repository/tracker state | checkout orientation and health | `status`, health command |
| Choose and scope work | issue inventory, objective detail, blocker visibility | `work queue`, `issue show` |
| Plan missions/epics/issues | create typed work and relationships | `issue create`, `issue link`, `bundle` |
| Delegate implementation | inspect issue contract, readiness, branch/worktree state, expected proof | `issue show`, `issue transition`, `status` |
| Execute implementation | start/close workflow, leave notes, record proof | `issue transition`, `issue note`, `evidence record` |
| Migrate/demolish old paths | identify stale surfaces, prove rejection/removal, avoid shims | health command, `issue show`, tests/docs |
| Review | inspect diff context, issue proof expectations, review artifact state | `issue show`, `review show`, `issue transition` |
| Validate | inspect claim, run proof, record evidence, classify failures | `issue show`, `evidence record`, health command |
| Docs cleanup | find source of truth, prove help/docs consistency | health command, command help, docs |
| Audit/readiness | assess operability from executable surfaces | `status`, `work queue`, `issue show`, health command |

## Inventory

| Capability | Current surface(s) | Proposed owner | Possible syntax | Risk | Notes |
| --- | --- | --- | --- | --- | --- |
| Initialize tracker state | `init`, hidden/import migration surfaces | `init` | `atelier init`, `atelier init --import-beads` while migration remains | Low | Keep setup distinct. Remove standalone migration commands when possible. |
| Orient in current checkout | `status`, parts of `doctor`, parts of branch/status-like outputs | `status` | `atelier status` | Low | Must stay compact: active work, ready count, health/freshness, next commands. |
| Validate committed tracker records | `lint`, parts of `doctor`, hidden `workflow check` | one health command | `atelier check [<id>]` or temporary `atelier lint [<id>]` | Medium | Need one public health surface. Avoid separate lint/doctor/workflow normal paths. |
| Repair ignored runtime/projection state | `doctor --fix`, `rebuild` | health command | `atelier check --fix` | Medium | Must not edit tracked canonical Markdown. |
| Inspect raw workflow policy diagnostics | hidden `workflow check`, `lint` errors | health command or hidden dev command | `atelier check --workflow` if user-facing; otherwise hidden | Low | Keep out of normal workflow unless it explains a real recovery path. |
| Inspect slow command telemetry | hidden `diagnostics slow` | hidden dev/admin surface | hidden only | Low | Not normal product workflow. |
| Create ordinary issue work | `issue create` | `issue create` | `atelier issue create "..."` | Low | Core mutation. |
| Create mission/objective record | `issue create --issue-type mission`, old/retired mission create docs | `issue create` | `atelier issue create "..." --type mission` | Low | Mission is an issue type, not a namespace. |
| Create epic/typed work | `issue create --issue-type ...` | `issue create` | `atelier issue create "..." --type epic` | Low | Use configured issue types. |
| Bulk create or apply authored work graph | `bundle preview`, `bundle apply` | `bundle` for now | `bundle preview <file>`, `bundle apply <file>` | Medium | Capability is real. Root `bundle` is imperfect, but `issue apply` is not clearly better. Keep it bounded until a better owner is proven. |
| List global work queue | `work queue` | `work queue` | `atelier work queue` | Low | Core read surface. |
| List ready work | `work queue --ready`, `status` signpost, mission selectable work | `work queue` | `atelier work queue --ready` | Low | Global ready work stays simple. |
| List blocked work | `issue show`, `work queue --blocked`, queue footers | `work queue` | `atelier work queue --blocked` | Low | Remove `issue show` once `issue show` handles detail. |
| Filter by workflow status | `work queue --status`, `issue table --status` | `work queue` | `atelier work queue --status <status>` | Low | Workflow status is a field/filter, not a separate command. |
| Filter by issue type | `issue table --kind mission`, `issue table --issue-type`, ad hoc mission list | `work queue` | `atelier work queue --type mission` | Low | Replace mission inventory/table. |
| Filter by label/priority | `work queue --label`, `work queue --priority` | `work queue` | existing simple flags | Low | Keep simple metadata filters. |
| Search issue text when ID is unknown | root `search`, maybe work queue/search behavior | none in this cut | none | Medium | Remove search entirely for now. Do not create `work queue --query` as a quieter replacement. A future search design must prove a stronger cross-record job. |
| Show issue record detail | `issue show` | `issue show` | `atelier issue show <id>` | Low | Core durable record view. |
| Show issue workflow status field | `issue show`, `work queue`, `issue transition`, misleading `issue status` name | `issue show` and `work queue` | displayed as `Status:` in show/list | Low | The status field is valuable; a separate `issue status` command is not. |
| Show issue linkers and blocking relationships | `issue show`, `issue show <id>`, `issue status`, `mission status` | `issue show` | `atelier issue show <id>` | Medium | Detail must include enough blocker meaning to remove `issue show`. |
| Show downstream impact | `issue show`, retired graph impact | `issue show` | `atelier issue show <id>` | Low | Keep as bounded section. |
| Show parent/children hierarchy | `issue show`, `issue table`, retired graph tree | `issue show` | `atelier issue show <id>` | Low | Hierarchy belongs to record detail. |
| Show linked issues/relationships | `issue show`, `issue link/list internals`, retired graph views | `issue show` | `atelier issue show <id>` | Low | Detail should be bounded and readable. |
| Show objective rollup for missions/epics | `mission status`, `issue status`, `issue table`, `issue show` partial detail | `issue show` | `atelier issue show <objective-id>` | High | Must preserve linked-work semantics: objective scope comes from relationship links, not only hierarchy. |
| Show objective linked work roots | `mission status`, `issue show` links | `issue show` | Objective section in `issue show` | Medium | Show direct linked roots separately from descendants so scope is explainable. |
| Show objective work counts | `mission status`, `issue status`, `issue table` | `issue show` | Objective Progress section | Medium | Counts should cover linked work roots and descendant work under roots. |
| Show bounded ready work inside objective | `mission status`, `issue status` | `issue show` | bounded Objective Progress sample | Medium | Avoid new scoped query language unless bounded sample is insufficient. |
| Show bounded blocked work inside objective | `mission status`, `issue status`, `issue show` | `issue show` | bounded Objective Progress sample | Medium | Must show blocker titles/counts, not just IDs. |
| Select all work inside objective for bulk action | no clean current owner; mission status prints samples | undecided | maybe not supported; maybe future selector | High | This is where a selector DSL might become justified. Do not add ad hoc `--within` casually. |
| Inspect objective terminal readiness | `mission status`, `issue status`, `issue transition close transition options` | `issue transition` plus summary in `issue show` | `atelier issue transition <objective-id> close` | Medium | Readiness gates belong to transition; show may summarize. |
| Show available workflow transitions | `issue transition`, `issue show` summary | `issue transition` | `atelier issue transition <id>` with no transition name | Medium | Long term, remove awkward `transition options`. |
| Execute workflow transition | `issue transition <id> <transition>` | `issue transition` | unchanged | Low | Core lifecycle surface. |
| Start work and prepare branch | `issue transition start`, `branch for-epic` | `issue transition` | `atelier issue transition <id> start` | Medium | Branch command should become recovery/admin only if needed. |
| Close work and enforce gates | `issue transition close`, `mission close` retired docs, `mission status` guidance | `issue transition` | `atelier issue transition <id> close --reason "..."` | Low | One lifecycle path. |
| Manually inspect branch policy/state | `branch status`, `issue show` checkout summary, `issue transition` branch context, `status` | `status` / `issue transition`; hidden admin for repair | no normal public command unless needed | Medium | Preserve recovery capability without public branch namespace if possible. |
| Manually merge/recover owner branch | `branch merge` | hidden/admin or workflow transition | hidden/admin recovery command, or transition-owned integration | High | If workflow cannot safely own this, keep an admin escape hatch but hide from normal help. |
| Add issue note/activity | `issue note`, old root note docs | `issue note` or `issue update --note` | likely keep `issue note <id> "..."` | Low | This may be first-class enough to keep. |
| Inspect recent issue activity | `issue show` recent activity, `history --issue` | `issue show` | default bounded recent activity | Low | Default issue detail should not dump raw timelines. |
| Inspect high-level project history | root `history` | `history` | `atelier history` | Medium | Keep a high-level timeline view. It should answer broad recent activity questions without becoming search or a scoped query language. |
| Inspect full issue history | root `history --issue`, `issue show` recent activity | `issue show` only if needed; otherwise remove | maybe `issue show <id> --history` | Medium | Specific history belongs with the record owner only when it earns its keep. Do not proliferate scoped history variants. |
| Inspect mission/epic history including scoped work | `history --mission`, `history --epic` | likely remove or fold into objective detail | maybe objective section in `issue show` | High | Preserve objective scope where it matters for status/readiness, but do not keep mission/epic history commands by default. |
| Filter history by event kind/actor/since/limit | root `history` flags | high-level `history` only | bounded flags on `history` | Medium | Keep only filters needed for a high-level timeline. Specific record filters should not become a second query system. |
| Record manual evidence | `evidence record` | `evidence record` | unchanged | Low | Core proof surface. |
| Capture command-backed evidence | `evidence record -- <command>` | `evidence record` | unchanged | Low | Core proof surface. |
| Show evidence record | `evidence show` | `evidence show` | unchanged | Low | Evidence is a first-class record. |
| List evidence records | `evidence list` | `evidence list` | bounded list with filters | Low | Needs default limit; capability is distinct from work queue. |
| Attach existing evidence to issue | `evidence attach`, maybe `issue link` | relationship owner | `issue link <issue> <evidence> --role validates` or keep attach if cross-kind link is not ergonomic | Medium | Capability is real; separate attach verb may be unnecessary. |
| Show evidence for an issue | `issue show`, `evidence list`, `history` | `issue show` plus evidence list filter if needed | `issue show <id>`; maybe `evidence list --target issue/<id>` | Medium | Avoid overloading issue show with huge evidence transcripts. |
| Open review artifact | `review open` | `review open` | infer title/body/branches from issue when possible | Medium | Current command exposes too much provider plumbing. |
| Link existing review artifact | `review link` | `review open` or `review attach` style | `review open --existing <url>` | Low | Fold into open if possible. |
| Show review status/detail | `review status`, `review show`, issue/history references | `review show` | `review show [--issue <id>]` | Low | Remove separate status if show can be concise by default. |
| Show review comments/findings | `review comments`, `review show`, provider UI | `review show` | `review show --comments [--unresolved]` | Medium | Preserve unresolved filtering. |
| Add review comment/finding | `review comment` | `review submit` | `review submit --comment "..." [--finding]` | Medium | Collapse submit-like actions. |
| Approve/request changes | `review approve`, `review request-changes` | `review submit` | `review submit --approve`, `review submit --request-changes` | Low | Avoid one command per review outcome. |
| Resolve review finding | `review resolve` | `review resolve` | unchanged | Low | Specific mutation may earn its keep. |
| Merge review artifact | `review merge`, workflow close actions | `review merge` or workflow transition | undecided | Medium | Keep only if merging review artifact is separate from Atelier workflow transition. |
| Configure/check review provider | `forgejo roles check/provision` | `review provider ...` or admin docs | `review provider check/provision` | Medium | Provider-specific root commands should go away. |
| Prune supported artifacts | `prune`, `prune --apply` | `prune` | unchanged | Low | Dry-run/apply is a coherent admin surface. |
| Delete arbitrary record | `maintenance delete` | hidden/admin escape hatch or remove | no normal public command | Medium | Public destructive surgery is suspect. |
| Import predecessor data | `init --import-beads`, hidden `import-beads` | `init` | `init --import-beads` while needed | Low | Remove standalone import path. |
| Render/export canonical state for diagnostics | hidden `export` | hidden test/dev path or health command | no normal command | Low | Not workflow. |
| Rebuild projection cache | hidden `rebuild`, `doctor --fix` | health command | `check --fix` | Low | Not a separate user command. |
| Role-specific operating guidance | `man`, root help, docs, status next actions | `man` | `atelier man [role]` | Low | Keep as slightly smart guidance. It should route to surviving commands and not become a duplicate workflow engine. |

## Capability Gaps To Resolve Before Cutting

These are the areas where deleting commands without a replacement could remove
real capability.

### Objective Scope

Mission work is linked through issue relationships, not normal hierarchy.
Generic objective rollup must therefore preserve this behavior:

- start from objective links, especially `advances`;
- include directly linked work roots;
- include descendants of linked work roots when hierarchy applies;
- deduplicate reachable issues;
- explain why each group is in scope.

If `issue show <objective-id>` only follows hierarchy, it is not an adequate
replacement for mission status.

### Full Scoped Work Lists

Bounded objective rollup in `issue show` may be enough for orientation, but it
may not replace a full selectable list of objective-scoped work.

Options before adding a DSL:

- keep only bounded rollup and rely on global `work queue --ready`;
- add a single explicit non-query mode to `issue show`, such as
  `issue show <id> --work`;
- add one selector expression mechanism later;
- keep a focused command only if no simpler owner works.

Do not add many scoped flags piecemeal.

### History Boundaries

Root `history` stays as a high-level timeline view. The risk is not that history
exists; the risk is scoped variants spreading across every record type.

Implementation should decide:

- which high-level filters are necessary for project timeline browsing;
- whether issue-specific history earns a focused `issue show` mode;
- which mission/epic scoped history modes should simply be removed.

### Bulk Graph Apply

`bundle` is not the ideal root noun, but it stays for now. Bulk graph authoring
is a real capability, and `issue apply` is not obviously better.

The retained surface must stay bounded:

- non-mutating preview;
- explicit apply;
- clear output that can be reviewed before mutation;
- no expansion into a general import/export namespace.

### Branch Recovery

If workflow transitions can own all normal branch preparation and integration,
the visible `branch` namespace can go away.

If manual recovery is still required, keep it hidden/admin and route to it only
from failed transitions.

### Health Command Rename

Moving from `lint`/`doctor` to `check` is product-clean, but the migration
should avoid losing:

- record validation;
- workflow-policy diagnostics;
- runtime/projection health;
- safe local repair.

## Selector DSL Question

A selector DSL is not the decision yet. The real decision is how to preserve
selection capability without command proliferation.

A DSL becomes plausible only if simple list filters and objective rollup cannot
cover repeated user jobs without a growing pile of flags.

Signs a selector may be justified:

- users need composable saved views;
- objective-scoped selection needs more than bounded `show` output;
- global issue selection needs combinations that do not fit a small fixed
  filter set;
- filters start turning into ad hoc boolean logic.

Signs a selector is premature:

- command ownership is still unclear;
- issue/objective/show/transition boundaries are still moving;
- the selector would sit beside many old commands instead of replacing them.

Do not use a DSL to avoid making command ownership decisions. Use it only if it
replaces multiple command/filter surfaces with one clearer selection model.

## Proposed Cut Order

1. Define objective scope traversal as a shared capability.
2. Move mission/objective rollup into `issue show <objective-id>`.
3. Remove `mission status` and `issue status` only after the rollup is in
   `issue show`.
4. Remove `issue show` only after blocker detail in `issue show` is adequate.
5. Remove `issue table` after `work queue --type mission` and list formatting
   cover mission inventory.
6. Remove root `search` entirely rather than replacing it with `work queue
   --query`.
7. Keep high-level `history`, and remove or fold specific scoped history
   variants only where they earn a focused owner.
8. Keep `bundle` as the graph preview/apply owner for now.
9. Collapse review submit/status/comment commands.
10. Merge health surfaces.
11. Keep `man` as smart guidance and update it to stop teaching deleted
    commands.
12. Hide or remove branch/provider/admin escape hatches from normal help.
