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

The non-author check compares review identities only with Git authors, and every agent on this host commits as the human, so it can never catch an agent approving its own work by accident. Let `atelier start --identity <identity>` add to the set of implementers on the owner (children and take-overs add theirs; an identity already present writes nothing, so an approval stays current), compare against that set too, and document the `<role>/<instance>` naming convention from decision atelier-80qd. This is a nudge for well-meaning agents, not access control: identity stays self-declared, and Atelier keeps no list of roles.

## Done when

Tests show: after `start --identity implementer/a`, `review approve --identity implementer/a` draws the same warning at recording and the same `review.current` refusal as an approval by a Git author, ignoring case and surrounding space as today; a `defer` or `no_action` disposition under that identity is treated the same; `review approve --identity reviewer/b` passes. The comparison is on the normalized text whatever its shape, so `start --identity Rae` then `approve --identity Rae` is caught and nothing parses a role. A second start by implementer/b, or a child's start, leaves implementer/a still caught, and rerunning a start with a recorded identity changes no record. Warnings and refusals name the recorded implementer rather than saying the identity matches a Git author. Without `start --identity`, behaviour is unchanged. Reverting the comparison makes the first two tests fail. docs/product/reviews-and-merge.md describes the convention and says it is not enforced.

## Scope

`start` and the owner's record, `independent` and `author_identities` in crates/atelier-app/src/review_gate.rs, and the docs above. Out of scope: any role list or validation, rules about which role may do what, and migrating existing review records, which keep their labels. `land` stays Git-only: `merge` may already refuse an approval `land` accepts (atelier-p5qp), and the check at `merge` is enough for a nudge.

## Context

Decision atelier-80qd. Stated principle: PRODUCT_INTENT.md, "the guards prevent routine mistakes rather than defend against malicious actors". The skill-side half, every delegation stating the role and identity, is still to be made, in spadaval/.agents (ADR 0023), which has no tracker; until it lands nobody passes `--identity` and this check never fires. Whoever closes this item confirms that wording exists or says plainly in the Close Reason that it does not.
