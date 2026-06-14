# Local Command Diagnostics

Atelier records command telemetry as local diagnostics, not as durable project
records. The diagnostics store exists to help operators find slow commands,
failed workflows, and stale projections without making agent sessions or raw
command interactions part of committed `.atelier/` records.

This policy unblocks command instrumentation while keeping
`atelier-000i` deferred: future run/session export can project from these local
facts, but this slice does not create durable run or session records.

## Storage Contract

The default diagnostics root is:

```text
${ATELIER_HOME:-$XDG_STATE_HOME/atelier}/diagnostics
```

When neither `ATELIER_HOME` nor `XDG_STATE_HOME` is set, Atelier uses:

```text
~/.local/state/atelier/diagnostics
```

This directory is global to the user account and intentionally outside both the
workspace `.atelier/runtime/` directory and committed `.atelier/` records.
Implementations must create it only when diagnostics are enabled and a command
has an event to write.

The first event log format is newline-delimited JSON:

```text
<diagnostics-root>/commands/YYYY-MM-DD.ndjson
```

Each line is one completed command event. Writes append a single JSON object and
must not rewrite previous events. If appending fails, the user-facing command
must continue unless the user explicitly invoked a diagnostics command whose
primary job is to read or maintain the diagnostics store.

## Controls

Diagnostics are enabled by default. Operators can disable them with either:

```text
ATELIER_TELEMETRY=off
ATELIER_DIAGNOSTICS=off
```

Accepted false values are `0`, `false`, `off`, and `disabled`, compared
case-insensitively. `ATELIER_TELEMETRY` is the public opt-out name;
`ATELIER_DIAGNOSTICS` is accepted for local clarity and tests. If either
variable disables diagnostics, no command event is written.

Operators can override the diagnostics root with:

```text
ATELIER_DIAGNOSTICS_DIR=/path/to/diagnostics
```

The override points at the diagnostics root itself, not at the `commands/`
subdirectory. A relative override is invalid and should produce an actionable
warning while disabling event writes for that command.

Verbose argument capture is disabled by default. It can be enabled for local
debugging with:

```text
ATELIER_DIAGNOSTICS_VERBOSE=1
```

Verbose mode is still local-only. It must not affect export, rebuild, lint,
Mission Control projection files, or committed tracker records.

## Command Event Fields

Every event uses `schema: "atelier.command_event"` and `schema_version: 1`.
The required fields and defaults are:

| Field | Type | Default or rule |
| --- | --- | --- |
| `schema` | string | Always `atelier.command_event`. |
| `schema_version` | integer | Always `1` for the first format. |
| `event_id` | string | Locally unique opaque ID; timestamp plus randomness is sufficient. |
| `command` | string | Top-level command path such as `issue show`, `doctor`, or an advanced diagnostic command. |
| `argv_redacted` | array of strings | Redacted command arguments. Empty unless verbose capture is enabled or a diagnostics command needs safe filters. |
| `argv_capture` | string | `none`, `redacted`, or `verbose`. Default `none`. |
| `started_at` | string | UTC RFC3339 timestamp recorded before command execution. |
| `finished_at` | string | UTC RFC3339 timestamp recorded after command execution. |
| `duration_ms` | integer | Wall-clock duration in milliseconds. |
| `exit_code` | integer or null | Process exit code when known. Null only when the process is interrupted before an exit code exists. |
| `result` | string | `success`, `failure`, or `interrupted`. |
| `workspace_id` | string or null | Stable hash of the workspace root path when a workspace is detected. |
| `workspace_root` | string or null | Redacted or omitted by default. Present only in verbose mode. |
| `state_path` | string or null | Redacted path to the canonical `.atelier/` record root when relevant. |
| `agent_id` | string or null | Local agent identity when available from runtime state or environment. |
| `phase_timings` | object | Named duration fields in milliseconds. Empty object when no phase timings are available. |
| `redaction` | object | Redaction policy metadata described below. |

`workspace_id` is not a durable project ID. It is a local grouping key for slow
command summaries. The initial implementation should compute it as a stable
SHA-256 hash over the canonicalized workspace root plus a fixed schema label,
truncated to at least 16 hex characters. It must not use random per-run values,
because summaries need to group events across workspaces and days.

`state_path` and `workspace_root` must never be required for grouping. Query
surfaces should prefer `workspace_id` and display human paths only when verbose
diagnostics explicitly captured them.

## Redaction Policy

Default diagnostics avoid sensitive arguments instead of attempting perfect
post-hoc redaction. In default mode:

- `argv_redacted` is empty;
- `argv_capture` is `none`;
- command names include only the command group and subcommand, not free-form
  titles, descriptions, comments, file contents, paths supplied as arguments,
  tokens, URLs, or shell fragments.

When redacted argument capture is explicitly needed for diagnostics commands,
the writer must replace values for these argument classes with stable labels:

- tokens, API keys, passwords, cookies, and authorization headers;
- URLs containing credentials or query strings;
- file paths outside the workspace root;
- issue descriptions, comments, notes, close reasons, and plan bodies;
- environment variable values;
- arbitrary positional arguments not recognized as safe enums, IDs, or numeric
  thresholds.

