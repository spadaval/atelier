# Mission Authoring

[Agent Factory](../../.agents/skills/agent-factory/SKILL.md) owns planning.
Its [tracker-record templates](../../.agents/skills/agent-factory/references/tracker-records.md)
are the authoring reference. Atelier validates the resulting records and graph.

A mission is a front door to a strategy, not a second strategy or a status table.
Link the strategy by path, state the outcome, and identify success criteria.
When no separate strategy exists, the mission body carries that intent.

An epic describes an observable Outcome. Draft epics can retain Unknowns;
before ready, add the Proof route and immediate child work with real dependency
links. Add Delivers when it helps connect the increment to mission criteria.
Keep Design as a link to the owning document.

An executable issue has Goal and Done when. Done when names a check or scenario
that could fail if the slice were missing or wrong. Scope and Context give the
worker enough boundaries and interfaces to act without the author's context.
A validation item follows the executable issue shape: it references the claims
it checks in Goal and its method in Context, and records each result as an
attached evidence receipt.

Keep status in front matter and progress in dated Notes. Put follow-up findings
in new issues. Use native blockers for sequencing, parent fields for direct epic
children, and `advances` links for mission scope. Do not put mission membership
in parent fields.

Use `atelier issue create` for a single item. For a prepared multi-record graph,
inspect `atelier bundle preview <file>` before applying it. Read the
[bundle contract](../spec/bundle/schema.md) for its supported input fields.
