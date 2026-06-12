# Validation

## Proof Routing Policy

Proof starts on the work item that made the claim. The issue `Evidence`
section names the expected proof, and the worker records that proof before
closeout. Escalate by risk and scope: ordinary local work proves itself on the
issue, while risky, broad, or parent-level claims require independent
validation and first-class evidence.

Epics coordinate work; they are not the normal executable proof surface. Epic
and mission claims are proven by child issue evidence plus a closeout or
validation item that audits the parent outcome.

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
| First-class evidence record | Any non-trivial proof: command transcripts, focused tests, migration results, docs/help parity, workflow validation, closeout audit tables, screenshots, or `fail`, `blocked`, `deferred`, and `not-applicable` classifications that should survive handoff. Attach it to the issue, mission, milestone, or parent record it validates. | Proof that must be performed independently by another worker, or unresolved defects that need their own owner. |
| Separate validation issue | Independent judgment is required, proof is broad enough to be its own work, or the implementer should not validate their own claim. Link it as validation or a blocker before relying on its result. | Tiny local checks where the issue worker can produce objective proof directly on the implementing issue. |

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

## Example Routing Matrix

| Work item | Proof on the implementing item | Evidence destination | Independent validation |
| --- | --- | --- | --- |
| Docs-only issue | Documentation diff plus `git diff --check -- '*.md'`; run `atelier lint <id>` and `atelier export --check` when tracker records changed. | Durable note can be enough for typo-scale docs. First-class evidence is required for process policy or docs that gate later work. | Not required unless the docs define policy, closeout, public contracts, or docs/help parity. |
| CLI behavior change | Focused CLI integration test or human transcript for success and rejection paths; update docs/help proof when the surface changes. | First-class evidence attached to the issue. | Required for public command contract changes, docs/help parity, or cross-command workflow behavior. |
| Persistence migration | Migration diff inspection, round-trip or rebuild proof, deterministic export check, and degraded-state or recovery transcript when relevant. | First-class evidence attached to the issue and any affected parent criterion. | Required unless the migration is a throwaway fixture-only spike with no durable state effect. |
| Agent Factory process change | Diff of `AGENTFACTORY.md`, skill/process docs, or mapped quality docs plus a dogfood transcript showing the guidance is actionable through `atelier` commands. | First-class evidence for policy changes; durable notes only for local wording caveats. | Required when the process change affects validation, closeout, mission orchestration, or future worker behavior. |
| Epic closeout | Closeout issue maps each epic Outcome line to child work and evidence, runs `atelier workflow validate issue <epic-id>` or the configured closeout check, and records residual risks. | First-class evidence attached to the epic and closeout issue. | Always required, performed by a closeout or validation worker that did not implement the bulk of the children. |
| Mission closeout | Contract audit maps each mission validation expectation and linked epic outcome to evidence; run mission status, workflow validation, lint, doctor, export, docs/help parity, and stale-test inventory checks. | First-class evidence attached to the mission and closeout issue. | Always required, including an adversarial validation pass by a worker that did not implement the mission slices. |

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

`atelier workflow validate` failures are command failures by default. Mission
completion is valid only when all linked work is closed, required evidence is
attached, workflow validators pass, and the Git worktree is clean.

## Scenario Proof

- CLI behavior changes should include command-level tests or transcript evidence.
- Persistence changes should include RecordStore round-trip, projection rebuild,
  or runtime-state migration proof as appropriate.
- Export/rebuild changes should prove deterministic output and derived-state
  repair behavior.
- Workflow, validator, evidence, mission, milestone, or plan changes should
  include human-output transcript evidence and projection/rebuild proof when
  machine-readable state is involved.
- Mission closeout proof should show linked work closed, evidence attached,
  workflow validators passing, and clean Git state.
- Agent Factory and tracker workflow validation should use human command output
  plus explicit drill-down commands. Do not rely on command-result `--json`;
  validate durable state with tracked `.atelier/` records, compatibility
  `atelier export --check`, `atelier lint`, `atelier doctor`, and focused
  `show` or `workflow validate` commands.
- Migration work should classify expected breakage and name reconnect or
  closeout ownership.

## Result States

- `pass`: the check or scenario completed and met its pass criteria.
- `fail`: the check ran and exposed a defect.
- `blocked`: the check could not run because a prerequisite is missing or broken.
- `deferred`: the check is intentionally postponed to a named follow-up owner.
- `not-applicable`: the check does not apply to the changed surface.
