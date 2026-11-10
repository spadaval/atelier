---
schema: atelier.review
schema_version: 2
id: atelier-6es7
title: approval on atelier-pz1y @9a5e7e94
target: atelier-pz1y
identity: sol-pz1y-r4
commit: 9a5e7e9411ac224308934b7667b4c38afaabfe51
created_at: 2026-11-10T22:50:12.182057344Z
event:
  event: approval
---

Verdict: merge. Moving Git-action guards into the three hooks meets the requested outcome with no unnecessary layer.
Sizes: complexity medium (aggregate loader, staged reads and first-parent push inspection); value high (the required Git guards); risk medium (Git commands in other repositories can be refused, with visible diagnostics and recoveries).
Contract: pass. Reviewed the branch-owned commits with git log -p --reverse 6de2edc7..9a5e7e94 ^master, all seven Done when lines, Report and architecture changes. R1-R6 are resolved: orphan reviews are refused on their own paths, uncommitted config is listed, ignored records are read, unread-only pushes name history repair, done recovery removes the bad commit, and unreadable checkout config gets the unskippable diagnostic and recovery.
Fitness: pass. The loader and is_record own record selection and aggregate diagnostics; staged uses the existing commit reader; status transitions use the existing act table; only first-parent commits newly reaching the configured base are examined. No unsupported moving part found.
Findings: none. No fix_now recommendation remains.
Fresh proof at 9a5e7e9411ac224308934b7667b4c38afaabfe51: requested git diff --check HEAD^ HEAD -- . :!.atelier passed at the root; cargo fmt -- --check and cargo metadata --locked --format-version 1 passed in both root and next. In next, cargo build --locked --all-targets -j 4, cargo clippy --locked -j 4 --all-targets -- -D warnings, and cargo nextest run --locked --build-jobs 4 -j 4 all passed. Read the full nextest output: 182 passed, 0 skipped, including all 11 real-Git hook scenarios and the prior-fix regression cases. Cargo was offline; tests used the prescribed TMPDIR and local remotes.
Five extra scratch probes passed: a path-only commit uses Git's temporary index despite an unrelated broken staged item; a ready-to-draft mission commit is refused; config-only changes are listed without failure; ignored malformed mission and review files are both reported; post-checkout with missing config returns 0. Scratch files removed; no code or shared hooks changed.
Open questions: none. Residual risk and checks not rerun: setup and ATELIER_SKIP belong to later items; this approval covers the hook modes, not hook installation or bypass integration. I reran the all-targets build and suite in the worktree, not the destructive CI scratch-clone script or the author's mutation/performance experiments.
