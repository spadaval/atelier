---
schema: atelier.review
schema_version: 1
id: atelier-qro4
title: merge_gates.rs new_out_of_scope_follow_up_issue_keeps_approval_but_new_child_does_not covers the follow-up and new-child cases but not the guards in review_gate.rs:122-128 (mode 100644, direct child of .atelier/issues, .md). I checked them by hand with the built binary in a scratch repo. An executable new issue file gives FAIL review.current. A symlinked issue file is refused as not a regular committed file. Files at .atelier/issues/sub/x.md or .atelier/issues/notes.txt are refused as invalid record files. So nothing can slip through today; a regression test is optional.
target: atelier-cafy
identity: Independent reviewer (Claude subagent)
commit: b1537aef4483d7a6867ef4924b3b16f603f550e5
created_at: 2026-10-16T18:27:43.566551710Z
event:
  event: finding
  recommendation: no_action
---

merge_gates.rs new_out_of_scope_follow_up_issue_keeps_approval_but_new_child_does_not covers the follow-up and new-child cases but not the guards in review_gate.rs:122-128 (mode 100644, direct child of .atelier/issues, .md). I checked them by hand with the built binary in a scratch repo. An executable new issue file gives FAIL review.current. A symlinked issue file is refused as not a regular committed file. Files at .atelier/issues/sub/x.md or .atelier/issues/notes.txt are refused as invalid record files. So nothing can slip through today; a regression test is optional.
