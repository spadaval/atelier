---
schema: atelier.item
schema_version: 3
id: atelier-1z52
title: Replay three changes whose defect was found after merge as review cases, and report what one review costs
status: approved
mission: atelier-kw8u
labels: []
priority: medium
blocked_by:
- atelier-b9o6
- atelier-8e8e
implementers:
- thread:delegated-task:command%3Amcp%3Adfff1aea-5166-4f31-a652-65819a65bb75%3Adelegate-task%3Ab5915a39-b98f-42ba-891e-26f39f0866dc
ready:
  by: 'the human, in conversation on 2026-11-18: put the mission together and then run it'
  at: 2026-11-18T23:49:47.284668623Z
trivial: null
caused_by: null
created_at: 2026-11-18T20:05:52.155883252Z
---

## Goal

The reviewer is the role the control pilot of 2026-11-10 found to matter
most, and it has two invented cases. This repository holds real ones:
`docs/architecture/quality/scorecard-baseline/defects.md` traces 15 defects
in atelier that were found after the merge that caused them, each to its
causing item and commits. Replay three as review cases, and learn what one
such review costs before building more.

## Done when

- [x] A shared setup builds a fixture from a case file: this repository's
  tree at the change's base, tracked by today's binary with new records, and
  the change on an item's branch in review with the Goal and Done when it
  had, under a new item ID. The old records are not carried, the fixture's
  history ends at the change, it has no remote, and the case file stays
  outside it.
- [x] A replay run is kept from the later fix. The repository is public,
  this host holds other checkouts of it, the case files name the fix, and
  today a run has the network and `$HOME`. So in a replay run: no process
  of the run can read this host's other checkouts of this repository or
  the case files; the agent's own web search and fetch tools are off; and
  its commands cannot fetch this repository from GitHub. The model's API
  stays reachable.
- [x] A test, run where the confinement is, shows each: a command in a
  replay run's environment that reads another checkout, reads a case file,
  or fetches this repository fails.
- [x] As a backstop, a run whose transcript shows it reached the later fix
  some other way is invalid and not graded, and the Report gives those
  runs.
- [x] Three case files, each naming its row in `defects.md`, its base and
  change commits, the defect in a sentence and the commit that fixed it.
  They are chosen for a defect a reviewer could find from the change, and a
  diff one reviewer can hold.
- [x] The rubric asks whether a fix-now finding names the known defect, and,
  for each other fix-now finding, whether the code supports it. The judge is
  given the defect and the fix's diff. The reviewer is told nothing.
- [x] `test.sh` builds each fixture and grades a recorded review, with no
  model and no `cargo` build.
- [x] The Report gives one review of each case by the reviewer of `sol` and
  by the reviewer of `house`: found or not, unsupported findings, minutes
  and tokens. It says what five runs of ten such cases would cost in each
  setup. Reviews that build with `cargo` run one at a time, each in its own
  target directory.
- [x] The Report says what its author saw that would help or block leaving
  a replayed item ready for an implementer, or several ready under an open
  mission, in place of in review. It is an observation: no code is written
  for either.

## Scope

`evals/agent-factory/scenarios/` and `evals/agent-factory/harness/`; CI's
`evals` job where the setup needs this repository's history.

## Report

Both reviewers found the known defect in one replayed change of three, the
same one, and no other fix-now finding was unsupported. One review costs 1
to 3.5 minutes and 0.26M to 0.71M tokens in. Five runs of ten such cases
would take about 1.5 hours in `sol` and 2.3 in `house`, one at a time. The
runs are in `evals/agent-factory/results/2026-11-18-replayed-reviews.md`.

| Case | `sol` | `house` |
| --- | --- | --- |
| `approval-follow-up` (atelier-cafy, row 40) | approved, not found; 0 unsupported; 1.1 min, 262k in, 2.1k out | approved, not found; 0 unsupported; 1.8 min, 339k in, 3.6k out |
| `base-record-commits` (atelier-3mjv, row 32) | not found; 0 of 2 unsupported; 2.2 min, 516k in, 4.7k out | not found; 0 of 3 unsupported; 3.4 min, 712k in, 7.6k out |
| `base-done-claims` (atelier-fj1t, row 68) | found, reproduced; 0 of 1 unsupported; 1.8 min, 408k in, 3.9k out | found, reproduced; 0 of 1 unsupported; 2.8 min, 603k in, 6.7k out |

Tokens in are mostly cached (71% to 92%). Each judge, Claude Opus for both
setups since both reviewers are `gpt-6.1-sol`, took 3 to 11 seconds and
about 14k tokens in. Both setups' reviewers are the same model, at high
and xhigh, so on review cases `house` measures the cost of more effort:
here about 1.5 times the time and tokens, for the same findings. I read
each review: the judge's six `found` answers match mine. No run was
invalid. No transcript showed the fix and no reviewer tried GitHub; each
proxy refused only `ab.chatgpt.com`, Codex's own telemetry.

What changed:

- Shared setup (`harness/replay.py`, 8c14d4c16). A `case.toml` names the
  defects.md row, the causing item, base, change and fix commits, the
  defect, and the item's Goal, Done when, Context and Close Reason as they
  stood. `build()` archives the tree at the base from the harness's
  checkout, leaves out `.atelier/`, `.agents/`, `.claude/` and `.codex/`
  and AGENTS.md's three process sections, tracks it with today's binary
  under a new ID, commits the change on the item's branch with its own
  message, and submits it. The fixture has no remote, two branches, one
  commit on main and four on the branch. The old tree's `.claude/` would
  otherwise have run `bd prime` hooks and its `.agents/` loaded the old
  skill into Codex.
