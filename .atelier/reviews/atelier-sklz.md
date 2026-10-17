---
schema: atelier.review
schema_version: 1
id: atelier-sklz
title: 'crates/atelier-cli/src/commands/bundle.rs:1320-1333 — resolve_existing_ref resolves bundle issue ids through the shared case-insensitive short-ID matcher, but evidence ids only by exact full ID (index.evidence(id)), and an unknown id gives ''Record id X not found'' instead of the ''<kind> <ref> not found'' form. A bundle that names existing evidence as ''052D'' or ''052d'' fails with ''Reference id ... does not resolve'', while the same short ID works for issues in the same field and works in ''evidence show''. This was already so and the change does not make it worse; bundle files are not command arguments, and the Goal does not list this path. Follow-up: route it through RecordIndex::resolve for both kinds.'
target: atelier-qrwr
identity: Independent reviewer (Claude subagent)
commit: dcde3fc2e9f623ec96a1a5f679d8a24e0268a87f
created_at: 2026-10-17T07:16:44.680186356Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/src/commands/bundle.rs:1320-1333 — resolve_existing_ref resolves bundle issue ids through the shared case-insensitive short-ID matcher, but evidence ids only by exact full ID (index.evidence(id)), and an unknown id gives 'Record id X not found' instead of the '<kind> <ref> not found' form. A bundle that names existing evidence as '052D' or '052d' fails with 'Reference id ... does not resolve', while the same short ID works for issues in the same field and works in 'evidence show'. This was already so and the change does not make it worse; bundle files are not command arguments, and the Goal does not list this path. Follow-up: route it through RecordIndex::resolve for both kinds.
