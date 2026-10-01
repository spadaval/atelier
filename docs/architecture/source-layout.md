# Source Layout

The repository root is a virtual Cargo workspace. The product binary is
`atelier`, built by `atelier-cli`. Each crate owns a distinct boundary:

| Crate | Responsibility |
| --- | --- |
| `atelier-core` | Shared domain values, issues, relationships, and policy types. |
| `atelier-records` | Typed Markdown records, parsing/rendering, validation, IDs, persistence, and mutation locking. |
| `atelier-workflow` | Workflow schema parsing and structural policy validation. |
| `atelier-app` | Repository loading, direct record index, graph queries, classification, lint, policy resolution, and application use cases. |
| `atelier-cli` | Clap arguments, command dispatch, Git and shell adapters, identity, transcripts, and human output. |

CLI commands depend on application use cases and typed domain/record values.
Record persistence does not parse command output or depend on the CLI. Graph
queries use the direct index rather than a database adapter. Workflow checks
should return actionable invariant failures rather than perform side effects.

CLI integration and smoke tests live in `crates/atelier-cli/tests/`.
Crate-local unit tests cover their own types and rules. Fuzz targets remain in
`fuzz/`. Use [development setup](../product/development-setup.md) and
[quality standards](quality/standards.md) for the supported check commands.

There are no provider, review-room, SQLite cache, transition-action, import,
export, or telemetry subsystems. Historical root `src/` paths in dated audits
refer to the earlier monolithic layout.
