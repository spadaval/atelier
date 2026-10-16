---
created_at: "2026-10-16T20:07:20.542876901+00:00"
id: "atelier-jy9t"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Editing a done record's prose reopens its completion claim"
updated_at: "2026-10-16T20:07:20.542876901+00:00"
---

## Goal

`completion_claim_matches` compares a record's whole body, so editing any section of a done record (Design, Background, Unknowns) after it merged makes `land` treat its claim as new. Its gates then run as current claims, and FIX NOW resolutions need an approval that is current at HEAD, which later code has staled. Correcting a done record's Design (atelier-qohr, for atelier-v44v) failed land this way.

## Done when

Decide which sections are part of a completion claim (Goal and Done when, plus title, type and status) and compare only those, so commentary sections of done records can be corrected on the base; or document that done records are not edited and corrections go in a follow-up record. A test covers the chosen rule.

## Context

Found 2026-10-16. crates/atelier-app/src/evidence_freshness.rs acceptance_matches.
