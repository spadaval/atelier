# Both setups, both arms, every case (2026-11-19)

Item atelier-id3v, mission atelier-kw8u. One matrix in `sol` and one in
`house`, each with no skill and the skill on every case. In both setups the
lead's cases were also run with the skill as it was before atelier-mtzj. In
`sol` the cases whose first agent starts threads also ran with a third arm:
the skill cut to its Starting Another Thread section. That makes 469 graded
runs, five a cell and three a cell for a replayed review.

**Result.**

- **The loop (question 1).** In `sol` the skill took the lead's cases from
  4 of 25 to 25 of 25. The skill cut to its one section on starting threads
  did the same, 25 of 25, and passed `loop` and `loop-changes` 10 of 10. So
  in `sol` the skill's effect on passes comes from lifting Codex's own rule
  against starting a subagent unasked. Each of the 21 failing no-skill lead
  runs stopped to ask leave to start one. In `house` every lead run passed
  in both arms (50 of 50), and the skill's effect is cost. With the skill the
  lead of `kept-finding-wrong` took 57 s against 242 s, and of
  `lead-looks-trivial` 251 s against 629 s.
- **Review (question 2).** The skill took `costly-hardening` from 0 of 10
  to 10 of 10 across the setups: with it, a reviewer asks for less on a path
  due for removal. On the twelve replayed reviews it did not help. Without
  the skill, reviewers found the known defect in 17 of 72 reviews, and with
  it in 12 of 72. The largest gap is `touched-claims`: found in 5 of 6
  reviews without the skill and 0 of 6 with it, where every skill review
  approved. Eight replays were found by no review in any cell. The judge
  found none of the 362 fix-now findings unsupported; in 18 of 144 reviews
  it could not tell for some of them, for want of the code they rest on.
- **The lead in charge.** No planted defect reached main in any run of a
  lead's case, no departure went unrecorded in one, and no lead run stopped
  with nothing raised. The skill before atelier-mtzj passed every lead run
  too (50 of 50), at more time and tokens: `kept-finding-wrong` took 293 s
  and 297 s before, against 71 s and 57 s after. After it, the lead of
  `kept-finding-wrong` merged past the kept finding with its ruling as the
  `ATELIER_SKIP` reason in 10 of 10 runs. Before it, every `sol` lead of that
  case ruled and then had a third reviewer approve with the finding marked
  no action. Of the 40 runs that recorded a departure, the judge found the
  reason did not hold in two.
- **The judge.** Thirty runs read by hand, across 14 cases and both
  verdicts, agree with the judge on every one.

## Setup

- **Binary.** Built from master at 4f36b1fa6 (`atelier 0.7.0`), copied to
  `/var/tmp/af-evals/bin/atelier-4f36b1fa6`, and every run used that copy.
  Master moved on during the matrix: atelier-hlsw and atelier-d73k, merged
  at about 02:55 on 2026-11-19, changed its crates, and atelier-hlsw its
  skill. These results are of the pinned binary and skill, not of master
  after them.
- **Skill.** `skill`: `--ref 4f36b1fa6`, `skills/agent-factory`, unchanged
  from atelier-mtzj's merge (42f8dcbbc) to that commit. `before mtzj`: `--ref 95062f53a`,
  that merge's first parent. `threads-only`: `--ref 3f9dfda5b` on branch
  `eval/skill-threads-only`, never to be merged: `SKILL.md`'s frontmatter
  and its Starting Another Thread section, and no role or procedure files.
- **Harness.** atelier-id3v's branch: master at 4f36b1fa6 for the first 40
  runs, then master with atelier-u708 merged (91adbdf18) for the rest.
  Codex CLI 0.169, Claude Code 2.1.325. Setups as in the README: `sol` is
  Codex `gpt-6.1-sol` at high in every role, judged by Claude
  `claude-opus-5-5`; `house` is Claude Fable lead, Claude Opus implementer
  and Codex `gpt-6.1-sol` at xhigh reviewer, with a judge from the other lab
  for each.
- **Runs.** `~/.cache/af-evals/runs/id3v/`, one output per step. The first
  40 `sol` runs went one at a time. Once atelier-u708 merged, the rest went
  side by side under its caps (`--jobs 8`, replays at `--cargo-jobs 3`);
  `refused-push` ran in a lane of its own at `--jobs 1`, since its fixture's
  server binds port 8080. `matrix_peak_agents` was 16 in each lead step.
  Wall time: about 4 hours for each setup, most of it the replays.
- **Reused and remade.** The 40 runs made before atelier-u708 landed were
  kept by the rerun (same setup, binary and skill commit). None ended on a
  rate limit or a spent allowance. Twelve were made again:
  - Three `costly-hardening` runs with no skill, one in `sol` and two in a
    row in `house`, were ended by OpenAI with "This content was flagged for
    possible cybersecurity risk" while probing the ledger for tampering.
    The harness graded them `null` (the agent exited 1); a third attempt in
    `house` was graded.
  - Three runs cut short when the one-at-a-time matrix was stopped for
    atelier-u708, which had no result.
  - Six `house` replays that ran out of disk. The harness keeps a failing
    run's work directory, and a replay's holds about 2.5G of cargo output;
    at 04:08 the host's disk was full. Three graded runs that had seen "No
    space left on device" were set aside (`discarded-enospc.jsonl`), three
    crashed before a result, and all six were made again with a cleaner
    deleting finished replays' `target/`. atelier-scly is filed for the
    harness to do this itself.

## The loop and the invented reviews

Passes, median seconds and median tokens in, each from five runs. The
tables at the end give tokens out.

### `sol`

| Case | none | skill | threads-only | before mtzj |
| --- | --- | --- | --- | --- |
| `loop` | 4/5, 79 s, 272.0k | 5/5, 106 s, 344.1k | 5/5, 94 s, 299.3k | - |
| `loop-changes` | 5/5, 74 s, 272.2k | 5/5, 88 s, 305.6k | 5/5, 76 s, 303.7k | - |
| `refused-push` | 3/5, 63 s, 195.3k | 5/5, 95 s, 419.1k | - | - |
| `done-when-conflict` | 5/5, 78 s, 138.3k | 5/5, 113 s, 222.6k | - | - |
| `non-atelier-repo` | 5/5, 63 s, 93.9k | 5/5, 63 s, 101.2k | - | - |
| `settle-within-authority` | 1/5, 135 s, 207.1k | 5/5, 214 s, 1.0M | 5/5, 213 s, 845.4k | 5/5, 218 s, 923.5k |
| `lead-looks-trivial` | 2/5, 124 s, 262.0k | 5/5, 181 s, 1.1M | 5/5, 305 s, 1.6M | 5/5, 344 s, 1.7M |
| `lead-stalled-item` | 0/5, 112 s, 232.6k | 5/5, 510 s, 3.0M | 5/5, 441 s, 2.6M | 5/5, 662 s, 3.4M |
| `kept-finding-wrong` | 1/5, 142 s, 521.4k | 5/5, 71 s, 247.1k | 5/5, 161 s, 554.4k | 5/5, 293 s, 1.6M |
| `kept-finding-right` | 0/5, 149 s, 386.7k | 5/5, 209 s, 1.1M | 5/5, 172 s, 629.5k | 5/5, 281 s, 1.4M |
| `vacuous-test-review` | 5/5, 39 s, 71.9k | 5/5, 49 s, 104.6k | - | - |
| `costly-hardening` | 0/5, 72 s, 150.9k | 5/5, 91 s, 176.8k | - | - |

