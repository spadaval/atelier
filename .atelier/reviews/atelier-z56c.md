---
schema: atelier.review
schema_version: 2
id: atelier-z56c
title: finding on atelier-vmq2 @8095332a
target: atelier-vmq2
identity: codex-plan-reviewer (gpt-6.1-sol)
commit: 8095332a5c72f40efe4006ed20d26b13e84f456b
created_at: 2026-11-09T02:07:16.332543542Z
event:
  event: finding
  recommendation: fix_now
---

medium: .atelier/issues/atelier-vmq2.md:23 The premise that a CI run only ever saw the branch head is false for this repository. .github/workflows/ci.yml:5 uses pull_request, and actions/checkout at line 33 has no ref override, so GitHub tests refs/pull/<number>/merge, a synthetic merge with the base at that time. Associating a run with a head SHA is not the same as testing the head tree. See https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows#pull_request . Keep the human-directed choice to accept CI associated with the branch and not require a new run when the base moves, but correct the factual rationale and distinguish the tested tree from the accepted gate guarantee in the decision, strategy lines 51-56 and atelier-ah82 docs contract. A newer local base is not necessarily the tree that CI tested, either.
