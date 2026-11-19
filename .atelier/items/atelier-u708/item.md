---
schema: atelier.item
schema_version: 3
id: atelier-u708
title: 'Run a matrix''s runs side by side: make the cap on runs at once a setting, by kind of run'
status: building
mission: atelier-kw8u
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A3a9fab39-7f29-4732-8284-d6c07f2deac6%3Adelegate-task%3A9d593cd6-175c-4a28-a844-d3290c45da60
ready:
  by: 'the human, by a side chat on 2026-11-18: lift the limit and run the matrix in parallel'
  at: 2026-11-19T03:43:07.963292839Z
trivial: null
caused_by: null
created_at: 2026-11-19T03:43:07.921274705Z
---

## Goal

The harness refuses `--jobs` above 2, and above 1 where a case starts
threads, so the matrix of atelier-id3v, about 500 runs, takes about a day.
The limit of two eval agents alive at once was the human's, set on
2026-11-10 for mission atelier-qr5s, for token cost and after ten agents
building Rust pushed this host's load to 45. On 2026-11-18 they lifted it:
spend on an eval is no concern, and the host sat at load 0.5 on 12 cores
under a running matrix, since a small case's agent mostly waits on its
model. Only the replayed reviews build Rust, and this host holds about
three such builds at once.

## Done when

- [x] `--jobs` is a setting and no longer a refusal. The harness caps the
  runs going at once by kind: runs that build Rust, the replayed reviews,
  and all others. Each cap can be set, and the defaults are three and ten.
  Cases that start threads run side by side like any other.
- [x] Runs going at once do not touch one another: each has its own work
  directory, locks, agent homes and sign-in links, and a test runs two
  thread cases at once with no model and shows it.
- [x] Within one run nothing changes: its first agent and one child at a
  time, as today.
- [x] A run that ends on a provider's rate limit or spent allowance is
  invalid, with that reason, and not failed. Running the matrix again into
  the same output makes only the runs that are missing or invalid, so kept
  runs are reused.
- [x] The matrix still prints the most agents running at once.
- [x] The README says what the caps are, why they differ, and whose limit
  the old one was. The Report gives the host's load and any rate limit met
  with the defaults on a real matrix of at least 30 runs.

## Scope

`evals/agent-factory/harness/` and the README.

## Report

A matrix now makes its runs side by side, and a real one of 32 runs on
`sol` took 10 minutes 12 seconds instead of the 72 minutes its runs summed
to one at a time, with no capacity error.

The lead changed one Done when line on the human's word of 2026-11-18: the
default cap on runs that build no Rust is ten, not six. The human is on a
ChatGPT Pro plan he expects to bear at least ten agents at once, and said to
drop back if capacity errors appear. The cap on the replayed reviews stays
three, since that one is this host's limit on Rust builds, not a
provider's.

What changed, in `evals/agent-factory/harness/`:

- `run.py` no longer refuses `--jobs`. `--cargo-jobs` (default 3) caps the
  runs whose scenario has `"cargo": true`, which are exactly the replayed
  reviews (a test checks that), and `--jobs` (default 10) caps every other
  run, cases with threads included. Each kind has its own pool, and results
  are written as runs finish. `judge.py --jobs` lost its refusal too and
  defaults to 10.
- Runs going at once were already separate: each has its own work
  directory, holding its `delegate` lock and log, Codex homes, sign-in
  links and `TMPDIR`; a replay's sandbox, proxy socket and `target/` are
  its own. Only `<work root>/cargo-home` is shared, which cargo locks.
  `test_two_thread_runs_at_once_do_not_touch_one_another` runs two thread
  cases of `house` at once with stand-in agents: their first agents
  overlap, each run's reviewer ran in its own run's Codex home with the
  sign-in linked while it ran and gone after, each lock, log, transcript
  and failed `atelier` call belongs to its own run, and each run's peak is
  still 2.
- Within a run nothing changed: `delegate`'s lock and `max_threads = 1`
  are as they were.
- `provider_limit` reads a transcript and returns what the provider said
  if the agent ended on a rate limit or spent allowance: a rejected
  `rate_limit_event`, a `rate_limit` or `billing_error` API error, or an
  error result for Claude Code, and an `error` or `turn.failed` naming a
  usage limit, 429 or quota for Codex. A limit followed by a reply does not
  count. A run whose first agent, `delegate` thread or judge ended so is
  invalid with `rate limit or spent allowance: <who> ended on '<said>'`. I
  found no real limit in any saved transcript, so the shapes come from
  Claude Code's stream-json and Codex's exec events, and the tests use
  them.
- A matrix run again into the same `--out` keeps every graded run of the
  same setup, binary (`EVAL_ATELIER`, now in `result.json`) and, in the
  skill arm, skill commit and directory, and makes the rest. The matrix
  pins `--ref` to its SHA once at the start. `table.py` leaves out a null
  run that a later result made again. `matrix_peak_agents` is still
  printed, over the runs made this time.
- The README's running section is one matrix per setup, and its limit
  bullet now gives the caps, why they differ, and that the old limit of two
  was the human's, set on 2026-11-10 for atelier-qr5s and lifted on
  2026-11-18.

The real matrix, in `~/.cache/af-evals/runs/u708-parallel/`: `sol`, both
arms, one run each of the 12 cases without Rust and four replays
(`approval-follow-up`, `merge-sync`, `quiet-tokens`, `touched-claims`), 32
runs at the defaults, with id3v's matrix running one run at a time beside
it on the same plan.

- It opened with ten runs and three replays at once. `matrix_peak_agents`
  was 19: a run with threads has two agents, and judges follow each run.
- No run ended on a capacity error, and none was invalid: 18 passed and 14
  failed. No transcript holds an `error` or `turn.failed` event, and every
  Claude judge's `rate_limit_event` said `allowed`. id3v's runs in that
  window all graded too.
- Host load, sampled every 30 s on 12 cores: the 1-minute load peaked at
  12.9 four minutes in, with replay builds and ten agents running, and sat
  near 5 to 7 after; the 5-minute load peaked at 6.5. Available memory
  never fell below 6.1 GB of 14. (`load.log` beside the results.)
- Running the same command again printed `{"kept": 32, "making": 0}` and
  ran no canary.

Tests: `python3 -m unittest test_run test_confine` in the harness, 96
pass. I did not run the scenarios' own fixtures, which this change does
not touch; CI's evals job runs them.

Not separated: the host's network. `refused-push`'s server reads its port
from `PORT` with 8080 as the default, so two runs whose agents start it
unchanged would meet on one port. The README says so.
