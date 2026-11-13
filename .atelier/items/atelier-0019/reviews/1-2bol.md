---
schema: atelier.review
schema_version: 3
round: 1
commit: 164adb287739e3f2c409031c50488c6be601683e
reviewers:
- thread:delegated-task:command%3Amcp%3A2560d28f-3fa1-4403-8714-38800f2bcecc%3Adelegate-task%3A16067166-649c-4792-a83b-a506b75ffdee
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-11T18:05:07.616785605Z
---

## Summary

Complexity: low. This is one garden report with supporting workings, two documentation repairs and two deferred-finding settlements; it adds no runtime behavior.

Value: medium. It repairs instructions agents use now and gives the first post-cutover garden session counted evidence.

Risk of merging it: low. An inaccurate readiness result can mislead that session, but these documentation and record changes are easy to correct.

The value warrants the size. The upkeep is narrow and the proposals remain proposals. Correct the readiness result below before treating this as the first measured page.

I reviewed `master...164adb287739e3f2c409031c50488c6be601683e`, every Done when line, the Report and both artifacts against garden.md and readiness.md. Independently selected the seven merged items from `atelier log`, compared their copied log lines and recomputed status seconds, rounded minutes, medians, rounds and the skip. The reported timings match. Reconstructed the measured base's open records with this run's branch start overlaid: 106 draft, 22 ready and 1 building; excluding completed blockers reproduces 5 draft and 3 ready blocked items. Confirmed all 39 non-dropped bug-labelled items lack a cause, and checked the carried values against the dated baseline.

Read the window's reviews, the source readers behind P1, the merge/CI configuration behind P2, both new defect records behind P3 and all ten atelier-j86z entries. Read the three navigation sample threads through T3 and checked their first-edit boundaries and file lists. Verified the repaired package paths and design anchors. Both deferred findings are settled, `atelier list --deferred` reports none left, and the page is 48 lines with exactly three proposals and an empty Session. No changed rule, check, gate, interface or default was found.

`atelier check` and `git diff --check master...164adb287739e3f2c409031c50488c6be601683e` pass. GitHub confirms the base CI run cited by the checklist passed and the reviewed PR's CI passed (Desk and changes passed; Rust skipped). I did not repeat the reported scratch-clone builds, clippy, unit tests or browser tests; their local timings and totals remain the author's validation. I reproduced the documentation failure below without a build. I changed only this review file and started no agents.

## Finding 1: Distinguish the failed base build instructions from their repair
Recommendation: fix now

`.atelier/items/atelier-0019/artifacts/checklist.md:10` marks readiness line 4 met, under an explicit claim that the checklist was checked on master at `87ee3ccd0`. Its evidence lists the CLI installation and the frontend build, which do not build the Desk's Rust server. At that measured base, `desk/README.md:33` tells the reader to run `cd ../next && cargo build -p atelier-desk`, and `docs/architecture/desk.md:84` says `cd next && cargo build -p atelier-desk`. The `next/` directory does not exist in that tree. I checked it with `git ls-tree 87ee3ccd0 next` and reproduced the README command's directory change failing with `No such file or directory`.

Readiness line 4 specifically requires a fresh clone to build using the documented commands alone, including those the map links. This base therefore does not meet that line, even though the branch correctly repairs both commands. Reporting 5 met and 4 not met in `item.md:62` currently turns a repaired failure into an unsupported pass for the measured base.

Record line 4 as not met on the measured base, name the upkeep commit as its repair and update the page's checklist counts. If also reporting a passing result after the repair, distinguish that later tree and verify the corrected Desk server build instead of using the CLI installation and frontend build as its evidence. No additional proposal or implementation change is needed.
