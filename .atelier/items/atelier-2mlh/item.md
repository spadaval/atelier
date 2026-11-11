---
schema: atelier.item
schema_version: 3
id: atelier-2mlh
title: Run the first real items under the new tool, and fix what first use breaks
status: draft
mission: atelier-qr5s
labels:
- redesign
priority: medium
blocked_by:
- atelier-wd1i
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-10T00:56:10.176751966Z
---

## Goal

Find what the rehearsal could not.

## Done when

- [ ] Provisional: non-test Rust is reported against the 48,988 lines before the redesign; one item in atelier and one in a product repository have gone from ready to done under the new binary; the fallback branch of CI's `check` step is removed; each fix made along the way is inside the specifications or was escalated.

## Context

Preliminary. This step is planned in full at its boundary, with one planning review, as `.atelier/missions/atelier-qr5s/` says. Target design: `docs/strategies/redesign-product.md` and `docs/strategies/redesign-architecture.md`.

## Unknowns

- What first use breaks.
