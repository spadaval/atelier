---
schema: atelier.review
schema_version: 2
id: atelier-7a1e
title: finding on atelier-nafc @f317ac78
target: atelier-nafc
identity: gpt-6.1-sol
commit: f317ac78fdffcef88ab75a2773d9ee5f59688361
created_at: 2026-11-10T21:55:58.467781280Z
event:
  event: finding
  recommendation: fix_now
---

R6 .atelier/issues/atelier-nafc.md:53: medium. The Report must state as fact whether each real forge requires a pull request for its base, but the allowed evidence sources cannot establish that fact. Branch protection and rulesets are server-side policy, not determined by Git history, workflow files or AGENTS.md; the local bare remote and gh test double only establish the sandbox policy. Scope forbids sending anything from the product repositories to any outside service and provides no authoritative policy snapshot. A worker must either guess a cutover-critical fact or invent an exception to that boundary. Name the evidence source before work starts: for example an authoritative supplied metadata snapshot, or an explicitly limited read-only query for forge policy that sends no repository content. Define how unavailable/inaccessible policy is reported and blocks the affected cutover decision; absence of evidence must not become a claim that direct pushes are allowed.
