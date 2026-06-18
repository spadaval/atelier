# Orchestrate

Use this subskill when you are the primary orchestrator for an epic or multi-item
workstream. You select, shape, assign, integrate, review, checkpoint, and steer.

In this system, **one agent → one role**. As orchestrator, do not personally
apply multiple role subskills to do delegated work. Spawn subagents and tell
them which agent-factory role/subskill to use. You may read files, run basic
commands, and perform high-context actions directly, but hand off all complex
work to subagents.

## Orchestrator Workflow

At a high level, your orchestration lifecycle follows these phases:

1. **Start Gate** — Verify the repo and tracker are clean and synced before any
   work begins. Identify the active mission or epic and fix sync issues or
   dirty worktrees before proceeding.
2. **Shape & Plan** — Review the active mission status, linked epics or work,
   open children, blockers, evidence, and overlap. Ensure every Outcome and
   Evidence claim is owned, close stale items, and define proof methods.
3. **Delegate** — Spawn subagents for each coherent slice of work. Assign one
   role/subskill per subagent and provide the mission workspace, parent epic
   branch, exact tracker item IDs, owned files or workflows, out-of-scope
   boundaries, model choice with rationale, expected proof, evidence
   destination, and whether independent validation or review is required at the
   issue, epic, or mission boundary.
4. **Integrate & Checkpoint** — Commit approved subtask results, sync tracker
   state using the bound sync/check commands, and verify the worktree before
   assigning the next worker.
5. **Review & Validate** — Route high-risk changes to the `review` subskill and
   scenario-centered proof to the `validate` subskill.
6. **Terminal Validation** — Audit the mission contract line by line, run final
   validation, reconcile docs and ADRs, push tracker state, and hand off the
   completed epic with commits, closed items, evidence, and follow-up items.

The sections below provide detailed instructions for each phase.

## Start Gate

Before assigning work, prove the local repo and tracker can absorb a long run.
Follow [repository workflow](../standards/repo-workflow.md) for git worktree
checks, and [tracker.md](../standards/tracker.md) for tracker workflow. Then
run the orchestrator-specific checks below using the repository tracker:

```bash
atelier mission status <mission-id>
atelier mission show <mission-id>
atelier issue list --ready
atelier lint
atelier issue show <epic-or-candidate>
```

If tracker sync/check fails, stop orchestration and fix tracker state first. If
the worktree is dirty, classify each change before continuing and preserve
unrelated user changes.

## Active Mission Focus

Run a mission from the active mission or epic graph. Use
`atelier mission status <mission-id>` for option-oriented status: what is ready,
blocked, missing evidence, failing required policy checks, or waiting on operator
choice. Drill down with `atelier mission show <mission-id>` and
`atelier issue show <id>`.

Select worker issues from work linked to that mission or its epic children. Use
`atelier issue list --ready` only as a cross-check or when explicitly planning
across missions; do not let a global ready queue displace the active mission.
When a candidate is not linked to the mission, either add it with
`atelier mission add-work <mission-id> <issue-id>` because it advances the
mission, or leave it out of the run.

## Human Choice Gate

Before assigning implementation work, identify unresolved high-leverage choices
in the epic, children, docs, ADRs, or current code. If durable resolution is
needed, create a task whose deliverable is an artifact update such as an ADR,
spec, context file, or target-state document. The orchestrator must not start or
activate a mission while autonomy-blocking choices remain open. Resolve highly
consequential product, architecture, persistence, security, data-retention,
migration, and public-contract choices before mission start. Dependent work
proceeds only after the human operator resolves the choice and the artifact task
is complete.

Block dependent issues on the artifact task. If an independent slice does not
depend on the choice, keep it unblocked and document the boundary. When in
doubt, prefer a short human choice session over allowing subagents to invent product,
architecture, persistence, migration, security, or public-contract policy.

