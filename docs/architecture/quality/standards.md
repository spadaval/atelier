# Standards

## Rust

- Run `cargo fmt` before handing off code changes.
- Use `cargo nextest run` as the default Rust test runner. Use `cargo test`
  only for libtest-specific behavior, `--no-run`, reproducing standard
  test-harness output, or when `nextest` is unavailable, and state the reason
  in the handoff or evidence.
- Use [rust-quality-hazard-scans.md](./rust-quality-hazard-scans.md) for the
  classified debt-marker, dead/unused-code, large-function, and unused-
  dependency review commands.
- Prefer `anyhow::Context` on fallible IO, database, and external-command
  boundaries.
- Keep command output stable when tests or agents depend on it.
- Follow the human-output grammar in
  [Human CLI Output](../human-cli-output.md) for non-JSON detail, queue, and
  hierarchy views.
- Add or update CLI integration tests for user-visible command behavior.
- Add focused database tests for schema, migration, transaction, and invariant
  changes.
- Preserve documented authored-input and distinct derived JSON compatibility unless a
  migration bead or ADR explicitly changes that contract. Do not treat retired
  command-result JSON as the default automation boundary.

## Data And State

- Treat record files as durable repository state and SQLite as a disposable
  domain cache.
- Do not merge SQLite databases through Git.
- Any domain-cache rebuild change must define how changed record-file sources
  are detected and repaired.
- Use typed links for semantic relationships; reserve dependencies for real
  sequencing.

## Documentation

- Update `PRODUCT_INTENT.md` when product intent changes.
- Update `CONTEXT.md` when terminology or model choices change.
- Add ADRs for costly, surprising, or repeatedly relevant architecture choices.
- Keep current target-state docs separate from historical rationale.

## Agent Workflow

- Track work in Atelier.
- Use explicit noninteractive `atelier issue` commands.
- Record follow-up work as Atelier issues.
- Commit tracked `.atelier/` record changes with related tracker updates; treat
  `.atelier/runtime/state.db` as rebuildable local runtime state.
- Include validation evidence in handoff notes when checks are skipped, fail, or
  only partially cover the change.
