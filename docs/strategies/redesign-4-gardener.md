---
status: draft
last_changed: "2026-11-09: first draft"
---

# Why

The gardener is the role meant to keep a codebase easy to navigate, easy to
change and easy to validate. As written it collects incidents. It has no
numbers, so it cannot tell whether its last fixes worked. It hears refusals
and retry loops and misses slow costs, such as an agent reading 40 files
before it can edit. Its one digest, atelier-j86z, is ten entries about the
tool and none about a codebase, and it waits in a queue where 6 of 63 items
have been answered.

After the cutover, records carry each item's status and threads, so most of
the scorecard can be computed without reading transcripts.

# Outcome

A gardener run starts from two things it does not have to assemble by hand: a
readiness checklist for the repository and the scorecard's current numbers
against the baseline. It reads transcripts only to explain a number that
moved. It ends with one page for Sumeet: the numbers, the worst spots, and at
most three proposals. One run has been made in each of the five repositories.

# Success Criteria

- one-page: a gardener run in each of the five repositories produces a report
  of one page that holds the scorecard, the worst spots under each of the
  three codebase properties, and at most three proposals.
- from-records: the scorecard scripts find each item's threads from the
  records' thread fields, with no matching on thread titles.

# Sizing

- Complexity: medium. It rewrites one procedure, extends existing scripts and
  adds one checklist.
- Value: medium. It is how the redesign's results are seen and kept, and it
  is not needed for the redesign to work.
- Risk: low. It reads, and its fixes are reviewed like any change.

It is worth its cost because without it the scorecard is measured once by
mission 1 and never again, and the redesign's effect stays an impression.

# Shape

- **Numbers first, transcripts second.** The run is cheap because it starts
  from the scripts' output.
- **Two layers.** The checklist is static and covers preconditions: can an
  agent build, test, launch and find its way. The scorecard lags and shows
  effects. A criterion stays on the checklist only while failing it shows up
  in the numbers.
- **Three tiers of fix.** The gardener does upkeep and sign-level fixes
  itself, reviewed like any change. It proposes new or removed checks,
  changed defaults and restructures. It never changes a rule on its own.
- **Net size.** A run that proposes a rule or check names one it considered
  removing.
- **No new tool surface.** The scripts stay scripts. The checklist is a file.
  Nothing is added to the `atelier` binary or the site.

# Design

Four steps, all in the backlog until the cutover has happened.

| Step | Does |
| --- | --- |
| A. Scorecard from records | Extend `scripts/scorecard/` to read the new records: status changes from Git history, threads from the records' fields. Remove the title matching mission 1 needed. |
| B. Readiness checklist | A short default list in the skill, mapped to the three properties, each line with the command or observation that tests it. A repository may add lines. Drawn from Factory's agent-readiness pillars, without the team-shaped criteria. |
| C. Procedure | Rewrite `garden.md` around the two layers, the three tiers and the one-page report. Fold `audit.md` into it. The gardener also sorts `list --deferred`. |
| D. First runs | One run in each of the five repositories. |

The three properties and what the report shows for each:

| Property | Worst spots shown |
| --- | --- |
| Easy to navigate | Files agents read most before their first edit |
| Easy and reliable to change | Files with the most rework and the most fix-now findings |
| Easy to validate | Features with no proof command in the verify skill; defects caught after merge |

# Derisking

The riskiest assumption is that the scorecard's numbers move enough, and
soon enough, for a run to have something to say. Step D's first runs come
with only days of data under the new tool. They are expected to report the
baseline and the checklist, and to say which numbers are not yet meaningful.
The comparison that judges the redesign is a separate item outside this
mission, picked up two weeks after the cutover.

# Scope

- In: the scripts, the checklist, the gardener procedure, the first runs.
- Out: a scorecard page on the site; a scheduler for the gardener; the groom
  procedure; scoring agents on any measure; the comparison two weeks after
  the cutover, which is its own item.
- Target environment: the five repositories on this host.

# Authority

The orchestrator may choose the checklist's lines, the report's layout, and
which transcript reading a run needs. It may not add a command or site page,
add a rule or check to any repository from a gardener run, or change a
measure's definition in the product specification.

# Risks and Unknowns

- **Too little data.** See Derisking.
- **The checklist grows.** Each line names the number that would show its
  failure, and a line with no such number is cut.
- **Where a repository's own checklist lines live** is settled in step B.