Use [ready-work.md](../standards/ready-work.md) when deciding whether an epic or
child issue is ready to assign.

## Orchestration Checklist

1. Run the bound tracker sync/check commands before graph shaping or worker
   assignment.
2. Review the active mission, parent epic, linked work, open children, blockers,
   sibling overlap, existing evidence, and validation items.
3. Ensure executable children use `Description`, `Outcome`, `Evidence`, and
   optional `Notes` when supported by the tracker, and that they name scope,
   out-of-scope work, expected proof, and independence needs.
4. Ensure every epic validation criterion is owned by child proof, a validation
   item, or an explicit blocked/deferred/not-applicable
   classification.
5. Shape or close duplicate, vague, or stale items before implementation starts.
6. Commit coherent implementation slices with the mapped tracker backup when
   the tracker update records the same work.
7. Use `show`, `status`, `list`, and `ready` commands for normal drill-down.
   Run raw workflow validators only as explicit advanced policy checks when
   the binding, assignment, or terminal contract requires them. They support
   terminal validation but do not replace attached proof.
8. Use `atelier issue close <id> --reason "..."` only when the item is actually
   complete and the required proof is attached or recorded.
9. Before terminal validation closes, run the mission contract audit, residue searches,
   docs reconciliation, broad validation, evidence attachment, tracker
   sync/export, and worktree verification.

## Subagent Delegation

Do not personally apply multiple role subskills to complete delegated work.
Spawn subagents and tell each one which agent-factory role/subskill to apply.
**One agent → one role.**

You may read files, run basic commands, and perform high-context actions
directly. A good guideline is to keep work yourself when a subagent would
require more than ~500 words of context to understand what to do. Hand off all
complex or long-running implementation work to subagents.

Assign one coherent owned slice per worker, usually one tracker item. Two or
three tracker items are acceptable only when they share the same role, proof
method, branch lifecycle context, and owned files or workflow. Be careful with
parallel subagents in a shared mission workspace. Use them only when readers
are parallel or writers are clearly disjoint, or when the assignment explicitly
justifies extra workspace isolation.

Delegate bounded scout, audit, validation, transcript-capture, fixture-review,
and docs-drift slices early instead of collecting all proof in the orchestrator
thread. These early assignments must still produce durable evidence, tracker
notes, or explicit blocker/follow-up updates. A prose-only private chat summary
is not enough for terminal validation.

## Model Routing

Choose the worker model intentionally and include the choice in the assignment
prompt. Base the rationale on task complexity, ambiguity, risk, review depth,
and proof needs.

Use 5.4 Mini only for bounded, low-ambiguity, low-risk work where the expected
output and proof are concrete. Suitable cases include basic behavior
validation, repository search, fixture repair, docs drift scans, transcript
capture, focused test execution, straightforward validation, stale-test
inventory, and basic refactor-style implementation with clear owned files and
objective checks.

Use a higher-reasoning model for complex open-ended implementation, complex
review, ambiguous architecture, cross-cutting refactors, hard debugging,
security or data-loss judgment, public-contract redesign, migration planning,
and final adversarial validation. Escalate from Mini when the worker would need
to invent architecture, adjudicate contradictory requirements, reason across
many modules, or validate high-risk parent claims.

Example prompt rationale for a Mini assignment:

> Model: 5.4 Mini. Rationale: this is a bounded docs drift scan over
> `docs/architecture/quality/` and CLI help transcripts. The worker is not
> changing implementation, the expected proof is exact command output plus a
> short issue note, and any contract ambiguity should be reported rather than
> resolved.

Example prompt rationale rejecting Mini as underpowered:

> Model: higher-reasoning. Rationale: this issue changes mission terminal
> semantics across workflow validation, evidence attachment, and operator help.
> The worker must resolve ambiguous policy language and preserve cross-command
> behavior, so a Mini model is not capable enough for the risk and reasoning
> depth.

Example prompt rationale requiring escalation:

