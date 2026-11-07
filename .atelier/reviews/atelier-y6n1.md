---
schema: atelier.review
schema_version: 2
id: atelier-y6n1
title: finding on atelier-r9hb @b297d60f
target: atelier-r9hb
identity: lighter-workflow reviewer for atelier-r9hb (claude-opus-5-5)
commit: b297d60ff5d217f00c2b72bb08a0f144f81726f2
created_at: 2026-11-07T00:12:05.570012068Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: implementer/atelier-r9hb
    commit: 8c7d4f648cf071af4958827076cbea27ad396f47
    recorded_at: 2026-11-07T00:54:25.074858852Z
    resolution_commit: 8c7d4f648cf071af4958827076cbea27ad396f47
    reason: The debrief line names the item (shared done_when_edit text), so a child's edit under its parent reads as the child's; new test debrief::a_child_done_when_edit_shown_under_its_parent_names_the_child, which fails on the old text.
    migrated_from: atelier-ompc
---

medium: crates/atelier-cli/src/commands/mission_review.rs:338 (change_text) and crates/atelier-app/src/mission_review.rs:590 (work_entry folds each child's changes into its parent's entry): the debrief's Done when line does not name the item it belongs to, so a child's edit reads as the parent's. Real repo: 'atelier debrief atelier-kgcr' prints, under epic atelier-1ioo after its child lines, two bare 'Changed: Done when edited at 584f98e1 after the item was started, after failing receipt ...' lines; they belong to children atelier-8ud1 and atelier-xz11, but nothing says so, and a reader takes them as 1ioo's Done when (the HTML change_html is the same). The Done when asks that the debrief list the edit under the item; merge's Note line names the item, the debrief's does not. The tests only cover an edit on the entry's own item. Fix: name the item in the debrief text (e.g. '<id> Done when edited at ...', at least when edit.issue differs from the entry), and add a test with a scoped child of an epic whose Done when is edited after its start.
