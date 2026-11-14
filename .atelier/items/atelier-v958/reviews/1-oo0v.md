---
schema: atelier.review
schema_version: 3
round: 1
commit: 8c1b0c933af40bde38119fd5407e11739c387147
reviewers:
- thread:delegated-task:command%3Amcp%3Ab13f1839-3e0a-4726-bee5-0d8939767c53%3Adelegate-task%3Ae2466f96-2448-4c5e-a0dd-c78cb2a6566e
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-14T02:05:36.177675661Z
---

## Summary

Complexity: high. This changes the Desk's read contract across branches,
adds persistent news and remote polling, and publishes stops through Git and
the forge, alongside a core/CLI refactor and new interface pages.

Value: high. The human needs to see proposed work and reliably stop it now
that plans need no go-ahead.

Risk of merging it: high. A stop can silently leave the mission running
while the Desk says it is stopped, and proposed work can be omitted.

The value justifies the moving parts, but the findings below need fixing.
I found no substantially smaller change that delivers the stated outcome.

I reviewed `master...8c1b0c933af40bde38119fd5407e11739c387147`, including
the core/CLI changes outside the stated Scope and the two Desk documents.
The revised boundary for stopping agreed missions and the opt-in mail
configuration are documented consistently with the Report. The old decide
act, checkout finder and say-go control are removed.

Validation:

- `cargo nextest run -p atelier-desk -p atelier-app`: 70 passed.
- `cargo nextest run -p atelier-cli`: 207 passed, 1 skipped.
- `cargo clippy --all-targets -- -D warnings`, `cargo fmt -- --check`,
  `git diff --check`, frontend typechecking,
  `pnpm test` (41 tests), `pnpm build`, and `pnpm e2e` (48 desktop/phone
  tests) passed.
- `desk/service/notify-mail.test.sh` passed against its stand-in. No mail
  was sent.
- The installed configuration names the five live repositories and a
  300-second fetch interval. `/api/repos` reports successful fetches.
  `desk-matches-cli` against that service reports all 16 missions and five
  inboxes equal. The installed service reports build `932f1cc7`; its
  difference from the reviewed commit includes wording and documentation.
- `act-is-a-record` with the reviewed debug binary and freshly built app
  passed: one pushed commit, identical record/message/paths to the CLI, a
  closed stand-in pull request, and a stopped page.
- The PR's `changes`, `desk`, `plan` and `test` checks are green.
- Additional temporary repositories and a bare remote reproduced findings
  1–4 below through the real HTTP server. A standalone harness linked to
  this build reproduced finding 5 through `News::observe`. API inspection
  and the component code establish finding 6. These fixtures stayed outside
  the repository; no implementation file was changed.

I did not stop a real live mission, push a stop to GitHub, test the installed
service's SSH/forge credentials by writing, or send a real notification.

## Finding 1: Recheck whether the plan has merged after fetching the stop clone
Recommendation: fix now

`crates/atelier-desk/src/stop.rs:115` opens the freshly fetched clone but
immediately calls `drop_mission` without repeating the proposed-plan check
from line 70. If the plan merged after the served repository's last fetch,
the initial check passes, but the new clone knows it is agreed.
`drop_mission` then correctly writes an agreed mission's drop; the Desk
incorrectly publishes it only by pushing the old plan branch and closing
its pull request. That cannot stop an agreed mission, whose drop must merge
into the base. I reproduced this by pushing a plan, fetching the served
clone, then merging the plan remotely before posting the stop, with a
successful `gh pr close` stand-in. The API returned `done` and “The plan is
stopped”; the page said `stopped`, while the remote base's mission still
said `status: open`. Refuse the proposed-plan stop if the fresh clone finds
it agreed. Also make `read.rs:136` honor `stop_on` for an agreed mission
before treating a drop on its own pushed branch as published; such a drop
is pending until it merges, regardless of the pull request's state. Add a
regression covering a merge between the served fetch and the act.

