---
schema: atelier.review
schema_version: 2
id: atelier-2t1h
title: finding on atelier-uwxy @3831974f
target: atelier-uwxy
identity: reviewer-r2
commit: 3831974f50a0443df40a31d19abaa9d314a02413
created_at: 2026-11-11T06:07:14.360310904Z
event:
  event: finding
  recommendation: fix_now
---

low: next/crates/atelier-desk/src/server.rs:132: R1: GET /api/ falls through to the app instead of returning a JSON 404. With a configured app directory containing index.html, the built binary at 3831974f returned 200 text/html and the index contents for /api/, while /api and /api/unknown returned 404 application/json with error. The /api/{*rest} wildcard does not match an empty rest, so /api/ reaches the ServeDir/index fallback on line 138. This violates the Done when requirement that the index fallback is for paths outside /api and docs/architecture/desk.md's unknown-route JSON 404 contract. Add an explicit /api/ no_route route, or otherwise contain the fallback outside the API namespace, and assert status, content type and error for /api/ with an app configured. Recommendation: fix_now. This is a current contract miss with a trivial routing fix, so it meets review.md's exception for a new finding during review of a low-severity fix.

Verdict: merge after R1 is fixed and independently reviewed; no approval now. The reading API is required by atelier-94pv and needs no shrink. Sizes: complexity medium (HTTP boundary, response mapping, commit cache, generated types); value high (required API for the app); risk low (read-only service, visible and reversible failures). Contract: fail at /api/ with an app configured; other checked requirements pass. Fitness: routing boundary needs R1; dependency direction, shared view ownership and read-only access are sound. Every added moving part traces to the specified routes, configuration, caching, generated types or file serving.

Earlier finding atelier-xxwm: the d69a8bd6 fix works for its stated cases. Independently sent missing /doc query, malformed done boolean, invalid UTF-8 id, POST /api/repos and DELETE on an item to the built binary; all returned their expected 400/405 status, application/json and a string error. Both 405 responses preserved Allow: GET,HEAD. POST /api/act, /api/threads and a history route returned JSON 404. Foreign Host returned JSON 403. This checks the earlier finding's fix but is not an approval resolving it while R1 remains.

Proof run independently at 3831974f50a0443df40a31d19abaa9d314a02413: next/ cargo fmt -- --check; cargo metadata --locked --format-version 1; cargo clippy --all-targets -j 4 -- -D warnings; cargo nextest run -j 4 (266 passed, 0 skipped); cargo build --locked --all-targets -j 4; scripts/ci-jobs.test.sh (26 cases passed); git diff --check HEAD^ HEAD -- . :!.atelier and the whole review range (clean). Tests cover committed values despite checkout edits, cache invalidation on base and item branches, dependency isolation and generated types. cargo install --path crates/atelier-cli --locked --root <temporary scratch directory> -j 4 from next/ installed exactly one binary, atelier. Release compilation emitted the pre-existing unused-variable warning at atelier-cli/src/commands.rs:278. Read the dist opt-out [package.metadata.dist] dist = false, docs/architecture/desk.md, ADR 0032, the strategy and all branch-own code changes.

Fresh built-server validation on loopback port 40217 with the converted clone at base 2116defdde63afdadad2dddead0945cb50c21c42: enumerated committed mission records independently; the rail contained exactly all eight open missions. Every mission route returned 200 and its title, status and per-status counts equalled the new CLI show output. Inbox needs_human equalled list --needs-human: atelier-3km4, atelier-7w9s, atelier-8i86, atelier-j86z, atelier-pywh. Item and done=true routes returned 200, and a document response equalled its committed blob. An ordinary app path returned index.html as required. The server was stopped, temporary configuration, app and install directories removed, and the converted clone stayed clean. No code edited.

Open questions: none. Residual risk/tests not run: no browser, tailnet, service installation or switched-tree release validation, which belong to later items. Old-workspace smoke test not run because this branch changes the new workspace. Outcome: review complete, R1 blocks approval. Changes: only this review record, on atelier-uwxy, pinned to the requested head. Open items: fix R1 and arrange a fresh review.
