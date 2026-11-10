---
schema: atelier.review
schema_version: 2
id: atelier-w2dj
title: finding on atelier-fcb7 @61c42ae9
target: atelier-fcb7
identity: gpt-6-astra-plan-r2
commit: 61c42ae925d43631e91b5584cb9d22878e0c55f2
created_at: 2026-11-10T01:17:35.082241974Z
event:
  event: finding
  recommendation: fix_now
---

medium atelier-engh and docs/strategies/redesign-architecture.md:221-231 (atelier-utim still open): atomic creation of the branch is not atomic publication of its building record. Between branch creation and the status commit its tip is still merged into base and contains ready, so base discovery hides the claim; a crash leaves an existing branch with ready and neither start transition applies. Specify atomic publication of the initial building commit and branch, or a defined recovery state, and test failure between creation and commit as well as simultaneous starts.
