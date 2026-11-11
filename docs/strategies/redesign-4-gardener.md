---
status: draft
last_changed: "2026-11-10: amended at Sumeet's direction: a proposal is a root cause with its trips as evidence; something broken today is a bug, not a gardener entry; a run ends in a session with Sumeet"
---

# Why

The gardener is the role meant to keep a codebase easy to navigate, easy to
change and easy to validate. As written it collects incidents. It has no
numbers, so it cannot tell whether its last fixes worked. It hears refusals
and retry loops and misses slow costs, such as an agent reading 40 files
before it can edit. Its one digest, atelier-j86z, is ten entries about the
tool and none about a codebase, and it waits in a queue where 6 of 63 items
have been answered.

It also works like a firefighter. Its unit is the trip, and it asks Sumeet
for a yes or no on each one, which gives every incident the same weight. A
trip is a data point. Read by cause, the ten entries of atelier-j86z are four
causes and one stray: records that live on the base while agents read
elsewhere (two entries), docs and help that lag the code (three), checks that
run in the wrong place (three), and an error the tool hides (one). Four of
the ten are help or doc fixes that needed no decision.

After the cutover the acts that move an item leave their mark in its records,
and so in Git: a status, a holder, a verdict, an answered question. The
history of what happened to each item can be read from there. Nothing reads
it yet.

# Outcome

`atelier log` prints the history of acts. A gardener run starts from that,
from `atelier list`, and from a readiness checklist for the repository, and
its procedure says which numbers to take from them. It reads transcripts
only to explain a number that moved. It settles the deferred findings. It
ends with one page for Sumeet: the numbers against the baseline, the worst
spots, and at most three proposals, each a root cause with the trips that
show it. One run has been made in each of the
five repositories.

# Success Criteria

- one-page: a gardener run in each of the five repositories produces a report
  of one page that holds the scorecard, the worst spots under each of the
  three codebase properties, and at most three proposals. Each proposal
  names one root cause, the trips or numbers that are its evidence, and the
  number it should move. No page asks for an answer per trip.
- from-log: the scorecard's numbers about what happened to items, which are
  time in each status from `ready` on, rounds, roles taken over, escalations
  and the skips `log` shows, are computed from the output of `atelier log`
  by the steps the
  gardener's procedure gives, and no script walks record history itself.
  The scripts that remain tie a thread to an item by the thread IDs in the
  log, with no matching on thread titles.

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
| A. The log | Build `atelier log` as the [architecture specification](redesign-architecture.md#the-log) describes. Remove from `scripts/scorecard/` what `log` now supplies, and tie the remaining scripts to items by the log's thread IDs. |
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

# Scope

- In: `atelier log`, the scripts that remain, the checklist, the gardener
  procedure, the first runs.
- Out: a command that computes the scorecard; a scorecard page on the site;
  a command that starts a run or runs the checklist; a scheduler for the
  gardener; the groom procedure; scoring agents on any measure; the
  comparison two weeks after the cutover, which is its own item.
- Target environment: the five repositories on this host.

# Authority

The orchestrator may choose the checklist's lines, the report's layout, the
layout of a log line, and which transcript reading a run needs. It may not
add a command beyond `log` or a site page, add a rule or check to any
repository from a gardener run, or change a measure's definition in the
product specification.

# Risks and Unknowns

- **Too little data.** See Derisking.
- **The checklist grows.** Each line names the number that would show its
  failure, and a line with no such number is cut.
- **The history walk.** `log` has to follow both sides of each merge, which
  today's debrief reader does not, and astromech has the most records. The
  debrief's walk went from 7.8 s to 1.04 s when it became one `git log` over
  the records; step A's planning settles how `log` stays a single walk.
- **Where a repository's own checklist lines live** is settled in step B.
