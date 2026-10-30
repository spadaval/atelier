# Standards

## Rust

- Format with `cargo fmt` and check with `cargo fmt -- --check`.
- Use `cargo nextest run` as the default Rust test runner. Use `cargo test` for
  libtest-specific behavior, `--no-run`, reproduction, or unavailable nextest;
  state the reason in the handoff.
- Default tests exclude injected failure hooks. Use
  `cargo nextest run --features test-hooks` for rollback, cleanup-failure, and
  concurrency checks. Normal release builds omit fault-injection environment
  hooks.
- Run checks appropriate to the changed claim; add focused CLI integration
  coverage for user-visible commands and invariant rejection paths.
- Use [Rust hazard scans](rust-quality-hazard-scans.md) for debt markers,
  dead code, large functions, and unused dependencies when applicable.
- Use `anyhow::Context` on fallible IO and external-command boundaries.
- Keep documented authored input and deterministic rendering stable unless a
  reviewed migration deliberately changes their contract.
- Follow the [human output contract](../../product/human-cli-output.md) for
  normal command views and actionable errors.

## State and boundaries

- Treat Markdown records as durable repository state. Parse them directly.
- Validate graph and policy before mutating records; coordinate canonical
  writers with the mutation lock.
- Use typed links for semantic relationships and blockers for sequencing.
- Keep read-only checks separate from explicit Git mutation commands.
- Never create commits in Atelier. Commit records with related implementation.
- Derive integration from Git; do not persist a competing merge state.
- Keep evidence honest and commit-bound, reviews append-only, and merge gates
  governed by the base branch's reviewed policy.

## Documentation and process

Update Product Intent for direction, Context for terminology, and the owning
product or architecture page for behavior. Add ADRs for consequential choices.
Keep dated historical material distinct from current guidance.

Use [Agent Factory](https://github.com/spadaval/.agents/blob/main/skills/agent-factory/SKILL.md) for planning,
review, validation, and handoff. Non-trivial executable changes require
independent review before local merge. Agents may push reviewed work, including
`master` after a gated merge; force-pushing `master` or deleting remote branches
requires the human, as recorded in [AGENTS.md](../../../AGENTS.md).
