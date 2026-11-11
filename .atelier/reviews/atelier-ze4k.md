---
schema: atelier.review
schema_version: 2
id: atelier-ze4k
title: approval on atelier-oci2 @e1886010
target: atelier-oci2
identity: GPT-6.1 Sol independent reviewer
commit: e1886010f56198192ce0cde987a7c85326e3db0b
created_at: 2026-11-11T04:28:35.254972930Z
event:
  event: approval
---

Verdict: merge. The specifications remove the tracker site from the cutover path while preserving restoration of the old site on rollback.
Sizes: complexity low (four bounded prose corrections, no new runtime parts); value high (required input to step F); risk low (documentation and proof only, reversible).
Contract: pass. Checked all six Done when lines against the full branch diff and resulting documents. Workspace prose names four crates and excludes atelier-workflow and atelier-site. Product commands and removed commands, record readers, log, Site, Transition and open questions match the contract and ADR 0032. Words, Lifecycle, Review, both last_changed lines and Transition are unchanged from branch base 565f321f. The prescribed old-workspace diff is empty.
Fitness: pass. No code changes or unnecessary moving parts. No findings recommended fix_now, defer, no_action or wont_fix.
Proof rerun at e1886010: next/ metadata lists atelier-app, atelier-cli, atelier-core and atelier-records, with no atelier-site in the dependency resolve. Tracked next/ has no build.rs, package.json, pnpm files or symlinks; source and dependency build-script scans found no Node tool references. With TMPDIR=$HOME/.cache/atelier-qr5s/tmp/oci2 and PATH=$HOME/.cargo/bin:$HOME/.cache/atelier-qr5s/tmp/oci2/nonode-bin, node, nodejs, npm, npx, pnpm, pnpx, corepack and yarn were each confirmed absent. In next/, timeout 540 cargo build --locked -j 4 --target-dir target/review-oci2-no-node rebuilt dependencies and all four workspace crates successfully in 7.21s. Its binary --help has no serve. CI next job was read and has no Node step; its destructive removal step was not run.
Proof: cargo nextest run -j 4 -p atelier-app -E 'test(this_repositorys_strategies_parse_without_problems)' passed 1 test, 127 skipped. cargo fmt -- --check passed in both workspaces; cargo metadata --locked --format-version 1 passed in both, with CARGO_BUILD_JOBS=4 for these non-build subcommands. git diff --check HEAD^ HEAD -- . ':!.atelier' and the substantive review-range whitespace check passed.
Open questions: none. Residual risk: this is proof of the current tracker build and specification corrections, not execution of the future cutover or rollback. The strategy test reads these specifications but neither has a Success Criteria section to parse; their correctness was inspected directly. Full old-workspace tests were intentionally not run. No service, branch, source or personal details were changed by this review.
