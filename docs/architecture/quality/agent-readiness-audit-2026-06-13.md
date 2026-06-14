# Agent Readiness Audit

Date: 2026-06-13

Validation issue: `atelier-1krs`

Mission: `atelier-man9`

Parent epic: `atelier-cve1`

Scope: independent readiness audit for fresh-agent entry, setup, task discovery,
validation/handoff commands, docs freshness, quality hazard tooling,
ownership/collaboration scaffolding, dependency hygiene, and secrets/env
onboarding.

Proof method: file inspection plus bounded operator command transcripts. No
source implementation was changed during this audit.

## Score

- Pass: 5
- Fail: 6
- Deferred: 1
- Skipped: 0

Overall readiness result: `fail`

The repository has a strong committed tracker/documentation spine and runnable
baseline health commands, but it is not yet ready for a fresh agent to set up
and hand off without private context because setup/env guidance, docs freshness
checks, quality hazard scans, and repo ownership scaffolding are still missing
or unclassified.

## Criterion Table

| Criterion | Class | Evidence | Existing owner or follow-up |
| --- | --- | --- | --- |
| Committed onboarding entry points exist and point to durable repository sources. | pass | `AGENTS.md:5-36` requires `AGENTFACTORY.md`, Atelier tracking, explicit issue commands, and durable `.atelier/` state. `AGENTFACTORY.md:7-32` names the docs map, product/architecture docs, and installed-binary rule. `docs/index.md:3-66` gives a committed documentation map. `atelier prime` printed the durable tracker state, ignored local state, essential commands, and repository notes. | none |
| Task discovery and work-selection paths are explicit and executable. | pass | `AGENTS.md:38-55` lists the common work/validation commands. `docs/product/cli-surface.md:43-78` and `:139-179` define `atelier prime`, `atelier status`, `atelier mission status`, and `atelier issue list --ready` as the normal operator surfaces. `atelier issue list --ready` returned a ready queue, and `atelier mission status atelier-man9` reported blockers, proof gaps, and next actions. | none |
| Baseline handoff/health tooling is present and runnable. | pass | `AGENTFACTORY.md:110-120` names `git diff --check`, `cargo fmt -- --check`, `cargo nextest run`, `atelier export --check`, `atelier lint`, and `atelier doctor`. `rust-toolchain.toml:1-3` pins Rust 1.95.0 with `rustfmt` and `clippy`. `Cargo.toml:28-44` declares normal and dev dependencies for the Rust CLI. `cargo fmt -- --check` passed. `atelier doctor` passed and reported `projection_fresh: ok` and `rebuild_ready: ok`. | none |
| First-class validation and evidence routing are durable and inspectable. | pass | `docs/architecture/quality/validation.md:5-22` requires attached proof on the accountable work item, `:99-112` defines validation/evidence destinations, and `:114-142` defines the evidence-recording contract. `docs/architecture/quality/agent-factory-atelier-validation.md:10-41` shows prior first-class validation through Atelier commands. | none |
| Duplicate and follow-up gap checks were performed before creating any new work. | pass | `atelier search "dev environment"` found `atelier-1vdl`; `atelier search "CODEOWNERS"` found `atelier-1xuf`; `atelier search "freshness checks"` found `atelier-c9ej`; `atelier search "quality hazard"` and `atelier search "unused dependency"` found `atelier-e723`; `atelier search "ownership boundary"` found `atelier-a0fd`. `find . ...` returned no `.github`, `.devcontainer`, `.env*`, `CODEOWNERS`, or `CONTRIBUTING*` files. `rg -n "cargo-(deny|machete|udeps)|cargo deny|cargo machete|cargo udeps|renovate|dependabot|CODEOWNERS|\\.env\\.example|devcontainer|TODO|FIXME" .` found only general TODO/FIXME checks in helper resources, not repo-level readiness guardrails. No new issue was needed. | none |
| Fresh setup is verifiable from committed repository files. | fail | `atelier-cve1` requires fresh-agent setup to be verifiable from committed files (`.atelier/issues/atelier-cve1.md:33-45`). Root `find` found no `README*`, `CONTRIBUTING*`, `.devcontainer`, or `.env*` file. `Cargo.toml:9` points the package readme at `../README.md`, but no root README is present in the repository search. There is no setup document in `docs/index.md:3-66`. | existing blocker `atelier-1vdl` (`.atelier/issues/atelier-1vdl.md:23-35`) |
| Secrets and environment-variable onboarding are explicit. | fail | No `.env*` file exists in the repo-root search. `Cargo.toml:30` enables Clap `env` support and `Cargo.toml:38` enables tracing subscriber `env-filter`, which implies an environment surface may matter, but no committed doc states whether normal development needs any local env or explicitly says that no secrets/env file is required. This inference is from dependency configuration plus missing docs. | existing blocker `atelier-1vdl` (`.atelier/issues/atelier-1vdl.md:27-35`) |
| Agent-facing docs have a freshness check that catches stale commands. | fail | `atelier-c9ej` exists specifically to add this guardrail (`.atelier/issues/atelier-c9ej.md:25-37`). `find scripts -maxdepth 3 -type f` found only `scripts/migrate_sqlite_comments_to_activity.py`; no dedicated docs-command freshness script was present. The readiness and quality docs inspected here do not name a command that validates AGENTS/AGENTFACTORY command examples end to end. | existing blocker `atelier-c9ej` (`.atelier/issues/atelier-c9ej.md:25-37`) |
| Factory-style quality hazard scans are documented and routed for normal use. | fail | `atelier-e723` exists because the repo still needs complexity, dead-code, TODO/FIXME, and unused-dependency scans (`.atelier/issues/atelier-e723.md:26-38`). The historical `rg` hazard scan found TODO/FIXME patterns only in helper resources under `resources/claude/...`; that resource tree is removed by `atelier-vau5`. The same scan found no `cargo deny`, `cargo machete`, `cargo udeps`, `dependabot`, or `renovate` usage. | existing blocker `atelier-e723` (`.atelier/issues/atelier-e723.md:26-38`), related cleanup issue `atelier-10qm` via `.atelier/issues/atelier-e723.md:13-16` |
| Repo ownership and collaboration scaffolding are present or explicitly classified. | fail | Root `find` found no `.github` tree, `CODEOWNERS`, or `CONTRIBUTING*`. `atelier-1xuf` exists because those items are not yet classified (`.atelier/issues/atelier-1xuf.md:23-35`). | existing blocker `atelier-1xuf` (`.atelier/issues/atelier-1xuf.md:23-35`) |
| Dependency update automation is present or explicitly deferred. | fail | The `rg` duplicate/hygiene scan found no `dependabot` or `renovate` configuration. `atelier search "dependency update automation"` mapped this exact gap to `atelier-1xuf`. The repo therefore lacks both the automation and the documented defer/not-applicable decision required by the readiness epic. | existing blocker `atelier-1xuf` (`.atelier/issues/atelier-1xuf.md:27-35`) |
| Atelier-owned versus Agent Factory-owned readiness rules are resolved enough for closeout. | deferred | `docs/architecture/quality/validation.md:67-89` defines the intended ownership split, but the repository-specific boundary decision is still active work. `atelier search "ownership boundary"` returned `atelier-a0fd`, and that issue is currently in review. Until that artifact lands, this criterion should not be scored as pass. | existing owner `atelier-a0fd` (`.atelier/issues/atelier-a0fd.md:35-47`) |
| Mission-level readiness gaps are already mapped to accountable work. | pass | `atelier-1krs` blocks `atelier-1vdl`, `atelier-1xuf`, `atelier-c9ej`, and `atelier-e723` (`.atelier/issues/atelier-1krs.md:9-18`). `atelier-cve1` links the full readiness workset, including the boundary artifact issue `atelier-a0fd` (`.atelier/issues/atelier-cve1.md:35-45`). Every real gap found in this audit already has an owner or in-flight artifact. | none |

