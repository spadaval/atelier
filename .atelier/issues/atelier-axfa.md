---
created_at: "2026-11-10T00:56:08.066927831+00:00"
id: "atelier-axfa"
issue_type: "task"
labels:
- "redesign"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
  - kind: "issue"
    id: "atelier-jciw"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-jq24"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-yptz"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Add `escalate` and `decide`, and carry a decision through the review loop"
updated_at: "2026-11-10T21:30:49.465874229+00:00"
---

## Goal

Give a disagreement between an implementer and a reviewer its exits in the tool. A fix-now finding can already be fixed or rebutted. This item adds the third exit: the implementer escalates a question, a decider who is neither the author nor the finding's reviewer answers it, and the answer travels with the finding so that no later review reopens it.

## Done when

- [ ] `escalate "<question>"`, run by the item's implementer on its branch while the item is `building`, `changes_requested` or `approved`, appends an unanswered entry to the item's Decisions section in the form the architecture specification's Records section defines. `--finding <name>` ties it to a finding and is refused, with nothing written, for a name no review of the item has and for a finding that is not open. It makes no commit: it prints the path it wrote and a handoff line for a decider, which follows the reviewer handoff's conventions (the decider is another thread; the task has no backticks) and names `show` and `decide`.
- [ ] While a question is unanswered, the implementer's `status` shows it and says it waits on a decider, and `submit` and `merge` are refused naming it. Once every question is answered they are no longer refused for it.
- [ ] `decide <id> "<answer>"`, run in a checkout of the item's branch, writes `Answer by <thread>: <answer>` on the item's oldest unanswered question and says whether another is waiting. On a question tied to a finding, `--defer` or `--no-action` also writes the `Outcome:` line; either flag is refused on a question tied to no finding. It makes no commit and tells the caller what the implementer does next.
- [ ] `decide` is refused, with nothing written and a line that names the caller's role, for a thread anywhere in the item's `implementers`, and, on a question tied to a finding, for a thread anywhere in the `reviewers` of the review that raised it. It is refused when the item has no unanswered question, and when the checkout is not on the item's branch, naming the checkout that is.
- [ ] A decision with an outcome settles its finding: the finding is no longer open, so `submit` asks for no Response to it. The next round's review file carries it with a `Decided: defer` or `Decided: no action` line that the tool writes, the brief shows the question and the answer beside it, `verdict` is refused if the reviewer changed or removed that line, and `verdict approve` is allowed while it stands.
- [ ] Scenario tests drive the real binary through each of: a rebuttal the next reviewer withdraws; a rebuttal the next reviewer keeps, after which the implementer's `status` shows the finding open under its new name and says a rebuttal has had its one try, so the exits left are a fix or `escalate`; an escalation tied to a finding, decided with `--defer`, then `submit`, a review that shows it decided, `verdict approve` and `merge`; and a question with no finding, answered, after which `submit` passes.
- [ ] A mission's lead sees its items' unanswered questions: `status` for the lead and `show <mission>` list each with its item and the command that answers it. For an item whose mission has no lead, or that has no mission, `escalate`'s handoff asks for a new thread.
- [ ] An approved item that has not merged says since when: `status`, for its implementer and for its mission's lead, and `show` give the time of the commit that recorded the approving verdict, so a wait that was lost is visible. Nothing is stored for it.
- [ ] The act table gains the rows for `escalate` and `decide`, and the unit test over the table covers them from every status.
- [ ] `docs/strategies/redesign-architecture.md` matches what was built.

## Scope

`escalate`, `decide`, and what they change in `status`, `show`, the brief and `verdict`, in `next/`. Deferred findings of done items (`list --deferred`, `new --from`, `drop` on a finding) are atelier-dxrj, built at the same time by another implementer. `ATELIER_SKIP` is step D. The product specification's rules that a rebuttal goes to a fresh reviewer and that escalation is required after three rounds are the skill's to state: the tool shows the round and the exits and enforces neither. A question for Sumeet is an ordinary item labelled `needs-human` and needs nothing here. Nothing is copied from the old workspace. No file under `crates/` is changed.

## Context

Design: `docs/strategies/redesign-architecture.md`, Records (the Decisions entry, `Was:`, `Now:` and `Decided:`), Status (the `escalate` and `decide` rows) and Threads and roles. Behaviour: `docs/strategies/redesign-product.md`, Lifecycle ("A question the implementer escalates is not a status"), Commands (Implementer, Decider and lead), Handoff lines and Review. The records already parse a Decisions entry and a `Decided:` line, and `is_open` in `atelier-core` already treats a decided finding as closed; the commands that write them are what is missing. The item that built `review` and `verdict` is atelier-5cuc, and the fresh-agent runs that set the handoff wording are in `evals/agent-factory/results/2026-11-10-next-loop.md`.

This is the first of step C's three items in `docs/strategies/redesign-2-core.md`.
