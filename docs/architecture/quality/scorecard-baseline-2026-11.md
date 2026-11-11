# Scorecard Baseline, November 2026

This is the scorecard of the [product specification](../../product.md#scorecard),
measured before the redesign changes anything. Later missions are judged
against it. It covers atelier, astromech, demiurge, life-on-rails and
cosplay-studio from 2026-10-14 to the cutoff, 2026-11-10T05:14:58Z, when the
inputs were frozen ([manifest.json](scorecard-baseline/manifest.json)). Every
number here is copied from the files under
[scorecard-baseline/](scorecard-baseline/); where one is derived, its factors
are shown.

## What the numbers say

- **Review catches far more defects than escape it.** 33 defects were found
  after the merge that caused them, in 780 merged items: 15 of 277 in atelier and 18 of 361 in astromech, about 5 per 100 in each.
  Review caught roughly 407 product defects in the same window, and CI 22.
  Atelier's 15 is a floor: 35 of its 77 bug fixes could not be traced.
- **The other three repositories file almost no bugs.** Demiurge,
  life-on-rails and cosplay-studio fixed one bug item between them, so their
  zero defects after merge mean nothing was filed, not that nothing escaped.
- **Process is a quarter of the work in atelier.** 28.0% of shell commands in
  threads working on a merged atelier item call `atelier` or touch
  `.atelier/`, and they take 21.6% of the token cost. Elsewhere it is 7.6% to
  20.3% of commands.
- **One `atelier` call in four is a refusal, an error or a help lookup.**
  25.2 of every 100 `atelier` calls in atelier were one of these,
  and the other repositories range from 16.39 to 24.27. Help lookups are about
  half of it.
- **Most refusals are friction; about half of fix-now findings are real
  defects.** 11 of 100 sampled refusals stopped a real
  mistake. 46 of 100 sampled fix-now findings were product defects.
- **A merged item costs a few dollars of tokens.** The median is 3.06 USD at
  list price in atelier and 3.22 to 13.81 USD elsewhere, not counting
  orchestrator threads.

## What Sumeet is asked to decide

These targets are **proposals for Sumeet to accept or change**, not
decisions. Each applies after the cutover, measured with the same scripts on
a window of the same length.

| Measure | Baseline | Proposed target | Why this number |
| --- | --- | --- | --- |
| Process share | atelier 28.0% of shell commands and 21.6% of token cost; astromech 17.9% and 13.2%; demiurge 7.6% and 12.5%; life-on-rails 20.3% and 16.2% | At most half the baseline, in both forms, in each repository with at least 20 merged items tied to a thread: atelier 14.0% and 10.8%, astromech 9.0% and 6.6%, demiurge 3.8% and 6.3%, life-on-rails 10.2% and 8.1% | The redesign deletes whole kinds of process command rather than trimming them: evidence records, `land`, the light path, `merge --sync` and `--check`, the nine `work` views and nine `review` subcommands. Records stay, and so do `status`, `submit`, `review`, `verdict` and `merge`, so half is a judgment of what remains, not a computation. The baseline cannot split process commands by subcommand. |
| Confusion | atelier 25.2 per 100 `atelier` calls; astromech 23.6; demiurge 16.39; life-on-rails 22.52; cosplay-studio 24.27 | At most half the baseline in every repository: atelier 12.6, astromech 11.8, demiurge 8.2, life-on-rails 11.3, cosplay-studio 12.1 | Both halves of it are designed away. Help lookups (632 of atelier's 1,164 confused calls) follow from a large surface; the redesign has eighteen commands and `status` prints the next one. Most friction refusals in the gate sample were an ID of the wrong record kind, a record on another branch, a base that moved with documents only, or a reviewer asking for its brief; only missions and items have IDs, the base's record is read first, a moved base needs no catch-up, and `review <id>` prints the brief. Half, not less, because wrong-role commands join the measure. |
| Tokens per merged item | Median atelier 3.06 USD, astromech 3.22, demiurge 7.9, life-on-rails 11.96 | Median at most baseline × (1 − token process share ÷ 2): atelier 3.06 × 0.892 = 2.73, astromech 3.22 × 0.934 = 3.01, demiurge 7.9 × 0.9375 = 7.41, life-on-rails 11.96 × 0.919 = 10.99 USD | It falls by the process tokens the first target removes, and no further. Review rounds are not a cost to cut (Principle 5), so the target asks nothing of reviewer or implementer work on the code. Prices stay those in [counted.md](scorecard-baseline/counted.md#list-prices), so a change in list prices does not move it. A cheaper model mix does: each model keeps its own price, so moving work to a cheaper model lowers the cost without removing any process. A changed mix confounds the comparison, and the after-run states it beside the number. |

Halves are rounded to one decimal. Cosplay-studio gets no process or token
target, since only 3 of its merged items have a tied thread. Confusion counts
every thread, so it is included there.

**The constraint: defects after merge must not rise.** In checkable form:

- **What is compared.** Defects after merge per 100 merged items, counted by
  [defects.md](scorecard-baseline/defects.md)'s method, with
  [defect-method.md](scorecard-baseline/defect-method.md)'s step 6 as amended
  there, counting only causes merged in the window. The denominator is
  defects.md's: done task, feature, bug and spike items closed in the window.
- **Against which numbers.** Astromech 18 of 361, 5.0 per 100, is the main
  comparison, since only 4 of its 52 bug fixes were untraceable. Atelier is
  12 of 277, 4.3 per 100 (its 15 less the 3 whose causes merged in June and
  July), a floor, compared second.
- **Over what period.** The after-run takes the first 27 days of merges
  after the cutover, the baseline window's length, and counts at the end of
  day 27. Recent merges have had less time to show their defects; every
  astromech cause merged in the first week of its window. An equal window
  caps how long any merge is watched, but does not make the ages equal: if
  the after-window's merges sit later in it than the baseline's, they have
  had less time to show defects. The after-run says how its merges are
  spread over the window, and where they sit materially later, the raw rate
  alone cannot show the constraint held. Neither count is redone later,
  since a later recount finds more in the older set.
- **When the comparison means nothing.** The method sees only defects filed
  as bug items. If the after-run files fewer bug fixes per 100 merged items
  than the baseline (atelier 77 of 277, 27.8; astromech 52 of 361, 14.4), a
  lower defect count is not evidence, and the after-run says so instead of
  claiming the constraint held. The redesign drops issue types, so the
  after-run counts a bug as an item labelled `bug` or carrying `caused-by`.
- **What a rise triggers.** Counts this small move by a few from chance
  alone. A rate above the baseline is a breach to investigate, item by
  item, not an automatic verdict on the redesign.

No target is proposed for interventions, gate precision, review yield or
the codebase health rows; the item asks for none.

## The scorecard

Per repository where the measure is per repository. Methods, coverage and
limits follow the table, one entry per row.

| # | Measure | atelier | astromech | demiurge | life-on-rails | cosplay-studio |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | Defects found after merge | 15 of 277 merged items (5.4 per 100) | 18 of 361 (5.0) | 0 of 86 | 0 of 38 | 0 of 18 |
| 2 | Interventions per mission | 0 to 7, 12 missions | no mission closed | 0, 0 and 17 | 12, 1 mission | no mission closed |
| 3 | Process share, shell commands | 28.0% of 14,295 | 17.9% of 17,485 | 7.6% of 3,021 | 20.3% of 5,314 | 12.2% of 408 |
| 3 | Process share, token cost | 21.6% | 13.2% | 12.5% | 16.2% | 7.0% |
| 4 | Confusion per 100 `atelier` calls | 25.2 of 4,619 | 23.6 of 4,136 | 16.39 of 671 | 22.52 of 1,239 | 24.27 of 342 |
| 5 | Tokens per merged item, median (mean) USD | 3.06 (7.61) | 3.22 (9.11) | 7.9 (11.79) | 11.96 (13.4) | 13.81 (18.2) |
| 6 | Gate precision | 4 of 54 | 3 of 28 | 1 of 6 | 3 of 12 | 0 of 0 |
| 7 | Review yield | 8 of 20 | 6 of 20 | 8 of 20 | 6 of 20 | 18 of 20 |
| 8 | Skips | no such mechanism | same | same | same | same |
| 9 | Calls before first edit, median | 22 (81 threads) | 18 (130) | 10 (49) | 17 (31) | 21 (4) |
| 10 | Rework | 10.8% of 57,671 lines | 7.2% of 209,453 | 3.4% of 38,189 | 5.5% of 74,203 | n/a, no eligible commit |
| 11 | Where caught: review, CI, after merge | about 191, 6, 15 | about 112, 16, 18 | about 29, no CI, 0 | about 23, no CI, 0 | about 52, no CI, 0 |
| 12 | Non-test lines | 48,989 Rust | 78,713 Rust | 73,155 TypeScript | 29,285 TypeScript | 8,380 TypeScript |

Totals across repositories: 33 defects after merge in 780 merged items;
interventions 0 to 17 per started mission, median 2.5, 60 in 16 missions;
gate precision 11 of 100; review yield 46 of 100.

### Each row: method, coverage, limits

1. **Defects after merge.** Every done bug item closed in the window (130)
   traced by `git blame` and a read of both changes to the merge that
   introduced it; after merge when the bug was filed after the cause reached
   the base ([defects.md](scorecard-baseline/defects.md)). Coverage: all
   130, no sample. Limits: only bugs filed as items are seen; 35 of
   atelier's 77 are untraceable and 12 trace to commits no item owns, so its
   count is a floor; 3 of its 15 causes merged before the window; recent
   merges have had less time to show defects. Window starts at adoption for
   astromech (10-15), demiurge (10-17) and life-on-rails (11-06).
2. **Interventions per mission.** Every message the owner typed in a closed
   mission's threads between its start and close, labelled by
   [rubric.md](scorecard-baseline/rubric.md); status questions, unsticking
   and correcting direction count ([classified.md](scorecard-baseline/classified.md#interventions-per-mission)).
   Coverage: 142 messages in the 16 missions that started, of 21 closed.
   Limits: one rater, 73% agreement with a second on 30; most disagreement is
   whether a message in the thread above an orchestrator is about the mission
   at all, and the rubric needs a sharper test there before the after-run.
   Counting approvals of a paused agent as unsticking would add 19.
3. **Process share.** Of shell commands in threads tied to a merged item,
   the share that call `atelier` or name an `.atelier/` path; the token form
   splits each response's list-price cost equally over its tool calls
   ([scripts/scorecard/README.md](../../../scripts/scorecard/README.md#how-each-measure-is-taken)).
   Coverage: token form over 342 of 346 tied threads in atelier, all of them
   elsewhere. Limits: commands are read by heuristic, judged on the frozen
   inputs; record edits made with an editor tool rather than the shell are
   not counted; a thread is tied by the first item ID in its branch, title
   or first message, so orchestrator threads naming a mission are left out.
4. **Confusion.** `atelier` calls whose output has a refusal or error line,
   or that look up help, per 100 calls, over every thread in the
   repository. Coverage: every T3 thread. Limits: commands run by the wrong
   role, which the specification's row names, are not measured, since no
   command has a role today; the after-run must recognise the new refusal
   text and the wrong-role line, or it will undercount.
5. **Tokens per merged item.** List-price cost of every thread tied to the
   item, summed; an item counts only when all its threads have transcripts
   and every request is priced. Coverage: 142, 191, 27, 34 and 3 items;
   977 of 981 tied threads have a transcript. Limits: orchestrator threads
   are not tied to one item and are left out, so this is not the whole cost;
   Fable 5.1, Fable 5.2 and GPT-6 Astra have no list price in the table and
   stay unpriced (6 threads); Codex sessions survive only from 2026-11-08.
6. **Gate precision.** A seeded sample of refusals of real work, labelled
   stopped a mistake or friction. Coverage: 100 of the 430 refusals in the
   population. Limits: one rater, 83% agreement; the rubric's line between a
   real problem and a state the agent already knew needs a sharper rule.
   This count of refusals (430) is narrower than row 4's (565 in atelier
   alone): it leaves out argument errors, scratch repositories, `check` and
   help runs.
7. **Review yield.** A seeded sample of fix-now findings, 20 per repository,
   labelled in seven categories; user bugs plus tooling bugs are product
   defects. Coverage: 100 of 1,056 fix-now findings (atelier 478, astromech
   372, life-on-rails 76, demiurge 72, cosplay-studio 58). Limits: one rater,
   97% agreement; entry 38 is mislabelled, and corrected the yield is 45 of
   100 and atelier's 7 of 20; findings on plans count as process by rule, so
   what planning review prevents is undercounted.
8. **Skips.** `ATELIER_SKIP` does not exist before the redesign. The
   baseline is that no such mechanism exists, not zero uses of one.
9. **Calls before first edit.** In implementer threads tied to a merged
   item, tool calls before the first write outside `.atelier/` and scratch
   directories; shell commands alone are in
   [counted.md](scorecard-baseline/counted.md) beside it. Limits: the same
   heuristic as row 3 decides what a shell write is; cosplay-studio has 4
   threads.
10. **Rework.** Of lines that reached the base from 2026-10-14 to 14 days
    before the cutover, the share deleted within 14 days; `.atelier/` and
    lock files left out. Limits: churn inside a branch before merge is not
    seen; cosplay-studio had no commits in the eligible span.
11. **Where caught.** Review: fix-now findings × the sample's review-yield
    rate (atelier 478 × 8/20 = 191). CI: failed runs on a work branch fixed
    by a later commit, read once from GitHub. After merge: row 1
    ([defects.md](scorecard-baseline/defects.md#where-defects-were-caught)).
    Limits: **defects caught by an author's own tests are not observable**
    before the redesign, since nothing records them; each review product
    rests on 20 findings (8 of 20 has a 95% interval of about 19% to 64%);
    atelier's CI dates from 2026-10-28; a failed run may be a runner fault.
12. **Non-test lines.** `scripts/count_non_test_rust.py` for the two Rust
    repositories; the main language's non-test physical lines otherwise.
    Counted at each repository's manifest commit. Limits: what counts as a
    test differs by language, as counted.md lists.

## Two counts of merged items

Two denominators exist, and each number above says which it uses.

- **Rows 1 and 11** use defects.md's: done task, feature, bug and spike
  items closed in the window (atelier 277, astromech 361).
- **Rows 3, 5 and 9** use counted.md's: done items, other than missions,
  named by a commit that changes code (atelier 243, astromech 339). An item
  that merged no code is not merged here, and cosplay-studio's commits name
  items by suffix, so only 8 of its 23 done items count.

## Where the numbers differ from the assessment's

The Before column held the assessment's numbers
(`/var/tmp/atelier-assessment/reports/`, outside the repository) until this
report replaced them.

- **Defects after merge:** "not measured" before; now traced, 15 and 18.
- **Interventions:** "2 to 8 per mission" came from reading November's five
  atelier missions without a rubric. The rubric counts only status,
  unsticking and direction, not approvals or messages about other work, and
  gives those five 2 or 3 each; across 16 missions it is 0 to 17.
- **Process share:** the assessment's 28% was a different measure that
  happens to round to the same number: the median share of an issue's calls
  after its last product edit, with 12% bookkeeping. This one is the share
  of all of an item's shell commands that are process, the strategy's
  definition.
- **Confusion:** 4.1% was refusals per `atelier` call in real use, with help
  lookups left out. This counts help lookups and argument errors too, per
  100 calls, so it is 25.2.
- **Tokens per merged item:** 11.50 to 20 USD was two days' total atelier
  spend, orchestrators included, divided by issues merged, with the upper
  end inferred. This is per item over the whole window, with orchestrator
  threads left out, so the median is 3.06.
- **Gate precision:** about 16% came from ten each of the five most common
  refusal kinds. This is a seeded sample of all refusals: 11 of 100.
- **Review yield:** 39% came from 25 findings in each product repository,
  atelier excluded. This samples 20 in each of the five, atelier included,
  under a written rubric: 46 of 100.
- **Skips:** unchanged; no such mechanism.
- **Calls before first edit and rework:** not measured before.
- **Where caught:** only review was counted; now review, CI and after merge
  are, and author's tests are stated as unobservable.
- **Non-test lines:** 48,988 on 2026-11-09; 48,989 at the manifest commit,
  a day later.

## Rerunning

The scripts at commit `df9c84a43` reproduce these numbers. Later scripts
read `atelier log` and the record layout of Atelier 0.5, which this
snapshot and these repositories' manifest commits do not have.

- **Counted measures** (rows 3, 4, 5, 9, 10, 12): run `counted.py` as
  [scripts/scorecard/README.md](../../../scripts/scorecard/README.md) says,
  and compare the SHA-256 of `counted.json`; `freeze.py verify` checks the
  snapshot. A fresh agent given only the README reproduced the file byte for
  byte.
- **Classified measures** (rows 2, 6, 7): `draw_samples.py` redraws the same
  samples, and a new rater labels them under
  [rubric.md](scorecard-baseline/rubric.md).
- **Defects and where caught** (rows 1, 11): the candidate loop in
  [defects.md](scorecard-baseline/defects.md#drawing-the-candidates) and the
  steps in [defect-method.md](scorecard-baseline/defect-method.md), with
  defects.md's changes to them.

What a rerun cannot reproduce:

- **The CI counts.** They were read once from GitHub, which expires old run
  history, so a later query may see fewer runs.
- **The judgment.** Another rater will differ: agreement was 73%, 83% and
  97%, and two tracers could pick a different cause on the rows defects.md
  names.
- **The snapshot.** It is 7.54 GB on one disk with no backup. If it is lost,
  the counted measures cannot be rerun, and these numbers stand only as
  recorded here.

## Sources

- [counted.md](scorecard-baseline/counted.md) and
  [counted.json](scorecard-baseline/counted.json): rows 3, 4, 5, 9, 10, 12.
- [classified.md](scorecard-baseline/classified.md),
  [rubric.md](scorecard-baseline/rubric.md) and
  [samples/](scorecard-baseline/samples/): rows 2, 6, 7.
- [defects.md](scorecard-baseline/defects.md) and
  [defect-method.md](scorecard-baseline/defect-method.md): rows 1, 11.
- [manifest.json](scorecard-baseline/manifest.json): the cutoff and each
  repository's commit.
- The mission's strategy: the definitions and risks. It left the tree with
  the old docs; `git show 96d02deb7:docs/strategies/redesign-1-baseline.md`
  prints it.
