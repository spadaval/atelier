---
schema: atelier.review
schema_version: 3
round: 1
commit: 119cfa49a7df1e0836c83e007a67c8a7732161ff
reviewers:
- thread:delegated-task:command%3Amcp%3A14e32e6f-5f3e-4abc-a646-e81d4882ba09%3Adelegate-task%3A2ea09431-ca7c-4845-871f-f70192b1241c
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-13T16:54:56.843191512Z
---

## Summary

Complexity: medium. The removal crosses four crates and adds dispute views,
but introduces no stored state.

Value: high. It removes an unused workflow from the shared CLI and gives
rebuttals their existing reviewer path.

Risk of merging: medium. The change affects review gates and record parsing
in other repositories; its failures can be corrected without a migration.

The simplification is worth the change's size. Fix the reason check below
before merging.

Reviewed `master...119cfa49a7df1e0836c83e007a67c8a7732161ff`, including command
removal, submit and merge conditions, carried findings and verdict guards,
legacy Decisions and Decided parsing, dispute collection and output, and
the documentation. Ran `cargo nextest run --workspace`: all 306 tests
passed. `cargo clippy --all-targets -- -D warnings`, `cargo fmt -- --check`
and `git diff --check master...119cfa49` passed.
PR #155's test check passed; its Desk job was skipped. Searched all local
and remote refs of the 11 checkouts under `/home/supadava/projects` with an
Atelier config and found no Decisions or Decided records in their item or
review files. The synthetic legacy tests therefore cover the compatibility
case available here; I could not reproduce the Report's count of twelve
repositories. I did not run browser or Desk e2e tests.

Also drove the built binary in a scratch repository with a round-1 fix-now
finding and a round-2 carrier kept at defer. With only the single-line
template comment, approval correctly failed for lack of a reason. Changing
only that comment to a multiline HTML comment made approval succeed.

## Finding 1: Exclude multiline HTML comments from a lowering's reason
Recommendation: fix now

`crates/atelier-records/src/body.rs:144-147` excludes a comment only when
the same line starts with `<!--` and ends with `-->`. In a multiline
comment, even the opening `<!--` line satisfies `reasoned`, so a carried
finding kept at defer or no action can be approved without the reviewer's
required explanation. For example, this comment alone after `Now: kept`
is enough:

```markdown
<!--
Set Now: to fixed, withdrawn or kept.
-->
```

I reproduced `atelier verdict approve` returning 0 and committing the
approval for that case; the otherwise identical one-line comment returns
2 with the expected `says why` error. This misses the Done when requirement
that lowering a finding needs a reason and the documented rule that HTML
comments do not count. Exclude the whole multiline comment before looking
for prose, and cover comment-only carriers and prose after a closed
multiline comment in a regression test.
