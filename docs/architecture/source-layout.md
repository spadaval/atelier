# Source Layout

Atelier's target source layout is a Cargo workspace with a virtual repository
root and internal crates under `crates/`. The root `Cargo.toml` owns workspace
membership and shared package metadata only; it must not define a root library
or binary package after the crate migration closes. The `atelier` executable is
owned by `crates/atelier-cli`.

The former single-crate root `src/` tree has moved under `crates/atelier-cli`.
During the rewrite, agents should use this document as the destination map and
treat remaining monolithic modules inside `atelier-cli` as code waiting to move
behind the lower target crate boundary.

## Target Workspace

| Crate | Responsibility | May depend on |
| --- | --- | --- |
| `atelier-core` | Shared domain types, record IDs, typed relationships, workflow status/category vocabulary, and pure formatting helpers that do not touch the filesystem, SQLite, Clap, or Git. | External pure utility crates only. |
| `atelier-workflow` | Repository-owned workflow policy loading, transition validation, readiness checks, and guidance text evaluation. | `atelier-core`. |
| `atelier-records` | Canonical `.atelier/` Markdown record discovery, parsing, deterministic rendering, ID allocation, relationship rendering, activity sidecar schema/IO, and atomic tracked-file mutation. | `atelier-core`, `atelier-workflow` when validation needs workflow policy. |
| `atelier-sqlite` | Rebuildable SQLite `ProjectionIndex` schema/query code for projection freshness and graph/search/list/status queries. It may project enough metadata to filter and order results, but it must not own canonical activity sidecar reads or writes. | `atelier-core`, `atelier-records`, `atelier-workflow`. |
| `atelier-app` | Use-case orchestration for mission, issue, issue-note/activity, evidence, workflow, status, doctor, export, lint, worktree, branch, and rebuild operations. It exposes request, outcome, and view-model APIs that are independent of Clap rendering. | `atelier-core`, `atelier-records`, `atelier-workflow`, `atelier-sqlite`. |
| `atelier-cli` | The `atelier` binary, Clap parser, command dispatch telemetry, terminal rendering, process exit mapping, and CLI-only transcript formatting. | `atelier-app`, plus lower crates only for types explicitly re-exported by `atelier-app` contracts. |

## Layering Rules

- Dependency direction flows from CLI to app to storage/workflow/domain crates.
  Lower crates must not call into `atelier-cli` or depend on Clap rendering.
- Rust crate APIs are internal to this repository. They may be replaced during
  the migration when doing so makes the crate boundary clearer.
- Tests and fuzz targets should move toward the lowest crate that owns the
  invariant being tested. CLI integration tests should stay in `atelier-cli`
  only when terminal behavior, Clap rejection, or exit-code mapping is the
  behavior under test.
- Temporary adapters are allowed only when tracked by explicit removal work.
  They must not become compatibility aliases, old-path re-exports, or staged
  deprecations unless a human explicitly requests a compatibility window.

## Workflow Ownership Contract

`atelier-workflow` is the single implementation owner for repository issue
workflow semantics. It owns:

- loading `.atelier/workflow.yaml` and rejecting missing, invalid, unknown, or
  deferred configuration;
- schema identity and version checks for `atelier.workflow`;
- status catalog parsing, status-category lookup, initial status lookup, and
  done-status membership;
- issue-type to workflow resolution;
- transition lookup from a record's current workflow status;
- required-field checks that are part of transition policy;
- built-in validator definitions, validator parameter parsing, and
  machine-readable validator results;
- transition guidance template evaluation; and
- branch policy derived from workflow configuration and the work
  graph.

`atelier-workflow` must not own terminal output, command parsing, Git command
execution, SQLite persistence, or canonical Markdown mutation. It returns typed
policy, transition, validator, guidance, and branch-lifecycle outcomes that
callers can render or apply.

`atelier-app` owns orchestration around those workflow APIs. It chooses when to
load policy for a use case, combines workflow results with RecordStore writes,
ProjectionIndex queries, Git/worktree checks, evidence lookup, lint/export
checks, and mission readiness, then returns request/outcome/view-model structs.
It may not duplicate workflow policy tables, hard-code status categories, or
reimplement transition availability rules as a parallel source of truth.

`atelier-cli` owns Clap command shape, process exit mapping, and terminal
rendering only. Commands such as `atelier issue transition <id> --options`,
`atelier issue transition <id> <transition>`, `atelier issue status
<objective-id>`, and `atelier issue list --ready` should render the
app/workflow outcomes they receive. CLI code may contain formatting labels such
as section headings, but not workflow policy decisions.

### Domain, Read Model, And Renderer Boundary

