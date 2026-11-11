---
schema: atelier.review
schema_version: 2
id: atelier-xxwm
title: finding on atelier-uwxy @fd90df9f
target: atelier-uwxy
identity: reviewer-r1
commit: fd90df9fa962c77e204f0f4163d8db05615f9d7b
created_at: 2026-11-11T06:01:24.251590972Z
event:
  event: finding
  recommendation: fix_now
---

low: next/crates/atelier-desk/src/server.rs:252: R1: API extractor and method rejections bypass the JSON error contract. Against the built binary serving the converted clone on 127.0.0.1:3782, GET /api/repos/atelier/doc with no query returned 400 text/plain with "Failed to deserialize query string: missing field `path`"; GET /api/repos/atelier/missions?done=garbage also returned 400 text/plain; POST /api/repos returned 405 with an empty body. Query/Path rejection happens before Failure::into_response, and the router has no JSON method fallback. docs/architecture/desk.md:123-127 promises JSON responses with error, and api::ApiError is documented as every non-200 body, so a client using the generated error type cannot decode these responses. Map API extractor rejections and unsupported methods to JSON ApiError, and add tests asserting status, content type and error for missing/invalid queries, invalid path encoding and unsupported methods. Recommendation: fix_now, because the API contract for the next app item is currently false.

Verdict: merge after R1 is fixed and independently re-reviewed; no approval now. The read API is needed by atelier-94pv and does not need shrinking. Sizes: complexity medium (HTTP boundary, response mapping, commit-keyed cache and generated types); value high (required API for the app); risk low (read-only service over converted clones, failures reversible). Contract: fail at the error-response boundary above; the explicit happy-path Done when claims otherwise hold. Fitness: sound dependency direction and view ownership; error handling needs R1. All added moving parts trace to the specified server, cache, files or types requirements.

Proof run independently at fd90df9fa962c77e204f0f4163d8db05615f9d7b: next/ cargo fmt -- --check; cargo metadata --locked --format-version 1; cargo build --locked --all-targets -j 4; cargo clippy --all-targets -j 4 -- -D warnings; cargo nextest run -j 4 (266 passed); scripts/ci-jobs.test.sh (all 26 cases passed); git diff --check HEAD^ HEAD -- . :!.atelier and the whole review range (clean). TMPDIR was /home/supadava/.cache/atelier-desk/tmp. cargo install --path crates/atelier-cli --locked --root <temporary scratch directory> -j 4 from next/ installed only atelier; release compilation emitted an existing unused-variable warning in atelier-cli commands.rs:278. The crate opts out with [package.metadata.dist] dist = false. Reviewed the six branch-own commits, generated-types check, dependency graph checks, current contract and governing ADR.

The built server read the converted clone at base 2116defd: independently enumerated all eight open mission records, verified the rail contains exactly them, and checked every mission route returns 200 with title, status and per-status counts equal to the new CLI show output. Inbox IDs exactly match list --needs-human: atelier-3km4, atelier-7w9s, atelier-8i86, atelier-j86z, atelier-pywh. Item, document and done=true routes returned 200; foreign Host returned 403 JSON. The server was stopped, temporary config/install directories removed, and the converted clone remained clean. No code edited.

Open questions: none. Residual risk/tests not run: no tailnet/browser/service-install validation or switched-tree release build, which belong to later items; old-workspace smoke script not run because this branch changes the new workspace. No shrink/reject or unrelated deferral recommended.
