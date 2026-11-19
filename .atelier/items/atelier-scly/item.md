---
schema: atelier.item
schema_version: 3
id: atelier-scly
title: Delete a replayed review's build output once it is graded, so a matrix cannot fill the disk
status: building
mission: atelier-kw8u
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A90b6e89e-5f0b-4a65-9258-88eb6abe0d8b%3Adelegate-task%3A653e24dd-a76a-40f9-ab1c-4f07a3b8c3f9
ready:
  by: 'the lead of atelier-kw8u: replayed reviews filled the host''s disk twice on 2026-11-19'
  at: 2026-11-19T10:32:10.603919033Z
trivial: null
caused_by: null
created_at: 2026-11-19T09:12:32.354952666Z
---

## Goal

A replayed review builds the old workspace with cargo in its own `target/`,
about 2.5G, and the harness keeps a run's work directory whenever the run
did not pass. Most replays do not pass, so a matrix keeps most of its
builds. On 2026-11-19 atelier-id3v's runs held 132G in `/var/tmp/af-evals`
and filled this host's disk, which the T3 Code server shares. Six house
replays ran out of space mid-review and had to be set aside and made again,
and the harness crashed writing `results.jsonl`, leaving a truncated line.

## Done when

- [x] Once a replayed review is graded, its `target/` is deleted, whether it
  passed or not; the rest of the work directory is kept as now. So is
  build output the reviewer made elsewhere under the work directory, such
  as a `target/` in a checkout under its `tmp/`.
- [x] A test with a stand-in agent shows a failing replay's work directory
  kept without its `target/`.

## Scope

`evals/agent-factory/harness/` and the README's line on what a run keeps.

## Report

Every run's cargo build output is now deleted once the run is graded,
whatever the grade and even with `--keep`; the rest of a kept work
directory stays. `run_one` in `harness/run.py` calls `drop_build_output`
in a `finally` after grading, so a harness error between the agent's end
and `result.json` deletes it too.

A target is any directory under the work directory holding cargo's
`CACHEDIR.TAG` and `.rustc_info.json`. That finds `repo/target` and one a
reviewer made with its own `CARGO_TARGET_DIR`: today's kept run
`20261119-045153-quiet-tokens-none-house-1-597a` holds a 695M
`tmp/atelier-quiet-review-baseline-target`, with no `repo/target`.
Cargo's registry cache carries the same tag but no `.rustc_info.json`,
so it stays; `.git` is not searched and symlinks are not followed. It
applies to every scenario, not only the replays, since only the replays
build Rust and a target is never graded: `grade.py`, `run_diff`,
`review_files` and `replay.cited` read the transcript and Git, not build
output.

Tests, in `harness/test_run.py`:

- `JudgeTest.test_a_failing_run_keeps_its_work_directory_without_its_build_output`:
  a stand-in agent builds in `repo/target` and in `tmp/base-target` and
  writes `tmp/notes.md`; the judge fails it. The work directory is kept,
  both targets are gone, and the notes and `repo/.git` stay. It fails with
  the deletion taken out.
- `test_only_cargo_target_directories_are_build_output`: the registry
  cache, a target under `.git` and one behind a symlink are left alone.

`python3 -m unittest test_run test_confine` passes, 102 tests. The
README's line on what a run keeps, the module docstring and `--keep`'s
help say the same.

Not done: the targets already under `/var/tmp/af-evals` from earlier
runs are left as they are, since another thread's matrix may still be
reading them. `find /var/tmp/af-evals -mindepth 2 -name .rustc_info.json`
lists them for whoever clears them.
