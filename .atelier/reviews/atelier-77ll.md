---
schema: atelier.review
schema_version: 2
id: atelier-77ll
title: finding on atelier-orb2 @ea6ff0ee
target: atelier-orb2
identity: GPT-6.1 Sol independent planning reviewer
commit: ea6ff0ee0165cd82b315020fb7d5be1b3816f070
created_at: 2026-11-11T02:51:08.634656027Z
event:
  event: finding
  recommendation: fix_now
---

.atelier/issues/atelier-orb2.md:34 (also :43), severity medium: Routing exclusively by the role that status names leaves planner and decider instructions unreachable on current paths. The architecture specification explicitly says no thread holds the planner role (redesign-architecture.md:398-400). The real status code labels an item caller only as reviewer, implementer, past reviewer or visitor (next/crates/atelier-cli/src/output.rs:361-385); on the base it starts with a generic thread heading (:468-473) and adds held items and led missions. A freshly dispatched decider is a visitor, not a held decider, and a planner on the base is not labelled planner. The required five status captures omit these two roles, so the proposed proof would miss the defect. Specify routing from the assignment or entry task when status names no applicable role, retaining the role-only read paths; extend the inspection evidence to those two entry cases. This needs no change to next/ and prevents a planning or decision worker from following the unrelated start-item suggestion.
