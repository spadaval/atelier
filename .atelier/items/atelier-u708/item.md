---
schema: atelier.item
schema_version: 3
id: atelier-u708
title: 'Run a matrix''s runs side by side: make the cap on runs at once a setting, by kind of run'
status: ready
mission: atelier-kw8u
labels: []
priority: medium
blocked_by: []
implementers: []
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

- [ ] `--jobs` is a setting and no longer a refusal. The harness caps the
  runs going at once by kind: runs that build Rust, the replayed reviews,
  and all others. Each cap can be set, and the defaults are three and six.
  Cases that start threads run side by side like any other.
- [ ] Runs going at once do not touch one another: each has its own work
  directory, locks, agent homes and sign-in links, and a test runs two
  thread cases at once with no model and shows it.
- [ ] Within one run nothing changes: its first agent and one child at a
  time, as today.
- [ ] A run that ends on a provider's rate limit or spent allowance is
  invalid, with that reason, and not failed. Running the matrix again into
  the same output makes only the runs that are missing or invalid, so kept
  runs are reused.
- [ ] The matrix still prints the most agents running at once.
- [ ] The README says what the caps are, why they differ, and whose limit
  the old one was. The Report gives the host's load and any rate limit met
  with the defaults on a real matrix of at least 30 runs.

## Scope

`evals/agent-factory/harness/` and the README.