The `redaction` object must include:

| Field | Type | Rule |
| --- | --- | --- |
| `mode` | string | `default`, `redacted`, or `verbose`. |
| `dropped_args` | boolean | True when any raw arguments were omitted. |
| `path_policy` | string | `hash`, `omit`, or `verbose`. |
| `notes` | array of strings | Short policy labels, not raw values. |

Verbose mode may include raw argument strings and paths for local debugging, but
it must mark `redaction.mode` as `verbose` and must not be used by tests that
assert the default privacy posture.

## Retention

Atelier retains command diagnostics for 30 days by default. The retention window
can be overridden with:

```text
ATELIER_DIAGNOSTICS_RETENTION_DAYS=<positive integer>
```

`0` means keep only the current day's log. Negative values and non-integers are
invalid and should fall back to the 30-day default with an actionable warning.

Retention cleanup is opportunistic. A command may delete old
`commands/YYYY-MM-DD.ndjson` files before or after appending the current event,
but cleanup failure must not fail the user-facing command. Diagnostics query
commands may expose cleanup warnings because diagnostics are their primary
surface.

The retention policy applies only to local diagnostics logs. It does not govern
evidence artifact retention, committed `.atelier/` records, or future
exported run/session records.

## Local-Only Versus Exported Data

Local-only diagnostics data:

- command event logs under the diagnostics root;
- raw, redacted, or verbose argument captures;
- workspace path hints and workspace hashes;
- agent identity hints from local runtime state;
- slow-command summaries computed from local logs.

Exported or committed data:

- none from command diagnostics in this slice;
- no changes to tracked `.atelier/` records;
- no Mission Control `runs[]`, `agents[]`, or command-performance fields until a
  later issue defines a projection contract.

`atelier lint`, `atelier doctor`, `doctor --fix`, and any advanced export or
rebuild diagnostics must not require the diagnostics store to exist. A missing
or unreadable diagnostics store may be reported by future diagnostics commands,
but it is not tracker corruption.

Mission Control may eventually consume aggregate diagnostics such as slow
command counts or last-seen agent hints. That projection must use summaries,
not raw command logs, and must be added by an explicit follow-up contract.

## Operator Boundary

Diagnostics JSON is for inspecting Atelier itself. It is appropriate for local
telemetry, command-performance analysis, debugging diagnostics storage, and
future tools that summarize command behavior. It is not the contract for normal
project operation.

Normal mission, issue, validation, and closeout workflows must use
human-oriented operator surfaces: `atelier status`, `atelier mission status`,
`atelier issue transition <id> --options`, `atelier lint`, `atelier doctor`,
and `atelier evidence record`. Agents and scripts must not parse diagnostics
JSON to choose ready work, decide blockers, prove validation, infer evidence
coverage, or close work.

This boundary applies even though diagnostics query commands intentionally emit
stable JSON. That JSON is stable for local diagnostic tooling, telemetry review,
and performance follow-up work. It must not appear in ordinary Agent Factory or
operator recipes for mission selection, issue readiness, blocker triage,
validation proof, evidence coverage, or closeout readiness.

## Slow Command Query Defaults

The slow-command query surface owned by follow-up work should read local event
logs and produce stable JSON summaries. The default query window is the last
7 days, capped by the retained logs on disk. The default slow threshold is
1000 ms.

The command surface is:

```bash
atelier diagnostics slow
atelier diagnostics slow --days 14 --threshold-ms 250
```

The output is JSON with `schema: "atelier.slow_commands"`,
`schema_version: 1`, the selected `window_days`, the selected `threshold_ms`,
and a deterministic `rows` array sorted by `workspace_id`, `command`, and UTC
day bucket. Missing diagnostics logs produce an empty `rows` array.

Summary rows should group by:

- `workspace_id`;
- `command`;
- time bucket, initially UTC calendar day.

Each summary row should include:

- `count`;
- `failure_count`;
- `min_duration_ms`;
- `max_duration_ms`;
- `mean_duration_ms`;
- `p50_duration_ms`;
- `p95_duration_ms`;
- `first_started_at`;
- `last_finished_at`.

The query output must not include raw arguments unless verbose diagnostics were
explicitly requested for the query command and the local logs contain verbose
events.

## Implementation Notes

Instrumentation should wrap both mutating and read-only command handlers so
successes, failures, disabled diagnostics, redaction behavior, and slow-command
queries can be tested without special-case command paths.

Phase timings are optional per command. When available, use stable snake_case
keys such as `parse_ms`, `load_state_ms`, `projection_check_ms`,
`record_write_ms`, `sqlite_write_ms`, `export_check_ms`, and
`render_output_ms`. Unknown future keys are allowed, but values must remain
non-negative integer millisecond durations.

The event writer should be isolated from canonical record writes. It must never
hold a transaction or lock required by `RecordStore`, `ProjectionIndex`, or
runtime work association while performing diagnostics IO.
