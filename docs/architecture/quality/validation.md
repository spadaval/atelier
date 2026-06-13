# Validation

## Proof Routing Policy

Proof starts on the work item that made the claim. The issue `Evidence`
section names the expected proof, and the worker records that proof before
closeout. Escalate by risk and scope: ordinary local work proves itself on the
issue, while risky, broad, or parent-level claims require independent
validation and first-class evidence.

Strong proof is claim-specific, reproducible, attached, classified, scoped, and
independent when required by risk. Weak proof is broad, summary-only,
unattached, unverifiable, stale, or not mapped to a claim. A full test suite,
lint run, or mission status page can support proof, but it is weak by itself
when it does not show the exact changed behavior, file content, command result,
rejected command, help text, benchmark, or evidence record named by the claim.

Epics and missions coordinate work; they are not the normal executable proof
surface. Their claims are proven by evidence on accountable child work plus a
closeout or validation item that audits the parent outcome. Direct evidence on a
mission is legacy, migration-only, or a final closeout artifact mirrored from an
accountable closeout issue; it is not the normal way to satisfy mission proof.

## Validation Placement

Each tracker layer owns a different question. Do not duplicate lower-level
detail at every parent layer.

| Layer | Owns | Avoid |
| --- | --- | --- |
| Mission `Validation` | Mission-level target state, closeout confidence, required independent or adversarial review, and the evidence classes needed to trust the whole mission. | Child implementation steps, exact file lists, or every command each issue must run. |
| Epic `Outcome`/`Evidence` | Cohesive product, process, or architecture result, plus how child work, a validation item, or closeout item will prove the parent claim. | Repeating every child issue's local proof or turning the epic into a second implementation spec. |
| Executable issue `Outcome`/`Evidence` | The local observable result and proof for the owned slice: command output, file content, tests, transcripts, screenshots, or evidence records. | Parent mission claims, broad closeout promises, or proof that belongs to an independent validation issue. |
| Dedicated validation issue `Outcome`/`Evidence` | Independent review scenarios, claim classification, evaluator context, baseline or scenario setup, and evidence capture. | Fixing defects while validating or restating the implementation plan as validation criteria. |

Anti-red-tape rule: add detail to a higher layer only when it changes scope,
risk, sequencing, or parent-level confidence. Otherwise, keep detail at the
lowest accountable layer and let closeout map parent claims to child evidence.

Broad persistence, canonical write, projection refresh, runtime-cache, and
worktree changes need early concurrency or scenario validation before final
closeout. Do not rely only on an end-of-mission audit for changes that can lose
canonical state, corrupt projections, or misroute work.

## Contract-First Work

Start with the durable contract before implementation when work changes public
CLI semantics, workflow policy, evidence schema, Agent Factory rules, mission
or issue transition behavior, public docs that must match help output, or other
cross-agent process contracts. Contract-first work can update specs, product
docs, quality guidance, command help contracts, tracker item wording, or tests
before code.

Test-first proof is required or strongly preferred for CLI behavior, workflow
validators, projection/rebuild behavior, evidence recording, regression fixes,
and rejected-command behavior. Strict TDD is optional for tiny local refactors,
typo-scale docs, mechanical renames, or low-risk internal cleanup where the
existing checks directly cover the claim. It is required when the defect,
public behavior, or workflow gate can be reproduced before the fix.

| Work | Strong proof | Weak proof |
| --- | --- | --- |
| Docs-first workflow policy | Documentation diff shows the new policy, a review artifact maps the policy to example work items, and tracker lint/export pass. | "Updated docs" plus a broad lint run with no mapping to the policy claim. |
| Test-first CLI rejection | A failing-before/passing-after test or transcript shows the rejected command and error text, with docs/help parity when public help changes. | Full test suite passes without showing the rejected command path. |
| Canonical write or projection refresh | Round-trip or rebuild transcript, concurrency or scenario proof, and attached evidence show canonical files remain source of truth. | End-of-mission audit only, with no early proof of the write or refresh path. |

## Agent Factory Boundary

`AGENTFACTORY.md` is the repository binding and delegation bridge, not the full
Atelier command or workflow contract. When a rule is durable product behavior,
Atelier must own the operator-facing surface and Agent Factory should route to
it instead of restating it as private process lore.

