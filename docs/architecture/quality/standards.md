# Standards

## Rust

- Format with `cargo fmt` and check with `cargo fmt -- --check`.
- Use `cargo nextest run` as the default Rust test runner. Use `cargo test` for
  libtest-specific behavior, `--no-run`, reproduction, or unavailable nextest;
  state the reason in the handoff.
- Run checks appropriate to the changed claim; add focused CLI integration
  coverage for user-visible commands and invariant rejection paths.
- Use `anyhow::Context` on fallible IO and external-command boundaries.
- Keep documented authored input and deterministic rendering stable unless a
  reviewed migration deliberately changes their contract.
- Follow the [product specification](../../product.md) for command output
  and refusals: dense Markdown, and a refusal that names the way out.

## State and boundaries

- Treat Markdown records as durable repository state. Parse them directly.
- Validate records before writing them; `merge` alone takes the merge lock.
- Use blockers for sequencing.
- Keep read-only checks separate from Git mutation commands.
- Atelier commits only what its acts write, as the
  [design record](../../design.md) lists; other records are committed with
  Git, with the related implementation.
- Derive integration from Git; do not persist a competing merge state.
- Keep reviews unchanged once they have a verdict, except for settling a
  deferred finding, and keep merge checks governed by the base branch's
  reviewed `config.toml`.

## Documentation and process

Update the [product specification](../../product.md) for what Atelier does
and the [design record](../../design.md) for how it is built and for each
consequential choice, edited in place. Keep dated historical material
distinct from current guidance.

Use [Agent Factory](../../../skills/agent-factory/SKILL.md) for planning,
review, validation, and handoff. Non-trivial executable changes require
independent review before local merge, and a branch that changes more than
tracker records also needs a passing CI run on its pull request first.
Agents may push reviewed work, including `master` after a gated merge;
force-pushing `master` or deleting remote branches requires the human, as
recorded in [AGENTS.md](../../../AGENTS.md).
