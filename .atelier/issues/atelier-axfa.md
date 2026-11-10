---
created_at: "2026-11-10T00:56:08.066927831+00:00"
id: "atelier-axfa"
issue_type: "task"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-axfa
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
status: "in_progress"
title: "Add `escalate` and `decide`, and carry a decision through the review loop"
updated_at: "2026-11-10T22:05:50.906814334+00:00"
---

## Goal

Give a disagreement between an implementer and a reviewer its exits in the tool. A fix-now finding can already be fixed or rebutted. This item adds the third exit: the implementer escalates a question, a decider who is neither the author nor the finding's reviewer answers it, and the answer travels with the finding so that no later review reopens it.

## Done when

- [x] `escalate "<question>"`, run by the item's implementer on its branch while the item is `building`, `changes_requested` or `approved`, appends an unanswered entry to the item's Decisions section in the form the architecture specification's Records section defines. `--finding <name>` ties it to a finding and is refused, with nothing written, for a name no review of the item has and for a finding that is not open. It makes no commit: it prints the path it wrote and a handoff line for a decider, which follows the reviewer handoff's conventions (the decider is another thread; the task has no backticks) and names `show` and `decide`.
- [x] While a question is unanswered, the implementer's `status` shows it and says it waits on a decider, and `submit` and `merge` are refused naming it. Once every question is answered they are no longer refused for it.
- [x] `decide <id> "<answer>"`, run in a checkout of the item's branch, writes `Answer by <thread>: <answer>` on the item's oldest unanswered question and says whether another is waiting. On a question tied to a finding, `--defer` or `--no-action` also writes the `Outcome:` line; either flag is refused on a question tied to no finding. It makes no commit and tells the caller what the implementer does next.
- [x] `decide` is refused, with nothing written and a line that names the caller's role, for a thread anywhere in the item's `implementers`, and, on a question tied to a finding, for a thread anywhere in the `reviewers` of the review that raised it. It is refused when the item has no unanswered question, and when the checkout is not on the item's branch, naming the checkout that is.
- [x] A decision with an outcome settles its finding: the finding is no longer open, so `submit` asks for no Response to it. The next round's review file carries it with a `Decided: defer` or `Decided: no action` line that the tool writes, the brief shows the question and the answer beside it, `verdict` is refused if the reviewer changed or removed that line, and `verdict approve` is allowed while it stands.
- [x] Scenario tests drive the real binary through each of: a rebuttal the next reviewer withdraws; a rebuttal the next reviewer keeps, after which the implementer's `status` shows the finding open under its new name and says a rebuttal has had its one try, so the exits left are a fix or `escalate`; an escalation tied to a finding, decided with `--defer`, then `submit`, a review that shows it decided, `verdict approve` and `merge`; and a question with no finding, answered, after which `submit` passes.
- [x] A mission's lead sees its items' unanswered questions: `status` for the lead and `show <mission>` list each with its item and the command that answers it. For an item whose mission has no lead, or that has no mission, `escalate`'s handoff asks for a new thread.
- [x] An approved item that has not merged says since when: `status`, for its implementer and for its mission's lead, and `show` give the time of the commit that recorded the approving verdict, so a wait that was lost is visible. Nothing is stored for it.
- [x] The act table gains the rows for `escalate` and `decide`, and the unit test over the table covers them from every status.
- [x] `docs/strategies/redesign-architecture.md` matches what was built.

## Scope

`escalate`, `decide`, and what they change in `status`, `show`, the brief and `verdict`, in `next/`. Deferred findings of done items (`list --deferred`, `new --from`, `drop` on a finding) are atelier-dxrj, built at the same time by another implementer. `ATELIER_SKIP` is step D. The product specification's rules that a rebuttal goes to a fresh reviewer and that escalation is required after three rounds are the skill's to state: the tool shows the round and the exits and enforces neither. A question for Sumeet is an ordinary item labelled `needs-human` and needs nothing here. Nothing is copied from the old workspace. No file under `crates/` is changed.

## Context

Design: `docs/strategies/redesign-architecture.md`, Records (the Decisions entry, `Was:`, `Now:` and `Decided:`), Status (the `escalate` and `decide` rows) and Threads and roles. Behaviour: `docs/strategies/redesign-product.md`, Lifecycle ("A question the implementer escalates is not a status"), Commands (Implementer, Decider and lead), Handoff lines and Review. The records already parse a Decisions entry and a `Decided:` line, and `is_open` in `atelier-core` already treats a decided finding as closed; the commands that write them are what is missing. The item that built `review` and `verdict` is atelier-5cuc, and the fresh-agent runs that set the handoff wording are in `evals/agent-factory/results/2026-11-10-next-loop.md`.