## Finding 2: Keep an unconfirmed pull-request close from becoming a completed stop
Recommendation: fix now

`crates/atelier-desk/src/read.rs:186` treats an absent `PrMark` as proof the
pull request is not open. But `fetch.rs:62` discards forge-read errors and
`pull_request` returns the same `None` for an unknown pull request and a
confirmed absence. With no cached entry, a pushed drop is therefore shown
stopped even if closing its pull request failed. I reproduced a successful
push followed by a failing `gh pr close` and failing `gh pr list`: the act
text said the stop was not yet published, but the refreshed mission page
had `pull_request: null` and `state: stopped`. No fetch error warned about
the failed forge read. Preserve and expose the distinction between an
unknown forge state and a confirmed closed/absent pull request, and keep the
stop pending while closing is unconfirmed. Cover failure on the first
forge read as well as failure after a previously successful read.

## Finding 3: Settle the stop state before filtering active and completed missions
Recommendation: fix now

`crates/atelier-desk/src/read.rs:272` filters missions using the state
before `server.rs` joins the pull-request state. For a proposed plan whose
drop is pushed but whose pull request remains open, the reader calls it
`stopped` and excludes it from the active list. The server subsequently
changes it to `stop_pending`, but only in the completed list where it was
already included. With a known open pull request I reproduced a detail page
saying `stop_pending`, an active list omitting the mission, and
`?done=true` returning that same pending mission. The main rail consequently
loses the unfinished stop. Apply the final state before the active/done
partition and test both list endpoints, not just the detail endpoint.

## Finding 4: List amendments on an agreed mission's own branch as proposed work
Recommendation: fix now

`crates/atelier-desk/src/read.rs:819` only enumerates `Located::changes`,
which contains changes on other branches and excludes the record's own
branch. An agreed mission's amended plan is read from its own branch, but
its state is `open`, so it also misses `plans` at line 743. In addition,
`own_item` at lines 756–762 mistakes the empty item locations created for
mission branch IDs for started items and suppresses all records from those
branches. I reproduced an agreed mission on `main`, an unmerged branch
named for it that changes its Why, and a checkout left on `main`: the
mission page reads the new Why, but `/api/proposed` returns both `plans: []`
and `changes: []`, without an error. This is information the existing
reader supplies, not a missing reader capability. Include the own-branch
amendment, with its branch, pull request and current plan-review verdicts;
exclude only actual started-item branches. Cover an agreed plan's
amendment and a record-only change carried on a mission branch.

## Finding 5: Report newly merged plans even when no poll saw them proposed
Recommendation: fix now

`crates/atelier-desk/src/news.rs:108` emits merged news only if the immediately
previous observation contained the mission in `was.proposed`. A plan
proposed and merged between the configured five-minute polls is first seen
as agreed and generates neither mail nor an inbox event. This also loses
the news for plans proposed and merged while the service is down, despite
its persistent previous snapshot. I reproduced `observe("demo", &[])`
followed by one new mission with `proposed=false, agreed=true`: both calls
returned zero events and `recent()` stayed empty. Keep the first-observation
baseline quiet as documented, but detect newly agreed IDs on later
observations without requiring a previously observed proposal. Add this
case to the news tests. The current test only covers a proposal that lives
long enough for a separate poll before it merges.

## Finding 6: Show fetch freshness on Inbox and Disputed
Recommendation: fix now

`crates/atelier-desk/src/api.rs:642` gives `DisputedRepo` no fetch mark;
`InboxRepo` also has none. Their server joins and the components in
`desk/src/pages/Inbox.tsx` and `desk/src/pages/Disputed.tsx` consequently
cannot show a repository's last fetch or a failed fetch. I confirmed neither
API response contains `fetched`. Both pages read repository records, so
this misses the Done when line that each page says when it last fetched,
and contradicts `docs/product/desk.md`'s same promise. Add a fetch mark per
repository to both responses, join it outside the commit cache, and display
`FetchedNote` on each repository section, including the empty state.