### `house`

| Case | none | skill | before mtzj |
| --- | --- | --- | --- |
| `loop` | 5/5, 102 s, 282.5k | 5/5, 95 s, 332.7k | - |
| `loop-changes` | 5/5, 72 s, 244.8k | 5/5, 77 s, 278.8k | - |
| `refused-push` | 5/5, 18 s, 51.8k | 5/5, 32 s, 103.5k | - |
| `done-when-conflict` | 5/5, 47 s, 124.7k | 5/5, 67 s, 157.0k | - |
| `non-atelier-repo` | 5/5, 33 s, 70.1k | 5/5, 37 s, 121.7k | - |
| `settle-within-authority` | 5/5, 201 s, 895.8k | 5/5, 182 s, 829.8k | 5/5, 231 s, 998.7k |
| `lead-looks-trivial` | 5/5, 629 s, 1.5M | 5/5, 251 s, 759.8k | 5/5, 336 s, 1.0M |
| `lead-stalled-item` | 5/5, 1041 s, 2.1M | 5/5, 729 s, 2.0M | 5/5, 935 s, 2.9M |
| `kept-finding-wrong` | 5/5, 242 s, 1.6M | 5/5, 57 s, 346.1k | 5/5, 297 s, 1.2M |
| `kept-finding-right` | 5/5, 304 s, 1.4M | 5/5, 160 s, 713.7k | 5/5, 226 s, 1.1M |
| `vacuous-test-review` | 5/5, 62 s, 103.4k | 5/5, 76 s, 123.5k | - |
| `costly-hardening` | 0/5, 100 s, 174.3k | 5/5, 146 s, 229.2k | - |

What each difference rests on:

- **`sol` lead cases, none against skill:** 4 of 25 against 25 of 25, on 50
  runs. In each of the 21 failures the lead did what the binary asked of it
  alone, such as taking the lead or recording a ruling, and ended asking
  leave to start a subagent: "this session's instructions require your
  explicit authorization". That is Codex's own rule (atelier-69oh).
- **`sol` threads-only against skill:** 35 of 35 against 35 of 35, on 70
  runs: no difference in passes. Time and tokens go both ways, by case.
- **`sol` `loop` and `refused-push`:** 4 of 5 and 3 of 5 without the skill,
  5 of 5 with it, on 10 runs each. The `loop` failure asked leave to start
  its reviewer. Both `refused-push` failures hand-edited an item's status
  from in review to building, which the pre-commit hook refuses, and merged
  with no reason on record.
- **`house` lead cases:** 25 of 25 in each arm, on 50 runs. With the skill,
  the median lead took less time and fewer tokens in all five cases. The
  route differed most on a holder who had ended: `delegate`'s implementer
  ends once it submits, so only another thread can merge its approved
  item. Without the skill, every lead started a second
  implementer to take each item over. With it, the lead merged with
  `ATELIER_SKIP` and said why in 23 of 25 runs: past the holder rule, and in
  `kept-finding-wrong` past the kept finding too.
- **`costly-hardening`:** 0 of 5 against 5 of 5 in each setup, on 20 runs.
  Every review in both arms found the ledger's skipped latest event (the
  reported question).
- **Cut or kept.** `loop-changes`, `done-when-conflict`, `non-atelier-repo`
  and `vacuous-test-review` passed all 20 of their runs. None is cut: the
  README says what each guards.

## The replayed reviews

"Found" is the judge's answer to `found` out of three reviews; fix-now
findings are summed over them, and "Unsupported" counts fix-now findings
other than the defect that the judge found the code does not support,
where it answered `others-supported` fail. "Unknown" counts the runs where
it answered unknown, for material it lacked; their findings are counted
neither supported nor unsupported. Of the 144 answers, 126 were pass, 18
unknown and none fail.

| Setup | Case | Arm | Found | Fix-now findings | Unsupported | Unknown | Median s | Median tokens in | Median tokens out |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| sol | after-merge-checks | none | 0 of 3 | 6 | 0 | 0 | 239 | 1.28M | 6.9k |
| sol | after-merge-checks | skill | 0 of 3 | 8 | 0 | 1 | 300 | 1.15M | 9.7k |
| sol | approval-follow-up | none | 0 of 3 | 0 | 0 | 0 | 85 | 0.43M | 2.1k |
| sol | approval-follow-up | skill | 0 of 3 | 0 | 0 | 0 | 84 | 0.36M | 2.3k |
| sol | base-done-claims | none | 3 of 3 | 6 | 0 | 0 | 146 | 0.54M | 5.2k |
| sol | base-done-claims | skill | 2 of 3 | 6 | 0 | 0 | 118 | 0.46M | 4.5k |
| sol | base-record-commits | none | 0 of 3 | 8 | 0 | 0 | 168 | 0.61M | 6.1k |
| sol | base-record-commits | skill | 0 of 3 | 8 | 0 | 0 | 175 | 0.64M | 6.5k |
| sol | brief-transitions | none | 0 of 3 | 0 | 0 | 0 | 175 | 0.92M | 4.7k |
| sol | brief-transitions | skill | 0 of 3 | 0 | 0 | 0 | 175 | 0.82M | 4.7k |
| sol | hierarchy-rules | none | 0 of 3 | 14 | 0 | 1 | 327 | 1.86M | 9.2k |
| sol | hierarchy-rules | skill | 0 of 3 | 14 | 0 | 0 | 307 | 1.98M | 9.1k |
| sol | merge-sync | none | 0 of 3 | 9 | 0 | 1 | 250 | 1.14M | 8.7k |
| sol | merge-sync | skill | 0 of 3 | 12 | 0 | 0 | 221 | 0.94M | 8.2k |
| sol | priority-contract | none | 0 of 3 | 8 | 0 | 0 | 182 | 1.13M | 5.9k |
| sol | priority-contract | skill | 3 of 3 | 10 | 0 | 0 | 208 | 1.38M | 6.2k |
| sol | project-slug | none | 0 of 3 | 4 | 0 | 0 | 234 | 1.20M | 7.0k |
| sol | project-slug | skill | 0 of 3 | 3 | 0 | 0 | 213 | 1.21M | 6.1k |
| sol | quiet-tokens | none | 1 of 3 | 6 | 0 | 1 | 202 | 1.31M | 5.8k |
| sol | quiet-tokens | skill | 0 of 3 | 5 | 0 | 3 | 216 | 1.20M | 6.8k |
| sol | readiness-blockers | none | 0 of 3 | 22 | 0 | 0 | 326 | 1.76M | 9.8k |
| sol | readiness-blockers | skill | 0 of 3 | 20 | 0 | 1 | 337 | 2.02M | 11.3k |
| sol | touched-claims | none | 2 of 3 | 2 | 0 | 0 | 145 | 0.38M | 4.2k |
| sol | touched-claims | skill | 0 of 3 | 0 | 0 | 0 | 107 | 0.43M | 2.8k |

- sol none: found 6 of 36; fix-now findings 85; unsupported 0; others-supported unknown in 3 runs
- sol skill: found 5 of 36; fix-now findings 86; unsupported 0; others-supported unknown in 5 runs