## Duplicate / Follow-Up Checks

The audit did not create any new issue.

Duplicate/follow-up ownership was checked with:

- `atelier search "dev environment"` -> `atelier-1vdl`
- `atelier search "CODEOWNERS"` -> `atelier-1xuf`
- `atelier search "freshness checks"` -> `atelier-c9ej`
- `atelier search "quality hazard"` -> `atelier-e723`
- `atelier search "unused dependency"` -> `atelier-e723`
- `atelier search "dependency update automation"` -> `atelier-1xuf`
- `atelier search "ownership boundary"` -> `atelier-a0fd`
- `find . -maxdepth 3 ...` -> no `.github`, `.devcontainer`, `.env*`,
  `CODEOWNERS`, or `CONTRIBUTING*`
- `rg -n "cargo-(deny|machete|udeps)|cargo deny|cargo machete|cargo udeps|renovate|dependabot|CODEOWNERS|\\.env\\.example|devcontainer|TODO|FIXME" .`
  -> TODO/FIXME detection exists only in helper resources, not as repo-level
  readiness tooling

## Recommendations

1. Keep `atelier-1vdl`, `atelier-1xuf`, `atelier-c9ej`, and `atelier-e723` as
   blockers for readiness closeout; they cover every concrete fail found here.
2. Finish `atelier-a0fd` before treating docs/help or process-policy work as
   ready for closeout, because the ownership split still affects where durable
   readiness rules belong.
3. Do not open additional readiness issues from this audit unless a later run
   finds an unowned gap outside setup/env, docs freshness, quality scans, repo
   hygiene, or boundary ownership.
