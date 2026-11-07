# Atelier Principles

Keep work and proof visible in the same repository as the implementation.
Store each fact in its owning place: strategy for intent, tracker for state and
sequencing, design docs for architecture, and Git for integration history.

Use concise records with observable outcomes. A receipt must state its result
and the commit it proves. An independent approval must cover the current code,
and only a code change makes it stale. A deferred or dismissed finding needs an
explicit disposition; the approval after its fix resolves a `fix_now` one.

Guard invariants at the point of action. Leave planning depth and coordination
to Agent Factory. Keep transitions simple record writes, and make Git side
effects explicit through start and merge. Agents commit; Atelier never does.

Make errors and branch context legible. Prefer direct parsing and a small CLI
to hidden runtime machinery. Retire obsolete commands and docs instead of
keeping competing contracts alive.

The governing sources are [Product Intent](../../PRODUCT_INTENT.md),
[ADR 0019](../adr/0019-verified-repo-native-tracker.md), and
[Agent Factory](https://github.com/spadaval/.agents/blob/main/skills/agent-factory/SKILL.md).