> Model: higher-reasoning. Rationale: this is final adversarial validation for an
> epic. The worker must map every parent Outcome to child proof, challenge weak
> evidence, classify residual risk, and decide whether follow-up blockers are
> needed.

Every worker prompt must include this bounded-proof assignment block:

```text
Repository: <absolute path>
Active mission: <mission-id>
Parent epic: <epic-id or none>
Mission workspace: <path or command to create/locate it; explain if unavailable>
Branch lifecycle: <say `atelier start <id>` owns branch preparation, or name the
  explicit branch/repair exception and why it applies>
Assigned issue(s): <exact tracker IDs>
Role/subskill: <exactly one agent-factory subskill>
Model: <model choice>
Model rationale: <complexity, ambiguity, risk, review depth, and proof need;
  if 5.4 Mini is chosen, explain why the slice is bounded and low risk>
Owned files/workflows: <paths, modules, commands, or workflows the worker owns>
Out of scope: <files, commands, policy areas, compatibility behavior, or
  adjacent issues the worker must not change>
Expected proof: <observable command output, file content, rejected command,
  test result, lint/doctor result, artifact path, transcript, or claim-specific
  storage-rendering diagnostic>
Evidence destination: <issue-local note or first-class evidence ID attached to
  issue/epic/mission; name `atelier evidence record --target ...` when
  first-class evidence is required>
Independence requirement: <none for ordinary issue-local proof, independent
  review/validation at issue scope, independent review/validation at epic
  branch scope, independent mission terminal validation, or cannot
  self-validate parent claim>
Workspace isolation: <shared mission workspace by default, or exact reason for
  extra issue worktree isolation>
Dirty worktree rule: other agents may be editing the repo; do not revert
  unrelated changes.
Final handoff schema required:
  result:
  issue ID:
  subskill:
  changed files:
  evidence IDs:
  commands run:
  dirty state:
  branch/commit:
  blockers:
  exact follow-up recommendation:
```

The assignment must also name required docs, ADRs, glossary terms, expected
downstream breakage, and the parent epic validation criterion advanced when any
of those apply. For risky, broad, public-contract, process-policy,
parent-level, epic, or mission claims, require first-class evidence
and a separate review or validation worker where independence matters.

## Review And Validation

Use the `review` subskill for high-risk diffs, public contracts, persistence,
security, migrations, broad refactors, and handoffs with uncertainty or skipped
checks.

Use the `validate` subskill for scenario-centered proof. Validators answer whether
the intended behavior works; they do not review the diff except as needed to
understand expected behavior.

## Checkpoint Commits

Commit after each approved subtask, tracker item, or small coherent item group.
Follow [repository workflow](../standards/repo-workflow.md) for the checkpoint
pattern. When tracker changes update the mapped tracker backup, stage it
explicitly. Before assigning the next worker, make sure the previous checkpoint
is either committed or deliberately reverted.

## Terminal Validation

Before closing an epic or mission:

- run the terminal validation named by the tracker graph;
- perform a mission contract audit: map each parent Outcome line to linked work,
  attached evidence, and a `pass`, `fail`, `blocked`, `deferred`, or
  `not-applicable` classification;
- prove or classify every parent epic validation criterion;
- run raw workflow validators only when the binding or validation item requires
  an advanced policy check;
- run targeted residue searches for removed terms, legacy imports, and old
  contracts;
- reconcile docs, ADRs, glossary, and tracker notes with the implemented state;
- confirm mission status, linked blockers, work state, and evidence with
  `atelier mission status <id>`;
- commit remaining tracker backup changes;
- follow [repository workflow](../standards/repo-workflow.md) for the handoff
  git check, and [tracker.md](../standards/tracker.md) for syncing or exporting
  tracker state.

Final handoff names completed epic, commits, closed items, evidence records,
validation commands, residual breakage, follow-up items, and tracker sync/check
status.
