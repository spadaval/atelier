# Does Sol question work this repository later removed as unneeded? (2026-11-19)

Item atelier-ad0z, for mission atelier-8az0. The five removal replays
([README](../README.md#removal-replays)) on `sol` (`gpt-6.1-sol` at
`high`), with no skill and with the skill, five runs a cell.

**Result.** No. In none of 50 reviews did a finding question the need for
the part later removed, or ask for less of it. The same reviewer named the
unneeded part of atelier-s6xk's planted cases in 29 of 30 runs with no
skill and 30 of 30 with it. The difference is where the excess sits. In
atelier-s6xk's cases, it is code beyond what the item asks for. In each
replayed change, the part later removed *is* what the item asks for: every
Done when line names it. A reviewer who checks a change against its item
finds nothing to cut, and these reviewers did that well. They asked for 119
fix-now fixes, and the judge found none unsupported. With the skill,
reviewers did weigh value against size, and each answer restated the
item's Goal. Every skill-arm review that can still be read, 20 of 25,
rates the value high, or medium for the base exemption. One adds: "The
value warrants this implementation's size; no substantially smaller
change provides the requested behavior."

| Case | Arm | Passes | Blocking | Verdicts | Fix-now findings | Median s | Median tokens in | Median tokens out |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `removed-merge-sync` | none | 0 of 5 | none questioned | changes 5 | 14 | 306 | 1.4M | 8.8k |
| `removed-merge-sync` | skill | 0 of 5 | none questioned | changes 5 | 17 | 281 | 1.0M | 8.9k |
| `removed-merge-wait` | none | 0 of 5 | none questioned | approve 3, changes 2 | 3 | 226 | 1.2M | 5.9k |
| `removed-merge-wait` | skill | 0 of 5 | none questioned | approve 2, changes 3 | 5 | 262 | 1.1M | 7.8k |
| `removed-base-exemption` | none | 0 of 5 | none questioned | changes 5 | 8 | 439 | 2.6M | 11.1k |
| `removed-base-exemption` | skill | 0 of 5 | none questioned | changes 5 | 6 | 322 | 1.8M | 9.3k |
| `removed-debrief` | none | 0 of 5 | none questioned | changes 5 | 19 | 227 | 1.2M | 7.3k |
| `removed-debrief` | skill | 0 of 5 | none questioned | changes 5 | 18 | 226 | 1.0M | 7.9k |
| `removed-decision-notes` | none | 0 of 5 | none questioned | changes 5 | 16 | 250 | 1.3M | 8.1k |
| `removed-decision-notes` | skill | 0 of 5 | none questioned | changes 5 | 13 | 270 | 1.4M | 9.1k |

Beside atelier-s6xk's numbers
([2026-11-19-overbuilt-told-apart.md](2026-11-19-overbuilt-told-apart.md)),
same reviewer and effort:

| Cases | Arm | Named the unneeded part | As fix now |
| --- | --- | --- | --- |
| atelier-s6xk's three overbuilt cases, 10 runs each | none | 29 of 30 | 20 |
| atelier-s6xk's three overbuilt cases, 10 runs each | skill | 30 of 30 | 30 |
| These five removal replays, 5 runs each | none | 0 of 25 | 0 |
| These five removal replays, 5 runs each | skill | 0 of 25 | 0 |

"Blocking" is the reported `blocks` answer, which counts only findings
that question the need; there were none. "Fix-now findings" counts every
fix-now finding of round 1 across the cell's five reviews. No run was
invalid, none hit a provider limit, no transcript showed the removal, and
every proxy refused only `ab.chatgpt.com`, Codex's telemetry host.

## Setup

- **Binary.** Built from this branch at e5b68aa0d, whose crates are
  master's (6850e181e), copied to `/var/tmp/af-evals/bin/atelier-e5b68aa0d`.
- **Skill.** `skills/agent-factory` as on master, staged from `--ref
  e5b68aa0d`.
- **Harness.** As on master, with the cases and `harness/removed_tests.py`
  of e5b68aa0d. Codex CLI 0.169; the judge is Claude Code on
  `claude-opus-5-5` at high, `sol`'s judge.
- **Runs.** `~/.cache/af-evals/runs/ad0z-removed/`, made in three passes
  into the same `--out`. The first, with `--keep` and `--cargo-jobs 3`,
  filled the host's disk after 13 graded runs: each kept work directory
  held a cargo `target/` of about 2.7 GB, on a disk with about 30 GB free.
  Its runs that hit `No space left on device` were graded `null`. The
  second and third passes made only the missing and `null` runs, without
  `--keep`. A loop deleted `target/` from each graded run's work directory,
  and the third pass used `--cargo-jobs 2`, since atelier-id3v's replay
  matrix was building on the host at the same time. Every cell's runs
  come from the same binary, skill commit and harness.

## What the reviews said

The judge's answer to `questions-need` matches every review. I read each
round-1 finding's title, and searched each kept review's text for any
point about size. The work directories of 13 runs from the first pass,
`merge --sync`'s ten and three no-skill `merge --wait` runs, were deleted
to free the disk, so their review text is read only from the judge's
answers and the facts. The only points about size are in skill-arm
summaries: a `Value:` line in each, high in 15 and medium in the five of
the base exemption, each giving the item's Goal as its reason. No-skill
reviews have no value line.

- **`merge --sync`.** All ten asked for changes, each with two to four
  fix-now defects in the sync. Every run found that it merges the base by
  its short name, which a same-named tag can shadow. Others found
  a child's approval reported as covering the owner, and reruns skipped
  after a record commit. None asked whether `git merge` and a rerun by
  hand would do, which is design.md's reason for the removal: "catching up
  is `git merge`".
- **`merge --push --wait`.** Five approved and five asked for changes, on
  `--wait` with no forge configured, a timeout inside the grace period, or
  a timeout lost across retries. Four deferred an overflow of very large
  `--timeout` values. The item's own Unknowns names Claude Code's
  10-minute foreground limit against the 30-minute default. No review took
  that up, though it is the reason `--wait` was later removed.
- **Base exemption.** All ten asked for changes. Most found that a
  discovery link to a base exemption leaves the tracker unloadable, or
  that an exemption recorded in a mission's closing second drops out of
  its debrief. Every skill-arm summary rates the value medium; one says
  there is "no substantially smaller change that preserves the owner-free
  record".
- **`debrief`.** All ten asked for changes, with three to six fix-now
  findings each: decision records that omit the choice, scope added in the
  same second as the start, children added after the start, and superseded
  decisions out of order. "Value: high — this delivers the strategy's S4
  post-mission review."
- **Decision notes.** All ten asked for changes: unanswered decision
  records counted as current, and a choice that begins with a field label
  saved as a malformed note. All ten also asked, as fix now, for the
  Agent Factory tracker guidance that the item's last Done when line asks
  for. That finding is an artifact of the shared setup, which drops
  `.agents/`, where the change wrote the guidance. The original reviewer
  saw it.

The judge found the other fix-now findings supported in 47 runs and
answered unknown in three. In two, a finding rests on a timestamp's
precision in a file no review cited. In one, its quoted line was not in
the material. None was found unsupported.

## What it means

- The planted cases test whether a reviewer notices code beyond what the
  item asks for. Sol does. These cases test whether a reviewer questions
  what the item asks for. Sol does not. The skill does ask it to, in
  `roles/reviewer.md`: "If the cost outweighs the value, record one that
  opens `Reject:`". But its value scale reads value from the outcome
  ("high, the outcome needs it"). Each skill-arm reviewer took the item's
  Goal as that outcome, rated the value high or medium, and so never
  reached `Reject:`. Here every part removed later serves a Done when
  line.
- Most of the reasons for removal came from facts after the change: the
  Desk, the rewrite's single path for trivial changes, the harness's limit
  as agents met it. Two were visible at the time: "catching up is `git
  merge`", and the 10-minute limit the `--wait` item names itself. Those
  two cases are the fairest test of a reviewer who questions the need.
  Ten runs of each, five per arm, found it in none.
- For atelier-ufbm's passage, which asks which requirement each new part
  serves: on these cases every part has an answer, a Done when line, so the
  passage cannot pass more runs here either. A passage that would has to
  make a reviewer weigh the requirement itself against its cost, not take
  the item's Goal as proof of value. These cases are not at their ceiling.
  They are at their floor, and they can show whether such a passage moves
  it.

## Limits

- Five runs a cell. With 0 of 25 in each arm, the cells agree, but they
  could not show a small effect.
- In each case the part later removed is the whole change. A finding that
  asks for less of the part asks for less of the item, so the case asks
  more of a reviewer than atelier-s6xk's did.
- The fixture drops the old tracker's files, so the decision-notes change
  lacks the skill guidance it wrote. The base-exemption change lacks one
  sentence of the vendored skill reference. See the README for the rows
  of Tried and removed that do not replay at all.
