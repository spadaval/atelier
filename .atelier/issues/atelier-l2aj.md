---
created_at: "2026-11-11T06:21:51.905925041+00:00"
id: "atelier-l2aj"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: atelier-l2aj
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-94pv"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-e05n"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-h3hl"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-z4cr"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Show agents on the Desk's pages and add Say go, the answer form and Start a planning session"
updated_at: "2026-11-11T07:55:56.263803568+00:00"
---

## Goal

Put the agents and the acts on the pages. After this item Sumeet sees who is working on a mission and its items, and from a page can say go on a draft mission, answer an open question, and start a planning session. It is the app's half of atelier-e05n and atelier-h3hl.

## Done when

- [x] The mission and item pages show their agents from `threads[]`: role, the thread's title as a link to T3 Code, state and model; an identity T3 Code does not know as text with "not running in T3 Code"; "No thread has started this item." for a ready item with none. When T3 Code cannot be reached the pages render and say threads are unavailable.
- [x] A live-threads page (`/agents`, and filtered to a repository at `/r/:repo/agents`) lists T3 Code's live threads with what each holds in the records, linking to the mission or item. It has no effort, context or token column.
- [x] The inbox lists the threads waiting on a person, each with a link to T3 Code.
- [x] A draft mission's page shows Say go when `can_ready` is true. Pressing it asks once for confirmation, sends the act with the app's header, and on `done` reloads the page, which shows the mission ready. On `refused` the page shows the refusal's text and next step as given, and nothing else changes.
- [x] An open question, on its item's page and in the inbox, has a form for the answer and its settlement. After `done` the question shows as answered from the Desk and not yet committed, from `pending_answers`, until a commit holds it. A refusal is shown as given.
- [x] A mission's page has a control that starts a planning session and then shows the thread's link; a refusal is shown as given.
- [x] No other act has a control. An identity is shown whole and never put in a URL path.
- [x] Unit tests cover the act and session requests and each result's state. Playwright tests on fixture responses cover, at a desktop and a phone width: agents shown and unavailable, Say go through to the ready page, a refusal, an answer through to its pending state, and starting a session.
- [x] `pnpm typecheck`, `pnpm test`, `pnpm build` and the Playwright suite pass, and the `desk` CI job runs them.
- [x] With `atelier-desk` serving a scratch copy of the converted clone, the Report has screenshots at desktop and phone widths of a mission page with its agents, of Say go before and after on a draft mission, and of an answered question in its pending state.

## Scope

`desk/` only, and `docs/architecture/desk.md` where the pages as built differ. No Rust: a missing field is an escalation unless it is a field of a view that exists. Acts tried by hand run only on a copy of the converted clone under `~/.cache/atelier-desk/scratch/`, and no T3 thread is started in a project of Sumeet's.

Expected size: about 900 lines in `desk/` with tests and fixtures.

## Context

- The API: `docs/architecture/desk.md`. The strategy: `docs/strategies/desk.md`, Design (Acts, Sessions, Access). The spike's `Agents.tsx` and `ItemDetail.tsx` at `/home/supadava/projects/atelier-spike-mc/crates/atelier-site/deck/src` show the layout to keep; the spike's transitions menu is replaced by the answer form.
- atelier-aab1 leaves open whether T3 Code's thread links open on the phone. Use the `link` the API gives; if the validation (atelier-va99) finds they do not open, the Desk links to the host's own T3 Code address instead.
- The real end-to-end proof, a browser saying go against the installed service and the commit compared with the CLI's, is atelier-va99's.
- Sizes: complexity medium, value high, risk low.

## Notes

- 2026-11-11 [note] Handoff: approved by gpt-6.1-sol-l2aj-r3 (atelier-rnip) at 9f33d5e6, after three review rounds; the last code commit is 1befc0f8. Fix-now findings atelier-obgr, atelier-bicv and atelier-pxrv are fixed, each with a regression test that failed before its fix, and none was deferred or left undisposed. Proof and screenshots are in the Report. Not merged and not pushed: the orchestrator merges.

## Report

