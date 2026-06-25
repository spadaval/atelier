# Repository Map

This file is a map to the repository's durable guidance. Keep process and
product policy in the linked sources instead of duplicating it here.

## Process Sources

- Agent Factory skill: `.agents/skills/agent-factory/SKILL.md`
- Tracker state: `.atelier/`
- Tracker command guidance: `atelier man manager`
- Current work and lifecycle guidance: `atelier status`,
  `atelier work ready`, `atelier issue show <id>`, and
  `atelier issue transition <id>`

## Product Sources

- Product intent: `PRODUCT_INTENT.md`
- Domain language: `CONTEXT.md`
- Product principles: `docs/product/zen.md`
- Documentation map: `docs/index.md`
- Product behavior: `docs/product/index.md`
- Architecture: `docs/architecture/index.md`
- Quality and validation: `docs/architecture/quality/`
- ADRs: `docs/adr/`
- Validation router: `docs/architecture/quality/validation.md`
- Code standards: `docs/architecture/quality/standards.md`

## Command References

- Installed CLI: `atelier`
- Local debug binary: `target/debug/atelier`
- Formatting: `cargo fmt -- --check`
- Rust tests: `cargo nextest run`
- Extended ignored tests: `cargo nextest run --profile extended --run-ignored=only`
- Whitespace check: `git diff --check`
- Tracker check: `atelier check`
