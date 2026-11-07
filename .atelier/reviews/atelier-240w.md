---
schema: atelier.review
schema_version: 2
id: atelier-240w
title: 'Coverage gap: the base-has-policy refusal of --install is untested. Mutating ''Some(_) if install => bail!(land_install_invalid ...)'' to never fire passes the whole atelier-cli suite (489 tests). Under that mutation, land --install on an unowned branch with no tracker changes, whose base already has a policy, silently passes as an ordinary land. Add an assertion: on an unowned branch of an adopted repository (e.g. the sneaky test before it edits base_branch, or a plain code branch off main), land --install refuses with land_install_invalid.'
target: atelier-bibd
identity: Independent reviewer (Claude subagent)
commit: 5ee51dd7f06deac69692a6ac9af9b8d4dc8a80d5
created_at: 2026-10-16T23:26:18.362131537Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (orchestrator)
    commit: c2f02115a1f58a17e8a2fef875e1664dae929787
    recorded_at: 2026-10-16T23:27:22.791664736Z
    resolution_commit: c2f02115a1f58a17e8a2fef875e1664dae929787
    reason: Added a land --install assertion on an unowned branch of an adopted repository
    migrated_from: atelier-hy26
---

Coverage gap: the base-has-policy refusal of --install is untested. Mutating 'Some(_) if install => bail!(land_install_invalid ...)' to never fire passes the whole atelier-cli suite (489 tests). Under that mutation, land --install on an unowned branch with no tracker changes, whose base already has a policy, silently passes as an ordinary land. Add an assertion: on an unowned branch of an adopted repository (e.g. the sneaky test before it edits base_branch, or a plain code branch off main), land --install refuses with land_install_invalid.
