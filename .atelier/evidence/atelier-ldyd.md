---
created_at: "2026-10-15T19:29:46.697869435+00:00"
id: "atelier-ldyd"
evidence_type: "validation"
commit: "8f13d015eda0d0d702e77bf318fb686565e62955"
result: "pass"
captured_at: "2026-10-15T19:29:46.697368+00:00"
agent_identity: "independent-validator-2026-10-15"
target:
  kind: "issue"
  id: "atelier-xz11"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-xz11"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "S2 PASS (independent-validator-2026-10-15, candidate 8f13d015). Scenario test merge_gates::scoped_children_and_open_transitive_blockers_prevent_merge closes child and epic on the epic branch and merges (asserts owner and scoped work terminal, child done after merge); approval_records_committed_by_author_remain_independent_and_merge_has_no_writes asserts HEAD, tree and rev-list --count --all unchanged, status --porcelain empty, branch deleted. Both PASS in nextest 592/592 and cargo test 588/588. Independent CLI scenario /tmp/xz11/scen.sh: epic+child closed on epic/demo-* branch, independent approval, atelier merge -> 'No commits or tracker records were created'; main == pre-merge branch tip (fast-forward per reflog), commit count 12 -> 12, git status --porcelain empty, only main branch remains; before merge main showed epic todo without code, and every main commit where the epic is done also contains the code."
updated_at: "2026-10-15T19:29:46.699727612+00:00"
---

S2 PASS (independent-validator-2026-10-15, candidate 8f13d015). Scenario test merge_gates::scoped_children_and_open_transitive_blockers_prevent_merge closes child and epic on the epic branch and merges (asserts owner and scoped work terminal, child done after merge); approval_records_committed_by_author_remain_independent_and_merge_has_no_writes asserts HEAD, tree and rev-list --count --all unchanged, status --porcelain empty, branch deleted. Both PASS in nextest 592/592 and cargo test 588/588. Independent CLI scenario /tmp/xz11/scen.sh: epic+child closed on epic/demo-* branch, independent approval, atelier merge -> 'No commits or tracker records were created'; main == pre-merge branch tip (fast-forward per reflog), commit count 12 -> 12, git status --porcelain empty, only main branch remains; before merge main showed epic todo without code, and every main commit where the epic is done also contains the code.
