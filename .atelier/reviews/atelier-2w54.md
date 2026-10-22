---
schema: atelier.review
schema_version: 1
id: atelier-2w54
title: 'Reviewed 2239abec..1cb41498. Remerge diff of 75d2dc13: the only conflict hunk resolves to edge_allowed(..., pin); a diff against each parent shows master''s approval_stale_edges, current_on_branch, approval_state and evaluate_scope change all kept, and the branch''s pin rule in allowed_edge, allowed_base_merge and author_identities_to kept. approval_is_current_to and approval_stale_edges now call edge_allowed with the same pin and ids. Threading pin into allowed_base_merge changes nothing there: it calls allowed_edge only for in-scope issues or conflicted non-issue paths, where the out-of-scope pin exemption cannot fire. New test sync_keeps_the_approval_across_a_follow_up_linked_after_it: mutating the pin to end in stale_edges fails its sync assertion, and doing so in approval_is_current_to fails its review.current assertion, so it catches drift either way. A disagreement I constructed (base takes branch work, then sync) already exists on master, and I deferred it as a finding. 1cb41498 holds only evidence records. Proof: nextest --workspace with test-hooks 874/874 and without 870/870, fmt --check, git diff --check HEAD^ HEAD excluding .atelier, cargo metadata --locked all pass.'
target: atelier-r28p
identity: Independent reviewer (Claude subagent)
commit: 1cb41498ad5694ca454fee6d93c4a6c81ad339bf
created_at: 2026-10-22T22:58:29.371975738Z
event:
  event: approval
---

Reviewed 2239abec..1cb41498. Remerge diff of 75d2dc13: the only conflict hunk resolves to edge_allowed(..., pin); a diff against each parent shows master's approval_stale_edges, current_on_branch, approval_state and evaluate_scope change all kept, and the branch's pin rule in allowed_edge, allowed_base_merge and author_identities_to kept. approval_is_current_to and approval_stale_edges now call edge_allowed with the same pin and ids. Threading pin into allowed_base_merge changes nothing there: it calls allowed_edge only for in-scope issues or conflicted non-issue paths, where the out-of-scope pin exemption cannot fire. New test sync_keeps_the_approval_across_a_follow_up_linked_after_it: mutating the pin to end in stale_edges fails its sync assertion, and doing so in approval_is_current_to fails its review.current assertion, so it catches drift either way. A disagreement I constructed (base takes branch work, then sync) already exists on master, and I deferred it as a finding. 1cb41498 holds only evidence records. Proof: nextest --workspace with test-hooks 874/874 and without 870/870, fmt --check, git diff --check HEAD^ HEAD excluding .atelier, cargo metadata --locked all pass.
