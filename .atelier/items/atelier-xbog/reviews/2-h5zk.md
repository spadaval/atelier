---
schema: atelier.review
schema_version: 3
round: 2
commit: ace0ea7176e045eda2e0fa141bde41e00491848d
reviewers:
- thread:delegated-task:command%3Amcp%3A14e32e6f-5f3e-4abc-a646-e81d4882ba09%3Adelegate-task%3A9b8cf329-a9b8-42e1-b84c-af3adb149fbb
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-13T16:59:07.521949650Z
---

## Summary

Complexity: medium. The removal spans four crates and adds dispute views,
but adds no stored state or migration.

Value: high. It removes the unused question workflow and makes existing
reviewer judgments visible through the CLI.

Risk of merging: medium. Review gates and record parsing affect other
repositories, but failures are visible and the change is reversible.

The simplification is worth its size. I found no new problem requiring a
change, and the carried finding is fixed.

Reviewed `master...ace0ea7176e045eda2e0fa141bde41e00491848d` and the diff
since round 1. Checked command removal, submit and merge conditions,
carried findings and verdict guards, legacy Decisions and Decided parsing,
dispute collection and mission filtering, CLI output, and the product and
design documents against the Done when lines. The Desk APIs left for
atelier-v958 are outside this item's removal; their Rust tests still pass.

Ran `cargo nextest run --workspace`: all 306 tests passed. Also passed
`cargo clippy --all-targets -- -D warnings`, `cargo fmt -- --check`, and
`git diff --check master...ace0ea7176e045eda2e0fa141bde41e00491848d`.
PR #155's CI test check passed on the reviewed head; the Desk job was
skipped. I did not run browser or Desk e2e tests.

Independently searched 1,075 local and remote refs across the 11 configured
checkouts under `/home/supadava/projects`. No item or review held a
Decisions section or Decided line. The Report correctly distinguishes
those checkouts from the twelve directories with `.atelier/`. With no real
legacy record available, compatibility is covered by the synthetic
old-format tests, which passed.

Also exercised the built CLI in a scratch repository: a round-2 carrier
kept at defer with only the multiline template comment refused approval
with the expected `says why` error and left HEAD unchanged. Adding prose
after the closed comment allowed approval, and `list --disputed` reported
the finding as kept at defer.

## Finding 1: Exclude multiline HTML comments from a lowering's reason
Recommendation: fix now
Was: 1-aj9b.1
Response: fixed in 42bc22e13
Now: fixed

`reasoned` now removes complete HTML comments, including multiline ones,
before looking for prose. An unclosed comment consumes the remaining text.
The regression test covers comment-only carriers, an unclosed comment, and
prose after a closed multiline comment. The independent CLI check above
confirms both refusal without a reason and approval with one.
