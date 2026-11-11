---
status: draft
last_changed: "2026-11-11: steps A and B planned in full after the cutover; step A is two items, the command and the scripts that read it; plan review 1-c27c answered; step C planned, plan review 3-x9w6 answered; step D planned"
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
  implementer, and its Report is the page. The page states the window it
  measured, and the next run's window starts where that one ended, so an
  item merged while a run waited for its session is counted by the next.

# Design

Four steps. The cutover ran on 2026-11-11, so the mission can start. Each
step is planned in full at its boundary, with one plan review, before its
work starts. Steps A and B do not depend on each other and are planned
first, then C, then D.

| Step | Item | Does |
| --- | --- | --- |
| A. The log | atelier-8b0a | Build `atelier log` as the [design record](../../../docs/design.md#the-log) describes. |
| A. The scripts | atelier-x3n5, after atelier-8b0a | Take from `log` what `scripts/scorecard/` found by walking history and matching thread titles, and delete those readers. |
| B. Readiness checklist | atelier-vl5n | A short default list in the skill, mapped to the three properties, each line with the command or observation that tests it. A repository may add lines. Drawn from Factory's agent-readiness pillars, without the team-shaped criteria. |
| C. Procedure | atelier-9dku, after atelier-8b0a and atelier-vl5n | Rewrite `garden.md` around the two layers, the three tiers and the one-page report, with the steps that turn `log` output into its numbers. Fold `audit.md` into it. The gardener settles deferred findings with `new --from` and `drop`, on its run's branch, so its choices are reviewed with the run. |
| D. First runs | atelier-613a, after atelier-9dku | One run in each of the five repositories. |

Decided while planning A and B:

- **Step A is two items.** The command is Rust and the scripts are Python,
  each with its own tests, and the scripts read the line layout the command
  fixes. Two items give each one review boundary, and the scripts can be
  done while step C is. Step C needs only the command.
- **The scripts keep one reader, for the new layout.** They stop reading the
  old layout, so they no longer reproduce the November baseline; the README
  names the commit whose scripts do. Keeping both readers would keep the
  code this mission is meant to delete.
- **The log is frozen with the other inputs.** `atelier log` reads the
  base and the branches as they are when it runs, and takes no commit to
  read at. So the scripts never run it: `freeze.py` saves its output into
  the snapshot, and a rerun from one manifest gives the same numbers
  whatever the repository has done since. `log` needs no flag for this.
- **The counted population does not change.** A merged item is still one
  whose merge changes a file outside `.atelier/`. The log's `merge` lines
  name the candidates and their commits, and the scripts keep that test.
- **atelier-c1z3 comes after atelier-x3n5.** Both edit `scripts/scorecard/`.
  atelier-c1z3 also needs the frozen inputs of the run after the cutover,
  which do not exist yet, so it is blocked by atelier-x3n5 and is not part
  of this mission. atelier-pwhn, the comparison, stays outside it too.
- **A log line** is the commit's time, its short hash, the record's ID, the
  act, the thread or `-`, then what changed, with a skip last. atelier-8b0a
  holds the layout in full.
- **A repository's own checklist lines** live in `.atelier/readiness.md` in
  that repository. `atelier check` accepts the file, and the pre-push hook
  counts it as code, so a line reaches the base only through a reviewed
  merge.
- **No scorecard number from T3 or the transcripts is new in this
  mission.** Those need a frozen snapshot, about 7.5 GB, which a routine run
  does not take. What a run does about them is settled in step C.

Decided while planning C, after atelier-8b0a (pull request 145) and
atelier-vl5n (pull request 144) merged:

- **The session comes before the merge.** A run is submitted and reviewed
  like any change, then waits approved for the session, which records what
  was agreed in the page and merges the run. A done item's directory cannot
  change, so the page has to be settled before the merge.
- **A run takes no snapshot.** It takes its numbers from `atelier log`,
  `atelier list` and the checklist, and carries the measures that need T3's
  events, the transcripts or a rater from the last time they were measured,
  with that date. atelier-pwhn refreshes them.
- **Navigation's worst spots come from reading at most three threads** the
  log names, since no script counts the files an agent reads. The page says
  it is a sample.
- **A page is at most 60 lines,** with the workings in the run item's
  `artifacts/`.
- **The trial is the first run.** atelier-9dku shows its arithmetic on
  atelier's real log, and atelier's run in step D goes first, as the
  procedure's trial, before the other four.

Decided while planning D, after atelier-9dku merged (pull request 146):

- **Each run is its own item in its own repository,** and atelier-613a
  holds only the result. atelier's run goes first, as the trial, and one
  item fixes what it shows wrong in the procedure before the other four
  start together.
- **A run stops at its approval.** The session, the run's merge and
  everything else the session settles are Sumeet's.
- **Only atelier and demiurge have merged an item since the cutover,** so
  three pages report the baseline, the checklist and what the records show,
  and say which numbers mean nothing yet.
- **A page holds nothing private.** astromech and demiurge are public.

The three properties and what the report shows for each:

| Property | Worst spots shown |
| --- | --- |
| Easy to navigate | Files agents read most before their first edit |
| Easy and reliable to change | Files most named in fix-now findings. Rework by file when a measurement gives it; a routine run carries the repository's rework share and does not rank files by how much they changed |
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
  the old debrief reader did not. atelier-8b0a settles it: one `git log`
  over the record paths and one `git cat-file --batch`. Only commits since
  the cutover hold records in the new layout, so the walk is short today: a
  `git log` over astromech's record paths took 0.04 s on 2026-11-11.
