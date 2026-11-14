---
schema: atelier.review
schema_version: 3
round: 5
commit: bd24e0296f9e2d7e3f3b656e1043ecc967062b90
reviewers:
- thread:delegated-task:command%3Amcp%3Ab13f1839-3e0a-4726-bee5-0d8939767c53%3Adelegate-task%3Ad749f395-8a6d-4898-85a3-639c11ae0b44
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-14T02:44:50.256414468Z
---

## Summary

Complexity: high. This adds remote and forge polling, persistent news,
stop publication, cross-branch pages and a shared-core/CLI refactor.

Value: high. The Desk supplies the live view and veto required after
removing the human's go-ahead.

Risk of merging it: high. An incomplete forge result can still silently
show a stop completed while the plan's pull request remains open.

The outcome warrants this scope; I found no substantially smaller change
that delivers it. The carried freshness finding is fixed, but Finding 2
must be fixed before merging.

I checked the change and the diff since round 4, inspected fetching and
state settlement, stop publication, the shared drop and removed decide
paths, news, server joins, T3 polling, the interface and the Desk documents.
The agreed-mission stop boundary and opt-in mail agree with the revised
Done when lines and the Report.

Validation on the reviewed code:

- `cargo nextest run -p atelier-desk -p atelier-app -p atelier-cli`: 287
  passed, 1 skipped, including both new round-4 regressions and the existing
  confirmed-stop/failed-refresh regression.
- `pnpm test`: 41 passed; `pnpm e2e`: 48 desktop/phone tests passed;
  `pnpm typecheck` passed.
- `cargo fmt -- --check`, `git diff --check master...HEAD` and
  `desk/service/notify-mail.test.sh` passed.
- The installed configuration names the five live repositories, fetches
  every 300 seconds and leaves mail disabled. Its build is `6b40fec117`;
  the later reviewed commit changes the Report only. The repos, proposed,
  motion and disputed APIs read all five without errors and carry fetch
  marks. `desk-matches-cli` passed: all 16 missions and five inboxes match
  this checkout's CLI. The PR's Rust and Desk CI checks passed.
- An additional test in an external scratch crate reused this branch's
  bare-remote/gh fixture and HTTP router. A successful but capped forge
  list after a failed close reproduced Finding 2. No implementation or
  repository test file changed.

I did not stop a live plan, push a stop to GitHub, close a real pull request
or send mail. Writes and authentication under the installed systemd service
remain untested; publication tests use a bare remote and a stand-in forge.

## Finding 1: Keep an unconfirmed pull-request close from becoming a completed stop
Recommendation: fix now
Was: 4-c4m7.1
Response: fixed in 6b40fec
Now: fixed

`Fetches::pr_state` now qualifies the forge result by the branch tip
captured before reading the list. A newly fetched drop cannot inherit a
list read at the previous tip, whether the Desk or another clone pushed
it. Both added regressions pass across the detail and active/completed
lists, including reads while the follow-up list is held. The existing test
where closure was confirmed after the drop and a later read fails still
passes. This resolves the two freshness failures carried from round 4.

## Finding 2: Do not treat a capped pull-request list as proof a plan's PR is closed
Recommendation: fix now

`crates/atelier-desk/src/fetch.rs:184-196` reads only the latest 200 pull
requests of any state. At line 129, an omitted branch becomes
`PrState::Known(None)`, so `read::settle_state` treats the omission as
confirmation that a pushed stop has no open PR. An open plan can be older
than those 200 results; a fresh list at the drop's tip does not make this
limited result exhaustive. The same omission also removes the plan's PR
link from Proposed and its mission page.

I reproduced this with the existing HTTP fixture: first read the plan's
OPEN PR #7, then let `gh pr close` fail and let the subsequent successful
`gh pr list` return 200 newer, unrelated CLOSED PRs (#8 through #207).
The act correctly said its pull request was not closed, but `shown`
returned `(stopped, false, true)`: the detail said stopped, the active list
omitted it and the completed list included it. The required result is
`(stop_pending, true, false)`. Unlike the carried finding, this failure
needs no stale cache or concurrent read; every fetch succeeds and the
branch tips match.

Establish closure or absence for the actual branch, or read an exhaustive
set before using absence as confirmation. Merely increasing the limit
moves the failure. Cover an older open PR excluded from a full capped
response, including the failed-close result and the detail and both
mission lists.
