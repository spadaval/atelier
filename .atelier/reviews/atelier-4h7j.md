---
schema: atelier.review
schema_version: 1
id: atelier-4h7j
title: 'crates/atelier-app/src/use_cases.rs:43-44 — the rustdoc on pub fn resolve_evidence_target_ref still says ''any other kind is left for target validation to reject'', but this change makes the function itself reject any other kind with InvalidValue (line 49). The doc now states the opposite of the code on a public app-layer function; a reader adding a target kind would look for a later validator that no longer runs first. Fix: one-line doc update (e.g. ''rejects any other kind as an invalid value'').'
target: atelier-pyfr
identity: Independent reviewer (Claude subagent)
commit: 50adccc26e44b3a509a834acccb8e7f573d8cd69
created_at: 2026-10-17T09:52:30.900962341Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-app/src/use_cases.rs:43-44 — the rustdoc on pub fn resolve_evidence_target_ref still says 'any other kind is left for target validation to reject', but this change makes the function itself reject any other kind with InvalidValue (line 49). The doc now states the opposite of the code on a public app-layer function; a reader adding a target kind would look for a later validator that no longer runs first. Fix: one-line doc update (e.g. 'rejects any other kind as an invalid value').
