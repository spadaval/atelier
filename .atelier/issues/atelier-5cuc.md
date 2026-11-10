---
created_at: "2026-11-10T00:56:07.002116482+00:00"
id: "atelier-5cuc"
issue_type: "task"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-5cuc
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-engh"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Add `review` and `verdict`, with the brief and one file per review"
updated_at: "2026-11-10T16:50:20.825982654+00:00"
---

## Goal

Build the reviewer's side of the loop: one command that gives a reviewer everything it needs and a file to write in, and one that records its verdict. Each submission opens a new round, and a dead reviewer can be replaced.

## Done when

- [x] Scenario test: on an item in review, thread `t2` runs `atelier review <id>` with the new binary. It creates `reviews/1-<suffix>.md` with the head commit, `t2` as reviewer and the template, and prints a brief holding the item's Goal, Done when, Scope and Report, the command that shows the diff, the path of the review file and the two `verdict` commands. Run again by `t2`, it creates nothing and prints the same brief.
- [x] A thread anywhere in the item's `implementers`, current or past, that runs `review <id>` gets the brief and a line saying an author does not review its own work. No file is created and no reviewer is changed. A test covers a thread that implemented the item and was then taken over.
- [x] `verdict changes` on a file with an open fix-now finding sets `changes_requested` and commits the review file in one commit; on a file with none it is refused. `verdict approve` on a file with an open fix-now finding is refused; otherwise it sets `approved`.
- [x] After `changes_requested`, the implementer's `status` lists each open fix-now finding by number and title. `submit` changes nothing until the Report's Responses has a line for each one, as fixed in a commit or as rebutted.
- [x] Scenario test, second round: after that `submit`, `review <id>` by a thread creates `reviews/2-<suffix>.md`, which carries each open finding of round 1 by its name, such as `1-x7qd.1`, with the author's response under `Was:`. The reviewer sets `Now:` to fixed, withdrawn or kept, and a kept fix-now finding counts as open for the verdict. The same holds after an approval went stale and the item was submitted again.
- [x] Scenario test, reviewer taken over: with round 1's file created by `t2` and no verdict, `review <id>` by `t3` appends `t3` to the file's `reviewers` so that it holds the review, keeps what the file holds, and says `t2` held it.
- [x] A thread bound as reviewer that runs `submit` gets one line naming its role and its two commands, a non-zero exit, and no change.
- [x] The Records section of `docs/strategies/redesign-architecture.md` matches the review file as built.

## Scope

Reviews of items. Plan reviews of missions belong to step B, and findings settled by a decision to step C. The rule that a rebuttal goes to a fresh reviewer is the skill's to state; this item does not enforce it.

## Report