Every Done when line holds. In `desk/e2e/acts.spec.ts`, each test runs at 1440x900 and at 390x844: "a mission's page shows its agents" and "an item's page shows its agents, and a ready item with none says no thread has started it" cover the agents, with the role, the title linking to T3 Code, the state, the model, "not running in T3 Code", and the whole identity in no link. "when T3 Code cannot be reached..." covers a mission and an item rendering with "Threads are unavailable". "the live-threads page lists..." covers `/agents` and `/r/atelier/agents`, each hold linking to its mission or item, with no effort, context or token text. "the inbox lists the threads waiting on a person" covers the inbox's threads. "Say go asks once..." checks that nothing is sent before the confirmation, that the request is exactly `{"act":"ready","repo":"atelier","ids":["atelier-295u"]}` with `x-atelier-desk: 1`, and that the page then shows Ready and the sidebar's inbox count drops. With the reload after a done act removed, that test fails. "a refused go..." covers a refusal shown as given with nothing else changed. "an answer on an item's page..." and "an answer from the inbox..." cover the answer form, its settlement, the request body, the pending state on the item and mission pages, and a refusal. "a mission's page starts a planning session..." covers the started link and a refusal. `expectOnlyTheActs` (`desk/e2e/helpers.ts`) runs on the pages and finds no act control beyond the three. The unit tests in `desk/src/lib/acts.test.ts` cover the three requests, the method, path and header, and each result's state: done, refused, started, 403, not JSON, unreadable and no connection. `desk/src/lib/threads.test.ts` covers the repository filter and the T3 note. At the final code commit, `pnpm typecheck` passes, `pnpm test` passes 41 tests in 6 files, `pnpm build` passes, and `pnpm e2e` passes 50 tests. Review round 1 found that an answer form carried its text to the item's next question (atelier-obgr), and that the inbox kept an answer marked as not committed after a commit (atelier-bicv). Both are fixed. "after an answer the item's next question gets an empty, closed form", "an open answer form does not carry over to a question a poll put in its place" and "the inbox drops an answer's not-committed mark once a commit holds it" cover them, and each fails with `desk/src` as it was at 1cc3679c. Round 2 found that an open answer form followed the human to another item reached by a link (atelier-pxrv). That is fixed, and "an open answer form does not follow the human to another item's page" covers it; it failed at both widths before the fix. The `desk` job in `.github/workflows/ci.yml` already runs all four.

### Validation

I ran `~/.cache/atelier-desk/bin/atelier-desk`, built from master, on 127.0.0.1:3884. It served this branch's `desk/dist` over a clone of `~/.cache/atelier-desk/clones/atelier` under `~/.cache/atelier-desk/scratch/l2aj`, with `[t3]` set to the 0.1.9 nightly. I ran `atelier start atelier-1ie1` in a worktree of that clone from inside this item's T3 thread, and `atelier escalate` there gave it a question. Screenshots, each at desktop and phone width, are in `~/.cache/atelier-desk/shots/l2aj/`:

- `mission-agents-*.png` and `item-agents-*.png` show this thread as the live implementer, linking to app.t3.codes.
- `say-go-before-*.png` and `say-go-after-*.png` show Say go on draft atelier-295u, with the confirmation in `say-go-confirm-desktop.png`. It made one commit, `d1596f48 Ready atelier-295u`, touching only that mission's `mission.md`.
- `answer-pending-*.png` and `mission-after-answer-*.png` show an answer in its pending state. The answer was written into `item.md` in the worktree and not committed, and the inbox dropped the question.

Two refusals came back from the real server and match the fixtures' shape: `decide` on an item with no checkout, and `ready` on a mission that was already ready. A `ready` sent without the header got 403. I stopped the server with SIGINT, and `t3 auth session list` then showed no atelier-desk session. The scratch copy is deleted. No planning session was started against T3 Code; that control is proved on fixtures only.

### Decisions

- Confirmation is an inline group ("Confirm saying go", with "Yes, say go" and "Not yet"), not `window.confirm`, so it works the same on a phone and in a test. Starting a planning session asks once the same way, because it starts a real thread.
- The answer is a one-line input, because the core's `decide` refuses an answer with a line break. The settlement choices show only when the question names a finding, because `decide` refuses a settlement otherwise.
- `pending_answers` has no index. The app matches each pending answer to the first unanswered question with the same item, text and finding (`withPending`, tested). The inbox has no `pending_answers`, so an answer given there stays on the page for that visit only, marked as not committed yet.
- On the mission page, agents T3 Code does not list as running fold after four, behind "Show N more".
- `who()`, which cut identities in the middle of the text, is gone. Every identity is now whole in the DOM, cut short with CSS only.

### Surprises

- Size: net growth is about 1,720 lines against the 900 expected. 370 of them are CSS, because the app had no button, field or form styles yet. Tests and fixtures are about 690 lines.
- The real refusal texts speak in CLI terms, such as "Run it there, or `git switch atelier-psjw` here". They are shown as given.
