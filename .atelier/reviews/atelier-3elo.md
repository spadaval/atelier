---
schema: atelier.review
schema_version: 2
id: atelier-3elo
title: finding on atelier-ym3z @f445b302
target: atelier-ym3z
identity: sol-ym3z-r1
commit: f445b3022d1f828d44f69e36add71c58ec13a8ad
created_at: 2026-11-10T23:16:43.114252380Z
event:
  event: finding
  recommendation: fix_now
---

Medium: next/crates/atelier-cli/src/check.rs:118: R1: Preserved executable shell hooks without a shebang stop working. Git falls back to the shell for these hooks, but Command::spawn does not. In an offline scratch repository, a mode-0755 pre-commit containing only `echo own-hook-ran >&2` and `exit 0` ran successfully on an actual Git commit before setup (exit 0). After setup moved the unchanged script to pre-commit.local, the same kind of commit failed (exit 1) with `it could not start: Exec format error (os error 8)`, and the kept hook never ran. The common runner affects all three hook kinds: commits and pushes can be blocked, and post-checkout work can be skipped. Preserve Git's shell fallback on ENOEXEC, with the same arguments and stdin, and add a regression test driving Git before and after setup. Recommendation: FIX NOW; this breaks the explicit requirement that a clone's existing hook keeps working.

Verdict: merge after R1 is fixed and independently re-reviewed; approval withheld now. Setup serves a required orientation and guard path, and no material simplification is needed.
Sizes: complexity medium (shared hook installation, chaining, and one reusable JSON merger); value high (required acceptance outcomes); risk medium (changes existing hooks and harness settings across clones, visible and reversible).
Contract: fail on existing-hook preservation, R1. The named acceptance scenarios otherwise passed in fresh scratch repositories. Architecture changes match the ordinary implementation, and the Report identifies the copied merging code and Codex SessionStart event.
Fitness: fail until the kept-hook runner matches Git execution semantics. Each added moving part traces to a requirement: hook identification and Step support repeat setup and preservation; Harness selects the requested project files; merge_hooks preserves existing settings; replace avoids partial file writes; the external-path refusal enforces repository-only writes.

Proof at f445b302: offline cargo build --locked -j 4 -p atelier-cli passed; cargo fmt -- --check and cargo metadata --offline --locked --format-version 1 passed in next/ with CARGO_BUILD_JOBS=4. Both the brief's HEAD^ whitespace check and the entire branch's diff --check passed. cargo nextest run --offline --locked --build-jobs 4 --test-threads 4 -E 'not binary(setup)' passed all 189 selected tests. Tests used TMPDIR=/home/supadava/.cache/atelier-fcb7/tmp. Manually reran the setup acceptance paths under ym3z/review/: shared hooks and refusals in two worktrees; exact scripts and modes; repeat setup and status; actual local pre-push arguments/stdin, refusal and rule ordering; occupied .local byte/mode preservation; diagnostic-only post-checkout failure; relative symlink and non-executable hook preservation; recovery after a prior move; external core.hooksPath refusal; Claude merge, invalid config and symlink refusal; Codex SessionStart and idempotence. Also confirmed symlinked hooks directories and dotdot paths outside the clone refuse safely. All scratch repositories were removed. No code was edited and no CI job steps were run.

Open questions: none that block disposition beyond R1.
Residual risk and tests not run: the committed setup test binary was not run because its fixtures are created directly under TMPDIR rather than the mandated review directory; its acceptance paths were exercised manually there instead. No new real Codex or Claude session was launched under the offline constraint. The recorded Codex transcript and evidence were read, not independently regenerated. No network or destructive CI validation was attempted.
