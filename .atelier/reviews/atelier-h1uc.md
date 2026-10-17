---
schema: atelier.review
schema_version: 1
id: atelier-h1uc
title: 'crates/atelier-cli/tests/cli_integration/decisions.rs:525: the comment ''An item with no decisions shows no Decisions line.'' sits above the assertion that the old record shows ''Decisions: 2 current''; the no-decisions case is the ''Lone'' item further down. Misleading residue only; move the comment down to the Lone assertion when next touching the file.'
target: atelier-1mk4
identity: Independent reviewer (Claude subagent)
commit: 2254ca708e0ee34d8b3d2f3ba857fc6726b5f23b
created_at: 2026-10-17T18:57:57.391959308Z
event:
  event: finding
  recommendation: no_action
---

crates/atelier-cli/tests/cli_integration/decisions.rs:525: the comment 'An item with no decisions shows no Decisions line.' sits above the assertion that the old record shows 'Decisions: 2 current'; the no-decisions case is the 'Lone' item further down. Misleading residue only; move the comment down to the Lone assertion when next touching the file.
