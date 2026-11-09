---
schema: atelier.review
schema_version: 2
id: atelier-b4ih
title: approval on atelier-fad5 @82c1a766
target: atelier-fad5
identity: codex-reviewer (gpt-6.1-sol)
commit: 82c1a766e154c245422cfbb484de3deef18476b0
created_at: 2026-11-09T05:06:25.491095509Z
event:
  event: approval
---

Verdict: merge. The publishing slice makes merge and base publication share one exit status and keeps branch push, PR creation and CI waiting within atelier-vqoy.
Sizes: complexity medium (Git refresh, integration, land judgment and forge confirmation); value high (required publishing outcome); risk medium (publishes real base state, with explicit refusal and recovery paths).
Contract: pass. All six Done when lines have current passing integration coverage; docs accurately mark steps 2/3 and --wait as future work.
Fitness: pass. Existing merge gates and land judgment remain the owners of their checks, and the three reported defects are fixed.

Findings: none remaining; no FIX NOW recommendation.
Earlier R1 / atelier-kl16 (high): CommittedSnapshot::require_clean checks the captured base before fast-forward; the regression verifies ignored draft bytes, local base and source HEAD remain unchanged.
Earlier R2 / atelier-gfwz (medium): publication compares forge policy after fetch and against integration's first parent before pushing, refusing removal or changes. The fetched-base test covers removal and workflow replacement. The claim about a retry changing the forge is valid for the normal retry path: first_code_commit treats .atelier/workflow.yaml as code, so base_moved refuses rather than retries. This does not prove the defensive post-integration mismatch branch is globally unreachable; a concurrent base move between the outer check and the merge's initial capture can reach it, and the comparison fails safely.
Earlier R3 / atelier-sihf (medium): lookup uses branch/base, confirms an unpushed approval head's existing PR, and distinguishes no PR from a lookup error.

Proof independently run on 82c1a766: cargo nextest run -j 4 -p atelier-cli --test forge_ci --test merge_gates --test session: 259 passed, zero skipped; same with --features test-hooks: 263 passed, zero skipped. Used CARGO_TARGET_DIR=/home/supadava/.cache/atelier-target/fad5-review and TMPDIR=/home/supadava/.cache/atelier-tmp/fad5-review; read complete compact output. cargo fmt -- --check, git diff --check HEAD^ HEAD -- . ':!.atelier', git diff --check master...HEAD, cargo metadata --locked --format-version 1, and branch-built atelier check atelier-fad5 passed. git grep found no tail -1 under docs/product. Read all three earlier findings, the branch changes, requirements, ADR 0031, strategy and atelier-vqoy boundary. Read-only GitHub check confirmed PR 64 head 82c1a766 and successful Linux/macOS CI.

Open questions: none blocking.
Residual risk and tests not run: fake gh/bare-remote tests do not exercise live GitHub indirect-merge timing; no new live push, full workspace suite or cross-repository smoke run. The author's pre-fix red runs were not independently repeated under the read-only-code constraint; the assertions were compared with the pre-fix implementation. A base/head combination can remain untested until base CI, as ADR 0031 explicitly accepts.
Outcome: approve pinned head; this later approval resolves atelier-kl16, atelier-gfwz and atelier-sihf. Changes: review record only; code untouched, no push.
