---
created_at: "2026-11-11T02:50:24.936776564+00:00"
id: "atelier-yudj"
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
    id: "atelier-qr5s"
    type: "advances"
  - kind: "issue"
    id: "atelier-t6o6"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Have the converter write a Done when with no checkbox as one unticked line"
updated_at: "2026-11-11T02:51:32.619016180+00:00"
---

## Goal

Stop the cutover from producing ready items that `submit` would send to review with nothing checked. The new tool counts a Done when with no checkbox line as ticked, and 93 ready items across four of the five repositories have a Done when written as prose. Sumeet's answer of 2026-11-11 to atelier-t6o6, point 4: the converter writes the prose as one unticked line. That changes no rule of the new tool.

## Done when

- [ ] `scripts/cutover/convert_records.py`: when an open record's Done when section holds text and no checkbox line (`- [ ]` or `- [x]`, outside fenced code, as the new binary reads it), the converted item's Done when starts with one line, `- [ ] ` followed by the section's prose with each run of whitespace made one space. Where the section holds no fenced code block and no table, the prose is all of its text and that line is the whole section. Where it holds one, the prose is the text before the first fence or table row, and everything from there on follows the line unchanged, byte for byte, so a sketch keeps its layout; atelier-h0nt in this repository is such a record. A section that already has a checkbox line is converted as it is today. A section the converter fills in itself today, such as a decision's, is unchanged.
- [ ] `scripts/cutover/test_convert_records.py` covers: one paragraph; several paragraphs; plain list items with no checkbox; a section with one checkbox line and prose beside it, which is left as it is; prose, then a fenced block whose indentation matters, then more prose, where the block and what follows it come through unchanged; and a run of the converter twice giving the same bytes, with `--verify` passing.
- [ ] Before writing the rule, the implementer counts the shapes of the checkbox-less sections in scratch clones of the five repositories. The Report gives the count per shape: plain prose, prose with plain list items, prose before a fenced block or a table, and anything else. A section that begins with a fenced block or a table, so that no prose comes before it, or any other shape the rule above would corrupt or leave without a checkbox line, is escalated to the orchestrator with its count and a proposal, instead of a rule invented for it. The Report gives counts only, and nothing of any private repository's content.
- [ ] `scripts/cutover/rehearse.sh` passes on fresh clones of all five repositories with a binary built from this branch's `next/`. In each converted clone no draft or ready item has a Done when without a checkbox line, shown by a count before and after the change. In one converted clone, `submit` on a started item that had a prose Done when is refused until its line is ticked.
- [ ] The paragraph of `docs/strategies/redesign-architecture.md`, Transition, part 2, that says what the converter writes gains one sentence saying this. Its `last_changed` line is left alone; the orchestrator writes it once.

## Scope

`scripts/cutover/convert_records.py`, its tests, that one paragraph of the architecture specification, and this item's record. No change under `next/` or `crates/`: the new binary's rule that no checkbox counts as ticked stays as it is. The four product repositories under `/home/supadava/projects/` are read-only and some are private: work only on scratch clones under `$HOME/.cache/atelier-qr5s/yudj/`, commit nothing from them here, send nothing from them to any outside service, and report process facts only.

## Context

Mission atelier-qr5s. The converter and its `--verify` mode are described in `docs/strategies/redesign-architecture.md`, Transition, part 2; the rehearsal is `scripts/cutover/rehearse.sh`, which makes the clones, converts them and runs the new binary on each in about 36 seconds, and needs `next/` built first (`cargo build -j 4` in `next/`). The step F rollback trials and the cutover run the converter this item changes.

Expected size: about 30 lines of Python and 60 of tests.
