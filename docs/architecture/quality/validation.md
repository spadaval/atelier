# Validation

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
| `atelier doctor` | tracker install, cache, projection-rebuild, diagnostics, and runtime health |

`atelier workflow validate` failures are command failures by default. Mission
completion is valid only when all linked work is closed, required evidence is
attached, workflow validators pass, and the Git worktree is clean.

## Scenario Proof

- CLI behavior changes should include command-level tests or transcript evidence.
- Persistence changes should include RecordStore round-trip, projection rebuild,
  or runtime-state migration proof as appropriate.
- Export/rebuild changes should prove deterministic output and stale durable
  state or stale projection detection.
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
