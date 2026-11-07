---
schema: atelier.review
schema_version: 2
id: atelier-896c
title: approval on atelier-jnrh @1ff63396
target: atelier-jnrh
identity: independent reviewer subagent (claude-opus-5-5), atelier-jnrh
commit: 1ff633960d646b415c083d724c2780b77acb7d38
created_at: 2026-11-07T07:42:33.153086433Z
event:
  event: approval
---

Re-review of the fix diff 78f0e369..f9634a22 and the dispositions in 1ff63396, after my full review at 8798b61e. atelier-2ygf resolved: refuse_unfinished covers the named item, or the owner plus its active children when the owner is named. In a scratch clone of astromech on feature/astromech-bp23, naming a started child refused only for that child, naming the owner listed the owner and the started child but no unstarted sibling, and naming a done child produced the brief. atelier-nrq4 resolved: merge and merge --check print a Note for each item they mark done with an unticked box (After merge lines aside) or an empty Report, and refuse nothing. merge_notes_work_it_marks_done_unticked_or_unreported pins both notes in merge --check and merge, for an untick after approval and a child added after it. atelier-kyd9 resolved: the validation doc's Current Equivalent Workflow, the merge help summary and the session.rs comment now describe the merge-commit mechanics, and nothing tells an agent to close work. ADR 0028 decision 2, reviews-and-merge.md and work-model.md match the code. Proof at 1ff63396: git diff --check, cargo fmt --check and cargo metadata --locked are clean; cargo nextest run -j 4 --workspace passed 992, and with --features test-hooks passed 998. CI on the new head was still running when this was recorded, and merge-ci-passed.sh gates the merge on it.
