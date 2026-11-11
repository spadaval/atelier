---
created_at: "2026-11-11T02:50:24.936776564+00:00"
id: "atelier-yudj"
issue_type: "task"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-yudj
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-qr5s"
    type: "advances"
  - kind: "issue"
    id: "atelier-t6o6"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T04:15:47.476438357+00:00"
status: "done"
title: "Have the converter write a Done when with no checkbox as one unticked line"
updated_at: "2026-11-11T04:15:47.476438357+00:00"
---

## Goal

Stop the cutover from producing ready items that `submit` would send to review with nothing checked. The new tool counts a Done when with no checkbox line as ticked, and 93 ready items across four of the five repositories have a Done when written as prose. Sumeet's answer of 2026-11-11 to atelier-t6o6, point 4: the converter writes the prose as one unticked line. That changes no rule of the new tool.

## Done when

- [x] `scripts/cutover/convert_records.py`: when an open record's Done when section holds text and no checkbox line (`- [ ]` or `- [x]`, outside fenced code, as the new binary reads it), the converted item's Done when starts with one line, `- [ ] ` followed by the section's prose with each run of whitespace made one space. Where the section holds no fenced code block and no table, the prose is all of its text and that line is the whole section. Where it holds one, the prose is the text before the first fence or table row, and everything from there on follows the line unchanged, byte for byte, so a sketch keeps its layout; atelier-h0nt in this repository is such a record. A section that already has a checkbox line is converted as it is today. A section the converter fills in itself today, such as a decision's, is unchanged.
- [x] `scripts/cutover/test_convert_records.py` covers: one paragraph; several paragraphs; plain list items with no checkbox; a section with one checkbox line and prose beside it, which is left as it is; prose, then a fenced block whose indentation matters, then more prose, where the block and what follows it come through unchanged; and a run of the converter twice giving the same bytes, with `--verify` passing.
- [x] Before writing the rule, the implementer counts the shapes of the checkbox-less sections in scratch clones of the five repositories. The Report gives the count per shape: plain prose, prose with plain list items, prose before a fenced block or a table, and anything else. A section that begins with a fenced block or a table, so that no prose comes before it, or any other shape the rule above would corrupt or leave without a checkbox line, is escalated to the orchestrator with its count and a proposal, instead of a rule invented for it. The Report gives counts only, and nothing of any private repository's content.
- [x] `scripts/cutover/rehearse.sh` passes on fresh clones of all five repositories with a binary built from this branch's `next/`. In each converted clone no draft or ready item has a Done when without a checkbox line, shown by a count before and after the change. In one converted clone, `submit` on a started item that had a prose Done when is refused until its line is ticked.
- [x] The paragraph of `docs/strategies/redesign-architecture.md`, Transition, part 2, that says what the converter writes gains one sentence saying this. Its `last_changed` line is left alone; the orchestrator writes it once.

## Scope

`scripts/cutover/convert_records.py`, its tests, that one paragraph of the architecture specification, and this item's record. No change under `next/` or `crates/`: the new binary's rule that no checkbox counts as ticked stays as it is. The four product repositories under `/home/supadava/projects/` are read-only and some are private: work only on scratch clones under `$HOME/.cache/atelier-qr5s/yudj/`, commit nothing from them here, send nothing from them to any outside service, and report process facts only.

## Context

Mission atelier-qr5s. The converter and its `--verify` mode are described in `docs/strategies/redesign-architecture.md`, Transition, part 2; the rehearsal is `scripts/cutover/rehearse.sh`, which makes the clones, converts them and runs the new binary on each in about 36 seconds, and needs `next/` built first (`cargo build -j 4` in `next/`). The step F rollback trials and the cutover run the converter this item changes.

Expected size: about 30 lines of Python and 60 of tests.

## Notes

- 2026-11-10 [note] Handoff: approved by GPT-6.1 Sol (atelier-pgpn) at 4cd92436 in one round with no findings; proof and shape counts are in the Report. No escalations.

## Report

The converter now gives an open record's Done when that holds text and no checkbox line one unticked line of its prose, with whitespace runs made one space, ahead of any fenced block or table, which follow byte for byte (`checkbox_done_when` in `scripts/cutover/convert_records.py`). Its checkbox rule is the new binary's (`done_when` in `next/crates/atelier-records/src/body.rs`), read outside fences by the same fence rule. Sections with a checkbox line, empty sections and sections the converter fills in itself are unchanged. Proof, all run after the last code change:

- Tests: `TMPDIR=$HOME/.cache/atelier-qr5s/tmp/yudj python3 -m unittest discover scripts/cutover` runs 16 tests, all passing with none skipped (the new binary was in `next/target/debug/atelier`). The two new tests, `test_a_done_when_with_no_checkbox_becomes_one_unticked_line` and `test_checkbox_done_when_reads_checkbox_lines_as_the_new_binary_does`, cover one paragraph, several paragraphs, plain list items, a checkbox line with prose beside it, prose then an indentation-sensitive fence then more prose, prose then a table, the filled-in sections, and two runs giving the same bytes with `--verify` passing. Both fail on `master`'s converter.
- Shapes, counted with the old records read by the converter's own parser in scratch clones of the five repositories, open non-mission records only (counts only):

  | Shape of the Done when | draft | ready |
  |---|---|---|
  | has a checkbox line (unchanged) | 150 | 67 |
  | plain prose | 162 | 69 |
  | prose with plain list items | 34 | 24 |
  | prose before a fenced block or table | 1 | 0 |
  | starts with a fenced block or table | 0 | 0 |
  | no section (decision filler) | 0 | 9 |
  | empty section, or anything else | 0 | 0 |

  None of the 290 rewritten sections holds a heading, link reference definition, blockquote, HTML, hard line break or indented code, which collapsing to one line would break, so nothing was escalated. The one prose-before-fence record is atelier-h0nt.
- Before and after, draft and ready items with no checkbox line after converting each clone with `master`'s converter and with this branch's: astromech 129 and 59 before, atelier 42 and 1, cosplay-studio 7 and 29, demiurge 17 and 0, life-on-rails 2 and 4 (197 draft, 93 ready); 0 in every repository after, and 0 in the rehearsal's own clones at its end.
- Rehearsal: `scripts/cutover/rehearse.sh` on a new directory, with the prebuilt `next/` binary from master at 3ba4a62f (this branch changes nothing under `next/`) copied to `next/target/debug/atelier`, passed every step on all five repositories in 34 s, 12 gh calls all to the double.
- Refused submit: in the rehearsal's atelier clone, atelier-3km4 (ready, prose Done when) converted to one `- [ ]` line. After `atelier start`, a commit and a Report, `atelier submit` exited 1 with "`atelier submit` refused; nothing changed" listing the unticked Done when line; after ticking it, `submit` exited 0.
- The architecture specification's Transition, part 2, gains one sentence; its `last_changed` line is untouched.