| AGENTFACTORY topic | Classification | Durable owner or destination | Notes |
| --- | --- | --- | --- |
| `Sources`; tracker identity; durable `.atelier/` state; ignored runtime/cache state; preferred binary for ordinary work | Repository binding | `AGENTFACTORY.md`, `docs/index.md`, repository instructions | Keep in the binding so a fresh worker can locate the repository's authoritative docs and tracker shape. |
| Role assignment; one-subskill delegation; subskill selection; model routing; mutating-subagent worktree judgment; `--json` avoidance as an Agent Factory automation contract; independent-review judgment when the tracker does not carry first-class assignment metadata | Orchestration-only guidance | Agent Factory prompts, skills, and explicit assignments | Keep in Agent Factory unless Atelier gains first-class assignment metadata or explicit workflow fields that own the same decision. |
| Mission, issue, worktree, evidence, history, and relationship command purposes | Atelier-owned product behavior | `atelier --help`, `docs/product/cli-surface.md` | The binding may name entrypoints, but the public command contract belongs to Atelier help and product docs. |
| Workflow transitions, readiness rules, and advanced diagnostics meant for operator drill-down | Atelier-owned process behavior | `.atelier/workflow.yaml`, `docs/product/workflow-configuration.md`, `atelier issue transition --options` | Agent Factory should invoke the product surface rather than carrying a second transition cookbook. |
| Proof routing, evidence placement, independent-validation triggers, and parent closeout expectations | Atelier-owned process behavior | `docs/architecture/quality/validation.md`, `atelier evidence record`, `atelier mission audit`, `atelier mission status` | Process-policy work still requires first-class evidence and often separate validation, but the durable rule is Atelier-owned. |
| Tracker freshness, health, and readiness output | Atelier-owned product behavior | `atelier status`, `atelier mission status`, `atelier lint`, `atelier export --check`, `atelier doctor` | Missing proof, blockers, stale state, and health failures should be surfaced by Atelier-owned commands or validators. |
| Agent-facing command freshness for `AGENTS.md`, `AGENTFACTORY.md`, product docs, and command-surface tests | Atelier-owned product behavior | `atelier workflow check`, `atelier mission status --verbose`, mission closeout | Routine handoff can run `atelier workflow check`; mission status and closeout surface the same docs/help drift validator when a mission is being closed. |
| Removed-command policy, compatibility windows, and public workflow recovery guidance | Atelier-owned product behavior | `docs/product/cli-surface.md`, help text, workflow policy, readiness checks | Agent Factory may honor the policy, but it should not be the only durable place that defines it. |

For the current binding, this means the retained `Sources` and tracker-shape
material are repository binding; the retained model-routing, worktree, and
assignment rules are orchestration-only; and the former command-family cookbook
(`Mission`, `Work/evidence`, `Issues`, `Sync/state`, `Health`, and mission
completion detail) should be routed to Atelier-owned help, docs, status, audit,
lint, doctor, export, and workflow-policy surfaces.

## Default Proof By Issue Type

| Issue type | Default proof expectation | Escalate when |
| --- | --- | --- |
| `task` | Focused diff inspection plus the narrow checks named by the task. Docs-only tasks usually need the documentation diff, Markdown whitespace check, tracker lint for the item, and export freshness when tracker state changed. | The task changes process policy, public docs that must match help output, workflow gates, Agent Factory guidance, or another artifact that future work depends on. |
| `feature` | Positive and negative behavior proof through focused tests, command transcripts, screenshots, or docs/help parity checks for the user-visible surface. | The feature changes a public command/API contract, crosses storage/projection/workflow boundaries, or broad green tests could miss the claim. |
| `story` | End-to-end scenario proof for the operator workflow, including the important success path and rejected or blocked path. | The story spans multiple commands, records, UI views, or mission criteria, or claims a parent-level user outcome. |
| `bug` | A failing-before and passing-after reproduction when practical, plus a regression test or transcript that shows the defect no longer appears. | The bug involved data loss, stale projection, migration, closeout, safety, security, or behavior that is hard for the implementer to disprove alone. |
| `validation` | An independent validator classifies each relevant claim as `pass`, `fail`, `blocked`, `deferred`, or `not-applicable` and attaches first-class evidence. | Validation discovers new defects, stale tests, missing proof, or ambiguous criteria; create follow-up implementation or artifact-update issues instead of silently widening scope. |
| `closeout` | A closeout worker maps every parent Outcome or validation criterion to linked work, attached evidence, and workflow validation output. | Epic and mission closeout always require independent validation. Mission closeout also requires adversarial validation by a worker that did not implement the slices being closed. |
| `spike` | A durable decision, investigation result, or artifact update that names what was learned, what remains unknown, and what work is now unblocked. | The conclusion commits the repository to a migration, public contract, architecture boundary, workflow policy, or costly sequencing choice. |

