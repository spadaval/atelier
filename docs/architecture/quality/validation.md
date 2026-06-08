# Validation

## Commands

| Command | Owns |
| --- | --- |
| `git diff --check` | whitespace and patch hygiene |
| `git diff --check -- '*.md'` | Markdown whitespace hygiene |
| `cargo fmt -- --check` | Rust formatting |
| `cargo test` | full Rust unit, integration, and smoke test suite |
| `cargo test --test cli_integration` | user-visible CLI behavior |
| `cargo test --test smoke_tests` | smoke scenarios |
| `atelier export --check` | tracker export freshness |
| `atelier lint` | tracker structure |
| `atelier doctor` | tracker runtime and rebuild health |

## Scenario Proof

- CLI behavior changes should include command-level tests or transcript evidence.
- Persistence changes should include database round-trip or migration proof.
- Export/rebuild changes should prove deterministic output and stale-export
  detection.
- Workflow, gate, evidence, mission, milestone, or plan changes should include
  machine-readable result examples when JSON output is involved.
- Migration work should classify expected breakage and name reconnect or
  closeout ownership.

## Result States

- `pass`: the check or scenario completed and met its pass criteria.
- `fail`: the check ran and exposed a defect.
- `blocked`: the check could not run because a prerequisite is missing or broken.
- `deferred`: the check is intentionally postponed to a named follow-up owner.
- `not-applicable`: the check does not apply to the changed surface.
