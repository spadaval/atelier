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
updated_at: "2026-11-11T07:21:06.400699702+00:00"
---

## Goal

Put the agents and the acts on the pages. After this item Sumeet sees who is working on a mission and its items, and from a page can say go on a draft mission, answer an open question, and start a planning session. It is the app's half of atelier-e05n and atelier-h3hl.

## Done when

- [ ] The mission and item pages show their agents from `threads[]`: role, the thread's title as a link to T3 Code, state and model; an identity T3 Code does not know as text with "not running in T3 Code"; "No thread has started this item." for a ready item with none. When T3 Code cannot be reached the pages render and say threads are unavailable.
- [ ] A live-threads page (`/agents`, and filtered to a repository at `/r/:repo/agents`) lists T3 Code's live threads with what each holds in the records, linking to the mission or item. It has no effort, context or token column.
- [ ] The inbox lists the threads waiting on a person, each with a link to T3 Code.
- [ ] A draft mission's page shows Say go when `can_ready` is true. Pressing it asks once for confirmation, sends the act with the app's header, and on `done` reloads the page, which shows the mission ready. On `refused` the page shows the refusal's text and next step as given, and nothing else changes.
- [ ] An open question, on its item's page and in the inbox, has a form for the answer and its settlement. After `done` the question shows as answered from the Desk and not yet committed, from `pending_answers`, until a commit holds it. A refusal is shown as given.
- [ ] A mission's page has a control that starts a planning session and then shows the thread's link; a refusal is shown as given.
- [ ] No other act has a control. An identity is shown whole and never put in a URL path.
- [ ] Unit tests cover the act and session requests and each result's state. Playwright tests on fixture responses cover, at a desktop and a phone width: agents shown and unavailable, Say go through to the ready page, a refusal, an answer through to its pending state, and starting a session.
- [ ] `pnpm typecheck`, `pnpm test`, `pnpm build` and the Playwright suite pass, and the `desk` CI job runs them.
- [ ] With `atelier-desk` serving a scratch copy of the converted clone, the Report has screenshots at desktop and phone widths of a mission page with its agents, of Say go before and after on a draft mission, and of an answered question in its pending state.

## Scope

`desk/` only, and `docs/architecture/desk.md` where the pages as built differ. No Rust: a missing field is an escalation unless it is a field of a view that exists. Acts tried by hand run only on a copy of the converted clone under `~/.cache/atelier-desk/scratch/`, and no T3 thread is started in a project of Sumeet's.

Expected size: about 900 lines in `desk/` with tests and fixtures.

## Context

- The API: `docs/architecture/desk.md`. The strategy: `docs/strategies/desk.md`, Design (Acts, Sessions, Access). The spike's `Agents.tsx` and `ItemDetail.tsx` at `/home/supadava/projects/atelier-spike-mc/crates/atelier-site/deck/src` show the layout to keep; the spike's transitions menu is replaced by the answer form.
- atelier-aab1 leaves open whether T3 Code's thread links open on the phone. Use the `link` the API gives; if the validation (atelier-va99) finds they do not open, the Desk links to the host's own T3 Code address instead.
- The real end-to-end proof, a browser saying go against the installed service and the commit compared with the CLI's, is atelier-va99's.
- Sizes: complexity medium, value high, risk low.
