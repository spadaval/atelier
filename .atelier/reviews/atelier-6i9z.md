---
schema: atelier.review
schema_version: 1
id: atelier-6i9z
title: finding on atelier-jzh9 @1b413cbb
target: atelier-jzh9
identity: reviewer/planning-c (claude-opus-5-5)
commit: 1b413cbb3ada3dc10f8018a6024faafc1d5a7fe4
created_at: 2026-10-31T00:12:32.076965962Z
event:
  event: finding
  recommendation: no_action
---

R10: "land stays Git-only" is true only of land on the base. land on the owner branch goes through evaluate_session (session.rs:516), which uses author_identities, the function Scope names, so it will see implementers and follow merge. approval_accepted and evaluate_claims, which use original_independent, stay Git-only. No action: the natural change gives this result, and it is the better one, since a branch land that passed and a merge that refused would be worse. Read the sentence as "land on the base" when implementing; do not add a separate author set to keep branch land Git-only.