## Evidence Destination

Use the lightest durable record that still lets a later operator inspect the
claim without private context.

| Destination | Use when | Not enough for |
| --- | --- | --- |
| Durable issue note | Handoff context, caveats, skipped optional checks, small observations, or a trivial docs-only change where the issue Evidence section explicitly says no separate proof artifact is meaningful. | Required proof for behavior changes, parent closeout, validation items, process-policy changes, or workflow validators that require evidence. |
| First-class evidence record | Any non-trivial proof: command transcripts, focused tests, migration results, docs/help parity, workflow validation, closeout audit tables, screenshots, or `fail`, `blocked`, `deferred`, and `not-applicable` classifications that should survive handoff. Attach it to the accountable issue-shaped work that produced or validated the proof. Parent readiness reads those child links through implementation, review, validation, and closeout issues. | Proof that must be performed independently by another worker, or unresolved defects that need their own owner. |
| Separate validation issue | Independent judgment is required, proof is broad enough to be its own work, or the implementer should not validate their own claim. Link it as validation or a blocker before relying on its result. | Tiny local checks where the issue worker can produce objective proof directly on the implementing issue. |

## Evidence Recording Contract

Use one evidence-recording workflow for manual summaries and command
transcripts:

```text
atelier evidence record --target issue/<id> --kind validation --result pass "summary"
atelier evidence record --target issue/<id> --kind test --result pass -- <command>
```

The target syntax is `<kind>/<id>`. New proof normally targets `issue/<id>` so
the accountable implementation, review, validation, or closeout issue owns the
claim. Parent closeout reads those child evidence links rather than accepting a
mission-level proof dump.

Evidence records should include accountable targets, proof scope, kind, result,
commands or artifacts, agent identity, independence level, residual risks, and
follow-up IDs. Capture mode preserves command, exit status, success, timestamp,
and bounded stdout/stderr summaries. Manual mode stores the same classification
fields without command output.

Examples:

| Scenario | Evidence shape |
| --- | --- |
| Command transcript | `target=issue/atelier-z1p8`, `proof_scope="Outcome: rejected command errors"`, `kind=test`, `result=pass`, `commands=[{command, exit_status, stdout_summary, stderr_summary}]`, `independence_level=implementer`. |
| Audit table | `target=issue/atelier-closeout`, `proof_scope="Epic outcome coverage"`, `kind=audit`, `result=pass`, `artifacts=[docs or evidence IDs]`, `independence_level=closeout`, `residual_risks=[]`. |
| Failed validation | `target=issue/atelier-validation`, `kind=validation`, `result=fail`, `summary` names the failed behavior, `commands` or artifacts show the failure, and `follow_up_ids` names the defect owner. |
| Deferred result | `target=issue/atelier-validation`, `kind=validation`, `result=deferred`, `summary` explains why proof is postponed, `residual_risks` names the risk, and `follow_up_ids` names the issue that must complete it. |

## Independent Validation Triggers

Create or use a separate validation or closeout issue when any of these apply:

- migration, schema, canonical record, projection rebuild, or runtime-state
  repair work;
- public command/API contracts, CLI help, docs/help parity, or user-visible
  workflow behavior;
- cross-cutting behavior in storage, export, rebuild, workflow validators,
  evidence, mission status, Agent Factory process, or Mission Control;
- epic, mission, milestone, or other parent-level closeout claims;
- stale-test, ignored-test, fixture drift, or broad-green-test risk;
- irreversible, security-sensitive, data-loss, or hard-to-reproduce behavior;
- subjective process claims where the implementer has a conflict of interest;
- any case where the Evidence section says a different role must validate the
  result.

When validation discovers a real defect, create or identify a follow-up issue
and leave the validation result as `fail`, `blocked`, or `deferred` rather than
editing the implementation in the validation item.

