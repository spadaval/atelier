---
schema: atelier.review
schema_version: 3
round: 1
commit: 28ac4da656a19a7a8633b84de7cf292f8cea8e98
reviewers:
- thread:delegated-task:command%3Amcp%3Acad18721-010a-4ee8-a137-ecbbf4a20448%3Adelegate-task%3A652da34e-4708-4332-a39e-6a79b24ea7c2
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-16T20:12:21.407852063Z
---

## Summary

Complexity: medium. The change adds a CI job, a fixture Desk process and a test
runner, with path selection shared with the existing jobs.

Value: high. A Desk contract break now fails the pull request that introduces
it, which is the central requirement of this item and its mission.

Risk of merging it: low. It changes tests, CI and documentation, with visible
failures and an easy revert; it changes no shipped Desk or extension behavior.
The value warrants the added setup, and I found no smaller change that would
prove the same behavior.

I reviewed the diff from origin/master through submitted commit
28ac4da656a19a7a8633b84de7cf292f8cea8e98 and checked each Done when line.
Inspection and execution used my detached worktree at
/home/supadava/projects/atelier-7tg6-review. The installed CLI requires review
registration and the verdict in the item's branch checkout, so only the review
record and the verdict's item update are written there.

Local verification used TMPDIR=/home/supadava/.cache/atelier-tmp and this
worktree's target directory:

- `cargo build --locked -p atelier-desk -j 4`, `pnpm --dir desk install
  --frozen-lockfile` and `pnpm --dir desk build` passed.
- `BROWSER=/usr/bin/google-chrome node t3-extension/run-tests.mjs` passed all
  26 checks in 42 seconds against the Desk and app built from this commit.
- A second invocation supplied `DESK` from an independently started fixture
  Desk. All 26 checks passed in 42 seconds, and that Desk was still serving
  afterward, until my wrapper stopped it.
- All 34 `scripts/ci-jobs.test.sh` cases passed. The cases cover all required
  paths, extension-only and core-only changes, the workflow exclusions and
  missing bases. `git diff --check origin/master...HEAD` passed.
- In a separate temporary directory, I copied the runner and added one failing
  test, a later passing test and a failing `test-contract.mjs`. It discovered
  both ordinary tests, continued after the first failed, excluded the contract
  test, named the failing file and exited with status 1.

I inspected #182's diff and its CI run: only the extension job failed, with
`Fixture missionatelier · 1/undefined done` and
`1 of 1 tests failed: test-extension.mjs`; Rust and Desk passed. The probe
changes the serialized sidebar field, its generated TypeScript and the Desk's
own assertion, leaving the extension consumer unchanged.
Evidence: https://github.com/spadaval/atelier/actions/runs/43945959570/job/133575975011

I inspected #183's diff, all three attempts' job timestamps and the cache logs.
Its sole change is a comment in `t3-extension/run-tests.mjs`; Rust and Desk were
skipped and the extension passed each time. The API timestamps give 1m45s,
1m33s and 1m18s, all well under six minutes. Attempt 1 reports no Rust cache;
attempt 2 restores the full `v0-rust-extension-...` key. The pnpm store is
restored even in attempt 1, as the Report says. The Report's 1m44s for attempt
1 differs from the API's rounded start/end timestamps by one second and does
not affect the criterion.
Evidence: https://github.com/spadaval/atelier/actions/runs/43945984126

PR #184's CI run on the submitted head also concluded success: `changes`,
`test` and `extension` passed, and `desk` was skipped as expected. The workflow
retains the unconditional `changes` job, so an all-skipped test selection still
has a successful run.
Evidence: https://github.com/spadaval/atelier/actions/runs/43947224964

Requiring a fixture-serving Desk for `DESK` is consistent with removing live
record expectations. The remaining proxy responses and their resulting
coverage limits are documented. I did not run the real-T3 contract test,
Windows or macOS, or rerun the full Rust suite locally; the item excludes the
contract test and those CI platforms, and the submitted head's Rust CI passed.
The live Desk on port 3780 was left alone.

No fix-now finding is open. Approve, with the documentation follow-up below.

## Finding 1: Document port forwarding for a fixture Desk on another host
Recommendation: defer

`t3-extension/README.md:544-547` offers Windows pointing at a fixture Desk
started on another host, but gives only that Desk's loopback URL.
`t3-extension/fixture-desk.mjs:99` always binds `127.0.0.1:0`, so the printed
address cannot be reached directly from another host, and substituting the
host's name does not make that listener reachable. The same-machine `DESK`
workflow passed; the missing step concerns the documented remote example.

Add an SSH forwarding example using the printed remote port, then set `DESK`
to the forwarded local port. Keeping the fixture listener on loopback is
sufficient; this needs no new server option. This documentation omission does
not block the CI job or the verified local workflows.
