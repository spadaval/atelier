---
schema: atelier.review
schema_version: 2
id: atelier-cbb7
title: approval on atelier-e05n @df7c496b
target: atelier-e05n
identity: reviewer-sol-r1
commit: df7c496b739bf612afb2dfa1fc1a1eb12235a773
created_at: 2026-11-11T06:45:39.355997382Z
event:
  event: approval
---

Verdict: merge. The live-thread client and record joins provide the server half needed by the Desk, with no unneeded moving part found. Sizes: complexity high (new MCP authentication/session client, background polling and cache, and a cross-repository response contract); value high (required agent visibility on mission/item pages and in the inbox); risk medium (user-visible but read-only, bounded and reversible; connection failures preserve records and the last good list). Contract: pass. Read the branch-owned commits in 9219c035..df7c496b excluding master, relevant core view/record code, architecture contract, requirements, Report and evidence atelier-24fe/atelier-3xx2. Verified whole-string identities, no-lead mission aggregation, unknown-thread fallback, per-repository holds/waiting filters, disabled configuration, token renewal/revocation and cache behavior. Fresh proof at df7c496b: next/ cargo nextest run -j 4 passes all 273 tests, including stand-in MCP scenarios, dependency boundaries and generated TypeScript freshness; cargo fmt -- --check and cargo metadata --locked --format-version 1 pass with TMPDIR outside git and no shared target directory. git diff --check HEAD^ HEAD -- . ':!.atelier' and the whole branch code diff both pass. Fitness: pass, with no FIX NOW. Finding R1 / atelier-ear4 is low severity, DEFER: SIGTERM bypasses session revocation; independently reproduced with a fake command (SIGINT revokes once, SIGTERM never revokes). Address when integrating the service; current read-only token expires within an hour. Open questions: none blocking. Residual risk: MCP nightly shape drift; live waiting state and phone-link behavior remain unproved. The recorded live implementer and scratch-item responses and session revocation were inspected, not independently rerun against T3 Code; this review used the in-process stand-in and no T3 threads or userdata. Only review records were changed.
