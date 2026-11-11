---
schema: atelier.review
schema_version: 3
round: 3
commit: 12abb2f382eb77f4d49fca19b169f44e892f7104
reviewers:
- thread:delegated-task:command%3Amcp%3A201bf233-fdc5-4e41-814c-36f3d465a567%3Adelegate-task%3Aca5f8aee-07be-4ea7-bf86-80eed73fa29d
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-11T16:13:59.252005594Z
---

## Summary

Complexity: medium. Two specifications replace several documentation trees,
governing ADR decisions move into one record, and eval dependencies move.
Value: high. The installed tracker needs current guidance instead of pages
that describe commands and records it no longer has.
Risk of merging: low. Documentation and fixture mistakes are visible and
reversible through Git; executable changes only relocate fixture inputs.

The value warrants the size. Most of the diff deletes obsolete material;
a smaller cleanup would retain conflicting guidance.

I reviewed master...12abb2f382eb77f4d49fca19b169f44e892f7104 and the
changes since round 2's reviewed head, ed7221b488c785e57d9b257f9fa736f82cf9c64b.
I compared the moved specifications with their sources on the current merge
base, checked the ADR mapping and retained decisions, inspected entry points
and retained pages, and checked changes outside docs against Scope. The
master merge preserves atelier-ivbq's additions to both specifications, with
the architecture link updated. Words, Lifecycle and Review remain
byte-for-byte unchanged. Both earlier fix-now findings remain fixed. The
ADR 0028 rows retain its no-regrowth and one-binary decisions, with the
completed cutover identified as the exception to reading the old layout.

Validation:

- All ten next-* unit suites passed, 99 tests total, using the installed
  binary as NEXT_ATELIER and ~/.cache as TMPDIR. All ten setup.sh scripts
  separately exited 0 and created repo/ in fresh temporary directories.
- An independent Python scan checked 99 relative Markdown links in 29 files,
  including heading fragments, with 0 broken. It covered docs/, skills/,
  the root guidance files and the additional changed entry points.
- atelier check passed: 133 items, 7 missions and config.toml. cargo fmt --
  --check and git diff --check master...HEAD passed.
- Removed-path searches match the Report's explained exceptions. Searches
  for removed commands and record paths found no instruction to use them;
  other matches were the permitted history and ordinary prose such as
  "review findings". Open missions' mission.md and design.md references
  identify fleet-dashboard.md, retained unchanged in atelier-bnek, and the
  two specifications, now linked at their new paths.
- Both relocated eval inputs and fleet-dashboard.md have identical Git
  objects to their sources. Recorded eval results are unchanged. Rust,
  Cargo and cutover script edits change documentation references only;
  skill text is unchanged.
- Help succeeds for all 18 built commands, including setup; log and init
  are absent. atelier-8b0a is the draft item for log. The cutover's five
  counts and verify reports agree with Transition. Its journal has 85
  starts, 85 matching dones and no repeated step keys.
- PR #143 points at the reviewed head. At the check, CI's Rust test job
  passed and the Desk job was still running. Merge must verify final CI.

I did not rerun the cutover, operate the work host, start live eval agents
or rerun the Rust suite for comment-only Rust changes. The minor reference
cleanup below can follow separately; no blocking finding remains.

## Finding 1: Two Desk API references still name the deleted architecture specification
Recommendation: defer
Dropped by thread:delegated-task:command%3Amcp%3A1e4a355d-d905-4146-a4d8-89e6984f49f1%3Adelegate-task%3A1c7e22f2-4ce0-40e1-a447-156f728619a3: Fixed by the gardener's upkeep commit 94bbfa149 on atelier-0019: both citations now link the design record's sections.

docs/architecture/desk.md:756 and :786 still cite
`redesign-architecture.md`, respectively for The log and Records. This item
deletes that filename while moving those sections into docs/design.md, so
the retained page's prose sends a reader looking for a specification that
no longer exists. Replace the references with links to
`../design.md#the-log` and `../design.md#records`.

This is non-blocking: the page's introduction already links to the new
design record, both sections survive there, and no executable instruction
depends on these two parenthetical citations.
