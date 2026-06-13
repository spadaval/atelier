# Mission Log Insights

This note summarizes patterns found by mining recent Codex mission transcripts
from `/root/.codex` after the long Atelier implementation runs on June 11-13,
2026. The review focused on repeated mistakes, friction, and confusion that
could be reduced by improving repository docs, the Agent Factory skill, or the
`atelier` CLI.

## Method

- Built compact transcript packets under `/tmp/atelier_codex_mining/` from the
  relevant `~/.codex/sessions` JSONL files.
- Reviewed the controller and subagent logs for missions `atelier-dvxc`,
  `atelier-2nc9`, and `atelier-tcmr`.
- Split the review across read-only audit subagents:
  - `atelier-dvxc` and `atelier-2nc9` mission/subagent packets.
  - `atelier-tcmr` controller packets.
  - Cross-cutting command, status, proof, and handoff friction.
- Compared findings with the current docs map, Agent Factory binding, and ready
  follow-up mission `atelier-19mc`.

## High-Value Findings

### 1. Record-type identity was too easy to misuse

Agents repeatedly passed mission IDs to issue commands first, then recovered
after `Issue <id> was not found`. This happened in both `atelier-2nc9` and
`atelier-tcmr` controller logs.

Improvement:

- CLI: make record lookup type-aware for common read commands, or make the
  error say when the ID exists as another record kind.
- Docs: clarify when to use `mission show/status` versus `issue show`, and when
  epics are executable work versus parent validation scope.

Confidence: high.

### 2. Installed CLI drift broke mission orientation

During schema and readable Markdown changes, the installed `atelier` binary
lagged behind the repository state. Agents had to discover whether to use
installed `atelier`, `target/debug/atelier`, or `cargo run`, and sometimes hit
unrelated compile failures while trying to mutate tracker state.

Improvement:

- CLI: add stale-binary or schema-version diagnostics when the installed binary
  cannot read current canonical records.
- Docs and Agent Factory: keep the rule sharp: use installed `atelier` for
  normal tracker work; use a rebuilt local binary only when validating changed
  CLI behavior or when installed binary/schema drift is proven.

Confidence: high.

### 3. Projection and runtime rebuild races created false blockers

Parallel agents triggered stale projection reports, rebuild temp-file noise,
journal-file issues, readonly database errors, and rebuild contention. Some
commands required retries or manual interpretation even though the canonical
Markdown state was recoverable.

Improvement:

- CLI: make projection rebuilds atomic and lock-aware; filter runtime temp and
  journal artifacts from user-facing diagnostics.
- CLI: provide one clear recovery path for runtime/projection failures.
- Agent Factory: tell read-only auditors to record runtime/cache failures as
  friction evidence rather than branching into broad repair unless necessary.

Confidence: high.

### 4. Worktree and active-work state were not reliable enough for subagents

Subagents sometimes completed issue work but could not finish because the shared
checkout was dirty, active work moved to another issue, or a worktree setup
failed after partially mutating lifecycle state.

Improvement:

- CLI: make `worktree for` atomic; do not claim/start work until runtime
  association succeeds.
- CLI: add a first-class repair command for clearing or reconciling stale active
  work association.
- Agent Factory: require isolated issue worktrees for mutating subagents unless
  the assignment explicitly says otherwise.
- Agent Factory: standardize subagent handoff to include result, issue ID,
  subskill, changed files, evidence IDs, commands run, dirty state, blockers,
  commit/branch, and follow-up recommendation.

Confidence: high.

### 5. Mission readiness and closeout were split across too many surfaces

Controllers repeatedly stitched together `atelier status`, `mission status`,
`mission audit`, `workflow check`, `lint`, `doctor`, `export --check`,
evidence commands, and manual issue drill-down. The information was usually
available, but the path to a closeout decision was noisy.

Improvement:

- CLI: make `mission status` the normal operator surface for state, blockers,
  missing proof, stale projection, next action, and closeout readiness.
- CLI: fold mission audit output into contextual status or verbose/advanced
  modes.
- Docs and Agent Factory: keep `workflow check`, `issue transition --options`,
  and mission closeout surfaces as the documented proof path.

Confidence: high.

### 6. Evidence gates were directionally right but operationally weak

Independent validation found at least one item close-ready with broad pass
evidence even though the issue requested specific transcripts, inventory,
docs/help parity, and focused tests. Agents also had to choose between notes,
manual evidence, command-backed evidence, and attach flows.

Improvement:

- CLI: unify evidence recording so manual and command-backed proof can target
  and attach in one flow.
- CLI: closeout should require qualifying proof for the relevant Outcome or
  Evidence requirement, not merely any linked pass evidence.
- Docs and Agent Factory: strengthen the ordinary-note versus first-class
  evidence rule with concrete examples.

Confidence: high.

### 7. Concurrency validation arrived too late

Closeout validation exposed issues such as fixed temp filenames, `.md.tmp`
visibility during validation, and projection refresh races. These were found
late, after substantial implementation had already happened.

Improvement:

- Repo validation docs: require early concurrency/scenario validation for
  canonical write, projection refresh, worktree, and runtime-cache changes.
- Agent Factory: route broad persistence/runtime changes to an independent
  validator before final closeout, not only at mission end.

Confidence: high.

### 8. Repo shape assumptions were stale

Agents repeatedly searched nonexistent paths such as `crates/` or guessed old
command module names. This caused avoidable orientation churn.

Improvement:

- Docs: keep `docs/architecture/index.md` explicit about actual current source
  layout, including that Atelier is currently a single Rust crate.
- Agent Factory: tell implementation scouts to start from the architecture map
  and `rg --files`, not inherited assumptions from prior Rust workspaces.

Confidence: high.

### 9. Normal command surface still leaks competing models

The logs show friction around overlapping commands and concepts:
`issue create --template` versus `--issue-type`, `issue update --status closed`
versus `issue close`, `dep` versus `link` versus `graph`, and normal workflow
guidance still mentioning advanced validator commands.

Improvement:

- CLI: collapse normal workflows around fewer verbs and keep old or diagnostic
  surfaces out of the default path.
- Docs: make the public command taxonomy explicit and keep help text, product
  docs, and Agent Factory guidance synchronized.

Confidence: medium-high.

### 10. Main-agent orchestration still absorbed too much bounded work

The long `atelier-tcmr` runs used thousands of shell calls in controller
threads. The controller did use subagents, but still performed many bounded
implementation, validation, transcript capture, and fixture-review loops
directly.

Improvement:

- Agent Factory: delegate bounded evidence-producing work earlier.
- Agent Factory: document model routing: cheaper/faster models are appropriate
  for bounded validation, transcript capture, fixture repair, docs drift scans,
  straightforward validation, stale-test inventory, and basic refactor-style
  implementation; higher-reasoning models remain appropriate for ambiguous
  architecture, hard debugging, cross-cutting implementation, complex review,
  and adversarial closeout.

Confidence: medium-high.

## Existing Follow-Up Alignment

The current ready mission `atelier-19mc` already matches most of these
insights. Its epics cover:

- collapsing mission operator CLI into contextual status;
- redesigning evidence capture and proof coverage;
- improving Agent Factory delegation and model routing;
- defining strong proof and contract-first workflow;
- simplifying core command surfaces around user workflows.

That mission should remain the primary vehicle for turning these findings into
implementation work. The gaps most worth ensuring it covers explicitly are:

- type-aware record lookup or better wrong-kind ID errors;
- stale installed-binary/schema diagnostics;
- atomic worktree setup and active-work repair;
- projection rebuild contention and temp-file filtering;
- standardized subagent handoff output.