This is the first of step C's three items in `docs/strategies/redesign-2-core.md`.

## Report

Every Done when line holds, in `next/`. `escalate "<question>" [--finding <name>]` and `decide <id> "<answer>" [--defer | --no-action]` write the item's Decisions section in the checkout on its branch and commit nothing; `submit` commits them with the item. Tests, all passing: `cargo nextest run` in `next/` (179 tests), `cargo clippy --all-targets -- -D warnings` and `cargo fmt -- --check` clean.

- Escalate (line 1): `decisions::an_escalation_decided_as_defer_is_carried_as_decided_and_the_item_merges` checks the entry, the path and handoff it prints, and no commit; `decisions::escalate_is_its_implementers_on_its_branch_and_writes_nothing_when_refused` checks the refusals (a name no review has, a finding not open, two lines, another thread, in review, off the branch), each writing nothing. `atelier-records` `body::tests::a_question_asked_and_answered_reads_back_in_order` checks the record form.
- Waiting (line 2): the escalation scenario shows `status` listing the question with the decider's handoff, and `submit` refused naming it; `merge::an_unanswered_question_stops_the_merge_and_is_named` covers `merge`.
- Decide (lines 3 and 4): the escalation scenario shows refusals for the implementer and the finding's reviewer, the answer and `Outcome: defer` written, and `decide` refused with no question; `decisions::a_question_about_no_finding_once_answered_lets_submit_pass` shows `--defer` refused on a question with no finding and "Another question waits"; `decisions::decide_runs_in_the_checkout_on_the_items_branch_and_names_it` names the checkout. `acts::the_oldest_question_is_decided_by_a_thread_that_never_implemented_the_item_nor_reviewed_its_finding` covers every implementer, every reviewer of the finding and of the finding it carries (a taken-over reviewer too), and which question is the oldest.
- A decision in the review loop (line 5): `acts::a_verdict_keeps_what_a_decision_settled_as_the_tool_wrote_it` covers the `Decided:` line kept, changed, kept as `Now: kept`, removed, duplicated, claimed by a reviewer with no decision, and carried again a round later; `acts::submit_needs_each_open_finding_answered_and_each_question_decided` covers the Response rule. The escalation scenario shows the carried finding, the brief's question and answer, `verdict` refused for a changed line, then approve and merge.
- Scenarios (line 6): rebuttal withdrawn, `decisions::a_rebuttal_the_next_reviewer_withdraws_closes_its_finding`; rebuttal kept, `scenarios::later_rounds_carry_each_open_finding_under_its_latest_name_until_it_closes`, extended to check the one-try line and the `escalate` exit; the other two named above.
- The lead (lines 7 and 8): `decisions::a_missions_lead_decides_its_items_questions_and_sees_each_and_since_when_one_is_approved` covers the lead's handoff, its `status` and `show <mission>` listing the question with its command, the lead deciding, and "approved since" in its `status` and `show`. `merge::an_approved_item_merges_in_one_commit_that_marks_it_done` checks the implementer's `status`.
- The act table (line 9): `acts::every_act_in_the_table_from_every_status_sets_the_specified_status_or_is_refused`.

Each new test was seen to fail for its reason: two batches of deliberate breaks (the finding chain cut to one review, the `Decided:` guard removed, the Response rule reverted, the rebuttal flag forced off, `escalate`'s open check removed, decided findings not carried, the lead ignored, the branch check removed, no approval time) each failed the tests above and no others.

Review round 1's finding atelier-rop0 (fix now: a question `escalate` wrote was invisible to the mission's lead until committed) is fixed in 3ca83c17: while the question is uncommitted, the lead handoff gives the Git command that commits it, and `decisions::a_missions_lead_decides_its_items_questions_and_sees_each_and_since_when_one_is_approved` now checks the lead does not see it before that command runs and does after, with no commit of the test's own. The full suite passes again (179 tests), with clippy and fmt clean.

Review round 2's finding atelier-lsoh (fix now: away from its branch, the implementer's `status` showed neither a waiting question nor since when it was approved) is fixed in the commit that follows this Report change: the "You hold" list gives each held item's unanswered questions, as "A question waits on a decider", and "approved since <time>". The lead scenario test now checks both from a base worktree, and fails without the fix. 179 tests pass, clippy and fmt clean.

