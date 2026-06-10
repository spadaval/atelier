# Standards

## Rust

- Run `cargo fmt` before handing off code changes.
- Prefer `anyhow::Context` on fallible IO, database, and external-command
  boundaries.
- Keep command output stable when tests or agents depend on it.
- Follow the human-output grammar in
  [Human CLI Output](../human-cli-output.md) for non-JSON detail, queue, and
  hierarchy views.
- Add or update CLI integration tests for user-visible command behavior.
- Add focused database tests for schema, migration, transaction, and invariant
  changes.
- Preserve JSON compatibility unless a migration bead or ADR explicitly changes
  the contract.

## Data And State

- Treat SQLite as runtime state and deterministic exported files as the target
  durable repo state.
- Do not merge SQLite databases through Git.
- Any canonical projection or rebuild change must define how stale exports are
  detected.
- Use typed links for semantic relationships; reserve dependencies for real
  sequencing.

## Documentation

- Update `SPEC.md` when product intent changes.
- Update `CONTEXT.md` when terminology or ambiguity decisions change.
- Add ADRs for costly, surprising, or repeatedly relevant architecture choices.
- Keep current target-state docs separate from historical rationale.

## Agent Workflow

- Track work in Atelier.
- Use explicit noninteractive `atelier issue` commands.
- Record follow-up work as Atelier issues.
- Commit `.atelier-state/` changes with related tracker updates; treat
  `.atelier/state.db` as rebuildable local runtime state.
- Include validation evidence in handoff notes when checks are skipped, fail, or
  only partially cover the change.
