---
schema: atelier.review
schema_version: 2
id: atelier-ptna
title: approval on atelier-zpmb @4a604b87
target: atelier-zpmb
identity: gpt-6.1-sol-r4
commit: 4a604b87443f4382a3d99e35a9666d01f835bc5d
created_at: 2026-11-09T18:00:27.743704583Z
event:
  event: approval
---

Verdict: merge. The commit-pinned, read-only mission API meets all eight Done when lines at 4a604b87443f4382a3d99e35a9666d01f835bc5d within the loopback-only, one-host scope.
Sizes: complexity high (new server, commit-tree loader and generated API contract); value high (required mission brief/debrief access); risk medium (read-only presentation, reversible failures).
Contract: pass. Fitness: pass. No new findings or open questions.
Reviewed the brief, branch changes, architecture page, acceptance fixtures and all five prior fixes. atelier-aodt reserves both existing and allocated URL names; atelier-oowv moves the semaphore permit into the blocking build through completion; atelier-cuq9 keeps only successful record loads, views and commit facts; atelier-esxa and atelier-iq33 distinguish failed strategy/config reads from absent files and prevent retaining those failed results. Their regression tests pass. Since 34b9970c only tracker records changed. Sumeet disposed atelier-01wq as wont_fix; transient Git failures or objects becoming unreadable during builds remain outside this approval, with no further hardening requested.
Fresh proof in this checkout: cargo fmt -- --check; cargo metadata --locked --format-version 1; git diff --check HEAD^ HEAD and across the whole branch outside .atelier, all pass. cargo nextest run -j 4 -p atelier-site -p atelier-app passes 124 tests; the CLI --test serve suite passes all 19 tests. These exercise generated TypeScript and JSON freshness, API/CLI parity, workflow-selected review, discovery and error isolation, divergent common-directory HEAD with base strategy/criteria/marks/receipt ages/generated_from, read-only repositories, cache replacement on base movement, and capped builds including dropped requests and wait timeout.
Fresh load proof using the debug binary built here and the existing load.sh: 20 concurrent cold-cache atelier-k8h8 debrief requests all 200, slowest 2.033 s, peak RSS 75.8 MB; T3 Code slowest 0.002 s over 19 probes; ten warm requests all 200, slowest 0.013 s. Recorded release load evidence and Report also inspected. Every cargo command used the requested TMPDIR and separate review-r4-target CARGO_TARGET_DIR; compilations and tests used -j 4.
Limits: targeted local checks only; no full workspace/test-hooks rerun or CI run in this review, no release rebuild, no browser/app review (the app is another issue). No code changed.
