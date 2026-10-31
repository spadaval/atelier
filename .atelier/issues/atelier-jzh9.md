---
created_at: "2026-10-31T00:04:40.084530635+00:00"
id: "atelier-jzh9"
issue_type: "feature"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-80qd"
    type: "implements"
  - kind: "issue"
    id: "atelier-ll3p"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Record who implemented an item, so the non-author check catches honest slips"
updated_at: "2026-10-31T00:04:40.084530635+00:00"
---

## Goal

The non-author check compares review identities only with Git authors, and every agent on this host commits as the human, so it can never catch an agent approving its own work by accident. Let `atelier start --identity <identity>` record who implemented the owner, compare against that too, and document the `<role>/<instance>` naming convention from decision atelier-80qd. This is a nudge for well-meaning agents, not access control: identity stays self-declared, and Atelier keeps no list of roles.

## Done when

Tests show: after `start --identity implementer/a`, `review approve --identity implementer/a` draws the same warning at recording and the same `review.current` refusal as an approval by a Git author, ignoring case and surrounding space as today; a `defer` or `no_action` disposition under that identity is treated the same; `review approve --identity reviewer/b` passes. Without `start --identity`, and for identities such as `Rae` or `reviewer` that do not follow the convention, behaviour is unchanged. Reverting the comparison makes the first two tests fail. docs/product/reviews-and-merge.md describes the convention and says it is not enforced.

## Scope

`start` and the owner's record, `independent` and `author_identities` in crates/atelier-app/src/review_gate.rs, and the docs above. Out of scope: any role list or validation, rules about which role may do what, and migrating existing review records, which keep their labels.

## Context

Decision atelier-80qd. Stated principle: PRODUCT_INTENT.md, "the guards prevent routine mistakes rather than defend against malicious actors". The skill-side half, every delegation stating the role and identity, is made in spadaval/.agents (ADR 0023).
