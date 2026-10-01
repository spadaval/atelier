# Repository Map

This file is a map to the repository's durable guidance. Keep process and
product policy in the linked sources instead of duplicating it here.

## Current Mission

Atelier is being rebuilt as a verified, repo-native tracker. Read these first;
they override older product docs where they conflict:

- Strategy and temporary plan: `docs/strategies/atelier-rebuild.md`
- Decision: `docs/adr/0019-verified-repo-native-tracker.md`

`PRODUCT_INTENT.md`, `CONTEXT.md`, and most of `docs/product/` still describe
the superseded design until the rebuild's docs step rewrites them.

## Process Sources

- Agent Factory skill: `.agents/skills/agent-factory/SKILL.md`
- Tracker state: `.atelier/` (records are committed with the code they
  describe)

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
- Whitespace check: `git diff --check`