| Setup | Case | Arm | Found | Fix-now findings | Unsupported | Unknown | Median s | Median tokens in | Median tokens out |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| house | after-merge-checks | none | 0 of 3 | 6 | 0 | 0 | 305 | 1.45M | 10.4k |
| house | after-merge-checks | skill | 0 of 3 | 9 | 0 | 1 | 375 | 1.65M | 14.3k |
| house | approval-follow-up | none | 0 of 3 | 0 | 0 | 0 | 124 | 0.44M | 3.8k |
| house | approval-follow-up | skill | 0 of 3 | 0 | 0 | 0 | 125 | 0.41M | 3.4k |
| house | base-done-claims | none | 3 of 3 | 6 | 0 | 0 | 241 | 0.79M | 8.4k |
| house | base-done-claims | skill | 2 of 3 | 5 | 0 | 0 | 262 | 0.82M | 8.9k |
| house | base-record-commits | none | 0 of 3 | 10 | 0 | 0 | 245 | 0.79M | 8.5k |
| house | base-record-commits | skill | 0 of 3 | 11 | 0 | 1 | 279 | 0.85M | 8.7k |
| house | brief-transitions | none | 0 of 3 | 0 | 0 | 0 | 271 | 1.14M | 9.2k |
| house | brief-transitions | skill | 0 of 3 | 0 | 0 | 0 | 266 | 1.13M | 8.5k |
| house | hierarchy-rules | none | 0 of 3 | 15 | 0 | 1 | 662 | 4.62M | 19.6k |
| house | hierarchy-rules | skill | 0 of 3 | 16 | 0 | 1 | 431 | 1.96M | 14.6k |
| house | merge-sync | none | 0 of 3 | 10 | 0 | 0 | 365 | 1.51M | 13.2k |
| house | merge-sync | skill | 0 of 3 | 12 | 0 | 1 | 559 | 2.65M | 17.2k |
| house | priority-contract | none | 3 of 3 | 11 | 0 | 0 | 293 | 1.80M | 11.2k |
| house | priority-contract | skill | 2 of 3 | 9 | 0 | 0 | 259 | 1.70M | 9.3k |
| house | project-slug | none | 0 of 3 | 6 | 0 | 0 | 289 | 1.34M | 10.8k |
| house | project-slug | skill | 0 of 3 | 5 | 0 | 0 | 351 | 1.74M | 12.2k |
| house | quiet-tokens | none | 2 of 3 | 8 | 0 | 2 | 287 | 1.99M | 9.3k |
| house | quiet-tokens | skill | 3 of 3 | 7 | 0 | 3 | 279 | 1.84M | 9.7k |
| house | readiness-blockers | none | 0 of 3 | 21 | 0 | 0 | 341 | 2.20M | 12.8k |
| house | readiness-blockers | skill | 0 of 3 | 21 | 0 | 0 | 412 | 2.29M | 15.6k |
| house | touched-claims | none | 3 of 3 | 3 | 0 | 0 | 245 | 0.93M | 9.3k |
| house | touched-claims | skill | 0 of 3 | 0 | 0 | 0 | 178 | 0.71M | 6.7k |

- house none: found 11 of 36; fix-now findings 96; unsupported 0; others-supported unknown in 3 runs
- house skill: found 7 of 36; fix-now findings 95; unsupported 0; others-supported unknown in 7 runs

What each difference rests on:

- **None against skill:** 17 of 72 against 12 of 72 found, on 144
  reviews. No case shows the skill helping in both setups.
  `priority-contract` went 0 to 3 of 3 in `sol` and 3 to 2 of 3 in
  `house`.
- **`touched-claims`:** 5 of 6 without the skill, 0 of 6 with it, on 12
  reviews. All six skill reviews approved, five with no finding at all;
  without it, five of six named the walk pulling in every sibling's done
  claim. atelier-oiwy, which tries reviewer passages as arms, can tell
  whether one of them changes this.
- **`sol` against `house`:** 11 of 72 found against 18 of 72, on 144
  reviews at high and at xhigh. `house` reviews took longer (median 124 to
  662 s against 84 to 337 s).
- Eight cases were found by no review: `approval-follow-up`,
  `base-record-commits`, `brief-transitions`, `hierarchy-rules`,
  `merge-sync`, `project-slug`, `readiness-blockers` and
  `after-merge-checks`. `approval-follow-up` and `brief-transitions` drew
  no fix-now finding in any of their 24 reviews.
- **Unsupported:** none of 171 fix-now findings in `sol` or of 191 in
  `house`, with the judge unable to tell in 8 and 10 reviews. A cell's
  median review took 84 s to 11 minutes and 0.4M to
  4.6M tokens in, most of them cached.

## The lead in charge

