---
created_at: "2026-07-09T16:32:33.130112177+00:00"
id: "atelier-uxgk"
evidence_type: "validation"
captured_at: "2026-07-09T16:32:33.130105366+00:00"
path: "docs/adr/0018-independent-mission-plan-review.md"
agent_identity: "agent-factory.validate"
target:
  kind: "issue"
  id: "atelier-2uim"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-2uim"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "blocked: independent legacy-contract audit found one contradiction in atelier-0zhd target workflow examples; direct mission evidence.attached conflicts with retained child-owned proof routing; follow-up atelier-0zhd; executable behavior deferred to atelier-t876; see claim matrix in this evidence record"
updated_at: "2026-07-09T16:32:37.176086544+00:00"
---

## Result

`blocked` — independent retained-contract validation found one durable contract
contradiction. The target workflow examples require a direct
`evidence.attached` gate on mission publication, while the retained proof-routing
contract says ordinary mission publication derives proof from accountable child
work and does not require a second direct mission evidence record.

Evaluator: `agent-factory.validate`, independent from the `atelier-0zhd` and
`atelier-wlk4` authors. Inspected authored commits `5dc4d175` and `df2c4cf6`
against legacy decisions `atelier-a44d`, `atelier-ql9k`, and `atelier-1mga`.

## Claim Matrix

| Legacy source and rule | Reconciliation | Validation | Exact current location |
| --- | --- | --- | --- |
| `atelier-a44d`: mission lifecycle remains `draft`, `ready`, `active`, `closed`, with direct start/focus behavior | **Superseded** for the state set and direct readiness; the accepted target is `draft -> plan_review -> ready -> in_progress` | `pass` | `docs/adr/0018-independent-mission-plan-review.md:24`; `docs/product/work-model.md:242`; `docs/product/workflow-configuration.md:137` |
| `atelier-a44d`: terminal checks are workflow validator policy, not a separate closeout status subsystem | **Retained** | `pass` | `docs/product/workflow-configuration.md:82`; `docs/product/workflow-configuration.md:180`; `docs/architecture/quality/validation.md:368` |
| `atelier-a44d`: no arbitrary mission workflow engine, no mission review state, and no second parent acceptance-review issue type | **Retained** for bounded workflow ownership and rejection of a second parent issue; **superseded** only for the review-state prohibition by configured `plan_review` | `pass` | `docs/adr/0018-independent-mission-plan-review.md:42`; `docs/adr/0018-independent-mission-plan-review.md:92`; `docs/adr/0018-independent-mission-plan-review.md:140` |
| `atelier-a44d`: no hard-coded closeout-specific type or validator subsystem | **Retained**; the contract continues to use workflow-owned, lifecycle-neutral objective validators | `pass` | `docs/product/workflow-configuration.md:82`; `docs/product/workflow-configuration.md:595`; `docs/architecture/quality/validation.md:368` |
| `atelier-ql9k`: readiness requires a worker-usable `Outcome` understandable without private history | **Retained** as a necessary review criterion | `pass` | `docs/product/mission-authoring.md:44`; `docs/adr/0018-independent-mission-plan-review.md:134`; `.agents/skills/agent-factory/procedures/plan.md:77` |
| `atelier-ql9k`: Outcome clarity is the whole readiness review question | **Amended / superseded**; independent review now covers the entire reachable issue set, dependencies, ownership, sequencing, validation, closeout, and parallel safety | `pass` | `docs/adr/0018-independent-mission-plan-review.md:134`; `docs/product/mission-authoring.md:38` |
| `atelier-ql9k`: planners do not author validation scenarios or evidence checklists by default | **Retained** | `pass` | `docs/product/mission-authoring.md:78`; `docs/architecture/quality/validation.md:33`; `.agents/skills/agent-factory/procedures/plan.md:47` |
| `atelier-1mga`: mission scope is direct `advances` roots plus their descendants | **Retained** | `pass` | `PRODUCT_INTENT.md:149`; `docs/product/work-model.md:9`; `docs/product/mission-authoring.md:16` |
| `atelier-1mga`: mission lifecycle is owned by `.atelier/workflow.yaml` | **Retained** | `pass` | `CONTEXT.md:274`; `docs/product/work-model.md:125`; `docs/product/workflow-configuration.md:82` |
| `atelier-1mga`: planning is Outcome-led and validators select proof after implementation | **Retained** | `pass` | `PRODUCT_INTENT.md:165`; `docs/product/work-model.md:113`; `docs/architecture/quality/validation.md:5`; `docs/product/mission-authoring.md:78` |
| `atelier-1mga`: evidence is a receipt for checks that actually ran and normally belongs on accountable issue-shaped work | **Retained in the ADR, work model, authoring standard, and validation router** | `pass` for receipt semantics; `fail` for cross-source consistency because target workflow examples add a direct mission evidence gate | `PRODUCT_INTENT.md:211`; `PRODUCT_INTENT.md:242`; `docs/product/work-model.md:41`; `docs/product/mission-authoring.md:24`; `docs/architecture/quality/validation.md:145` |
| `atelier-1mga`: mission closeout does not require planner-authored validation prose or direct mission evidence | **Retained by stated proof policy; contradicted by target workflow examples** | `fail` | Retained: `docs/architecture/quality/validation.md:368`, `docs/product/work-model.md:140`, `PRODUCT_INTENT.md:242`. Contradiction: `PRODUCT_INTENT.md:361` and `docs/product/workflow-configuration.md:180` include `evidence.attached: { min_count: 1 }` on the mission transition. Current validator semantics count evidence linked to the transition target in `crates/atelier-app/src/workflow_validation.rs:196`. |
| Legacy direct `draft -> ready` readiness and author-only readiness judgment | **Superseded** | `pass` | `docs/adr/0018-independent-mission-plan-review.md:24`; `docs/adr/0018-independent-mission-plan-review.md:140`; `.atelier/workflow.yaml:4` marks the remaining live transition pre-cutover only |

