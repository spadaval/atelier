---
schema: atelier.review
schema_version: 3
round: 6
commit: 34a3b1661fff45d0d691395aaed4628c139cd15b
reviewers:
- thread:delegated-task:command%3Amcp%3Ab13f1839-3e0a-4726-bee5-0d8939767c53%3Adelegate-task%3A3097d96c-6ae2-4bb7-91f4-80b05f3ccbe0
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-14T02:52:33.352429413Z
---


## Summary

Complexity: high. This adds remote and forge polling, persistent news,
stop publication, cross-branch pages and a shared-core/CLI refactor.

Value: high. The Desk supplies the live view and veto required after
removing the human's go-ahead.

Risk of merging it: high. The Desk can still silently show a stop completed
while the plan's pull request remains open.

The outcome warrants this scope; I found no substantially smaller change
that delivers it. The carried capped-list finding is fixed, but Finding 2
must be fixed before merging.

I checked the change and the diff since round 5, including fetching and
state settlement, stop publication, the shared drop and removed decide
paths, news, server joins, T3 polling, the interface, service scripts and
Desk documents. The agreed-mission stop boundary and opt-in mail match
the revised Done when lines and Report.

Validation on the reviewed code:

- `cargo nextest run -p atelier-desk -p atelier-app`: 81 passed.
  `cargo nextest run -p atelier-cli`: 207 passed, 1 skipped.
- `pnpm test`: 41 passed; `pnpm build`, including typecheck, passed;
  `pnpm e2e`: 48 desktop/phone tests passed.
- `cargo fmt -- --check`, `git diff --check master...HEAD` and
  `desk/service/notify-mail.test.sh` passed.
- The installed configuration names the five live repositories, fetches
  every 300 seconds and leaves mail disabled. Its build is `e5c264e9`;
  the later reviewed commit changes the Report only. The repos, proposed,
  motion and disputed APIs read all five without errors and carry fetch
  marks. `desk-matches-cli` passed: all 16 missions and five inboxes match
  this checkout's CLI. The PR's Rust and Desk CI checks passed.
- Two extra tests in an external scratch crate reused this branch's
  bare-remote/gh fixture and HTTP router. A full 1,000-entry open list
  correctly leaves an omitted plan's stop pending. A complete list with
  open #7 and closed #8 on the same head reproduces Finding 2. No
  implementation or repository test file changed.

I did not stop a live plan, push a stop to GitHub, close a real pull request
or send mail. Writes and authentication under the installed systemd service
remain untested; publication tests use a bare remote and stand-in forge.

## Finding 1: Do not treat a capped pull-request list as proof a plan's PR is closed
Recommendation: fix now
Was: 5-cowv.2
Response: fixed in e5c264e
Now: fixed

The Desk now reads the open pull requests separately and records whether
that list is exhaustive. The added older-open-PR regression passes across
the detail and active/completed lists and retains its link. My additional
test returning exactly 1,000 open pull requests also passes: omission,
even alongside a recent CLOSED entry, leaves the stop pending rather than
certifying closure. This fixes the capped-response failure carried from
round 5.

## Finding 2: Keep an open pull request even when a newer closed one shares its branch
Recommendation: fix now

`crates/atelier-desk/src/fetch.rs:212-215` combines the recent and open
lists by highest PR number. An explicitly returned open PR is discarded
when a higher-numbered closed PR has the same `headRefName`. A branch can
have multiple historical PRs; for example, an older plan PR can be reopened
after its replacement is closed. The map then holds only the closed PR.
Since the open list was exhaustive, `pr_state` at line 142 certifies that
closed entry, and `settle_state` shows the plan stopped despite the open
PR.

I reproduced this through the existing HTTP fixture in an external scratch
crate. After first reading open #7, I made `gh pr close` fail, with
`gh pr list --state all` successfully returning closed #8 and open #7,
both for `demo-pppp`, and `--state open` returning #7. The act correctly
said its pull request was not closed. The detail and both mission lists
nevertheless returned `(stopped, false, true)`, instead of
`(stop_pending, true, false)`, and the detail linked closed #8. Every fetch
and list succeeded, and the open list contained only one entry.

Track whether any PR for the branch is open independently of which
historical PR has the largest number, or give open entries precedence.
Cover this same-head case across the detail, active/completed lists and
the PR link. An explicit open result must prevent confirmation of the
stop.
