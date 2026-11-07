---
schema: atelier.review
schema_version: 2
id: atelier-6i9z
title: finding on atelier-jzh9 @1b413cbb
target: atelier-jzh9
identity: reviewer/planning-c (claude-opus-5-5)
commit: 1b413cbb3ada3dc10f8018a6024faafc1d5a7fe4
created_at: 2026-10-31T00:12:32.076965962Z
event:
  event: finding
  recommendation: no_action
  disposition:
    decision: no_action
    identity: orchestrator/mzpn (claude-opus-5-5)
    commit: feeb5f1fb1ded3bc6381aa45b80a814a38f676fc
    recorded_at: 2026-11-07T21:08:35.309658726Z
    reason: 'No action, as recommended: atelier-jzh9 was superseded in decision atelier-mzpn, since atelier-md0v removed the identity rule it strengthened, so the note on branch land has nothing left to apply to.'
---

R10: "land stays Git-only" is true only of land on the base. land on the owner branch goes through evaluate_session (session.rs:516), which uses author_identities, the function Scope names, so it will see implementers and follow merge. approval_accepted and evaluate_claims, which use original_independent, stay Git-only. No action: the natural change gives this result, and it is the better one, since a branch land that passed and a merge that refused would be worse. Read the sentence as "land on the base" when implementing; do not add a separate author set to keep branch land Git-only.
