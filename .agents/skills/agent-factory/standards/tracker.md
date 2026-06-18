# Tracker Reference

Use the repository instructions to identify the tracker, then use that
tracker's own help/status surfaces for workflow behavior.

The tracker owns scope, outcomes, evidence expectations, status, dependencies,
and handoff.
Do not replace it with private notes or TODO files.

## Command Routing

Use tracker commands for:

- planning: create, split, reparent, label, prioritize, and search work;
- mission focus: inspect mission status, linked work, blockers,
  evidence, and readiness options;
- ready: list executable items with no open blockers;
- work lifecycle: create or locate the mission workspace when the tracker
  supports it, start work through the tracker-owned start surface that prepares
  branch context, inspect status-derived work state, and move issues through
  configured lifecycle transitions;
- evidence: create concise validation records and attach them to issues,
  missions, or other target records;
- policy checks: evaluate issue or mission transition policy when the
  assignment or terminal contract explicitly requires it;
- update: edit fields, append durable notes, and record handoff;
- dependency: add or remove real blocker relationships;
- close: mark completed work with a reason;
- lint: validate tracker records globally or for one item;
- sync/check: export, lint, and health.

Do not use interactive tracker commands. Prefer explicit command flags that can
run unattended.

## Atelier Repositories

For Atelier repositories, start with `atelier man`.
Dependency direction is `atelier issue block <blocked-id> <blocker-id>`: the
first item waits on the second. Commit durable tracker state with related work.
Runtime state is rebuildable.

## Atelier Mission And Work Standard

Use missions as the durable focus record. Select worker issues from the current
mission graph. Use the global ready list only for discovery.

Use `atelier mission status <mission-id>` for ready work, blockers, evidence
gaps, policy-check gaps, deferred work, and terminal checks. Use `show`, `list`,
`ready`, and `status` commands for normal planning and drill-down. Do not plan
or validate by parsing command-result JSON, and do not treat raw workflow
validator output as ordinary user-facing proof.

Worker flow:

```bash
atelier man <role>
atelier status
atelier issue show <id>
atelier mission status <mission-id>
atelier start <id>
# implement owned slice and run proof
atelier issue note <id> "handoff or review context when useful"
atelier evidence record --target issue/<id> --kind <kind> "summary"
atelier issue transition <id> --options
atelier issue close <id> --reason "..."
```

If the assignment requires checkout setup, follow the tracker-owned workspace
guidance before mutating. For Atelier, use mission workspaces by default and let
`atelier start <id>` prepare owner branches. Extra issue worktrees are
exceptional isolation for conflicting, dirty, high-risk, or explicitly isolated
slices rather than the default mutating-worker setup.

For Atelier, current work is derived from canonical issue status and checkout
context. Runtime pointers and hidden ownership markers are local diagnostics or
legacy implementation details, not portable Agent Factory workflow contracts.

If work cannot finish, leave handoff notes, attach useful evidence, and keep the
item open.

Run advanced workflow validators only when the assignment or parent terminal
contract asks for them. A passing validator is a policy signal; it does not
replace proof attached to the issue.

Before mission closure, run:

```bash
atelier mission status <mission-id>
atelier lint
atelier doctor
```

Add focused tests or transcripts for the mission's explicit claims. Run
storage-rendering diagnostics such as `atelier export --check` only when the
mission changes deterministic export, projection freshness, or migration/debug
surfaces. If the assignment requires a mission workflow validator, run it as an
explicit advanced policy check after the mission contract audit has mapped
parent Outcome lines to linked work and attached evidence.

## Ready Item Standard

A ready executable item answers:

- what is changing;
- why the work exists;
- what is in scope and out of scope;
- how to prove completion;
- expected breakage and the owner for reconnect or terminal validation;
- assigned subskill when not obvious.

Write tracker items using
[work-item-authoring.md](work-item-authoring.md): Outcome describes the desired
finished world, Evidence describes proof an independent validator could use,
and Notes carry context or non-goals. Do not encode a rigid implementation plan
unless the exact implementation path is the decision being tracked.

Ordinary issue work closes with proof on the owning issue. Risky, broad,
public-contract, process-policy, parent-level, epic, and mission claims require
first-class evidence and should name independent validation or review at the
epic or mission boundary where the implementer should not be the sole
validator.

For mission readiness, unresolved high-consequence choices block mission start.
Track durable resolution as artifact-update tasks.

Do not assign vague, oversized, ambiguous, or multi-deliverable items.
