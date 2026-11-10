---
schema: atelier.review
schema_version: 2
id: atelier-pj53
title: finding on atelier-jq24 @8426a78b
target: atelier-jq24
identity: gpt-6.1-sol
commit: 8426a78b209e2e384efbbfb36984fece4a45fee1
created_at: 2026-11-10T19:38:25.674976263Z
event:
  event: finding
  recommendation: fix_now
---

R3, low severity: .atelier/issues/atelier-jq24.md:32-43 requires storing and redisplaying a drop reason, but postpones deciding its persisted location until the architecture is updated to match what was built. Neither specification defines a drop-reason field or body syntax. This is a shared contract, not output wording: jq24 must make status on the base and the item's branch display the reason, while jciw independently replaces the base's status and reads terminal records. Their scopes put the workers in separate modules but give them no common representation or ownership for that data. A worker must choose a persisted format alone, and the parallel reader can choose a different one or overwrite the status support. The plan procedure explicitly requires persisted-format decisions to be settled in the design before implementation. Specify the existing body section and line form used for an item's and a mission's drop reason, preserving existing text and adding no front-matter fields; state who supplies its reader so both items can use the same contract, or order the affected work. Recommendation: FIX NOW, because this unresolved persisted interface undermines the claimed parallel independence.