Master merged in (bb26d23a), bringing atelier-pz1y (`check` and the hook rules) and atelier-dxrj (`list --deferred`, `new --from`, `drop` on a finding). Conflicts were all both-sides additions: the import lists in `acts.rs`, `tests/acts.rs` and `output.rs`, the act table's length (20 rows) and its unit test's table (15 item acts), the `unmet` arm for `Escalate` beside `Settle`, `show`'s item path (the approval time and the settlement read from the checkout), a unit test in `body.rs`, and the architecture specification's list of the table's rows; its `last_changed` line is master's. Nothing was built twice: dxrj's `is_deferred` already counts my `Decided: defer` carrier as the deferred finding and the finding it carries as not. `decisions::a_finding_deferred_by_a_decision_is_listed_under_its_carriers_name_and_settled_once` drives a finding `decide --defer` settles through merge, `list --deferred` under the round-2 carrier's name, and `new --from` in one run and `drop` in another, with `atelier check` passing after `escalate`, `decide`, `review` and the merge; it fails when `is_deferred` lists a carried finding. 200 tests pass.

### Decisions

- Who may decide counts the reviewers of every review in the finding's `Was:` chain, not only the review that raised the open name: a finding carried as kept is the same problem, and the product specification says a reviewer never decides its review's findings. The architecture specification's `decide` row now says so.
- A decision answered with no outcome leaves its finding open, so `submit` asks for a Response to it again; only an unanswered question stands in for one. The specification said "a line in Responses or a decision".
- `verdict` also refuses a `Decided:` line no decision wrote, a second carrier of a settled finding, and a later round carrying it again, so a reviewer can neither reopen a decision nor decide a finding itself. A plan review's `Decided:` line is refused the same way.
- The decider's task quotes the answer in single quotes: a double-quoted `"<answer>"` inside a task pasted into a double-quoted shell argument would end the quote and make `<answer>` a redirection.
- "Approved since" is `git log -S'verdict: approve'` on the review file at the branch's tip, the commit that set the verdict.
- `Entry` gains `approved` and `questions`, which the CLI fills only where a view shows them (the implementer's and the lead's `status`, `show`), so a broken Decisions section in some other item cannot fail `status` on the base.

### Validation

The item's third scenario, driven by hand with `next/target/debug/atelier` in a scratch repository (setup and quiet steps left out; each line indented two spaces):