## Qualitative And Quantitative Validation

Qualitative pass/fail judgment is valid for subjective product, UX,
documentation, information hierarchy, and process clarity claims. The evidence
must name the evaluator role or perspective, the scenario or baseline inspected,
the decision rationale, and captured artifacts such as command output, diff
locations, screenshots, or notes that another reviewer can inspect.

Quantitative validation is required or strongly preferred for numerical claims:
performance, latency, query count, file count, output length, size, coverage,
error rate, flake rate, or reduction/increase claims. Evidence should include
the metric, baseline when available, measurement command or fixture, observed
result, and acceptable threshold or reason a hard threshold is not practical.

Do not over-specify subjective output before implementation. For example, an
information-hierarchy issue for `mission list` should name the user task,
important information to surface, and evaluator context; it should not mandate
the final row layout unless that layout is the contract. A performance issue
should be concrete: "reduce `mission status` wall time on fixture X from about
1.2s to under 500ms, proven by the benchmark command transcript" is stronger
than "make mission status faster."

## Example Routing Matrix

| Work item | Proof on the implementing item | Evidence destination | Independent validation |
| --- | --- | --- | --- |
| Docs-only issue | Documentation diff plus `git diff --check -- '*.md'`; run `atelier lint <id>` and `atelier export --check` when tracker records changed. | Durable note can be enough for typo-scale docs. First-class evidence is required for process policy or docs that gate later work. | Not required unless the docs define policy, closeout, public contracts, or docs/help parity. |
| CLI behavior change | Focused CLI integration test or human transcript for success and rejection paths; update docs/help proof when the surface changes. | First-class evidence attached to the issue. | Required for public command contract changes, docs/help parity, or cross-command workflow behavior. |
| Persistence migration | Migration diff inspection, round-trip or rebuild proof, deterministic export check, and degraded-state or recovery transcript when relevant. | First-class evidence attached to the issue and any affected parent criterion. | Required unless the migration is a throwaway fixture-only spike with no durable state effect. |
| Agent Factory process change | Diff of `AGENTFACTORY.md`, skill/process docs, or mapped quality docs plus a dogfood transcript showing the guidance is actionable through `atelier` commands. | First-class evidence for policy changes; durable notes only for local wording caveats. | Required when the process change affects validation, closeout, mission orchestration, or future worker behavior. |
| Epic closeout | Closeout issue maps each epic Outcome line to child work and evidence, uses `atelier issue show <epic-id>`, `atelier issue transition <epic-id> --options`, or the configured closeout check, and records residual risks. | First-class evidence attached to the closeout issue; the epic derives readiness from that linked closeout plus child evidence. | Always required, performed by a closeout or validation worker that did not implement the bulk of the children. |
| Mission closeout | Contract audit maps each mission validation expectation and linked epic outcome to evidence; run mission status, workflow validation, lint, doctor, export, docs/help parity, and stale-test inventory checks. | First-class evidence attached to the mission closeout issue; a direct mission link is retained only when the closeout workflow explicitly mirrors the same artifact for legacy closeout gates. | Always required, including an adversarial validation pass by a worker that did not implement the mission slices. |

Unavailable optional tooling should not be converted into an implicit failure.
When a repo-supported tool has an install path, install it and rerun before
handoff when the slice requires that check. When an advisory or
environment-specific tool is unavailable, record `deferred` evidence that names
the missing tool, the reason it is not required for the current slice, and any
follow-up owner.

Parent coverage summaries should classify each parent Outcome or validation
line as `covered`, `missing`, `failed`, `blocked`, `deferred`, or
`not-applicable`, then cite the accountable child issue IDs and evidence IDs.
Stable claim anchors are optional and reserved for high-risk or
automation-heavy closeout; ordinary issue work should stay readable and avoid
mandatory line IDs.

`atelier mission status` is the normal operator surface for mission state,
blockers, evidence gaps, next actions, and closeout readiness. `atelier mission
audit` is closeout drill-down: it exists to map mission validation expectations
and linked epic outcomes to child work and evidence, and may be folded into a
verbose closeout mode of mission status. Hidden workflow validators are
advanced diagnostics; use them only when the binding, assignment, or closeout
contract explicitly requires them, and attach the human-readable result to the
accountable issue that performed the check.

