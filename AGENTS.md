# Agent Instructions

## Start Here

- Use the `agent-factory` skill for planning, orchestration, implementation,
  review, validation, documentation, audit, and readiness work. The repository
  copy lives at `.agents/skills/agent-factory/SKILL.md`.
- Use Atelier (`atelier`) for durable task tracking. Do not replace it with private
  chat notes or ad hoc TODO files.
- Treat Atelier command output as the source of process guidance. Use
  `atelier status`, `atelier issue show`, `atelier issue transition --options`,
  and `atelier mission status` to decide the next lifecycle action; do not infer
  review, branch, provider, merge, or completion steps from general agent habits,
  available remotes, or installed tools.
- Orchestrators assign exactly one agent-factory subskill to each subagent.
- Do not use interactive tracker commands; use explicit `atelier issue`
  commands instead.
- Do not plan or validate work by parsing command-result JSON. Use focused
  show/list/ready/status/transition commands and drill down explicitly.
- Treat `.atelier/` canonical Markdown and tracked config as the committed
  durable tracker state. Do not edit ignored Atelier runtime/cache files; use
  normal Atelier commands, and run repair commands only when Atelier reports
  local state needs repair.
- Do not add compatibility aliases, staged deprecations, fallback behavior, or
  old-command shims unless a human explicitly asks for them. Prefer hard
  removal, direct committed-state migrations, and docs/tests for only the new
  behavior.
- Issues must be actionable. For important choices, create artifact-update
  tasks and block dependent implementation on those tasks when needed.
- Validation criteria must name observable completion behavior: command output,
  rejected commands, help text, file content, tests, lint checks, or
  evidence records.

## Repository Shape

- Agent Factory skill source: `.agents/skills/agent-factory/SKILL.md`
- Product intent: `SPEC.md`
- Domain language: `CONTEXT.md`
- Documentation map: `docs/index.md`
- Product behavior: `docs/product/index.md`
- Architecture: `docs/architecture/index.md`
- Quality and validation: `docs/architecture/quality/`
- ADRs: `docs/adr/`
- Validation router: `docs/architecture/quality/validation.md`
- Code standards: `docs/architecture/quality/standards.md`

## Common Commands

Prefer the installed `atelier` command for normal tracker work. Use
`target/debug/atelier` only when testing local CLI changes that have not been
installed yet. Recompile first when that binary is missing or stale relative to
Rust sources/manifests; use `cargo run -- ...` only when a one-off rebuild plus
execution is specifically useful.

```bash
cargo fmt -- --check
cargo nextest run
cargo nextest run --profile extended --run-ignored=only
git diff --check
atelier issue list --ready
atelier lint
```

## Current Product State

Atelier is being built from the Chainlink Rust CLI codebase. Treat `SPEC.md` as
the target product intent and remove inherited behavior when it conflicts.
