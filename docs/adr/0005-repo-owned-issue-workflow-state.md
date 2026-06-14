# ADR 0005: Repo-Owned Issue Workflow State

## Status

Accepted.

## Context

Atelier currently inherits issue lifecycle semantics from Chainlink and still
documents workflow behavior that the repository does not enforce yet. The
workflow mission needs a stable contract before parser, transition, migration,
and closeout work lands.

The unresolved choices are architectural, not incidental implementation
details:

- whether issue `status` remains a small built-in lifecycle field or becomes
  repository-defined workflow state;
- whether operator surfaces should expose only raw status names or keep a
  smaller status-category vocabulary for orientation;
- whether workflow policy is discovered through an indirection layer or through
  one fixed tracked file;
- whether version 1 should cover every record type or only issues;
- whether the rollout should preserve inherited command aliases, fallback
  readers, and old output shims.

These choices affect issue records, ready queues, transition checks, help and
Agent Factory guidance, migration shape, and future validation work. They need
durable documentation that later workers can cite.

## Decision

Atelier adopts a repo-owned issue workflow model for version 1.

1. Issue `status` is workflow state.
   The canonical `status` field on an issue record stores the configured
   workflow status name. It is durable repository state, not a derived
   presentation field and not a hidden mapping from `open`, `in_progress`, and
   `closed`.

2. Status categories are derived orientation metadata, not the canonical state.
   Workflow policy may classify statuses into a smaller operator-facing
   vocabulary such as ready, active, blocked, done, or archived. Commands use
   categories to keep status, ready-queue, and next-action output scan-friendly
   across repositories, while validators and transitions continue to target the
   exact workflow status.

3. Version 1 workflow policy lives at one fixed tracked path:
   `.atelier/workflow.yaml`.
   Workflow discovery does not go through `.atelier/config.toml`, root
   `atelier.workflow.yaml`, or a repository-selected alternate path in version
   1. Root `atelier.workflow.yaml` hook behavior is removed rather than
   treated as an alternate policy source. The policy file is committed project
   state alongside other canonical `.atelier/` records.

4. Version 1 workflow scope is issue-only.
   The configured workflow engine, migration path, transition wrappers, and
   validator wiring apply only to issue records in this mission. Mission,
   milestone, plan, and evidence lifecycle policy stay on their current
   contracts until later work defines those record-specific workflows.

5. Version 1 removes obsolete lifecycle surfaces directly.
   The workflow rollout does not preserve compatibility aliases, fallback
   readers, staged deprecations, or old output shims unless a human explicitly
   asks for them. Help, docs, and Agent Factory guidance should teach only the
   new issue workflow path.

## Consequences

- Repository workflow policy becomes explicit, committed, and reviewable.
- Transition, closeout, and migration work can rely on one canonical issue
  state field instead of translating between built-in lifecycle names and
  workflow names.
- Operator output can stay compact because status categories provide stable
  orientation without constraining repositories to one exact status vocabulary.
- Version 1 implementation risk stays bounded because workflow semantics do not
  expand to missions, milestones, plans, or evidence in the same rollout.
- The rollout is intentionally breaking for inherited lifecycle commands and
  docs. Drift should be removed rather than hidden behind compatibility code.

## Tradeoffs

- Making `status` repository-defined increases migration and validation work
  because status semantics now come from policy rather than from a hardcoded
  enum.
- Derived status categories add one more concept to the domain model, but they
  avoid forcing every operator surface to understand arbitrary repository
  status names without grouping.
- A fixed `.atelier/workflow.yaml` path reduces flexibility for multi-policy or
  user-selected layouts, but it keeps discovery, linting, migration, and
  support simple while the feature is new.
- Limiting version 1 to issues defers a unified workflow story for other record
  types, but it prevents this mission from redefining every lifecycle in the
  product before issue transitions work.
