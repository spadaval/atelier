---
status: draft
last_changed: "2026-11-10: amended at Sumeet's direction: a proposal is a root cause with its trips as evidence; something broken today is a bug, not a gardener entry; a run ends in a session with Sumeet"
---

# Sizing

- Complexity: medium. It adds one read command, rewrites one procedure and
  adds one checklist.
- Value: medium. It is how the redesign's results are seen and kept, and it
  is not needed for the redesign to work.
- Risk: low. It reads, and its fixes are reviewed like any change.

It is worth its cost because without it the scorecard is measured once by
mission 1 and never again, and the redesign's effect stays an impression.

# Shape

- **The tool gives facts and the skill does the arithmetic.** `log` says what
  happened to each item and `list` how things stand. The binary computes no
  measure and knows no threshold, so nothing in it changes when the measures
  do. Rework and line counts still come from Git, tool calls and tokens from
  T3's events and the transcripts, and whether a finding was a product
  defect from a rater.
- **Numbers first, transcripts second.** The run is cheap because it starts
  from two commands' output.
- **Two layers.** The checklist is static and covers preconditions: can an
  agent build, test, launch and find its way. The scorecard lags and shows
  effects. A criterion stays on the checklist only while failing it shows up
  in the numbers.
- **Causes, not incidents.** A trip is evidence for a claim about one of the
  three properties, and is never itself the thing Sumeet is asked about. The
  run groups its trips by cause before it writes anything. A proposal is one
  cause; its trips are listed under it with how often each happened. A trip
  that fits no cause and happened once is left out.
- **Not a firefighter.** Something broken today, such as a command that
  fails or an error that hides its reason, is a bug filed by whoever hits
  it. It does not wait for a run, and a run does not spend its page on it.
  The gardener looks at what the numbers and the repeated trips say about
  the ground.
- **A run ends in a session.** Gardening is done with Sumeet, as planning
  is, and unlike the roles that run a mission unattended. The gardener does
  the reading, the numbers and the upkeep alone. It then brings Sumeet the
  page and they settle the proposals together. The page records what they
  agreed and what is still open. It is not a queue of entries for him to
  answer later.
- **Three tiers of fix.** The gardener does upkeep and sign-level fixes
  itself, reviewed like any change. It proposes new or removed checks,
  changed defaults and restructures. It never changes a rule on its own.
- **Net size.** A run that proposes a rule or check names one it considered
  removing.
- **A run is an item.** It is labelled `garden`, the gardener holds it as its
  implementer, and its Report is the page. The next run's window starts
  where the last one's item closed.

# Design

Four steps, all in the backlog until the cutover has happened.

| Step | Does |
| --- | --- |
| A. The log | Build `atelier log` as the [architecture specification](../../../docs/strategies/redesign-architecture.md#the-log) describes. Remove from `scripts/scorecard/` what `log` now supplies, and tie the remaining scripts to items by the log's thread IDs. |
| B. Readiness checklist | A short default list in the skill, mapped to the three properties, each line with the command or observation that tests it. A repository may add lines. Drawn from Factory's agent-readiness pillars, without the team-shaped criteria. |
| C. Procedure | Rewrite `garden.md` around the two layers, the three tiers and the one-page report, with the steps that turn `log` output into its numbers. Fold `audit.md` into it. The gardener settles deferred findings with `new --from` and `drop`, on its run's branch, so its choices are reviewed with the run. |
| D. First runs | One run in each of the five repositories. |

The three properties and what the report shows for each:

| Property | Worst spots shown |
| --- | --- |
| Easy to navigate | Files agents read most before their first edit |
| Easy and reliable to change | Files with the most rework and the most fix-now findings |
| Easy to validate | Features with no proof command in the verify skill; defects caught after merge, by the `caused-by` their bug items carry |

# Derisking

The riskiest assumption is that the scorecard's numbers move enough, and
soon enough, for a run to have something to say. Step D's first runs come
with only days of data under the new tool. They are expected to report the
baseline and the checklist, and to say which numbers are not yet meaningful.
The comparison that judges the redesign is a separate item outside this
mission, atelier-pwhn, picked up two weeks after the cutover.

# Risks and Unknowns

- **Too little data.** See Derisking.
- **The checklist grows.** Each line names the number that would show its
  failure, and a line with no such number is cut.
- **The history walk.** `log` has to follow both sides of each merge, which
  today's debrief reader does not, and astromech has the most records. The
  debrief's walk went from 7.8 s to 1.04 s when it became one `git log` over
  the records; step A's planning settles how `log` stays a single walk.
- **Where a repository's own checklist lines live** is settled in step B.
