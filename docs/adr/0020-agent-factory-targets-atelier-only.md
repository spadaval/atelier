# ADR 0020: Agent Factory Targets Atelier Only

## Status

Accepted (2026-10-15, human-directed).

Builds on [ADR 0019](0019-verified-repo-native-tracker.md). Decision 3 amended
by [ADR 0022](0022-status-orients-and-output-is-dense.md): session context
comes from `status`.

## Context

Agent Factory was written to work with either Atelier or GitHub Issues, so
that people could use it without Atelier. Supporting both had costs:

- Record templates had to fit both trackers, so the mission and validation
  templates did not match Atelier's typed records, and this repository's
  vendored skill carried local edits to work around them.
- The skill's Atelier command reference went stale after the ADR 0019
  rebuild. It described removed commands (`history`, `man`, review rooms,
  `check --fix`) and never mentioned `start`, `merge`, or `land`, so an agent
  following it in another Atelier repository would integrate with plain Git
  and bypass every merge gate.
- Many tracker-agnostic process skills now exist. What Agent Factory offers
  that they do not is the combination of its process with Atelier's records
  and gates.

## Decision

1. **Agent Factory's tracker is Atelier.** The skill drops its GitHub Issues
   adapter. Its record templates are Atelier's typed body sections, and its
   tracker guidance is one Atelier reference. Small work still needs no
   tracker, and repositories without Atelier keep the plan-file fallback;
   the skill's `install` procedure covers adopting Atelier.

2. **The skill has one source.** The canonical skill lives in
   `spadaval/.agents`. This repository vendors it verbatim under
   `.agents/skills/agent-factory`, with no local edits. Atelier-specific
   guidance that this repository needs belongs upstream.

3. **The skill maps concepts; Atelier owns syntax.** The skill's Atelier
   reference names which command serves which Agent Factory step, and defers
   flags and fields to `atelier help` and session context from `prime`. A
   change to Atelier's commands or record model that invalidates that map
   updates the upstream skill in the same piece of work.

4. **Any GitHub Issues sync is a one-way mirror.** If people need to see work
   on GitHub, records on the base branch may be projected to GitHub Issues by
   CI after each push to the base branch. GitHub is never written back into
   records, and the mirror is not part of the CLI. Because the base branch
   shows an item done only when its code merges, the mirror never runs ahead
   of integrated work. Build it only when a real consumer exists.

## Consequences

- Agent Factory and Atelier are released as a pair in practice. A repository
  that wants Agent Factory's full process installs Atelier.
- The vendored skill is replaced from upstream rather than edited here.
- The skill's Atelier reference no longer copies flag-level syntax, so it
  goes stale less often, but it still depends on command names.
- Atelier does not yet support a mission integration branch as a merge base,
  which the skill allows for missions and isolated rewrites. The skill states
  the limitation, and `atelier-w0su` tracks resolving it.

## Alternatives Considered

- **Keep GitHub Issues as a peer tracker.** Rejected: it kept the templates
  generic and let the Atelier guidance drift unnoticed.
- **Two-way GitHub sync.** Rejected: it recreates two sources of truth and
  the multi-step writes that ADR 0019 removed.
- **Ship the skill inside Atelier and install it with `atelier setup`.**
  Deferred: it would lock skill and CLI versions together, but nothing outside
  this repository consumes Atelier yet. Revisit when version skew appears.