Every Done when line holds, each proved by a test that failed under a mutation of the code it covers (17 mutations, each caught by its test). In `next/crates/atelier-cli/tests/scenarios.rs`: `review_opens_the_rounds_file_and_prints_the_brief_and_run_again_only_the_brief` (the file's round, head commit, `t2`, `model: null`, `verdict: null` and template; the brief's Goal, Done when, Scope, Report, `git diff main...<head>`, file path and both verdict commands; a second run prints the same bytes and changes no file; `status` speaks to t2 as reviewer); `an_author_past_or_present_is_shown_the_brief_and_opens_no_review` (t1 started the item, t9 took it over and submitted; both get the author line and the brief, exit 1, no file, and with t2's file open its reviewers stay `t2`); `a_verdict_needs_its_finding_state_and_commits_the_file_with_the_status` (changes refused with no finding and approve with one, each leaving HEAD and the file alone; then one commit holding exactly `item.md` and the review file, `changes_requested`, `verdict: changes`, the model; the implementer's `status` lists `<stem>.1 (fix now): No period` and not the deferred one, and `submit` refuses until it is answered); `later_rounds_carry_each_open_finding_under_its_latest_name_until_it_closes` (round 2 carries `1-x.1` with the rebuttal under `Was:`; kept, approve is refused and changes taken; the open finding is then named `2-y.1`, and round 3 carries `2-y.1` with both earlier texts in the brief; fixed, approve sets `approved`; the approval goes stale, `submit` opens round 4 with nothing carried, and a fix-now finding there blocks approve); `a_new_reviewer_takes_over_a_review_with_no_verdict_and_keeps_its_file` (`t2 t3`, the file otherwise byte for byte, "t2 held it", t2's verdict refused and t3's committed); `a_reviewer_that_runs_an_implementers_command_is_told_its_role` (`submit` and `start` by t2 print exactly the one role line, exit 1, nothing changed; `verdict` by t1 prints the implementer's); `review_runs_on_an_item_in_review_in_the_checkout_on_its_branch`; `a_refused_commit_leaves_the_verdict_to_be_run_again` (both files, HEAD and unrelated staged work as before, then the verdict succeeds). In `next/crates/atelier-app/tests/acts.rs`: the act table from every status with verdicts run by the round's reviewer, `status`'s conditions equal to `check`'s across four sets of reviews, the round rule, the role refusals, verdict conditions across rounds (kept, withdrawn, not carried, an invalid `Was:`), and what a round carries with each text and response. In `atelier-records`: `model: null` round-trips, and `section` and `finding_text` read what was written. The Records, Status, Threads and "Where records are read and written" sections of `docs/strategies/redesign-architecture.md` describe what was built. `cargo fmt -- --check`, `cargo clippy --all-targets --locked -- -D warnings` and the 78 tests of `next/` pass; the CI job's `next` steps pass as written in a fresh clone with `crates/` deleted; `git diff --stat master...HEAD -- crates/` is empty. Non-test Rust in `next/` went from 2,914 to 3,542 lines.

### Responses

- atelier-5edt: fixed in b2d8d01c. `submit`, `start` and the other refusals that only name a role or a holder no longer build the status entry, which parsed the reviewer's draft; `a_reviewer_that_runs_an_implementers_command_is_told_its_role` now leaves a half-written finding in the file first, and fails if the entry is built again.

### Decisions

See the decision notes of 2026-11-10: how `model` is set (`verdict --model`), `review` committing nothing, a round ending at its first verdict, approve counting every open finding of the item, carried findings starting kept, and the role line covering every implementer act.

### Surprises

- A shared sccache server on this host was started with an earlier item's `TMPDIR`, which is gone, so a cold build through it fails ("Failed to create temp dir"). The CI-steps run here set `RUSTC_WRAPPER=` to bypass it; the server was left alone.

### Open items

- The brief omits two things the product specification lists: decisions on findings (step C builds `decide`) and the latest merge check results (`merge` arrives in atelier-syyj).
- A former reviewer, after its verdict, is told by `status` (on the item's branch) and by a refused `review` to run `atelier start <id>`, since `acts::next` names the take-over for anyone who is neither holder nor eligible reviewer. The verdict's output tells the reviewer to stop, so the fresh-agent run should not reach it, but atelier-67nn may want different wording there.
- `status` on an item in review does not name who holds the round's review; the implementer sees only "Waiting for a reviewer's verdict".

### Transcript

The final binary in a scratch repository, two rounds:

    $ T3CODE_THREAD_ID=t1 atelier submit
    Submitted demo-ab12 for review.
    Start a reviewer with: Run `atelier review demo-ab12` and follow what it prints.
    [exit 0]
    $ T3CODE_THREAD_ID=t2 atelier review demo-ab12
    You are the reviewer of demo-ab12 (thread t2), round 1.
    Review the change on branch demo-ab12 as someone who did not write it: check that it meets each Done when line, that it is correct, and that the Report's claims hold. Read the code, run what proves it, and write down each problem you find. The author's reasoning is left out on purpose; judge what is there.

    Item demo-ab12: Say hello

    ## Goal

    Print hello.

    ## Done when

    - [x] `hello.txt` says hello.

    ## Scope

    Only the greeting.

    ## Report

    hello.txt says hello.

    The change, at commit 6d232ae126c860f638c9f5ed69a04b517708b94b:
      git diff main...6d232ae126c860f638c9f5ed69a04b517708b94b

    Your review file: .atelier/items/demo-ab12/reviews/1-ivq1.md
    Write your review there, and change no other file. Leave its front matter, between the `---` lines, as it is.
    Under `## Summary`, say what you checked and how, and what you could not check. Then add a section for each problem, numbered on from 1:

    ## Finding 1: <a one-line title>
    Recommendation: fix now
    <What is wrong, where (file and line), and how you know.>

    The Recommendation is `fix now` (it must be fixed before the change merges), `defer` (real, but for later work) or `no action` (a note; nothing to do).

    When the file is written, record your verdict. It checks the file, then commits it with the item's new status:
      atelier verdict changes --model <your model>   if the file holds an open fix-now finding
      atelier verdict approve --model <your model>   if it holds none
    Give the name of the model you run on, such as gpt-6.1-sol, or leave out `--model` if you do not know it. Your review is then done.
    [exit 0]
    $ (t2 writes a finding into .atelier/items/demo-ab12/reviews/1-ivq1.md)
    $ T3CODE_THREAD_ID=t2 atelier verdict changes --model gpt-6.1-sol
    Recorded your verdict on demo-ab12, round 1: changes requested. Committed .atelier/items/demo-ab12/reviews/1-ivq1.md with it.
    demo-ab12 is back with its implementer, t1, to answer your open findings. Your review is done: say so to whoever started you, and stop.
    [exit 0]
    $ T3CODE_THREAD_ID=t1 atelier status
    You are the implementer of demo-ab12 (thread t1).
    Item: .atelier/items/demo-ab12/item.md
    Title: Say hello
    Status: changes requested
    To return it to review:
      [ ] Finding 1-ivq1.1 (fix now): No period
    Answer each finding under `### Responses` in the Report: `<finding>: fixed in <commit>` or `<finding>: rebutted: <fact>`.
    Then: atelier submit
    [exit 0]
    $ (t1 adds the period, commits it, and answers the finding under ### Responses)
    $ T3CODE_THREAD_ID=t1 atelier submit
    Submitted demo-ab12 for review.
    Start a reviewer with: Run `atelier review demo-ab12` and follow what it prints.
    [exit 0]
    $ T3CODE_THREAD_ID=t3 atelier review demo-ab12
    You are the reviewer of demo-ab12 (thread t3), round 2.
    Review the change on branch demo-ab12 as someone who did not write it: check that it meets each Done when line, that it is correct, and that the Report's claims hold. Read the code, run what proves it, and write down each problem you find. The author's reasoning is left out on purpose; judge what is there.

    Item demo-ab12: Say hello

    ## Goal

    Print hello.

    ## Done when

    - [x] `hello.txt` says hello.

    ## Scope

    Only the greeting.

    ## Report

    hello.txt says hello.

    ### Responses

    - 1-ivq1.1: fixed in c14bbc9

    The change, at commit ef21054f6d0864c70fef18bcf7ebb2f554203979:
      git diff main...ef21054f6d0864c70fef18bcf7ebb2f554203979
    What changed since the last round's review:
      git diff 6d232ae126c860f638c9f5ed69a04b517708b94b ef21054f6d0864c70fef18bcf7ebb2f554203979

    Open findings from earlier rounds:
    Your review file carries each as a finding of its own, with the author's response. Judge the response, then set the finding's `Now:` line to `fixed`, to `withdrawn` (the rebuttal holds, or the finding was wrong), or leave it `kept` (it still stands, and stays a fix-now finding).

    1-ivq1.1, carried as Finding 1: No period
      As 1-ivq1.1:
        Recommendation: fix now
        hello.txt says `hello`; the Goal ends with a period.
      The author's response: fixed in c14bbc9

    Your review file: .atelier/items/demo-ab12/reviews/2-ytyi.md
    Write your review there, and change no other file. Leave its front matter, between the `---` lines, as it is.
    Under `## Summary`, say what you checked and how, and what you could not check. Then add a section for each problem, numbered on from 2:

    ## Finding 2: <a one-line title>
    Recommendation: fix now
    <What is wrong, where (file and line), and how you know.>

    The Recommendation is `fix now` (it must be fixed before the change merges), `defer` (real, but for later work) or `no action` (a note; nothing to do).

    When the file is written, record your verdict. It checks the file, then commits it with the item's new status:
      atelier verdict changes --model <your model>   if the file holds an open fix-now finding
      atelier verdict approve --model <your model>   if it holds none
    Give the name of the model you run on, such as gpt-6.1-sol, or leave out `--model` if you do not know it. Your review is then done.
    [exit 0]
    $ (t3 sets Now: fixed in .atelier/items/demo-ab12/reviews/2-ytyi.md)
    $ T3CODE_THREAD_ID=t3 atelier verdict approve --model gpt-6.1-sol
    Recorded your verdict on demo-ab12, round 2: approved. Committed .atelier/items/demo-ab12/reviews/2-ytyi.md with it.
    demo-ab12 is back with its implementer, t1, to merge. Your review is done: say so to whoever started you, and stop.
    [exit 0]
    $ T3CODE_THREAD_ID=t1 atelier status
    You are the implementer of demo-ab12 (thread t1).
    Item: .atelier/items/demo-ab12/item.md
    Title: Say hello
    Status: approved
    Then: atelier merge
    [exit 0]
    $ git log --oneline
    ccc2878 Review demo-ab12, round 2: approved
    ef21054 Submit demo-ab12 for review
    c14bbc9 Add the period
    fcc55ab Review demo-ab12, round 1: changes requested
    6d232ae Submit demo-ab12 for review
    e3b2f38 Say hello
    cb188a6 Start demo-ab12
    e9cd204 init
    $ cat .atelier/items/demo-ab12/reviews/2-ytyi.md
    ---
    schema: atelier.review
    schema_version: 3
    round: 2
    commit: ef21054f6d0864c70fef18bcf7ebb2f554203979
    reviewers:
    - t3
    model: gpt-6.1-sol
    verdict: approve
    created_at: 2026-11-10T16:50:05.109027940Z
    ---

    ## Summary

    <!-- What you checked and how, and what you could not check. -->

    ## Finding 1: No period
    Recommendation: fix now
    Was: 1-ivq1.1
    Response: fixed in c14bbc9
    Now: fixed
    <!-- Set Now: to fixed, withdrawn or kept. -->

## Context

Behaviour: `docs/strategies/redesign-product.md`, Commands (Reviewer), Review, and "When a thread dies". The reviewer works in the implementer's checkout, where T3 starts a delegated child by default.

## Notes

- 2026-11-10 [decision] The orchestrator narrowed the Done when line on a reviewer's wrong command from `submit` or `merge` to `submit` only
  Reason: `merge` does not exist until atelier-syyj; the orchestrator will tell that item's implementer to cover a reviewer who runs `merge`.
  Rejected: Build a stub merge here to test the reviewer's refusal
- 2026-11-10 [decision] Expected size: about 900 non-test lines of Rust, scenario and unit tests about as large again, and about 40 changed lines of the architecture specification
  Reason: The orchestrator's brief gives this size; past about twice it, the implementer escalates.
- 2026-11-10 [decision] The review file's model is set by an optional `verdict --model <name>`; without it the file says `model: null`
  Reason: Nothing on this host names the model in the environment (T3 and the harnesses set thread and session IDs only), the handoff line must stay as printed, and nobody edits front matter. The brief prints the flag on both verdict commands and says to leave it out when unknown.
  Rejected: A `review <id> --model` flag, which the fixed handoff line never passes
- 2026-11-10 [decision] `review` writes the round's file in the checkout and commits nothing; `verdict` commits it with item.md
  Reason: The act table says verdict commits the review file, no act commits only to change a status, and the Done when line has verdict commit the file in one commit.
  Rejected: Commit the file at `review`, which adds a commit per review and per take-over
- 2026-11-10 [decision] A round of an item ends with the first verdict among its reviews, not when every review has one
  Reason: A verdict moves the item out of review. With the other rule, a second reviewer's file left open in a round (two threads running `review` at once in one checkout) would stay the round's file after the first verdict, and the next submission would never open a new round.
  Rejected: The latest round stays open while any of its reviews has no verdict
- 2026-11-10 [decision] `verdict approve` is refused while any fix-now finding of the item is open, not only the file's own
  Reason: A reviewer that deletes a carried finding from its file would otherwise approve the item over an open finding, which the product's rule that only a reviewer withdraws a finding forbids. With carrying done right the two rules agree.
  Rejected: Count only the file's own findings, as the act table first said
- 2026-11-10 [decision] A carried finding starts as `Now: kept` in the template, followed by a comment naming the three values
  Reason: The template must parse, since every reader of the branch reads it; kept is the safe default, so approve is refused until the reviewer closes it on purpose.
  Rejected: Leave `Now:` blank, which does not parse
- 2026-11-10 [decision] The reviewer's one-line role refusal covers every implementer act (`start`, the take-over, `submit`, `merge`), and the implementer gets the matching line for `verdict`
  Reason: The product specification says a thread that runs another role's command gets one line back; checking the role before the status is what makes `submit` by a reviewer of an item in review say so rather than name the status.
  Rejected: Only `submit`, as the Done when line names
