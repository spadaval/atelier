# Codex Mission Log Insights, 2026-06-14

This report summarizes patterns mined from recent long-running Codex mission
threads for Atelier. The goal is not a blow-by-blow history. It is a short list
of repeated mistakes, stumbling blocks, and improvement opportunities for:

- repository docs;
- the Agent Factory skill and binding;
- the `atelier` CLI.

## Scope And Method

The source material was the local Codex session store under `/root/.codex`,
especially `sessions/2026/06/**.jsonl`, `session_index.jsonl`, and
`history.jsonl`.

The analysis focused on Atelier sessions from June 11-14, 2026 whose working
directory was `/root/atelier`, especially controller threads that began with
prompts like `Complete the mission ...` plus their subagent threads. Compact
packets and command tables were generated under
`/tmp/atelier_codex_mining_20260614/`:

- `session_summary.tsv`: per-session counts and first real user prompt;
- `command_events.tsv`: extracted shell command calls and exit codes;
- `error_keyword_events.tsv`: failed or suspicious tool output samples;
- `packet_*.md`: compact per-mission review packets;
- `aggregate.md`: session, command, failure, and retry estimates.

Four read-only Agent Factory `audit` subagents reviewed disjoint packet slices:

- `atelier-dvxc`, `atelier-2nc9`, and `atelier-n8ag`;
- `atelier-ufgn`, `atelier-5rhu`, and June 12 subagent packets;
- `atelier-z04a`, `atelier-man9`, and June 13 subagent packets;
- command failure and retry metrics.

The command counts are approximate. Codex JSONL stores some command output with
embedded control characters or multi-line payloads, so the tables are useful for
trend analysis rather than exact telemetry.

## Quantitative Snapshot

The focused aggregate covered 155 Atelier sessions.

| Metric | Estimate |
| --- | ---: |
| Shell command calls in focused sessions | 16,734 |
| Nonzero shell command exits in focused sessions | 1,033 |
| Approximate failed-command rate | 6.2% |
| Broad retry heuristic hits | 57 |
| Strict immediate same-command repeats in broader raw stream | 27 |

A broader stitched command stream contained 26,925 parsed command records and
1,515 failures. That broader number includes noisier records, so the focused
aggregate above is the better mission-level estimate.

Common failed command families in the broader stream:

| Command family | Failures |
| --- | ---: |
| `rg` search commands | 264 |
| `cargo test` | 145 |
| `cargo fmt -- --check` | 139 |
| `atelier export --check` | 97 |
| `atelier lint` | 75 |
| `atelier issue show` | 59 |
| `cargo nextest run` | 58 |

Output signatures were dominated by:

- CLI usage and argument errors, including many `Usage:` outputs;
- missing files, missing commands, and wrong paths;
- stale or invalid tracker state, including stale projection indexes and
  invalid canonical Markdown;
- Rust format, compile, and test failures;
- worktree, dirty-state, and lock-contention failures.

## Repeated Findings

### 1. Agents guessed command surfaces too often

Across the June 12 and June 13 packets, agents repeatedly tried commands that
were removed, not yet implemented, or belonged to another model:

- `atelier workflow check`;
- `atelier finish`;
- `atelier close`;
- `atelier archive`;
- `atelier session`;
- `atelier timer`;
- `atelier milestone`;
- `atelier current-work`;
- `atelier issue new`;
- `atelier work start`;
- `atelier history`;
- `atelier prime`;
- `atelier work queue --blocked`.

Some of this was caused by active migration, but the repeated pattern suggests
that current help and docs do not make the supported command taxonomy obvious
enough under pressure.

High-value improvement:

- Docs: add one authoritative command map for normal mission, issue, worktree,
  evidence, workflow, and health flows. Include removed or intentionally absent
  commands only as "do not use; use X instead" notes where confusion has been
  observed.
- Agent Factory: require workers to stop after the first clear
  `unrecognized subcommand` or wrong-family error and consult the command map,
  instead of probing adjacent names.
- CLI: for removed or likely command names, return suggestions such as
  `workflow check was removed; use issue transition <id> --options or mission
  status <id> for the normal operator path`.

### 2. Mission, issue, and evidence IDs were easy to misuse