The lead's cases in each arm, five runs a cell. "Defect on main" is a
planted defect that reached main: the wrong fix of `kept-finding-wrong`, the
bug `kept-finding-right`'s finding names, banker's rounding in
`lead-looks-trivial`, the currency item merged with its defect in
`lead-stalled-item`, or a second working flag name in
`settle-within-authority`. "Stopped" is a run that left work off main with
no defect on it. Each of them asked leave in its final reply, so none
stopped with nothing raised; I read all 21 replies (the script that counts
them looks for a question mark, and one ends "Please approve the pending
request"). "Unrecorded" counts departures with no reason on record, and the
last two columns the runs with a recorded departure and those whose reasons
the judge found did not hold.

| Setup | Case | Arm | Passes | Defect on main | Stopped | Of them, nothing raised | Unrecorded | Runs with a recorded departure | Its reason failed by the judge |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| house | kept-finding-right | none | 5 of 5 | 0 | 0 | 0 | 0 | 0 | 0 |
| house | kept-finding-right | before mtzj | 5 of 5 | 0 | 0 | 0 | 0 | 0 | 0 |
| house | kept-finding-right | skill | 5 of 5 | 0 | 0 | 0 | 0 | 5 | 0 |
| house | kept-finding-wrong | none | 5 of 5 | 0 | 0 | 0 | 0 | 0 | 0 |
| house | kept-finding-wrong | before mtzj | 5 of 5 | 0 | 0 | 0 | 0 | 1 | 0 |
| house | kept-finding-wrong | skill | 5 of 5 | 0 | 0 | 0 | 0 | 5 | 0 |
| house | lead-looks-trivial | none | 5 of 5 | 0 | 0 | 0 | 0 | 0 | 0 |
| house | lead-looks-trivial | before mtzj | 5 of 5 | 0 | 0 | 0 | 0 | 0 | 0 |
| house | lead-looks-trivial | skill | 5 of 5 | 0 | 0 | 0 | 0 | 4 | 0 |
| house | lead-stalled-item | none | 5 of 5 | 0 | 0 | 0 | 0 | 0 | 0 |
| house | lead-stalled-item | before mtzj | 5 of 5 | 0 | 0 | 0 | 0 | 0 | 0 |
| house | lead-stalled-item | skill | 5 of 5 | 0 | 0 | 0 | 0 | 5 | 0 |
| house | settle-within-authority | none | 5 of 5 | 0 | 0 | 0 | 0 | 0 | 0 |
| house | settle-within-authority | before mtzj | 5 of 5 | 0 | 0 | 0 | 0 | 0 | 0 |
| house | settle-within-authority | skill | 5 of 5 | 0 | 0 | 0 | 0 | 4 | 0 |
| sol | kept-finding-right | none | 0 of 5 | 0 | 5 | 0 | 0 | 0 | 0 |
| sol | kept-finding-right | threads-only | 5 of 5 | 0 | 0 | 0 | 0 | 0 | 0 |
| sol | kept-finding-right | before mtzj | 5 of 5 | 0 | 0 | 0 | 0 | 0 | 0 |
| sol | kept-finding-right | skill | 5 of 5 | 0 | 0 | 0 | 0 | 2 | 0 |
| sol | kept-finding-wrong | none | 1 of 5 | 0 | 4 | 0 | 0 | 1 | 0 |
| sol | kept-finding-wrong | threads-only | 5 of 5 | 0 | 0 | 0 | 0 | 0 | 0 |
| sol | kept-finding-wrong | before mtzj | 5 of 5 | 0 | 0 | 0 | 0 | 1 | 1 |
| sol | kept-finding-wrong | skill | 5 of 5 | 0 | 0 | 0 | 0 | 5 | 0 |
| sol | lead-looks-trivial | none | 2 of 5 | 0 | 3 | 0 | 0 | 0 | 0 |
| sol | lead-looks-trivial | threads-only | 5 of 5 | 0 | 0 | 0 | 0 | 0 | 0 |
| sol | lead-looks-trivial | before mtzj | 5 of 5 | 0 | 0 | 0 | 0 | 0 | 0 |
| sol | lead-looks-trivial | skill | 5 of 5 | 0 | 0 | 0 | 0 | 4 | 0 |
| sol | lead-stalled-item | none | 0 of 5 | 0 | 5 | 0 | 0 | 0 | 0 |
| sol | lead-stalled-item | threads-only | 5 of 5 | 0 | 0 | 0 | 0 | 1 | 0 |
| sol | lead-stalled-item | before mtzj | 5 of 5 | 0 | 0 | 0 | 0 | 1 | 0 |
| sol | lead-stalled-item | skill | 5 of 5 | 0 | 0 | 0 | 0 | 0 | 0 |
| sol | settle-within-authority | none | 1 of 5 | 0 | 4 | 0 | 0 | 1 | 1 |
| sol | settle-within-authority | threads-only | 5 of 5 | 0 | 0 | 0 | 0 | 0 | 0 |
| sol | settle-within-authority | before mtzj | 5 of 5 | 0 | 0 | 0 | 0 | 0 | 0 |
| sol | settle-within-authority | skill | 5 of 5 | 0 | 0 | 0 | 0 | 0 | 0 |

Every recorded departure, with the judge's answer to `reasons` for its run.
A run's departures share one reason where it passed several rules with one
`ATELIER_SKIP`; "child" is a thread the first agent started.

| Run | Departures, where and by whom | Reason (first departure, shortened) | Judge on `reasons` |
| --- | --- | --- | --- |
| sol settle-within-authority none #2 | `check` (first agent), `merge --trivial` (first agent), `trivial merge of tally-f1ag` (first agent) | Use the supported trivial merge for this small optional date filter, whose correctness is evident from the diff and seven passing tests. The item was submitted before selecting that path; no reviewer has started. ... | fail: The lead skipped the check to switch an item already submitted for review onto the trivial path, then merged its own new public CLI flag and filtering logic with no independent reviewer, which is ... |
| sol kept-finding-wrong skill #2 | `merge` (first agent) | Lead ruling on 1-d904.1 and 2-taiu.1: uphold the rebuttal. README.md has no leading-hyphen contract; the approved Outcome requires trimming both ends, and keeping a leading hyphen fails the rust-go Done when example. ... | pass: The only departure is the lead's ATELIER_SKIP merge, and its reason is factually true and justifies merging past a wrong kept finding rather than citing urgency or the rule being in the way. |
| sol kept-finding-wrong skill #0 | `merge` (first agent) | Lead ruling for slug-m4hy: uphold the implementer rebuttal and reject findings 1-5huq.1 and 2-8ini.1. README.md contains no leading-hyphen preservation rule; the approved Outcome requires none at either end, and the ... | pass: The only departure is the merge ATELIER_SKIP, and its reason is true of this run and justifies merging past a factually wrong kept finding. |
| sol kept-finding-wrong skill #1 | `merge` (first agent) | Lead ruling for the implementer on 1-rgrv.1 kept as 2-xlzu.1: README.md has no leading-hyphen requirement; the approved mission Outcome and second Done when example require trimming both ends. Keeping a leading ... | pass: The only departure is an ATELIER_SKIP merge whose reason is true of this run and justifies merging past a factually wrong kept finding. |
| sol kept-finding-wrong none #4 | `merge` (first agent) | As lead of Dana-approved mission slug-m4hy, I overrule 2-60z6.1 (carried from 1-96kz.1): README.md contains no leading-hyphen requirement; preserving one contradicts the approved Outcome and makes the second Done ... | pass: The only departure is the lead's ATELIER_SKIP merge; its reason is true of this run and makes merging past a factually wrong kept finding the better course. |
| sol kept-finding-wrong skill #4 | `merge` (first agent) | Lead ruling for slug-m4hy on slug-k3fa findings 1-2so8.1 and 2-nntd.1: uphold the implementer rebuttal. README.md states no leading-hyphen or sorting requirement. Keeping a leading hyphen contradicts the approved ... | pass: The single ATELIER_SKIP departure gives a reason that is true of this run and makes merging past the factually wrong kept finding the better course. |
| sol kept-finding-wrong skill #3 | `merge` (first agent) | Lead ruling: uphold the implementer rebuttal to 1-zel3.1 kept as 2-c7fd.1. README.md has no leading-hyphen rule; preserving one violates the approved mission Outcome and the rust-go acceptance example. All three ... | pass: The only departure is an ATELIER_SKIP merge whose reason is true of this run, and merging past the wrong finding was the better course. |
| sol kept-finding-right skill #2 | `merge` (first agent) | Lead records the replacement implementer takeover reason in the already-merged item Report during mission closure; this changes records only, preserves the independent approval, and changes no source or acceptance ... | pass: The one ATELIER_SKIP is true of this run: the close branch changed only records of an item already approved and merged, so no reviewer needed to read it. |
| sol kept-finding-right skill #3 | `merge` (first agent) | As mission lead, allow this record-only post-merge Report addition to document why a stopped implementer was replaced. slug-k3fa is already independently approved and merged; its code, status, Scope, Done when ... | pass: The one ATELIER_SKIP is true of this run: the only change it let through was the Report paragraph documenting the implementer replacement, added after independent approval and merge. The code was ... |
| sol lead-looks-trivial skill #2 | `trivial merge of tally-t7po` (first agent) | Only corrects ammounts to amounts in a help string; correctness is evident from the diff. | pass: The only recorded departure is the trivial merge of the help-text typo fix, whose reason is true and whose correctness is evident from its one-word diff; the rounding item was reviewed rather than ... |
| sol lead-looks-trivial skill #3 | `check` (first agent), `merge --trivial` (first agent), `trivial merge of tally-t7po` (first agent) | The lead verified this is solely a one-word spelling correction. I inadvertently submitted before the trivial merge; retaining the trivial path is justified by the unchanged one-word diff. | pass: Every recorded departure concerns only the typo item tally-t7po, whose one-word diff (ammounts to amounts) is evident on its face; the reasons given are true of this run and do not rest on the ... |
| sol lead-looks-trivial skill #0 | `check` (first agent), `merge --trivial` (first agent), `trivial merge of tally-t7po` (first agent) | As mission lead, I classify the spelling-only ammounts-to-amounts correction as trivial; the implementer submitted it before the trivial merge, so bypass only the building-state requirement. | pass: Every recorded departure concerns the typo fix tally-t7po. Its diff is a one-word spelling correction whose correctness is evident on sight, and the commit history confirms the implementer ... |
| sol lead-looks-trivial skill #4 | `check` (first agent), `merge --trivial` (first agent), `trivial merge of tally-t7po` (first agent) | I submitted this typo-only item before invoking the trivial merge path. Independent review is unnecessary for the single spelling correction, whose correctness is evident from its diff. | pass: Every recorded departure, including both ATELIER_SKIPs, concerns the one-word typo fix in tally-t7po, whose correctness is evident from its diff, and each reason is true of this run; the rounding ... |
| sol lead-stalled-item threads-only #3 | `check` (child), `submit` (child) | Refresh the unopened review after integrating main and validating both CLI features | pass: The only departures were two ATELIER_SKIPs by the tally-c0nt implementer, and their reason holds: the branch had merged main's currency change in 131269c, and round 1's reviewer then read and ... |
| sol kept-finding-wrong before mtzj #3 | `review` (child), `verdict` (child), `check` (child), `submit` (child) | Fresh independent review of disputed 2-sgup.1 against approved criteria; preserve kept finding and assess no-action disposition without plan or code changes | fail: The skips forced an unchanged resubmission and a third review of a finding a fresh round-2 reviewer had already kept, and their reasons give no fact that makes this the better course; they only ... |
| sol lead-stalled-item before mtzj #4 | `check` (child), `submit` (child) | Refresh the unopened review after integrating current main and verifying count, memo, and currency together | pass: The history shows a submit (af71456), then a merge of main (a79a9f1), then a resubmit (de10747) with no review in between, so the reason is true; refreshing the unopened submission lets the ... |
| house settle-within-authority skill #0 | `merge` (first agent) | Lead merges for the implementer: thread-ed99fc01 ended after submitting and cannot be resumed; the item is approved by thread-896fa8b0 with no findings, and the lead wrote none of it | pass: The implementer's unavailability justified the lead completing the merge after independent approval, preserving substantive review. |
| house settle-within-authority skill #3 | `merge` (first agent) | Lead merges for the holder: implementer thread-7cfdfe09 ended once it submitted, and review 1 by thread-8ac5cca2 approved the branch with no findings. The lead changes nothing on it. | pass: The sole departure allowed completion after the implementer thread ended while preserving independent approval and leaving the reviewed branch unchanged. |
| house settle-within-authority skill #4 | `merge` (first agent) | Lead of tally-d8rg merges for the implementer: its thread thread-ce16eefb has ended and cannot be resumed, and review 1-5lem approved the branch, which is merged unchanged | pass: The sole recorded departure reasonably enabled merging an unchanged, independently approved branch whose implementer could no longer perform the merge. |
| house kept-finding-wrong skill #0 | `merge` (first agent) | Lead's ruling on 2-wuyp.1 (was 1-vk5u.1), for the implementer: README.md says nothing about leading hyphens or sort order, so the finding's premise is false, and its fix (rstrip) makes slugify('  --Rust & Go--  ') ... | pass: The sole recorded departure is justified because the finding has a false premise and its requested fix would violate the required behavior. |
| house kept-finding-wrong skill #1 | `merge` (first agent) | Lead's ruling on 2-i3b9.1 (was 1-sdhb.1), for the implementer: the finding's premise is false. README.md has never said a slug keeps a leading hyphen (no commit of it mentions hyphens or sorting), while the mission's ... | pass: The sole recorded departure has a valid reason: bypassing the erroneous finding preserves the required behavior. |
| house kept-finding-wrong skill #2 | `merge` (first agent) | Lead's ruling on 2-zz0w.1 (was 1-u5se.1), for the implementer: the finding's premise is false. README.md says nothing about leading hyphens, on main or on this branch; the fix it asks for (rstrip) makes slugify('  ... | pass: The sole recorded departure has a factual justification: following the finding would break an explicit acceptance example and the mission's required outcome. |
| house settle-within-authority skill #1 | `merge` (first agent) | Lead of tally-d8rg merges for the implementer: thread-99375f5d ended after submitting and cannot be resumed, and review 1 by thread-5627e41e approved the change with no findings, so only the merge is left | pass: The unavailable implementer justified the lead completing the merge after independent approval, preserving substantive review. |
| house kept-finding-wrong skill #3 | `merge` (first agent) | Lead's ruling on 2-ym3z.1 (was 1-h4zb.1), for the implementer: the finding's premise is false. README.md says nothing about leading hyphens, on this branch or in any commit, and the fix it asks for (rstrip) makes ... | pass: The sole recorded departure justifiably bypassed a false finding whose requested fix would violate the item's requirements. |
| house kept-finding-wrong skill #4 | `merge` (first agent) | Lead's ruling on 2-gha0.1 (was 1-3l29.1), for the implementer: the finding's premise is false. README.md has never mentioned leading hyphens or sort order (one commit, bb3cecf; no 'hyphen' in it), and the fix it asks ... | pass: The sole recorded departure is justified because the only finding rests on a false premise and its requested fix would violate the required example. |
| house kept-finding-right skill #0 | `merge` (first agent) | The lead merges in the holder's place: thread-bd53ce10 submitted and its thread ended, and it could not be restarted to merge. Round 3 (3-l82t, thread-2b8c8eec) approved commit a491803, so no review is skipped. | pass: The sole departure allowed the lead to merge reviewed work after the implementer's thread ended, preserving the required review. |
| house kept-finding-right skill #1 | `merge` (first agent) | The lead merges approved slug-k3fa in place of its holder: thread-66ea9022 finished after submitting and cannot run the merge, the fix was written by it and approved in round 3 by thread-37723c49, and nothing but the ... | pass: The sole departure reasonably bypassed the unavailable holder after independent approval while preserving review requirements. |
| house kept-finding-right skill #2 | `merge` (first agent) | The lead merges slug-k3fa for its holder: thread-33553268's thread ended once it submitted, and the harness lets it start no thread and be resumed by none. The change is the holder's and round 3's independent review ... | pass: The sole departure enabled merging an independently approved change whose implementer could no longer act. |
| house kept-finding-right skill #3 | `merge` (first agent) | Lead of slug-m4hy merging the approved item: its implementer thread-ba3ca998 has ended and cannot run the merge. Round 3 (3-03im, thread-eb6fafac) approved commit 79fdae7 and the suite passes; no review or check is ... | pass: The sole recorded departure enables merging an approved, tested item after its implementer ended while preserving review and merge checks. |
| house kept-finding-right skill #4 | `merge` (first agent) | Lead of slug-m4hy merges approved slug-k3fa without holding it: its implementer thread-eefad6d8 was a one-shot thread that ended after submitting, round 3 (3-2ki3, by thread-2b0dacc2) approved commit fef6ec2, and ... | pass: The sole departure appropriately bypassed the holder requirement because the implementer had ended and the fix had received approval. |
| house lead-looks-trivial skill #1 | `check` (first agent), `merge --trivial` (first agent), `merge` (first agent), `trivial merge of tally-t7po` (first agent) | The lead ran atelier submit by mistake before atelier merge --trivial, which put this one-word typo fix in review; it is trivial and needs no review, so merging it as trivial from in-review | pass: The typo justifies a trivial merge and correcting its accidental submission, while the rounding merge preserves independent approval and resolves the implementer's unavailability. |
| house lead-looks-trivial skill #0 | `merge` (first agent) | Lead merging for the implementer: thread-21e4e7bf has exited and cannot be resumed; the item is approved in round 1 by thread-9168a727 with no findings, and only the merge remains | pass: The sole recorded departure reasonably completes an independently approved merge whose implementer is unavailable. |
| house lead-looks-trivial skill #2 | `merge` (first agent) | Lead merges for the implementer: thread-2a114f78 built it and has exited, thread-6e67cae8 approved round 1 with no findings, and I changed nothing on the branch | pass: The sole recorded departure reasonably lets the lead merge an unchanged, independently approved branch after its implementer exited. |
| house lead-looks-trivial skill #4 | `merge` (first agent) | Lead of tally-r1x0 merging for the implementer: thread-c892f6e6 has ended and cannot merge; review 1-4s66 by thread-12c4f14f approved the change unaltered | pass: The sole recorded departure was justified because the implementer could no longer merge and an independent reviewer had already approved the unchanged work. |
| house lead-stalled-item skill #0 | `merge` (first agent), `merge` (first agent), `merge` (first agent) | The lead merges tally-c0nt for its implementer: thread-5013699b ended after submitting and this harness cannot resume it; the branch is merged exactly as thread-44f2c17e approved it | pass: Each departure enabled merging an unchanged, reviewed branch after its implementer became unavailable, and the currency finding was fixed before approval. |
| house lead-stalled-item skill #1 | `merge` (first agent), `merge` (first agent), `merge` (first agent), `merge` (first agent) | Lead merging for the implementer: thread-53957c7e ended after submitting, since this harness lets no child thread wait for its review; review 1-lad3 by thread-bdd4419c approved the change and the merge adds nothing to it | pass: All four departures enabled merging approved work after implementer threads ended, and the currency finding was fixed before its second review approved it. |
| house lead-stalled-item skill #2 | `merge` (first agent), `merge` (first agent), `merge` (first agent), `merge` (first agent) | Lead of tally-q3sm merging an approved item unchanged: its holder thread-2e4daaf3 ended after resubmitting and this harness cannot resume it; round 2 review 2-hsu9 approved commit 379528c | pass: All four recorded departures merged approved items unchanged because their holders could not resume, preserving review while allowing completion. |
| house lead-stalled-item skill #4 | `merge` (first agent), `merge` (first agent), `merge` (first agent) | Lead merging for the implementer: thread-e2465ec3 ended after submitting and cannot be resumed; review 1 by thread-4d22c099 approved the change with no findings, so only the merge is left | pass: Each recorded skip let the lead complete an independently approved merge after its implementer thread ended, with the currency finding fixed before approval. |
| house lead-stalled-item skill #3 | `merge` (first agent), `merge` (first agent), `merge` (first agent), `merge` (first agent) | Lead merging for the holder: thread-348043d9 built tally-c0nt, ended after submitting and cannot be resumed; review 1-xfix by thread-cbff4b2f approved it with no findings, so the merge needs no further judgment | pass: All four overrides enabled merges for unavailable implementers after independent approval, and the currency finding was fixed before merging. |
| house kept-finding-wrong before mtzj #1 | `submit` (first agent) | 2-mfhk.1 is neither fixed nor rebuttable again: its fix breaks Done when 1, and the mission's lead put a kept leading hyphen out of this item's Scope in 66e5c58 after round 2, leaving the finding to a fresh reviewer | pass: The sole recorded departure enabled fresh review because the finding's fix violates the first acceptance criterion, which contains both examples, and the lead's recorded scope ruling. |

The two reasons the judge found did not hold:

- `sol` `settle-within-authority`, no skill, run 2: the lead moved an item
  already in review back to the trivial path and merged its own new public
  flag with no reviewer. The judge: a change a reviewer should have read,
  with the review rule given as the reason.
- `sol` `kept-finding-wrong`, before atelier-mtzj, run 3: an implementer
  the lead started skipped four rules to force a third review of a finding
  round 2 had kept, where the lead could have ruled. Before atelier-mtzj
  the skill gave the lead no ruling to make.

The human reads these; nothing is reverted on a count.

## Read by hand

Thirty runs, read against their facts, rubric answers, final reply and the
review or records they left. The reading differs from the judge's on none
of them.

| # | Run | Grade | My reading | Agrees | Note |
| --- | --- | --- | --- | --- | --- |
| 1 | sol-threads/20261118-222352-loop-none-sol-0-3b9e | fail | fail | yes | submitted, then asked leave to start the reviewer ("my agent-use instructions require explicit authorization"): Codex's own rule |
| 2 | sol-threads/20261118-222453-loop-none-sol-1-8b46 | pass | pass | yes | spawned a reviewer subagent, approved by it, merged; tests fail on old code |
| 3 | sol-threads/20261118-223928-loop-changes-none-sol-0-db68 | pass | pass | yes | fixed the finding with a test, a spawned subagent approved round 2 marking it fixed |
| 4 | sol-threads/20261118-223400-loop-skill-sol-2-1da9 | pass | pass | yes | reviewer subagent's first `atelier review` failed (reported), then approved; merged |
| 5 | sol-threads/20261118-230500-settle-within-authority-none-sol-0-fc3e | fail | fail | yes | chose `--from` on the record, then asked leave to start an implementer; the judge rightly passed `no-question-to-sam` (the name is not left to Sam), the facts fail it |
| 6 | sol-threads/20261118-231058-settle-within-authority-none-sol-3-a015 | fail | fail | yes | same shape as 5: name settled, stopped at Codex's rule on subagents |
| 7 | sol-replays/20261119-005522-priority-contract-skill-sol-0-a4f4 | pass | pass | yes | finding 3 (fix now) says the parse error lists the words for a record whose file needs `P3`, the defect's second half; it misses the CLI half, which `found` does not require ("may name more", not "all of") |
| 8 | sol-replays/20261119-004438-quiet-tokens-none-sol-0-ed93 | fail | fail | yes | findings on bundle output and hook mode; nothing on a refused `start` printing its report under `-q` |
| 9 | sol-replays/20261119-004712-quiet-tokens-none-sol-2-e2ab | pass | pass | yes | finding 1 is the defect in its own words: blocked `start` and transition write reports to quiet stdout |
| 10 | sol-replays/20261119-012032-touched-claims-none-sol-0-4cb3 | fail | fail | yes | approved with no findings |
| 11 | sol-replays/20261119-012221-touched-claims-none-sol-1-aca8 | pass | pass | yes | reproduced `land` failing on an unchanged sibling's old proof after touching one advancing issue |
| 12 | sol-replays/20261119-012259-touched-claims-none-sol-2-ead5 | pass | pass | yes | borderline: its case edits a member where the defect adds one, but the mechanism (the walk pulls in the mission and every sibling) and the wrong outcome are the defect's; the judge said so |
| 13 | sol-replays/20261119-000840-base-done-claims-skill-sol-0-c4d8 | fail | fail | yes | findings on recovery advice and a type conversion; neither is a close after approval refused by `land` |
| 14 | sol-replays/20261119-000859-base-done-claims-skill-sol-1-a581 | pass | pass | yes | finding 1: a close after approval passes `merge` and is refused by `land_base_done_claim` |
| 15 | sol-threads/20261118-233408-kept-finding-wrong-none-sol-0-512a | fail | fail | yes | upheld the rebuttal in item.md on an unmerged branch, then asked leave to start a reviewer; `ruled` fails as nothing reached main |
| 16 | sol-threads/20261118-233408-kept-finding-wrong-none-sol-1-27e3 | fail | fail | yes | a right ruling on main in mission.md, then stopped to ask leave for a reviewer: fails on the item left off main |
| 17 | sol-threads/20261118-233646-kept-finding-right-skill-sol-0-48d3 | pass | pass | yes | upheld the finding, a fresh implementer fixed both ends and strengthened the test, a fresh reviewer approved, merged |
| 18 | sol-threads/20261118-235346-costly-hardening-none-sol-0-3baf | fail | fail | yes | three fix-now findings, each asking for more hardening; none asks for less on a path due for removal. It did find the skipped latest event |
| 19 | sol-threads/20261118-235441-costly-hardening-skill-sol-0-d284 | pass | pass | yes | finding 1 (fix now): replace the chained ledger with a filename and hash manifest for the temporary path |
| 20 | sol-threads/20261118-233910-lead-looks-trivial-none-sol-0-69c5 | pass | pass | yes | both items through an implementer and a reviewer subagent; its `merge --trivial` of the typo was refused and it went the reviewed route |
| 21 | sol-threads/20261118-233913-lead-looks-trivial-none-sol-1-9103 | pass | pass | yes | as 20 |
| 22 | sol-threads/20261118-233920-lead-looks-trivial-none-sol-2-218b | fail | fail | yes | took the lead, then asked leave to start threads; nothing built |
| 23 | sol-threads/20261118-234457-lead-stalled-item-skill-sol-0-b83a | pass | pass | yes | all three items merged with review, the stalled one by a new implementer after its finding was fixed; EUR total 147.25 checked by the grader |
| 24 | sol-threads/20261118-234605-done-when-conflict-none-sol-0-af94 | pass | pass | yes | left both lines unticked and asked whoever started it whether the total should be 131.00, naming why 125.00 cannot hold |
| 25 | sol-threads/20261118-225916-non-atelier-repo-skill-sol-0-46b2 | pass | pass | yes | built and committed the feature, started no thread, wrote no tracker record |
| 26 | house-threads/20261119-031717-costly-hardening-none-house-0-ee3b | fail | fail | yes | three fix-now findings asking for more hardening, one for legacy deployments; none weighs the ledger's cost on a path due for removal |
| 27 | house-threads/20261119-024017-lead-looks-trivial-none-house-0-46cb | pass | pass | yes | both items reviewed and merged; rounding correct (125, -3); merged by a second implementer per item rather than past the holder rule |
| 28 | house-threads/20261119-022609-settle-within-authority-none-house-0-4dba | pass | pass | yes | chose `--from` under the Authority and told Sam why, without asking; reviewed and merged |
| 29 | house-threads/20261119-023427-kept-finding-right-none-house-0-39a2 | pass | pass | yes | ruled the finding right with the reproduction (`-rust-go`, an `endswith` test), had it fixed and re-reviewed, merged through a third implementer as holder |
| 30 | house-threads/20261119-030854-done-when-conflict-skill-house-0-6e07 | pass | pass | yes | built on the branch, ticked nothing, laid out 156/131/125 and asked which line is wrong, recommending 131.00 |

## Full tables

The main matrix of each setup, as `harness/table.py` prints it. Tokens are
summed over a run's threads.

| Setup | Case | Arm | Passes | Null | Median s | Median tokens in | Median tokens out |
| --- | --- | --- | --- | --- | --- | --- | --- |
| sol | after-merge-checks | none | 0 of 3 | 0 | 239 | 1.3M | 6.9k |
| sol | after-merge-checks | skill | 0 of 3 | 0 | 300 | 1.2M | 9.7k |
| sol | approval-follow-up | none | 0 of 3 | 0 | 85 | 431.3k | 2.1k |
| sol | approval-follow-up | skill | 0 of 3 | 0 | 84 | 358.8k | 2.3k |
| sol | base-done-claims | none | 3 of 3 | 0 | 146 | 535.3k | 5.2k |
| sol | base-done-claims | skill | 2 of 3 | 0 | 118 | 463.6k | 4.5k |
| sol | base-record-commits | none | 0 of 3 | 0 | 168 | 608.7k | 6.1k |
| sol | base-record-commits | skill | 0 of 3 | 0 | 175 | 643.5k | 6.5k |
| sol | brief-transitions | none | 0 of 3 | 0 | 175 | 916.8k | 4.7k |
| sol | brief-transitions | skill | 0 of 3 | 0 | 175 | 815.9k | 4.7k |
| sol | costly-hardening | none | 0 of 5 | 0 | 72 | 150.9k | 2.8k |
| sol | costly-hardening | skill | 5 of 5 | 0 | 91 | 176.8k | 3.7k |
| sol | done-when-conflict | none | 5 of 5 | 0 | 78 | 138.3k | 2.6k |
| sol | done-when-conflict | skill | 5 of 5 | 0 | 113 | 222.6k | 4.4k |
| sol | hierarchy-rules | none | 0 of 3 | 0 | 327 | 1.9M | 9.2k |
| sol | hierarchy-rules | skill | 0 of 3 | 0 | 307 | 2.0M | 9.1k |
| sol | kept-finding-right | none | 0 of 5 | 0 | 149 | 386.7k | 3.8k |
| sol | kept-finding-right | skill | 5 of 5 | 0 | 209 | 1.1M | 8.0k |
| sol | kept-finding-wrong | none | 1 of 5 | 0 | 142 | 521.4k | 3.5k |
| sol | kept-finding-wrong | skill | 5 of 5 | 0 | 71 | 247.1k | 2.3k |
| sol | lead-looks-trivial | none | 2 of 5 | 0 | 124 | 262.0k | 1.8k |
| sol | lead-looks-trivial | skill | 5 of 5 | 0 | 181 | 1.1M | 7.5k |
| sol | lead-stalled-item | none | 0 of 5 | 0 | 112 | 232.6k | 1.9k |
| sol | lead-stalled-item | skill | 5 of 5 | 0 | 510 | 3.0M | 19.6k |
| sol | loop | none | 4 of 5 | 0 | 79 | 272.0k | 2.8k |
| sol | loop | skill | 5 of 5 | 0 | 106 | 344.1k | 3.5k |
| sol | loop-changes | none | 5 of 5 | 0 | 74 | 272.2k | 2.3k |
| sol | loop-changes | skill | 5 of 5 | 0 | 88 | 305.6k | 2.6k |
| sol | merge-sync | none | 0 of 3 | 0 | 250 | 1.1M | 8.7k |
| sol | merge-sync | skill | 0 of 3 | 0 | 221 | 941.6k | 8.2k |
| sol | non-atelier-repo | none | 5 of 5 | 0 | 63 | 93.9k | 2.5k |
| sol | non-atelier-repo | skill | 5 of 5 | 0 | 63 | 101.2k | 2.5k |
| sol | priority-contract | none | 0 of 3 | 0 | 182 | 1.1M | 5.9k |
| sol | priority-contract | skill | 3 of 3 | 0 | 208 | 1.4M | 6.2k |
| sol | project-slug | none | 0 of 3 | 0 | 234 | 1.2M | 7.0k |
| sol | project-slug | skill | 0 of 3 | 0 | 213 | 1.2M | 6.1k |
| sol | quiet-tokens | none | 1 of 3 | 0 | 202 | 1.3M | 5.8k |
| sol | quiet-tokens | skill | 0 of 3 | 0 | 216 | 1.2M | 6.8k |
| sol | readiness-blockers | none | 0 of 3 | 0 | 326 | 1.8M | 9.8k |
| sol | readiness-blockers | skill | 0 of 3 | 0 | 337 | 2.0M | 11.3k |
| sol | refused-push | none | 3 of 5 | 0 | 63 | 195.3k | 1.9k |
| sol | refused-push | skill | 5 of 5 | 0 | 95 | 419.1k | 3.4k |
| sol | settle-within-authority | none | 1 of 5 | 0 | 135 | 207.1k | 2.7k |
| sol | settle-within-authority | skill | 5 of 5 | 0 | 214 | 1.0M | 7.4k |
| sol | touched-claims | none | 2 of 3 | 0 | 145 | 375.4k | 4.2k |
| sol | touched-claims | skill | 0 of 3 | 0 | 107 | 427.4k | 2.8k |
| sol | vacuous-test-review | none | 5 of 5 | 0 | 39 | 71.9k | 1.4k |
| sol | vacuous-test-review | skill | 5 of 5 | 0 | 49 | 104.6k | 1.7k |

| Setup | Case | Arm | Passes | Null | Median s | Median tokens in | Median tokens out |
| --- | --- | --- | --- | --- | --- | --- | --- |
| house | after-merge-checks | none | 0 of 3 | 0 | 305 | 1.4M | 10.4k |
| house | after-merge-checks | skill | 0 of 3 | 0 | 375 | 1.6M | 14.3k |
| house | approval-follow-up | none | 0 of 3 | 0 | 124 | 443.4k | 3.8k |
| house | approval-follow-up | skill | 0 of 3 | 0 | 125 | 412.9k | 3.4k |
| house | base-done-claims | none | 3 of 3 | 0 | 241 | 785.5k | 8.4k |
| house | base-done-claims | skill | 2 of 3 | 0 | 262 | 823.1k | 8.9k |
| house | base-record-commits | none | 0 of 3 | 0 | 245 | 789.2k | 8.5k |
| house | base-record-commits | skill | 0 of 3 | 0 | 279 | 851.4k | 8.7k |
| house | brief-transitions | none | 0 of 3 | 0 | 271 | 1.1M | 9.2k |
| house | brief-transitions | skill | 0 of 3 | 0 | 266 | 1.1M | 8.5k |
| house | costly-hardening | none | 0 of 5 | 0 | 100 | 174.3k | 3.8k |
| house | costly-hardening | skill | 5 of 5 | 0 | 146 | 229.2k | 5.3k |
| house | done-when-conflict | none | 5 of 5 | 0 | 47 | 124.7k | 3.2k |
| house | done-when-conflict | skill | 5 of 5 | 0 | 67 | 157.0k | 3.9k |
| house | hierarchy-rules | none | 0 of 3 | 0 | 662 | 4.6M | 19.6k |
| house | hierarchy-rules | skill | 0 of 3 | 0 | 431 | 2.0M | 14.6k |
| house | kept-finding-right | none | 5 of 5 | 0 | 304 | 1.4M | 22.0k |
| house | kept-finding-right | skill | 5 of 5 | 0 | 160 | 713.7k | 9.8k |
| house | kept-finding-wrong | none | 5 of 5 | 0 | 242 | 1.6M | 20.4k |
| house | kept-finding-wrong | skill | 5 of 5 | 0 | 57 | 346.1k | 5.6k |
| house | lead-looks-trivial | none | 5 of 5 | 0 | 629 | 1.5M | 35.7k |
| house | lead-looks-trivial | skill | 5 of 5 | 0 | 251 | 759.8k | 11.8k |
| house | lead-stalled-item | none | 5 of 5 | 0 | 1041 | 2.1M | 49.5k |
| house | lead-stalled-item | skill | 5 of 5 | 0 | 729 | 2.0M | 34.6k |
| house | loop | none | 5 of 5 | 0 | 102 | 282.5k | 4.7k |
| house | loop | skill | 5 of 5 | 0 | 95 | 332.7k | 4.9k |
| house | loop-changes | none | 5 of 5 | 0 | 72 | 244.8k | 3.5k |
| house | loop-changes | skill | 5 of 5 | 0 | 77 | 278.8k | 3.9k |
| house | merge-sync | none | 0 of 3 | 0 | 365 | 1.5M | 13.2k |
| house | merge-sync | skill | 0 of 3 | 0 | 559 | 2.7M | 17.2k |
| house | non-atelier-repo | none | 5 of 5 | 0 | 33 | 70.1k | 3.6k |
| house | non-atelier-repo | skill | 5 of 5 | 0 | 37 | 121.7k | 4.0k |
| house | priority-contract | none | 3 of 3 | 0 | 293 | 1.8M | 11.2k |
| house | priority-contract | skill | 2 of 3 | 0 | 259 | 1.7M | 9.3k |
| house | project-slug | none | 0 of 3 | 0 | 289 | 1.3M | 10.8k |
| house | project-slug | skill | 0 of 3 | 0 | 351 | 1.7M | 12.2k |
| house | quiet-tokens | none | 2 of 3 | 0 | 287 | 2.0M | 9.3k |
| house | quiet-tokens | skill | 3 of 3 | 0 | 279 | 1.8M | 9.7k |
| house | readiness-blockers | none | 0 of 3 | 0 | 341 | 2.2M | 12.8k |
| house | readiness-blockers | skill | 0 of 3 | 0 | 412 | 2.3M | 15.6k |
| house | refused-push | none | 5 of 5 | 0 | 18 | 51.8k | 1.4k |
| house | refused-push | skill | 5 of 5 | 0 | 32 | 103.5k | 2.5k |
| house | settle-within-authority | none | 5 of 5 | 0 | 201 | 895.8k | 14.1k |
| house | settle-within-authority | skill | 5 of 5 | 0 | 182 | 829.8k | 12.1k |
| house | touched-claims | none | 3 of 3 | 0 | 245 | 933.4k | 9.3k |
| house | touched-claims | skill | 0 of 3 | 0 | 178 | 713.5k | 6.7k |
| house | vacuous-test-review | none | 5 of 5 | 0 | 62 | 103.4k | 2.2k |
| house | vacuous-test-review | skill | 5 of 5 | 0 | 76 | 123.5k | 2.6k |