- Confinement (`harness/confine.py`). The agent runs under bubblewrap with
  an allowlist of mounts: its work directory, `/usr`, `/etc`, `/sys`, the
  agent's own program, `~/.cargo/bin`, `~/.rustup`, a crates.io cache and
  its sign-in. Everything else under `/home`, `/var`, `/tmp` and `/run` is
  absent, other runs' work directories included. Its network namespace is
  empty but for a forwarder to a CONNECT proxy outside, which tunnels only
  to its model's API on 443. Claude Code's WebSearch and WebFetch are
  disallowed and Codex's `web_search` is `"disabled"`. Claude Code's
  `~/.claude` is bound with every entry but `.credentials.json` hidden, so
  a token refresh writes the host's real file and old session transcripts
  stay out. Codex 0.169 under `codex exec` offered no web search even by
  default on this account (tried on luna and sol), so that setting is a
  backstop.
- Backstop (`replay.leak`). A transcript showing the fix's abbreviated
  commit, its subject or the bug item's ID, or a web search or fetch of
  the agent's own, makes the run invalid. `result.json` lists what the
  proxy refused (`proxy_refused`).
- Tests. `harness/test_confine.py`, in test.sh, runs commands in the
  sandbox: `cat` and `git log` of this checkout, a sibling run's fixture,
  and a case file fail; `git ls-remote` and `git clone` over HTTPS and
  SSH, and `curl` to GitHub with or without the proxy, fail; the run's own
  files work, and a CONNECT to an allowed local host passes. It skips with
  its reason where bwrap cannot make a sandbox. CI's evals job now installs
  bubblewrap, turns off Ubuntu's AppArmor userns restriction and checks out
  the whole history. `harness/replay_tests.py`, run from each case's
  `test_grade.py`, builds the fixture and checks it against the base and
  change commits, checks no marker or defect text reaches it, and grades
  recorded reviews and leaking transcripts. It needs no model and no cargo
  build: each case takes about 2 seconds.
- Rubric (`harness/replay-rubric.md`). `found` (graded) asks whether a
  fix-now finding names the known defect; `others-supported` (reported)
  asks whether the code supports each other fix-now finding and has the
  judge give the count. `judge.read_rubric` appends the defect, the
  change's diff and the fix's diff, which only the judge sees.
- `run.py`: `"confine": true` runs the agent in the sandbox (refused with
  threads, and when bwrap cannot run); `"cargo": true` fetches crates into
  `<work root>/cargo-home` first and requires `--jobs 1`. Each reviewer
  built offline into its own `target/`; the old workspace compiled and ran
  its merge gate tests in under a minute.
- Two fixes found while running it. bcbb87a1f: a program in any `bin/`
  bound the directory above, so a plain `~/.local/bin` install would have
  exposed `~/.local`; only a Codex release now does. 8fecb9a59: the judge
  of `house`/`base-record-commits` quoted a line of the change's diff and
  the harness, which checked quotes only against the run's own material,
  turned a sound answer into unknown. The diffs now count as material, a
  test shows it, and `judge.py` re-judged that run: 0 of 3 unsupported.

Choices a reviewer may question:

- The three cases are the smallest single-commit changes among the 15
  after-merge rows. 37uq and 7u4o were left out because their defect came
  in a later review-fix commit on a branch with master merged in, so no
  one base and change replay the item as it was reviewed. The 3mjv and
  fj1t cases replay round 1 of items that had several rounds. The defect
  was in round 1's change and survived to the merge.
- The scenario names don't say "replay", because the run's work path names
  the scenario and the reviewer is to be told nothing. A results file holds
  the runs, as the other evals do.
- Old Done when paragraphs are split into one line per sentence, since
  today's binary ticks lines. The old Close Reason stands as the Report.

What would help or block leaving a replayed item ready for an implementer,
or several ready under an open mission (an observation; no code written):

- Help: `build()` already writes the item at `ready` with its Goal and Done
  when before `start`, so stopping there is the fixture. The old workspace
  builds and tests in the sandbox in under a minute. Each fix carries a
  regression test that drives the CLI (047155d4 adds 88 lines to
  `tests/session.rs`), the natural grader of whether merged code still has
  the defect.
- Block: an implementer replay needs threads (implement, then a reviewer),
  and a confined run refuses them today, because `delegate` starts each
  child with its own environment outside the sandbox. Grading needs that
  hidden test run with cargo, which `test.sh` cannot do, and each fix's
  test is written against later code (047155d4 sits six days and many
  merges after its base), so it would need porting per case. The items'
  Done when lines don't name the defect ("A test covers the chosen
  rule"), so only the hidden test can tell.
- Several under a mission: the three changes sit on three bases over 19
  hours (ab146920, b0895870, 3fa41567) with other merges between, and a
  fixture holds one base. 3mjv and fj1t do chain: fj1t's Goal is 3mjv's
  defect. A mission at 3mjv's base with fj1t blocked by 3mjv would replay
  that day, but fj1t would then be graded against the implementer's own
  3mjv, not the original.
