---
schema: atelier.review
schema_version: 2
id: atelier-ty1k
title: approval on atelier-ygul @f6bcf614
target: atelier-ygul
identity: codex-reviewer (gpt-6.1-sol)
commit: f6bcf614e0c535efc02eb100331830a463ee2fc1
created_at: 2026-11-09T18:44:49.473114544Z
event:
  event: approval
---

Verdict: merge. The change stops unrelated mission replans on the base from blocking done validation and mission claims, while related strategy edits and other code changes still invalidate approval. Sizes: complexity medium (historical issue graphs and scope classification), value high (two real land refusals), risk high (approval freshness can silently accept stale code). Contract: pass. Fitness: pass.

Independently reviewed all five commits in 3d3d251c..f6bcf614 excluding master, the complete branch diff and surrounding approval, scope, history and output logic. No new findings or fix_now recommendations. The scope-intersection predicate fixes R2 (atelier-ezxy): a parent approval now treats its child's mission as related. The historical graph lookup also retains R1's fix (atelier-5pk7). This approval pins f6bcf614e0c535efc02eb100331830a463ee2fc1 and resolves both earlier fix_now findings after their fixes. The revised Done when matches the diagnosed real failures; the exemption stays limited to unrelated mission strategies on the base.

Proof rerun in the requested worktree with CARGO_TARGET_DIR=$HOME/.cache/cargo-target/ygul-review, TMPDIR=$HOME/.cache/ygul-review-tmp and four build/test jobs: cargo nextest run -j 4 -p atelier-app -p atelier-cli passed 987 tests; the same with --features test-hooks passed 997. Both runs include the real-case regression, own-strategy refusal, unlink/prune stability, the parent-approval regression on new and historical claims, and retained_stale_approvals_never_age_into_acceptance. Read all emitted output. git diff --check HEAD^ HEAD excluding .atelier, whole-branch whitespace, cargo fmt -- --check, and cargo metadata --locked --format-version 1 all passed. cargo build --locked -j 4 -p atelier-cli passed; scripts/smoke-repos.sh --new with the review build found no regression across five repositories. Independently reran installed and branch land on the preserved fl12-close and v3nm-at-26186c06 histories: installed refuses both, branch passes both. The preserved parent-approval and unlink repros pass check but correctly refuse land, naming the actual stale strategy commit. atelier land atelier-ygul passed on this active branch.

Changes: one approval record only; no code edits, branch switch, rebase, merge or push. Open questions: none. Residual risk: Linux local proof only; macOS and CI were not run. CI and the normal merge gates remain required before integration.