Domain and workflow crates return rule evaluations and observed state, not UI
annotations, human-facing prose, or command strings. For example, a workflow
service may report that the `close` transition is blocked because an
`EvidenceAttached { min_count: 1, kind: validation }` requirement observed zero
matching evidence records. It must not report a display annotation such as
`needs linked evidence`, choose a panel, assign a visual severity, or construct
`atelier evidence record ...`.

`atelier-app` assembles command-specific read models from those evaluations. A
read model may decide that status output should consider active issues,
checkout state, review state, and transition evaluations together, but it still
keeps domain facts typed: current status, available transitions, unsatisfied
requirements, observed counts, open blockers, review state, changed-file
classes, and stale health checks.

`atelier-cli` translates read models into operator language. It owns section
names, row wording, indentation, truncation, colors, footer labels, and command
spelling. Rendering code may say `close blocked: needs linked validation
evidence` because it is translating a typed unsatisfied requirement. It must
not rediscover that requirement by scanning evidence records or re-running
workflow checks outside the domain/app service that produced the read model.

The intended direction is:

```text
domain/workflow services
  -> typed state, requirements, observations, and rule evaluations

app/read services
  -> command-specific view models assembled from those typed facts

cli/renderers
  -> human wording, panels, layout, command suggestions, and exit mapping
```

This boundary avoids both failure modes: domain code pretending to be
presentation-neutral while emitting UI annotations, and CLI code duplicating
workflow or evidence logic just to render helpful output.

`atelier-records` owns durable issue status storage as canonical Markdown data.
When it validates or defaults a status during record creation or bundle apply,
it should ask `atelier-workflow` for the configured initial status and status
catalog instead of using a local lifecycle constant.

`atelier-sqlite` owns rebuildable projection fields used for filtering and
ordering. It may store status strings and indexed relationships, but category
classification and transition readiness remain workflow/app questions. SQLite
queries should not decide whether a transition is allowed.

Follow-on workflow extraction issues should be accepted only when command output
and tests show that:

- `workflow check`, `issue transition --options`, `issue transition`,
  `issue list --ready`, `issue status <objective-id>`, bundle status defaults, and closeout
  validators all consume the same `atelier-workflow` policy API;
- invalid statuses and missing workflow configuration fail through the shared
  workflow diagnostics rather than command-specific messages;
- CLI integration tests remain only for command shape, rendered transcript, and
  exit behavior; and
- lower-crate unit tests cover parsing, status categories, transition
  selection, validator results, guidance rendering, and branch policy.

## Temporary Adapter Policy

A temporary adapter is an internal shim that lets one migration slice compile
while its caller or callee is still moving to the target crate boundary.
Adapters are permitted only inside implementation-owned modules and only when
the owning issue records all of the following:

- the adapter name or search marker;
- the issue that owns removal;
- the condition that makes removal possible; and
- proof that the adapter is not a public CLI behavior, public Rust API promise,
  root-package re-export, old-command alias, or fallback compatibility path.

Closeout for the crate migration must inventory temporary adapters before
claiming the root package is deleted. Any remaining adapter must either be
removed, linked to an open removal issue that blocks closeout, or explicitly
classified as not an adapter because it is target-state code.

## Current Migration Map

| Current migration area | Target owner |
| --- | --- |
| `crates/atelier-cli/src/models.rs`, `crates/atelier-cli/src/record_id.rs`, pure shared helpers | `atelier-core` |
| `crates/atelier-cli/src/workflow_policy.rs`, `crates/atelier-cli/src/commands/issue_workflow.rs`, `crates/atelier-cli/src/commands/workflow.rs` policy internals | `atelier-workflow` |
| `crates/atelier-cli/src/record_store.rs`, `crates/atelier-cli/src/record_store/` | `atelier-records` |
| `crates/atelier-cli/src/projection_index.rs`, `crates/atelier-cli/src/db/`, projection/rebuild storage internals | `atelier-sqlite` |
| `crates/atelier-cli/src/commands/*.rs` use-case logic, doctor/lint/export/status orchestration | `atelier-app` |
| `crates/atelier-cli/src/main.rs`, Clap definitions, terminal output, command-surface tracing | `atelier-cli` |

## Orientation Order

When a scout needs to orient during the migration, read in this order:

1. This target workspace map and [ADR 0009](../adr/0009-virtual-workspace-root-and-cli-binary.md).
2. [`CONTEXT.md`](../../CONTEXT.md) for domain vocabulary.
3. The current `crates/atelier-cli/src/` modules only as migration input for
   the target owner named above.
4. The child tracker issue for the crate being moved, because it names the
   current proof and any temporary adapter removal owner.
