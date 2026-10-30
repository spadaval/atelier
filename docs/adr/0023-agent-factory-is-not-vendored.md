# ADR 0023: Agent Factory Is Not Vendored

## Status

Accepted (2026-10-30, human-directed: Sumeet decided that the Agent Factory
skill has exactly one git-tracked copy; atelier-svi6).

Supersedes [ADR 0020](0020-agent-factory-targets-atelier-only.md) Decision 2.
ADR 0020's other decisions stand.

## Context

ADR 0020 made `spadaval/.agents` the skill's one source and had this
repository vendor it verbatim under `.agents/skills/agent-factory`. The
vendored copy did not stay verbatim. Missions edited it in place and left
resync work behind them (atelier-k4xg, atelier-kr30, atelier-92mz), and
guidance written here on 2026-10-23 was never synced upstream until it was
ported by hand (spadaval/.agents bee1476). A review of Agent Factory sessions
from 2026-10-15 to 2026-10-29 found about 25 copies of the skill on one
machine, including two checkouts of the upstream repository that had
diverged from each other, so the human had to repeat a settled decision
after one copy silently dropped it.

Within this repository, only documentation links and `atelier review request`
referred to the vendored path. A harness that discovers a repository's
`.agents/skills` could load the vendored copy instead of the installed one, so
the copy an agent followed depended on the harness.

## Decision

The skill has one git-tracked copy, in `spadaval/.agents`
(`skills/agent-factory`). This repository does not vendor it. Agents load the
installed copy, and changes to the skill, including Atelier-specific
guidance, are made there.

## Consequences

- `.agents/skills/agent-factory` is removed. Documentation links point at the
  skill in `spadaval/.agents` on GitHub, which shows what has been pushed
  there; the installed copy can be ahead of it.
- `atelier review request` no longer names a repository path for the review
  procedure. The brief stands on its own, and the agent dispatching the
  reviewer supplies any process guidance.
- ADR 0020 Decision 3 still applies: a change to Atelier's commands or record
  model that invalidates the skill's Atelier reference updates the skill in
  `spadaval/.agents` in the same piece of work.
- Closed records, review events and superseded strategies keep the vendored
  path they were written against.

## Alternatives Considered

- **Keep vendoring and add a drift check.** Rejected: it keeps a second copy
  that agents and humans can still edit, and a check can only report the
  drift after the fact.
- **Ship the skill with `atelier setup`.** Still deferred, as in ADR 0020.
