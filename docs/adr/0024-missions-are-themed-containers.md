# ADR 0024: Missions Are Themed Containers

## Status

Accepted (2026-11-04, human-directed: Sumeet decided that a mission is a
themed container whose success criteria are optional and achievable by its
own work; atelier-hhce).

## Context

Atelier required every mission to have Success criteria or a linked
strategy, and created missions with a "Success criteria were not specified."
placeholder that `check` rejected until someone replaced it. Planning
therefore wrote criteria for every mission, whether or not the mission had
a claim to make beyond its items.

On 2026-11-04 this repository's missions held 54 criteria. 29 offered only
item IDs as their proof: they restated the items and added no claim of their
own. About 6 depended on calendar time or outside events: transcripts
recorded after the mission landed, or adopters updating their hooks.

Those criteria stalled missions. A transcript-count criterion, "atelier
commands piped to head, grep, or sed in fewer than one run in five", was
handed from mission atelier-1w8c (its S6) to atelier-4pzj (S5) and then to
atelier-l63c (S3), each time because the mission that owned it could not
measure it before closing. Mission atelier-rc8j closed with its transcript
criterion S6 unproven and its receipt blocked: only 26 runs fell after its
work landed, mostly the mission's own reviewers' scratch scripts, so the
recount was split into atelier-p1vz. An agent asked to prove such a criterion
correctly stops and asks the human what it is expected to do, and the human
has to answer the same question again for each mission.

## Decision

1. **A mission is a themed container.** It holds changes that share a theme,
   and its Outcome says why they belong together. It does not need to be one
   deliverable.
2. **Success criteria are optional.** A mission with only an Outcome, and
   no strategy, is valid: `check`, the mission lint and the `ready`
   transition accept it, and `issue create --issue-type mission` writes no
   placeholder criteria.
3. **Any criterion must be achievable by the mission's own work.** It must be
   something the mission's work can make true and an agent can check when the
   mission closes. Work that waits on calendar time, outside adoption or
   post-ship measurement goes into its own backlog item outside the mission.

The process for shaping a mission, deciding whether it needs criteria, and
judging when it is ready belongs to Agent Factory
([ADR 0023](0023-agent-factory-is-not-vendored.md)), in its
[Shaping a Mission](https://github.com/spadaval/.agents/blob/main/skills/agent-factory/procedures/plan.md#shaping-a-mission)
and
[Ready Missions](https://github.com/spadaval/.agents/blob/main/skills/agent-factory/procedures/plan.md#ready-missions)
sections. Atelier does not restate it.

## Consequences

- Mission views (`brief`, `debrief`, `work mission`, in Markdown and HTML)
  leave out criteria when a mission has none, rather than showing an empty
  section or a placeholder.
- The criteria parser, `evidence record --criterion`, and criterion marks are
  unchanged; they apply when a mission has criteria.
- Existing missions keep their criteria. Criteria that only restate items, or
  that the mission's work cannot reach, are revised or moved to their own
  backlog items when those missions are next planned; closed missions and
  strategies under `docs/strategies` stay as they were written.
- A linked strategy with no Success Criteria heading is still reported as a
  criteria problem by mission views; that lookup is unchanged here.
- Atelier cannot check that a criterion is achievable. That stays a planning
  and review judgment, made against Agent Factory's guidance when a mission
  moves to `ready`.

## Alternatives Considered

- **Keep criteria required and lint for unreachable ones.** Rejected: whether
  a criterion depends on calendar time or outside events is a judgment about
  its text, not something a record check can see.
- **Keep the placeholder and let agents delete it.** Rejected: a default
  that `check` rejects still asks planners to write criteria for every
  mission.
