---
schema: atelier.review
schema_version: 3
round: 2
commit: 9f526318c0a5a74df95dad1c82ccee2602a831de
reviewers:
- thread:delegated-task:command%3Amcp%3Adcee2e12-d0ce-445f-9987-4067a7273df5%3Adelegate-task%3Acc4342d2-d150-465d-9444-8b9d0ba8cde6
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-19T02:55:12.234289255Z
---

## Summary

Complexity: medium. Three fixtures add facts, rubrics and walks within the
existing harness, without a new runner or runtime subsystem.
Value: high. They measure the lead's review and stalled-item decisions and
restore a genuinely conflicting Done when case.
Risk of merging: high. Incorrect grades can silently misstate what the
skill teaches, so the facts and the judge's inputs need direct checks.
The value warrants this bounded change and its model-free walks.

I read the complete change against master and the fixes since round 1,
checked each Done when line, and ran
`bash evals/agent-factory/test.sh target/debug/atelier`. The full harness
and every scenario passed, including 14 done-when-conflict walks, 10
lead-looks-trivial walks and 11 lead-stalled-item walks. `git diff --check`
also passed. Extra fixture runs confirmed that deleting the item record,
committed or uncommitted, fails its unchanged check, and that a linked item
checkout supplies its uncommitted ticks and code to the facts.

I inspected all six saved runs' final replies, result facts and judge
answers, the four regrades, and the currency run's actual review records.
They support the reported pass/fail results and the stated limitation of
both no-skill lead runs: those leads asked for authorization to start
agents before reaching the intended pressure. I did not rerun the models;
these single runs cannot establish a repeatable skill advantage. GitHub's
head was the reviewed commit; its evals job was still running when checked.

All four earlier findings are fixed. No fix-now finding remains. Finding 5
records minor Report inaccuracies that do not affect the results.

## Finding 1: A missing protected file passes the unchanged check
Recommendation: fix now
Was: 1-ylk8.1
Response: fixed in 2e7dec46b
Now: fixed

`changed()` now treats a failed `git show` and a missing checkout copy as
changes. The new sample-deletion walks pass. I also deleted the item record
in the checkout and in a commit; both grades failed `done_when_changed_in`.

## Finding 2: An unrelated human question settles the currency item
Recommendation: fix now
Was: 1-ylk8.2
Response: fixed in 2e7dec46b
Now: fixed

The facts now include the candidate questions' titles and bodies. The new
graded `currency-raised` question requires one to raise the currency item's
question. The unrelated-logo walk fails that question while the valid
escalation walk passes. The saved no-skill run's regrade also fails this
question because it filed no such item.

## Finding 3: Uncommitted ticks hide a failing Done when line from the judge
Recommendation: fix now
Was: 1-ylk8.3
Response: fixed in 2e7dec46b
Now: fixed

For an unmerged item, `work_left()` reads its linked checkout and includes
uncommitted code and ticks. Both new walks pass. I also exercised a separate
linked checkout: its changed implementation supplied `lines_holding`, and
its ticks produced the expected failing line in `failing_lines_ticked`.

## Finding 4: The README maps refused-push to the wrong scenario
Recommendation: fix now
Was: 1-ylk8.4
Response: fixed in 2e7dec46b
Now: fixed

The two-column mapping again maps `next-refused-push` to `refused-push`.
The three new rows are under the Scenario/Pressure/Pass header.

## Finding 5: The Report retains stale walk counts and misstates the currency approval round
Recommendation: no action

`.atelier/items/atelier-aq82/item.md:67` says there are 10 walks per case;
this head runs 14, 10 and 11. At line 165, the Report says the saved skill
currency fix passed a fresh round 2. Its actual round-2 review requested
changes for a merge conflict; round 3 approved the integrated fix. These
are minor reporting errors: the six runs exist, the pass/fail results hold,
and currency did receive independent approval before merging. They do not
warrant another implementation round.
