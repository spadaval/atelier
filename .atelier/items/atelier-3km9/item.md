---
schema: atelier.item
schema_version: 3
id: atelier-3km9
title: Let mission lead merge approved items, make architecture/decisions living docs, and update role skills
status: approved
mission: null
labels: []
priority: medium
blocked_by: []
implementers:
- cf983ef7-3317-404a-9c82-373f732b9e07
ready: null
trivial: null
caused_by: null
created_at: 2026-11-27T19:34:50.895984175Z
---

## Goal

Implement the process, CLI, and role-skill improvements from the multi-mission audit and planning/DACI review:

1. Allow a mission's lead to run `atelier merge` on any approved item of its mission without holding it as implementer or using `ATELIER_SKIP`, and have `atelier status` / `actions::next` offer `atelier merge` to the lead on approved items.
2. Exempt `architecture.md` and `decisions.md` in a mission directory from the plan-amendment gate alongside `notes.md`, so living technical contracts and DACI decision logs can be updated on `backlog/` branches without triggering two plan reviews while `mission.md` and `design.md` remain frozen human contracts.
3. Update the Agent Factory role skills (`planner.md`, `lead.md`, `implementer.md`) and durable docs (`docs/product.md`, `docs/design.md`) with uncertainty-budget sizing, the 4-doc mission packet (including measurable + subjective Definition of Done and Enablers vs. Delivery), Explore → Decide → Check (DACI), and operational lead/implementer improvements.

## Done when

- [x] `atelier-core::actions` and `atelier-core::view` allow a mission's lead to merge an approved item of its mission (and `actions::next` returns `Action::Merge` for the lead on `Status::Approved`), while non-lead non-holders remain refused and the lead still cannot `submit` or `merge --trivial` another thread's building item.
- [x] `atelier-core::plan::is_plan_file` exempts `architecture.md` and `decisions.md` alongside `notes.md` and `reviews/` so updating them on a mission or backlog branch does not require plan review.
- [x] Unit and CLI integration tests cover lead-merge and living `architecture.md` / `decisions.md` updates.
- [x] `skills/agent-factory/roles/planner.md`, `roles/lead.md`, and `roles/implementer.md` reflect the new planning (uncertainty budget, 4-doc packet, Definition of Done, Enablers vs. Delivery, Explore → Decide → Check DACI) and execution practices without bloating `SKILL.md` or `roles/reviewer.md`.
- [x] `docs/product.md` and `docs/design.md` document lead-merge and the non-plan living mission files.

## Scope

- `crates/atelier-core/src/{actions.rs,repo.rs,view.rs,plan.rs}`
- `crates/atelier-cli/src/commands.rs`
- `crates/atelier-core/tests/actions.rs`, `crates/atelier-cli/tests/{merge.rs,planning.rs,disputes.rs}`
- `skills/agent-factory/roles/{planner.md,lead.md,implementer.md}`
- `docs/{product.md,design.md}`

## Report

Wired `lead: Option<&'a Thread>` into `actions::Facts` (populated from `repo::links` and `view::entries`), allowing `Role::Implementer` on `Action::Merge` when `facts.is_lead()` and returning `Action::Merge` from `actions::next` for the mission's lead on an approved item. Extended `plan::is_plan_file` to exempt `architecture.md` and `decisions.md` alongside `notes.md` and `reviews/`. Updated `planner.md`, `lead.md`, `implementer.md`, `docs/product.md`, and `docs/design.md`, and added unit and CLI integration tests (updating `disputes.rs` where a lead merging past a kept finding no longer skips the implementer-holder rule).

### Validation

- `cargo fmt -- --check` — passed
- `cargo clippy --all-targets --locked -- -D warnings` — passed
- `cargo nextest run -p atelier-core -p atelier-cli` — passed
- `git diff --check` — passed

### Responses

- 1-e5xa.1: fixed in b7c28134c
- 1-e5xa.2: fixed in b7c28134c