## Placement Examples

For a subjective `mission list` information-hierarchy task:

- Mission validation says the mission operator can identify state, blockers,
  missing proof, and next action without private context.
- Epic outcome says the mission operator CLI presents a concise default view
  with drill-down available for audit detail.
- Executable issue outcome says `mission list` output groups active missions by
  status and exposes blockers/evidence gaps in the default human output.
- The validation issue says an independent evaluator reviews the default output
  for a representative mission fixture, classifies the information hierarchy,
  records rationale, and attaches the transcript or screenshot.

For a quantitative performance task:

- Mission validation says operator commands remain fast enough for routine
  mission work.
- Epic outcome names the affected command family and delegates proof to
  benchmarked child issues.
- Executable issue outcome names the target metric, fixture, baseline, and
  acceptable threshold.
- The validation issue reruns the benchmark command, records environment and
  observed numbers, and classifies the result.

For a canonical write or projection-refresh issue:

- Mission validation says canonical Markdown remains the durable source of
  truth and rebuildable projections stay current.
- Epic outcome names the write/rebuild boundary and delegates proof to child
  round-trip and concurrency scenarios.
- Executable issue evidence names the command transcript, targeted test, or
  evidence record that proves early concurrent write, rebuild, and export
  freshness behavior.
- The validation issue replays the scenario or inspects the evidence before
  final closeout rather than waiting for a mission audit to discover projection
  drift.

## Commands

Install `cargo-nextest` before running the default Rust test command:
`cargo install cargo-nextest --locked`.

| Command | Owns |
| --- | --- |
| `git diff --check` | whitespace and patch hygiene |
| `git diff --check -- '*.md'` | Markdown whitespace hygiene |
| `cargo fmt -- --check` | Rust formatting |
| `cargo nextest run` | default Rust unit, integration, and smoke test suite |
| `cargo nextest run --profile extended --run-ignored=only` | opt-in extended property tests marked `prop_extended_` |
| `cargo test` | Cargo/libtest compatibility check |
| `cargo test --test cli_integration` | user-visible CLI behavior |
| `cargo test --test smoke_tests` | smoke scenarios |
| `atelier export --check` | canonical record and derived projection freshness |
| `atelier lint` | tracker structure |
| `atelier doctor` | tracker install, local runtime, diagnostics, and workflow health |

Rust hazard scan classifications live in
`docs/architecture/quality/rust-quality-hazard-scans.md`.

Workflow and closeout validation failures are command failures by default.
Mission completion is valid only when all linked work is closed, required
evidence is attached, configured transition gates pass, docs/help/Agent Factory
command guidance has no drift, stale obsolete-command tests are explicitly
owned or deferred, and the Git worktree is clean.
The `validation_criteria_satisfied` closeout gate is Atelier-owned: for mission
closeout it delegates to `atelier mission audit`, so missing parent coverage,
missing validation evidence, and linked epic outcome gaps surface through
mission status, audit, and closeout failure output rather than through Agent
Factory prose alone.

## Scenario Proof

- CLI behavior changes should include command-level tests or transcript evidence.
- Persistence changes should include RecordStore round-trip, projection rebuild,
  or runtime-state migration proof as appropriate.
- Export/rebuild changes should prove deterministic output and derived-state
  repair behavior.
- Workflow, validator, evidence, mission, milestone, or plan changes should
  include human-output transcript evidence and projection/rebuild proof when
  machine-readable state is involved.
- Mission closeout proof should show linked work closed, evidence attached to
  accountable child work, configured transition gates passing, and clean Git
  state.
- Agent Factory and tracker workflow validation should use human command output
  plus explicit drill-down commands. Do not rely on command-result `--json`;
  validate durable state with tracked `.atelier/` records, `atelier export
  --check`, `atelier lint`, `atelier doctor`, `workflow check`, focused `show`
  commands, `issue transition --options`, and mission status or audit output.
- Migration work should classify expected breakage and name reconnect or
  closeout ownership.

## Result States

- `pass`: the check or scenario completed and met its pass criteria.
- `fail`: the check ran and exposed a defect.
- `blocked`: the check could not run because a prerequisite is missing or broken.
- `deferred`: the check is intentionally postponed to a named follow-up owner.
- `not-applicable`: the check does not apply to the changed surface.
