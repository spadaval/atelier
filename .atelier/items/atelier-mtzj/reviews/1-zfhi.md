---
schema: atelier.review
schema_version: 3
round: 1
commit: 70421b94c4cdcd9832b107c46c735fdb5214cc27
reviewers:
- thread:delegated-task:command%3Amcp%3A4e6430b6-baa9-4aee-82a6-523fdb40596f%3Adelegate-task%3A8a5448ee-c01d-43ef-8691-d163f314cde9
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-18T23:59:17.010527960Z
---

## Summary

- **Complexity:** low. The change updates policy text and CLI messages, with
  one new integration test and no new state or command.
- **Value:** high. It implements the human's decision about who is responsible
  for finishing a mission and resolving a kept finding.
- **Risk of merging it:** medium. Agents in other repositories use this skill
  to decide when to bypass review and how to record that decision.

The value warrants the size. I reviewed the full diff at
70421b94c4cdcd9832b107c46c735fdb5214cc27 against master, checked each Done
when line and the Report, and read the relevant merge, skip, record-write,
drop and dispute paths. The started-item drop instructions agree with the
binary, and the reported word counts reproduce: 1,231 before and 1,508
after. The new test demonstrates a lead merging past a kept fix-now finding
and checks the reason in `atelier log`; its existing attribution limitation
is explicitly assigned to atelier-nv22.

Validation: `cargo nextest run -p atelier-cli -p atelier-core` passed all 325
run tests (one skipped); `cargo build -p atelier-cli` and
`evals/agent-factory/test.sh` passed; `cargo fmt -- --check` passed. The
non-record diff passes `git diff --check`. The item record has an extra
blank line at EOF, outside that whitespace check. I did not run live model
evals; the fixture tests do not establish how agents behave under the new
policy.

## Finding 1: Preserve reasons for departures on commands that make no commit
Recommendation: fix now

`skills/agent-factory/roles/lead.md:17-20` promises that putting
`ATELIER_SKIP` before the command keeps the reason in the commit and
`atelier log`, and only requires a Report entry where no rule refuses.
That leaves a third case uncovered: a command passes a rule but makes no
commit. `ready`, `set`, `close` and `verdict` can do this. Their later plain
Git commit does not inherit the earlier command's skip or add its trailer.

This is established by `crates/atelier-core/src/plan.rs:322-384`, which
prints an ordinary Git commit command without the skip, and by the passing
`crates/atelier-cli/tests/skip.rs` tests: lines 95-105 commit a skipped
verdict with the reason only in the transcript, and lines 151-165
explicitly check that a skipped `ready` writes its skip nowhere. A lead
following the new instructions can therefore make a departure whose reason
never reaches the durable record the instructions promise.

Keep this within the text change if desired: distinguish commands that
commit their skips from commands that only write files, and require a
Report or mission-notes entry for the latter as well as for departures
that meet no binary refusal. Alternatively preserve their reasons in the
subsequent commit, with a test. The current fallback must not depend solely
on whether a rule refused.

## Finding 2: Align the product principles with the lead's new authority
Recommendation: fix now

The policy update in `docs/product.md:479-495` leaves the document's
principles at lines 51-55 saying both that no thread holds the lead and
implementer roles on the same work and that independent review is
"not negotiable". Those are the exact departures this change now authorizes:
`roles/lead.md:17-20` permits taking an item over with a recorded reason and
explicitly includes skipping review. The new design decision also says
that an unreviewed merge is within the lead's authority.

The Report correctly changed two other product passages to avoid
contradicting the decision, but these remaining principles still tell a
reader of the product policy to reject the newly supported behavior. State
role separation and independent review as defaults with the lead's
recorded exception, consistently with the skill and the dated decision.