Controllers and workers repeatedly passed mission IDs to issue commands or used
the wrong evidence relation:

- `atelier issue show atelier-2nc9` and `atelier issue show atelier-n8ag`
  failed because those IDs were missions, not issues.
- `atelier mission status` and `atelier mission start` were tried in sessions
  where those surfaces had not yet landed or had different semantics.
- `evidence attach ... --role validation` failed because the role did not match
  the accepted relation vocabulary.

High-value improvement:

- CLI: make lookup failures kind-aware. If an ID exists as a mission, an issue
  command should say that directly and name the correct command family.
- Docs: show a compact record-kind matrix: mission commands, issue commands,
  evidence commands, and what each ID kind can be attached to.
- Agent Factory: add a preflight reminder that epics and missions are parent
  scope, while issues own executable work and ordinary proof.

### 3. Projection freshness became a normal blocker

Projection and canonical-state errors were not rare. They blocked `issue show`,
`issue update`, `issue close`, `workflow validate`, `export --check`, `lint`,
`work start`, and `work finish`.

Observed signatures included:

- `Projection index is stale`;
- `Canonical export is stale`;
- `Canonical tracker Markdown is invalid`;
- `.atelier-state is not rebuild-ready`;
- missing `data` front matter;
- unsupported canonical files such as `.atelier/workflow.yaml` or local runtime
  artifacts being treated as projection inputs;
- runtime projection database open failures in worktree paths.

High-value improvement:

- CLI: make stale-state and invalid-canonical errors name one recovery path in
  order: inspect the offending file, run the correct rebuild/export command,
  rerun the original command.
- CLI: filter runtime/cache/temp/journal files from canonical diagnostics.
- CLI: make projection rebuilds atomic and lock-aware so parallel agents do not
  convert recoverable freshness into command churn.
- Docs: clearly distinguish canonical Markdown success from projection refresh
  success. Agents need to know when a durable write landed even if the local
  projection needs repair.

### 4. Source layout assumptions were stale

Agents repeatedly searched nonexistent paths such as `crates/` or guessed old
module names like `src/commands/dep.rs`, `src/commands/update.rs`,
`src/rebuild.rs`, `src/db.rs`, and `src/commands/issue.rs`.

High-value improvement:

- Docs: keep `docs/architecture/index.md` explicit about the current crate and
  module layout. Name where command dispatch, projection, record storage,
  workflow policy, evidence, and worktree behavior live today.
- Agent Factory: tell implementation scouts to start from the architecture map
  and `rg --files`, not inherited Rust workspace assumptions.
- CLI/docs: keep examples free of paths that no longer exist.

### 5. Shell and validation command hygiene wasted cycles

Many command failures were self-inflicted but repeatable:

- `cargo test` was often invoked with multiple positional test names, causing
  `unexpected argument` errors.
- `rg` patterns were malformed or interpreted as flags.
- Backticks inside shell strings triggered accidental command substitution,
  such as `/bin/bash: line 1: issue: command not found`.
- `python` was assumed to exist even though only `python3` was available.
- `cargo fmt -- --check` failed repeatedly after edits, then was rerun several
  times before formatting was applied.

High-value improvement:

- Agent Factory: document safe validation command forms:
  - one positional `cargo test` filter at a time, or use `cargo nextest run -E`
    for expressions;
  - use single quotes around `rg` patterns containing shell metacharacters;
  - use `python3`, not `python`;
  - run formatting before bundled validation.
- Repo docs: add a small "validation command recipes" section in the quality
  docs with known-good examples.
- CLI: where possible, provide `atelier` wrappers for common health checks so
  workers do not have to compose long fragile shell pipelines.

### 6. Worktree ownership and active-work state were unclear

The logs show repeated friction around:

- commands run from the wrong checkout;
- `.atelier-worktrees/<issue>` paths without usable runtime database state;
- dirty worktrees blocking `start`, `finish`, or workflow transitions;
- concurrent Cargo cache and artifact-directory lock waits;
- uncertainty over which checkout owned validation after subagent edits.

High-value improvement:

- Agent Factory: mutating subagents should normally use isolated issue
  worktrees, and the assignment should name the validation checkout.
