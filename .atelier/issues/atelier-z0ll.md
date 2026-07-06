---
created_at: "2026-06-24T19:25:36.629954615+00:00"
id: "atelier-z0ll"
issue_type: "mission"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-3llx"
    type: "advances"
  - kind: "issue"
    id: "atelier-3uew"
    type: "advances"
  - kind: "issue"
    id: "atelier-47cp"
    type: "advances"
  - kind: "issue"
    id: "atelier-55tk"
    type: "advances"
  - kind: "issue"
    id: "atelier-5km8"
    type: "advances"
  - kind: "issue"
    id: "atelier-82u0"
    type: "advances"
  - kind: "issue"
    id: "atelier-8c91"
    type: "advances"
  - kind: "issue"
    id: "atelier-fasv"
    type: "advances"
  - kind: "issue"
    id: "atelier-qcbx"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-25T01:54:42.695532638+00:00"
status: "closed"
title: "Rebuild command surface around work views and centralized read pipeline"
updated_at: "2026-06-25T01:54:42.695532638+00:00"
---

## Description

Replace the sprawling overlapping command surface with a smaller product model centered on checkout status, operational work views, single-record issue detail, explicit issue workflow transitions, first-class evidence, focused review operations, high-level history, bundle graph application, smart guidance, and one health/check surface. The mission should preserve real capabilities while deleting obsolete namespaces, bespoke status commands, root search, and implementation-shaped helper surfaces that do not earn their keep.

Constraints:
- Public command nouns stay small: status for checkout orientation, work for operational multi-issue views, issue for single-record detail and issue mutations, evidence for proof, review for review artifacts, and check for health.
- Do not preserve compatibility aliases, shims, deprecated command wrappers, or old-output contracts unless a human explicitly requests a compatibility window.
- Views are read-only. Mutations remain explicit commands such as issue create/update/link/transition/note, evidence record, and review submit/open/merge.
- Do not expose a global JSON output contract for every read view; canonical Markdown/YAML records remain the machine-readable API unless a deliberately versioned artifact is designed.
- The CLI crate must not own workflow truth, work selection truth, evidence gate truth, objective scope truth, relationship semantics, history scope, status next-action decisions, or bundle graph semantics. Move those into domain/app services and let CLI commands render typed view models.
- Keep root `bundle` for now. It is not ideal product vocabulary, but `issue apply` is also not clearly better, and bulk graph preview/apply is important enough to avoid churn until a better owner is proven.
- Keep root `history` as a high-level timeline view. More specific history belongs in focused commands where it genuinely helps, or should be removed instead of proliferating scoped history variants.
- Remove `search` entirely unless a future design proves a stronger cross-record search job. Do not replace root search with a casual `issue list --query` just to preserve the old surface.
- Keep `man` as slightly smart operating guidance, not as a second workflow system.
- Collapse `help` and `man` content ownership, but keep distinct user-facing surfaces: `help` remains deterministic command reference, while `man` remains role/task operating guidance.
- Output simplification must stay diagnostic, not opaque. Default output should be layered: concise summary, concrete cause, concrete next step, with verbose/detail modes exposing raw validators, policy names, paths, and underlying checks when needed.
- Do not replace anxious implementation dumps with vague failure messages. Blocked, failed, and warning states must explain what failed, why it matters, what concrete state caused it, and what to do next.
- Coordinate with `atelier-mska` instead of absorbing it. The storage/cache mission owns record-file storage, domain model boundaries, lazy cache access, and cache terminology. This mission owns the command surface, app/domain read services needed by commands, and human output contracts over those services.
- Keep `atelier-24xn` separate. Prune is a distinct retention/safety mission; this mission only ensures prune remains a coherent supported cleanup surface and does not leak into normal workflow guidance.
- Treat superseded Forgejo/session coordination context as product background, not active implementation scope. Review/provider lifecycle behavior that still matters should flow through the consolidated review and workflow surfaces here.

