---
created_at: "2026-06-17T17:58:56.047692017+00:00"
id: "atelier-tovs"
issue_type: "epic"
labels:
- "architecture"
- "pr"
- "sessions"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-495r"
  - kind: "issue"
    id: "atelier-4clo"
  - kind: "issue"
    id: "atelier-6375"
  - kind: "issue"
    id: "atelier-7g43"
  - kind: "issue"
    id: "atelier-95wv"
  - kind: "issue"
    id: "atelier-98mo"
  - kind: "issue"
    id: "atelier-c5oz"
  - kind: "issue"
    id: "atelier-cglp"
  - kind: "issue"
    id: "atelier-e7oj"
  - kind: "issue"
    id: "atelier-hw9t"
  - kind: "issue"
    id: "atelier-jhzk"
  - kind: "issue"
    id: "atelier-mpah"
  - kind: "issue"
    id: "atelier-nmkm"
  - kind: "issue"
    id: "atelier-o97w"
  - kind: "issue"
    id: "atelier-onie"
  - kind: "issue"
    id: "atelier-p7oa"
  - kind: "issue"
    id: "atelier-rgmg"
  - kind: "issue"
    id: "atelier-udny"
  - kind: "issue"
    id: "atelier-vg25"
  - kind: "issue"
    id: "atelier-vvs3"
  - kind: "issue"
    id: "atelier-x1fn"
  - kind: "issue"
    id: "atelier-y31v"
  - kind: "issue"
    id: "atelier-yrwm"
  children:
  - kind: "issue"
    id: "atelier-cbbx"
  - kind: "issue"
    id: "atelier-d7gd"
  - kind: "issue"
    id: "atelier-uoel"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-17T23:52:41.065977217+00:00"
status: "done"
title: "Epic: Settle architecture and product contract"
updated_at: "2026-06-17T23:52:41.065977217+00:00"
---

## Description

Settle the product and architecture contract before implementation begins.
This epic owns the ADR, glossary updates, and product documentation that
distinguish durable sessions, typed fields, and Forgejo PR artifacts from
legacy runtime session/current-work behavior.

## Outcome

- Future agents can explain why sessions are durable and optional.
- Product docs state that PRs are review artifacts and validators read PR
  state without letting PR commands drive workflow transitions.
- Typed issue fields are documented as workflow-policy-owned schema, not
  overloaded attachments.

## Evidence

- Review artifact or evidence record maps the ADR, CONTEXT.md updates, and
  product-doc changes to this epic's outcome.
- Command transcript shows `atelier lint` and `git diff --check` pass for the
  documentation slice.
