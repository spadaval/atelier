---
schema: atelier.review
schema_version: 2
id: atelier-kkd6
title: approval on atelier-ym3z @9eb8e89b
target: atelier-ym3z
identity: sol-ym3z-r2
commit: 9eb8e89b1cc6f099a691b01d4e69767bdcdffc4e
created_at: 2026-11-10T23:21:41.625544356Z
event:
  event: approval
---

Verdict: merge. Shared Git guards and session orientation are required outcomes, and this implementation is worth its complexity; no smaller change would meet the preservation and harness requirements.

Sizes: complexity medium (shared-hook installation, chaining, and a reusable JSON merger); value high (all six Done when outcomes); risk medium (changes hooks and project settings across clones, with visible refusals and preserved originals).

Contract: pass. Independently inspected the branch commits and current implementation against every Done when line. The 11 setup scenarios passed freshly: shared hooks and refused commits in two worktrees, repeat setup and exact scripts/modes, kept pre-push arguments/stdin and refusal ordering, occupied .local preservation, symlink/non-executable handling, diagnostic-only post-checkout failure, interrupted-install recovery, external hooksPath refusal, Claude settings preservation and idempotence, and Codex SessionStart configuration. The Report names the copied old-workspace merger, documents Codex 0.167 SessionStart, and includes a real fixture-session transcript. Architecture changes match the implementation.

Fitness: pass. No unsupported moving part found: exact script identification and Step implement idempotence and preservation; Harness selects the required project settings; merge_hooks preserves existing handlers; replace prevents partially written files; the hooksPath refusal enforces shared repository-local installation. The ENOEXEC fallback in the common kept-hook runner fixes atelier-3elo, retains arguments/stdin, and its Git-driven regression passes. This approval follows that fix and resolves the prior fix-now finding.

Findings: none. No new fix_now, defer, wont_fix or no_action record is needed.

Proof at 9eb8e89b1cc6f099a691b01d4e69767bdcdffc4e:
- git diff --check HEAD^ HEAD -- . ':!.atelier' and the whole branch's diff --check: passed.
- In next/, offline cargo fmt -- --check and cargo metadata --locked --format-version 1: passed, with CARGO_BUILD_JOBS=4 (these subcommands do not take -j).
- TMPDIR=/home/supadava/.cache/atelier-fcb7/tmp cargo nextest run --locked --offline --build-jobs 4 --test-threads 4: all 200 tests passed, 0 skipped; full output read.
- Offline cargo clippy --locked --offline -j 4 --all-targets -- -D warnings: passed.
- Before each cargo command, checked pgrep -c -x cargo; each reported 0. Commands ran sequentially, building only in next/target/.
- An additional fixture under ym3z/review/ ran four simultaneous setup processes from two worktrees for 100 attempts; the original pre-commit hook remained preserved in every attempt. This is a bounded stress observation, not proof against every possible interleaving.
- No file under crates/ changed. No code was edited, no CI job steps or run_next_job.py ran, and no command reached the network. Scratch files and the review directory created for this round were removed.

Open questions: none blocking.

Residual risk and tests not run: no fresh real Codex or Claude session was launched under the offline constraint. The recorded Codex fixture transcript was read and assessed, not independently regenerated; the Claude events are configured and tested as settings, without a live harness session. The author's mutation and destructive-CI claims were read, not rerun. No approval is claimed for those observations beyond the fresh checks above.

Outcome: approve this head. Changes: review record only on atelier-ym3z. Review: independent second implementation review, with atelier-3elo verified fixed. Open items: live-harness verification limits stated above; see the item's Report for the recorded session transcript.
