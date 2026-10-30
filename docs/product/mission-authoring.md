# Mission Authoring

[Agent Factory](https://github.com/spadaval/.agents/blob/main/skills/agent-factory/SKILL.md) owns planning.
Its [tracker-record templates](https://github.com/spadaval/.agents/blob/main/skills/agent-factory/references/tracker-records.md)
are the authoring reference. Atelier validates the resulting records and graph.

A mission is a front door to a strategy, not a second strategy or a status table.
Link the strategy by path, state the outcome, and identify success criteria.
A criterion is a `- <slug>: <text>` bullet under the strategy's Success
Criteria heading (any level), or under the mission's own Success criteria
section when it links no strategy (decision atelier-et6q). The slug is
lowercase kebab-case, such as `pipe-safe`, or a positional `S<n>`, and is
unique within the mission; indented lines continue the bullet before them.
Views list criteria in document order and report any other bullet in that
section as a problem.
When no separate strategy exists, the mission body carries that intent. Put
research, references, and motivation that the mission's work shares in its
Background section; a strategy still owns intent, boundaries, and tradeoffs.

The mission's strategy path is the canonical link. Write the strategy first,
then set the link with
`atelier issue create <title> --issue-type mission --strategy <path>`, change it
with `atelier issue update <mission> --strategy <path>`, or clear it with
`--no-strategy`. The strategy file does not need to name its mission (a blank
`mission:` counts as none), so creating or re-importing the mission needs no
edit to the strategy. If its front matter does name one, `atelier check`
reports a value that differs from the mission linking it, or any value when
several missions link the same strategy. `check` also reports a linked
strategy path that does not exist.

A parent is any work item with children: an observable increment worked on
one branch and reviewed together. Its Goal is the outcome and its Done when the
proof route, including how it delivers mission criteria. Unknowns record open
questions; keep later increments in `backlog` until their predecessor's
evidence arrives. Keep Design as a link to the owning document.

An executable issue has Goal and Done when. Done when names a check or scenario
that could fail if the slice were missing or wrong. Scope and Context give the
worker enough boundaries and interfaces to act without the author's context.
A validation item follows the executable issue shape: it references the claims
it checks in Goal and its method in Context, and records each result as an
attached evidence receipt.

Keep status in front matter and progress in dated Notes. Put follow-up findings
in new issues. Use native blockers for sequencing, parent fields for direct
children (one level), labels for themes, and `advances` links for mission
scope. Do not put mission membership
in parent fields.

Use `atelier issue create` for a single item. For a prepared multi-record graph,
inspect `atelier bundle preview <file>` before applying it. Read the
[bundle contract](../spec/bundle/schema.md) for its supported input fields.