Status redesign:
- `status` answers: what is happening in this checkout, and what needs attention before continuing.
- Keep checkout information prominent: current branch, dirty/clean state, and a concise changed-file overview split at least between tracker files and non-tracker files when dirty.
- Do not print healthy implementation state such as `Tracker: current`; show tracker/projection/runtime health only when there is a problem or an actionable warning.
- Remove the root `Evidence Status` panel. Evidence requirements appear only as workflow-derived annotations on relevant active work or as transition/check diagnostics.
- Remove placeholder `Recent Activity` output such as `no active mission focus`; activity appears only when it is genuinely checkout/work relevant.
- Replace generic `Next Actions` command advertising with one deterministic next-action decision that cites the state that caused it. If no specific action can be justified, say so instead of emitting generic commands.
- Show active work inline with workflow-derived annotations, for example `in progress atelier-1234 Title` followed by `-> close blocked: needs linked evidence`.
- If multiple transitions are possible, show choices or transition-scoped blockers instead of pretending there is one next state.
- Keep deeper diagnostic detail reachable from status-adjacent flows through focused commands or verbose modes; do not hide the raw machinery when it is needed for debugging.

Domain/app extraction:
- Extract workflow transition/readiness evaluation from CLI command modules into a domain/app workflow service. It owns transitions, validators, planned actions, blockers, diagnostics, and primary/available transition summaries.
- Extract work row and queue modeling from CLI into a work service. It owns active/ready/blocked/done/backlog buckets, blocker-aware ordering, selectable work logic, and inline annotations.
- Extract objective scope from CLI into an objective service. It owns mission/epic linked-work traversal, `advances` semantics, descendants under linked work roots, objective rollup counts, and objective blockers.
- Extract evidence gate evaluation from CLI into an evidence/workflow service. It owns linked validating evidence lookup, passing/failing proof decisions, min-count/kind constraints, and evidence diagnostics.
- Extract relationship semantics from CLI into a relationship service. It owns allowed roles, blocking semantics, validates/advances semantics, cross-kind linking rules, and graph mutation validation.
- Extract history collection and scoping from CLI into a history service. It owns activity rows, link/evidence events, high-level timeline scoping, and issue/objective inclusion rules.
- Extract status decision-making from CLI into a status service. It owns checkout summary modeling, active work summary, attention classification, and next-action selection.
- Extract bundle preview/apply semantics from CLI into a bundle service while keeping the public `bundle` command name.
- Extract issue create/update lifecycle rules from CLI into app services, including configured issue type validation, initial status selection, section/body construction rules, parent/type constraints, and canonical mutation orchestration.
- Extract review workflow integration from CLI into app/review services, including effective review targets, artifact lifecycle state, review gates, and provider setup checks.

Command decisions:
- Keep `init` as the setup surface.
- Keep `status` for checkout orientation only.
- Add `work` as the owner for operational multi-issue views; the current CLI has no `atelier work` root command, so this is a real new surviving surface, not just a rename.
- Keep `issue` for single-record issue detail and explicit issue mutations.
- Keep simple issue inventory under `issue list`; do not turn it into the general work/dashboard surface.
- Remove ordinary direct status mutation such as `issue update --status`; workflow status changes belong to `issue transition`, with any direct status repair hidden/admin-only if it remains necessary.
- Remove `mission`; mission is an issue type, not a command namespace.
- Remove `issue status`; objective rollup belongs in `issue show <objective-id>`, and terminal readiness belongs in `issue transition`.
- Remove `issue transition --options`; `issue transition <id>` should show available and blocked transitions, while `issue transition <id> <transition>` executes one.
- Remove `issue blocked`; blocked inventory belongs in `issue list --blocked`, and blocker detail belongs in `issue show <id>`.
- Remove `issue table`; table layout is output presentation for list/work views, not a command noun.
- Remove `issue block` and `issue unblock`; blocking is a relationship handled through `issue link` and `issue unlink`.
- Keep `issue note` as prose/activity only. Do not let note kinds become hidden workflow state; reduce or remove the note-kind taxonomy unless it has a clear display purpose.
- Remove `search` entirely unless a later design proves a stronger cross-record search job.
- Keep `history` as one high-level timeline view, while removing or folding issue/mission/epic-specific history variants that duplicate focused commands. Do not let history flags become a scoped query language.
- Keep `bundle preview` and `bundle apply` for now as the graph preview/apply surface.
- Keep `evidence record`, `evidence show`, and `evidence list`; fold existing-evidence attachment into the generic relationship model if cross-kind linking is ergonomic.
- Collapse review commands toward `review open`, `review show`, `review submit`, `review resolve`, and `review merge`.
- Fold provider setup such as `forgejo` under review/admin ownership or hide it from normal workflow help.
- Fold normal branch lifecycle into workflow transitions; keep branch repair only as hidden/admin recovery if transitions cannot own it safely.
- Collapse `lint`, `doctor`, workflow diagnostics, projection rebuild, and safe runtime repair into one health/check surface.
- Keep `prune` as the supported cleanup surface.
- Hide or remove destructive `maintenance` and implementation diagnostics from normal command guidance.
- Hide or de-emphasize global diagnostic flags such as `--log-level` and `--log-format` in normal help. `--log-format json` is diagnostic logging, not command-result JSON.
- Prefer product vocabulary in public flags. For example, converge from `--issue-type` toward `--type` where that does not create ambiguity.
- Keep `help` as syntax, flags, and subcommand reference generated from or aligned with the executable CLI.
- Keep `man` as smart role/task guidance that routes to surviving commands and stops teaching deleted surfaces.
- Validate that `help` and `man` share one command/product model so they cannot drift into contradictory guidance.

