# Nine more reviews from atelier's history, replayed (2026-11-18)

Item atelier-z8rg. One review of each of the nine replay cases built here,
by the reviewer of `sol`, skill arm, to show that each case runs and what
it costs. Like the first three ([2026-11-18-replayed-reviews.md](2026-11-18-replayed-reviews.md)),
it measures nothing about the skill: one run per case, and no none arm.

**Result.** The reviewer found the known defect in one case of nine,
`priority-contract`, with a reproduction. It approved two changes with no
findings and asked for other fixes in six. With the first three cases, a
`sol` review has found 2 of 12 known defects. The judge found 1 of 20 other
fix-now findings unsupported, and I read that one as supported. A review
took 1.5 to 5.2 minutes and 0.41M to 1.9M tokens in, 92% of them cached.
No run was invalid. No transcript showed the fix, and each proxy refused
only Codex's own telemetry host.

## Setup

- **Binary.** Built from atelier-z8rg at 12671f230 (master's crates at
  4d5ebd22c), copied to `/var/tmp/af-evals/bin/atelier-12671f230`.
- **Skill.** `--ref 12671f230`, and `b480f58d3` for the second
  `after-merge-checks` run. Both are `skills/agent-factory` as on master.
- **Harness.** Unchanged from master. Codex CLI 0.169; the judge Claude
  Code on `claude-opus-5-5` at high.
- **Reviewer.** `sol`: Codex `gpt-6.1-sol` at high, in every run.
- **Runs.** `~/.cache/af-evals/runs/z8rg-replay/` and, for the second
  `after-merge-checks` run, `z8rg-replay-2/`, `--jobs 1 --keep`. Every
  reviewer built the old workspace with cargo inside its sandbox (3 to 9
  cargo commands each), offline, in its own `target/`.

## Runs

"Found" is the judge's answer to `found`, which I confirmed by reading each
review. "Unsupported" is the judge's count for `others-supported`. Lines are
the change's diff outside the old tracker's files. Minutes and tokens are
the reviewer's; tokens are in (cached part) and out. Each judge took 4 to 16
seconds and 13k to 42k tokens in.

| Case (row) | Lines | Verdict | Fix-now findings | Found | Unsupported | Minutes | Tokens in (cached) | Tokens out |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `merge-sync` (7) | 834 | changes | 3 | no | 1 of 3 | 3.9 | 950k (883k) | 8.5k |
| `brief-transitions` (10) | 92 | approve | 0 | no | 0 of 0 | 1.5 | 421k (383k) | 2.7k |
| `hierarchy-rules` (11) | 616 | changes | 4 | no | 0 of 4 | 5.2 | 1.91M (1.82M) | 8.9k |
| `project-slug` (12) | 382 | changes | 1 | no | 0 of 1 | 2.8 | 856k (791k) | 5.6k |
| `quiet-tokens` (33) | 766 | changes | 1 | no | 0 of 1 | 2.6 | 993k (921k) | 5.2k |
| `priority-contract` (45) | 783 | changes | 3 | yes | 0 of 2 | 3.0 | 1.25M (1.16M) | 5.8k |
| `readiness-blockers` (49) | 673 | changes | 7 | no | 0 of 7 | 4.0 | 1.51M (1.42M) | 8.4k |
| `after-merge-checks` (63) | 937 | changes | 2 | no | 0 of 2 | 2.9 | 593k (516k) | 6.6k |
| `touched-claims` (64) | 109 | approve | 0 | no | 0 of 0 | 2.4 | 415k (337k) | 4.8k |

- `priority-contract`: Finding 3 reproduces the defect with the built
  binary. It edits a record's priority, and `lint` recommends the words. Then
  applying the recommended `high` still fails, since records take only
  `P0`-`P3`. It does not say that `-p P1` is refused, which is the other
  half of the same split. Its one failed `atelier` call was `issue show`,
  which the June tree's AGENTS.md names and today's binary lacks.
- `merge-sync`: the judge called Finding 3 unsupported. That finding says
  the new automatic receipt commit breaks the governing rule that Atelier
  never creates commits. The rule is in the fixture's ADR 0019 (decision 3),
  PRODUCT_INTENT.md and `docs/architecture/quality/standards.md`. The judge is given
  the two diffs, not the tree, so it could not see the rule. I count 0 of 3
  unsupported.
- `after-merge-checks` ran twice. In the first run (12671f230: changes, 3
  fix-now findings, not found, 3.5 min, 799k in), Finding 1 said that the
  Agent Factory lines of the Done when were unmet, and checked them against
  the run's staged skill. The case's fault: its Report had left out the
  line saying that text was committed in the skill's own repository.
  b480f58d3 puts it back, and the second run, in the table, made no such
  finding.
- `brief-transitions` and `touched-claims` are the two smallest diffs (92
  and 109 lines). Each is a later commit on an item's branch, and each was
  approved with no findings. Both defects lie in code the change does not
  touch: the full view the change left as it was, and a mission's siblings
  the new walk reaches.
- `project-slug`'s Done when asks for the slug the defect comes from. Its
  one finding was that the change fails `cargo fmt -- --check`.

## Cost

Nine reviews took 28 minutes of reviewer time, run one at a time, and
8.9M tokens in (0.68M uncached) and 57k out; the judges took 0.25M in.
These diffs run to 937 lines, against 68 to 218 in the first three, and the
mean review took 3.1 minutes against 1.7 there. Five runs of all twelve cases
in one arm of `sol` would take about 3 hours of reviewer time and about
50M tokens in, most of them cached.