## Blocking Finding

`fail` — `PRODUCT_INTENT.md:371` and
`docs/product/workflow-configuration.md:189` prescribe
`evidence.attached: { min_count: 1 }` on the target mission close/publish
transition. `PRODUCT_INTENT.md:242-244` says direct mission evidence is only for
legacy imports or migration notes, and
`docs/architecture/quality/validation.md:368-380` says routine publish does not
require a second direct mission evidence record because child work and
`validation.criteria_satisfied` own proof. The implemented validator checks
evidence linked to its target issue (`crates/atelier-app/src/workflow_validation.rs:196-219`),
so this is a real placement requirement, not an aggregate child-proof alias.

Affected IDs: contract owner `atelier-0zhd`, parent epic `atelier-6tne`, and
blocked dependent implementation `atelier-amfw`, `atelier-wyxn`, and
`atelier-l5mw`. Follow-up ID: `atelier-0zhd` must be reopened or replaced by an
explicit contract-repair issue owned under `atelier-6tne`; remove the direct
mission `evidence.attached` gate from both target examples or durably amend all
proof-routing sources before this audit is rerun.

## Other Classifications

- `pass` — authoring standard is Outcome-led, routes lifecycle and proof
  authority to ADR/work-model/validation sources, makes validation independently
  accountable, and keeps evidence as later receipts.
- `pass` — live `.atelier/workflow.yaml:4-9` explicitly labels direct
  `draft -> ready` as pre-cutover observable behavior, not target architecture.
- `pass` — issue/branch resolution: `atelier-2uim` is under `atelier-6tne` on
  `epic/atelier-6tne`; lifecycle activity records branch owner `atelier-6tne`
  and base/review/integration target `mission/atelier-p4z2`, with no dependency
  on missing `mission/atelier-1mga`. Transcript: evidence `atelier-qyyf`.
- `pass` — with `atelier-wlk4` terminal and `atelier-2uim` open,
  `atelier work ready` excludes `atelier-amfw`, `atelier-wyxn`, and
  `atelier-l5mw`: evidence `atelier-h4i8`.
- `pass` — in an isolated clone, changing only `atelier-2uim` to a configured
  terminal status makes those three issues selectable; each asserted blocker
  list contains only `atelier-2uim` and terminal `atelier-wlk4`: evidence
  `atelier-vr5e`.
- `deferred` — executable lifecycle, migration, rebuild, direct/transitive
  dependency enforcement, diagnostics, and Agent Factory behavior belong to
  `atelier-t876`. This audit did not claim runtime correctness.
- `not-applicable` — ignored/skipped-test freshness: this is a contract audit
  over Markdown and tracker graph behavior, not executable implementation proof.

Residual risk: dependent implementation must remain blocked until the contract
contradiction is repaired and this independent compatibility audit is rerun.