Risks:
- Cutting commands before their capabilities have a surviving owner could strand agents that need objective scope, blocker detail, history, bulk graph apply, branch recovery, or health repair.
- A generic view system could become a new DSL or dashboard product before command ownership is settled.
- Agent Factory, command help, product docs, tests, and tracker proof expectations currently name surfaces targeted for removal, especially issue status and mission status.
- Moving domain logic out of CLI can accidentally preserve old command-shaped models unless the extracted services return typed product facts instead of preformatted command output.
- Combining this work with storage/cache or prune missions would create a mission too broad to validate. Cross-link concepts in docs and app-service boundaries, but keep implementation closeout separate.

Sequencing:
- Start with `atelier-55tk` to settle the product contract before implementation or deletion work changes public command behavior.
- `atelier-fasv`, `atelier-82u0`, and `atelier-3uew` are blocked on that contract so read models, secondary command consolidation, and drift checks target the same command model.
- `atelier-47cp` is blocked on `atelier-fasv`; command deletion should happen only after replacement read surfaces preserve the old capabilities that survive.
- `atelier-qcbx`, `atelier-3llx`, and `atelier-5km8` are focused correctness/proof gaps that can be worked independently, but they must close before mission validation.
- `atelier-8c91` is the final closeout epic. It should not start until replacement, removal, consolidation, drift-check, output, evidence, and objective-status proof work is complete.

Validation:
- docs/product/command-audit/command-surface-cut-plan.md and docs/product/command-audit/command-surface-capability-inventory.md describe the final public command model and every removed surface's replacement capability.
- Root help, focused command help, man guidance, docs/product/cli-surface.md, command-audit docs, AGENTS.md, and Agent Factory guidance agree on the surviving command surface and no longer teach removed issue status, mission status, issue blocked, issue table, root search, or scoped history variants.
- Focused CLI tests or transcripts prove status, work, issue show, issue transition, evidence, review, history, bundle, man, and check preserve the capabilities that should survive while mission status, issue status, issue blocked, issue table, search, lint, doctor, and related hidden diagnostics are removed or folded.
- Focused status tests prove `status` shows checkout branch and changed-file summary, active work rows with workflow-derived annotations, attention-only health/evidence messages, and state-cited next action decisions without generic command advertising.
- Domain/app tests prove workflow readiness, work rows, objective scope, evidence gates, relationship semantics, history scope, status decisions, bundle preview/apply, issue lifecycle mutation, and review lifecycle decisions can be evaluated without depending on CLI renderers.
- Output tests prove blocked/failed/warning states are neither vague nor dumpy: they include a concise summary, concrete cause, concrete next step, and a path to deeper diagnostic detail where relevant.
- Removed commands fail without compatibility aliases after replacement surfaces land.

## Outcome

A drastically smaller command surface exists for tracker work: `status` orients the checkout, `work` owns multi-issue operational views, `issue show` owns single-record detail, explicit issue commands own mutations, `evidence` owns proof, `review` owns review artifacts, `history` owns high-level timelines, `bundle` owns graph preview/apply for now, `man` owns smart guidance, and `check` owns health. Redundant status/list/table/search/mission/admin escape hatches are removed or folded into those surviving owners without losing core capabilities needed by humans or Agent Factory workflows. The CLI crate renders typed product facts from domain/app services instead of owning workflow, evidence, objective, relationship, history, status, bundle, issue lifecycle, or review truth.
