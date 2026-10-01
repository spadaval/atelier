# Repository Map

This file is a map to the repository's durable guidance. Keep process and
product policy in the linked sources instead of duplicating it here.

## Current Mission

Atelier is being rebuilt as a verified, repo-native tracker. Governing sources:

- Strategy: `docs/strategies/atelier-rebuild.md`
- Native mission: `.atelier/issues/atelier-kgcr.md`
- Decision: `docs/adr/0019-verified-repo-native-tracker.md`

Current product and architecture docs describe the rebuilt model. The mission
record owns execution state; strategy acceptance does not mean mission completion.

## Process Sources

- Agent Factory skill: `.agents/skills/agent-factory/SKILL.md`
- Tracker state: `.atelier/` (records are committed with the code they
  describe)
- Start a session with `target/debug/atelier prime`; before handoff commit
  canonical records and run `target/debug/atelier land`. Active work may remain
  open, with unfinished code handed off honestly.
- Use `target/debug/atelier setup claude --binary target/debug/atelier` for the
  supported Claude Code hooks while the installed binary remains old.

## Merge Policy

- Agents may merge independently reviewed work into `master` locally without
  asking. Unreviewed non-trivial changes never reach `master`.
- Pushing to any remote requires the human.

## Product Sources

- Product intent: `PRODUCT_INTENT.md`
- Domain language: `CONTEXT.md`
- Documentation map: `docs/index.md`
- Architecture: `docs/architecture/index.md`
- ADRs: `docs/adr/`
- Code standards: `docs/architecture/quality/standards.md`

## Command References

- Installed CLI: `atelier` (old model; not updated until the rebuild is stable)
- Local debug binary: `target/debug/atelier`
- Formatting: `cargo fmt -- --check`
- Rust tests: `cargo nextest run`
- Failure-path tests: `cargo nextest run --features test-hooks`
- Whitespace check: `git diff --check`
