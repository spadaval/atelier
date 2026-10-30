# Atelier Principles

Keep work and proof visible in the same repository as the implementation.
Store each fact in its owning place: strategy for intent, tracker for state and
sequencing, design docs for architecture, and Git for integration history.

Use concise records with observable outcomes and checks. A receipt must state
its result and checked commit. Independent approval comes from another identity
and must remain fresh. Findings need explicit dispositions.

Guard invariants at the point of action. Leave planning depth and coordination
to Agent Factory. Keep transitions simple record writes, and make Git side
effects explicit through start and merge. Agents commit; Atelier never does.

Make errors and branch context legible. Prefer direct parsing and a small CLI
to hidden runtime machinery. Retire obsolete commands and docs instead of
keeping competing contracts alive.

The governing sources are [Product Intent](../../PRODUCT_INTENT.md),
[ADR 0019](../adr/0019-verified-repo-native-tracker.md), and
[Agent Factory](https://github.com/spadaval/.agents/blob/main/skills/agent-factory/SKILL.md).
