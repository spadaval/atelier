---
schema: atelier.review
schema_version: 2
id: atelier-iax9
title: approval on atelier-uwxy @55d18f46
target: atelier-uwxy
identity: reviewer-r3
commit: 55d18f463f79a4ac30d8481102637bebeb5d1795
created_at: 2026-11-11T06:13:31.280572345Z
event:
  event: approval
---

Verdict: merge. The Desk reading API gives the app its required commit-backed views without adding server dependencies to the tracker. Sizes: complexity medium (one HTTP boundary, response types and a commit-keyed cache); value high (required for the Desk app); risk low (read-only, validated on scratch repositories and the converted clone). Contract: pass. Checked every Done when line against c9fc2161..55d18f46, the implementation, generated types, tests, Report and docs/architecture/desk.md. Fitness: pass. The server uses atelier-app views and committed record readers, leaves domain rules with their owners, validates request IDs and paths, checks Host, and keeps API fallback separate from app files. Each added part serves a stated requirement. Findings: none. Independently confirmed fixes for atelier-xxwm and atelier-2t1h with their regression tests and live HTTP requests. Proof at the pinned head: git diff --check HEAD^ HEAD and the whole branch excluding .atelier passed; in next/, cargo fmt -- --check, cargo metadata --locked --format-version 1, cargo build --locked --all-targets -j 4, cargo clippy --locked --all-targets -j 4 -- -D warnings and cargo nextest run --locked -j 4 passed (266/266). scripts/ci-jobs.test.sh passed all 26 cases. cargo install --path crates/atelier-cli --locked -j 4 --root <fresh scratch directory> in next/ installed exactly one executable, atelier; scratch install removed. The manifest opts atelier-desk out through package.metadata.dist.dist = false. Served the converted clone at base 2116defd on 127.0.0.1:3784: all eight open mission routes returned 200 with title, status and counts equal to the new CLI show output; inbox needs_human IDs equalled list --needs-human. Item, document and done-mission routes and SPA fallback returned 200; /api, /api/, /api//repos and unavailable routes returned JSON 404; malformed queries returned JSON 400; a wrong method returned JSON 405; foreign Host returned JSON 403. The server was stopped and the converted clone stayed clean. Open questions: none. Residual risk and tests not run: no tailnet/browser or service-install validation, which belongs to later Desk work; no switched-tree release build, explicitly owned by atelier-u1ko. No code edits, merges or pushes.
