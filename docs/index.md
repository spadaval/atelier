# Documentation Map

- [Product specification](product.md): what Atelier is for, its words, the
  lifecycle of an item and a mission, the commands, the records, review,
  merge, hooks and the scorecard.
- [Design record](design.md): how the tool is built, what was tried and
  removed, and the decisions that govern it. It replaced the ADRs.
- The Desk, the human's tool: its [product intent](product/desk.md) and
  [its API](architecture/desk.md).
- [Agent Factory](../skills/agent-factory/SKILL.md): how agents decide, by
  role. Its one copy is `skills/agent-factory`, and agents load the installed
  skill.
- [Agent Factory's evals](../evals/agent-factory/README.md): the scenarios
  that judge the skill, and their recorded results.
- [Engineering standards](architecture/quality/standards.md).
- [The scorecard baseline, November 2026](architecture/quality/scorecard-baseline-2026-11.md):
  the scorecard measured before the redesign, with its method and data. It
  is history and describes the old tool.

A mission's own documents are in its directory under `.atelier/missions/`.