```text
  $ T3CODE_THREAD_ID=r1 atelier verdict changes
  Recorded your verdict on demo-ab12, round 1: changes requested. Committed .atelier/items/demo-ab12/reviews/1-qudt.md with it.
  demo-ab12 is back with its implementer, t1, to answer your open findings. Your review is done: say so to whoever started you, and stop.
  [exit 0]
  $ T3CODE_THREAD_ID=t1 atelier escalate --finding 1-qudt.1 Is\ the\ period\ worth\ a\ round\?\ The\ Goal\'s\ period\ ends\ its\ sentence.
  Asked under `## Decisions` in .atelier/items/demo-ab12/item.md, and did not commit it:
    2026-11-10: Is the period worth a round? The Goal's period ends its sentence.
  It is about finding 1-qudt.1.
  `atelier submit` and `atelier merge` wait until it is answered, and `atelier submit` commits it with the item.
  A decider must be another thread: not you, and not a subagent of yours, which runs as your thread. Start one with your harness's tool for a new agent thread, and give it this task:
    Run atelier show demo-ab12, and read the question waiting under its Decisions and finding 1-qudt.1 in .atelier/items/demo-ab12/reviews/1-qudt.md. Answer it with atelier decide demo-ab12 '<your answer>', adding --defer to defer the finding or --no-action to close it without a fix, or neither to leave it to be fixed.
  Then wait for its answer, and run `atelier status`.
  [exit 0]
  $ T3CODE_THREAD_ID=t1 atelier status
  You are the implementer of demo-ab12 (thread t1).
  Item: .atelier/items/demo-ab12/item.md
  Title: Say hello
  Status: changes requested
  To return it to review:
    [ ] Question answered: 2026-11-10: Is the period worth a round? The Goal's period ends its sentence.
  A decider must be another thread: not you, and not a subagent of yours, which runs as your thread. Start one with your harness's tool for a new agent thread, and give it this task:
    Run atelier show demo-ab12, and read the question waiting under its Decisions and finding 1-qudt.1 in .atelier/items/demo-ab12/reviews/1-qudt.md. Answer it with atelier decide demo-ab12 '<your answer>', adding --defer to defer the finding or --no-action to close it without a fix, or neither to leave it to be fixed.
  Then wait for its answer, and run `atelier status`.
  `atelier submit` commits only the item's directory, so commit your change first.
  Then: atelier submit
  [exit 0]
  $ T3CODE_THREAD_ID=t1 atelier submit
  `atelier submit` refused; nothing changed.
  To return it to review:
    [ ] Question answered: 2026-11-10: Is the period worth a round? The Goal's period ends its sentence.
  A decider answers each question with `atelier decide demo-ab12`: a thread that never implemented this item nor reviewed the finding a question is about. `atelier status` says who.
  `atelier submit` commits only the item's directory, so commit your change first.
  Then: atelier submit
  [exit 1]
  $ T3CODE_THREAD_ID=t1 atelier decide demo-ab12 No.
  `atelier decide demo-ab12` refused; nothing changed.
  You (thread t1) implemented demo-ab12, and none of its implementers, past or present, decides its questions.
  A decider must be another thread: not you, and not a subagent of yours, which runs as your thread. Start one with your harness's tool for a new agent thread, and give it this task:
    Run atelier show demo-ab12, and read the question waiting under its Decisions and finding 1-qudt.1 in .atelier/items/demo-ab12/reviews/1-qudt.md. Answer it with atelier decide demo-ab12 '<your answer>', adding --defer to defer the finding or --no-action to close it without a fix, or neither to leave it to be fixed.
  Then wait for its answer, and run `atelier status`.
  [exit 1]
  $ T3CODE_THREAD_ID=r1 atelier decide demo-ab12 No. --no-action
  `atelier decide demo-ab12` refused; nothing changed.
  You (thread r1) reviewed finding 1-qudt.1, which its question is about, and none of its reviewers decides it.
  Say so to whoever started you, and stop.
  [exit 1]
  $ T3CODE_THREAD_ID=d1 atelier decide demo-ab12 Not\ now:\ a\ later\ item\ covers\ punctuation. --defer
  Answered a question on demo-ab12, as thread d1:
    2026-11-10: Is the period worth a round? The Goal's period ends its sentence.
    Answer: Not now: a later item covers punctuation.
  Finding 1-qudt.1 is settled, `Decided: defer`: the next review carries it so, and no review reopens it.
  Wrote .atelier/items/demo-ab12/item.md, and did not commit it.
  No other question waits. Its implementer, thread t1, runs `atelier submit`, which commits your answer with the item. Your decision is done: say so to whoever started you, and stop.
  [exit 0]
  $ T3CODE_THREAD_ID=t1 atelier status
  You are the implementer of demo-ab12 (thread t1).
  Item: .atelier/items/demo-ab12/item.md
  Title: Say hello
  Status: changes requested
  `atelier submit` commits only the item's directory, so commit your change first.
  Then: atelier submit
  [exit 0]
  $ T3CODE_THREAD_ID=t1 atelier submit
  Submitted demo-ab12 for review.
  A reviewer must be another thread: not you, and not a subagent of yours, which runs as your thread. Start one with your harness's tool for a new agent thread, and give it this task:
    Run atelier review demo-ab12 and follow what it prints.
  Then wait for its verdict, and run `atelier status`.
  [exit 0]
  Open findings from earlier rounds:
  Your review file carries each as a finding of its own, with the author's response. Judge it, then set the finding's `Now:` line to `fixed`, to `withdrawn` (the rebuttal holds, or the finding was wrong), or leave it `kept` (it still stands, and stays a fix-now finding).
  One a decision settled has a `Decided:` line instead, which the tool wrote: leave it as it is.

  1-qudt.1, carried as Finding 1: No period
    As 1-qudt.1:
      Recommendation: fix now
      hello.txt lacks the period.
    Decided: defer. The question: 2026-11-10: Is the period worth a round? The Goal's period ends its sentence.
    Answer by d1: Not now: a later item covers punctuation.

  Your review file: .atelier/items/demo-ab12/reviews/2-637c.md
  $ sed -n '/^## Finding/,$p' .atelier/items/demo-ab12/reviews/2-637c.md
  ## Finding 1: No period
  Recommendation: fix now
  Was: 1-qudt.1
  Decided: defer
  <!-- A decision settled it, and the tool wrote `Decided:`. Leave this finding as it is. -->
  $ T3CODE_THREAD_ID=r2 atelier verdict approve
  Recorded your verdict on demo-ab12, round 2: approved. Committed .atelier/items/demo-ab12/reviews/2-637c.md with it.
  demo-ab12 is back with its implementer, t1, to merge. Your review is done: say so to whoever started you, and stop.
  [exit 0]
  $ T3CODE_THREAD_ID=t1 atelier status
  You are the implementer of demo-ab12 (thread t1).
  Item: .atelier/items/demo-ab12/item.md
  Title: Say hello
  Status: approved since 2026-11-10T17:28:12-05:00
  Then: atelier merge
  [exit 0]
  $ T3CODE_THREAD_ID=t1 atelier merge
  Merged demo-ab12 into main with merge commit 67b9451, which marks it done.
  [exit 0]
```
