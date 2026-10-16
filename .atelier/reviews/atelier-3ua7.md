---
schema: atelier.review
schema_version: 1
id: atelier-3ua7
title: 'Re-review at 58e899ce. All three earlier findings are fixed. 0odr: master, which includes atelier-r7cd (done), is merged in; prune.rs kept_content (line 939) protects decision records and records with a Decision or Background section, and issue_candidates calls it (line 912), so the decision''s statement that prune keeps them now holds. q0g2: lint::size_labels lowercases sizes and refuses unknown sizes and a second size; create and update -l both call it. In a scratch repo, create -l size-huge, create -l size-s -l size-l and update -l size-huge all fail with the size message. size-M is stored as size-m and size-XL replaces size-m as size-xl. issue list --label size-xl filters correctly. A hand-edited size-Q is caught by check, and issue show then hides Size while still listing the label. iukz: the reserved form is now size-s|m|l|xl; astromech-atelier holds only those four values, and the new binary''s check reports no size findings there. Lint passes on this repository. The decision record, work-model.md, retention-and-prune.md and the updated test agree with the code. Evidence atelier-nfk6 (nextest, exit 0) is at 9f26e3b8; HEAD adds only records on top of it. The existing check rules are unchanged.'
target: atelier-b7hb
identity: Independent reviewer (Claude subagent)
commit: 58e899ce34d9f5e3fede7b4648dd01283a1d8fc2
created_at: 2026-10-16T01:41:46.915124641Z
event:
  event: approval
---

Re-review at 58e899ce. All three earlier findings are fixed. 0odr: master, which includes atelier-r7cd (done), is merged in; prune.rs kept_content (line 939) protects decision records and records with a Decision or Background section, and issue_candidates calls it (line 912), so the decision's statement that prune keeps them now holds. q0g2: lint::size_labels lowercases sizes and refuses unknown sizes and a second size; create and update -l both call it. In a scratch repo, create -l size-huge, create -l size-s -l size-l and update -l size-huge all fail with the size message. size-M is stored as size-m and size-XL replaces size-m as size-xl. issue list --label size-xl filters correctly. A hand-edited size-Q is caught by check, and issue show then hides Size while still listing the label. iukz: the reserved form is now size-s|m|l|xl; astromech-atelier holds only those four values, and the new binary's check reports no size findings there. Lint passes on this repository. The decision record, work-model.md, retention-and-prune.md and the updated test agree with the code. Evidence atelier-nfk6 (nextest, exit 0) is at 9f26e3b8; HEAD adds only records on top of it. The existing check rules are unchanged.