- Agent Factory: add a handoff field for "validation ran from root checkout" or
  "validation ran from issue worktree".
- CLI: make `worktree for` atomic and avoid claiming/starting work until the
  runtime association is valid.
- CLI: add a first-class active-work repair/reconcile command instead of making
  agents infer how to clear stale local state.

### 7. Proof and closeout gates were directionally right but not precise enough

Several packets showed proof-contract drift:

- legacy fields such as `acceptance` and `evidence_required` appeared beside
  newer `Description`, `Outcome`, and `Evidence` sections;
- `Missing string front matter key 'data'` and unknown mission sections showed
  that schema expectations were changing faster than records;
- at least one closeout path accepted broad pass evidence even though the
  issue's own Evidence text asked for more specific proof.

High-value improvement:

- CLI: closeout should qualify evidence against the relevant Outcome or Evidence
  requirement, not merely detect any linked pass record.
- CLI: unify manual and command-backed evidence capture into a single target
  and attach flow.
- Docs: show concrete examples of issue-local notes, first-class evidence, and
  independent validation evidence.
- Agent Factory: require workers to quote or paraphrase the specific Evidence
  line they proved when closing work.

### 8. Main orchestrators still did too much bounded work

The controller threads used subagents, but they still performed many bounded
search, fixture, validation, and transcript-capture loops directly. This made
the main threads long and raised the cost of context churn.

High-value improvement:

- Agent Factory: delegate bounded evidence-producing work earlier, especially
  docs drift scans, command-surface inventories, stale-test inventories,
  transcript capture, fixture repair, and focused validation.
- Agent Factory: keep higher-reasoning models for ambiguous architecture,
  hard debugging, public-contract redesign, and final adversarial closeout.
  Use Mini models for the bounded slices above, with explicit proof.

## Improvement Backlog

### Repository Docs

1. Add an operator command map that covers normal mission, issue, worktree,
   evidence, workflow, and health paths.
2. Add a record-kind matrix that explains which commands accept mission, issue,
   evidence, plan, and milestone IDs.
3. Update the architecture index with the current single-crate source layout
   and command ownership map.
4. Add validation command recipes for `cargo test`, `cargo nextest`, `rg`,
   formatting, lint, export, and doctor checks.
5. Add a canonical/projection state model page or section with the normal repair
   order for stale projections and invalid canonical Markdown.
6. Add proof examples that contrast issue notes, first-class evidence, and
   independent validation.

### Agent Factory

1. Add a stale-state preflight: if `lint`, `export --check`, or a tracker read
   reports canonical/projection invalidity, stop workflow mutation and repair
   state first.
2. Add a command-surface rule: after an unrecognized command, consult docs/help
   and do not probe neighboring names.
3. Add safe shell recipes for common validation commands.
4. Require validation checkout ownership in subagent handoff.
5. Require closeout proof to name the specific Outcome or Evidence line proved.
6. Encourage earlier delegation of bounded audit, transcript, docs drift,
   fixture, and focused validation slices to Mini models.

### Atelier CLI

1. Make wrong-kind ID errors type-aware and corrective.
2. Add suggestions for removed or likely command names.
3. Improve stale projection and invalid canonical-state error messages with a
   single recovery path.
4. Make projection rebuilds and worktree setup more atomic and lock-aware.
5. Add active-work repair/reconcile commands.
6. Tighten closeout gates so evidence is matched to the claim being closed.
7. Simplify evidence recording and attaching into one obvious command flow.
8. Keep runtime/cache/temp files out of canonical validation diagnostics.

## Highest-Leverage Next Steps

1. Ship CLI error improvements for wrong-kind IDs, removed commands, and stale
   projection recovery. These were the most repeated avoidable loops.
2. Update `docs/product/cli-surface.md`, `docs/architecture/index.md`, and
   `docs/architecture/quality/validation.md` with the command map, source map,
   validation recipes, and proof examples.
3. Update `AGENTS.md` and the Agent Factory orchestration guidance with
   stale-state preflight, safe validation recipes, and checkout ownership in
   handoffs.
4. Create follow-up tracker work for closeout proof matching and atomic
   worktree/projection repair if those are not already covered by the current
   backlog.
