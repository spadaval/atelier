---
schema: atelier.review
schema_version: 2
id: atelier-46s5
title: finding on atelier-fcb7 @dab1f733
target: atelier-fcb7
identity: gpt-6-astra-amend-r1
commit: dab1f733ffb6d2e4bfc406ed27ccda58563b272d
created_at: 2026-11-10T02:43:18.889560627Z
event:
  event: finding
  recommendation: fix_now
---

high docs/strategies/redesign-architecture.md:249: The pre-push rule checks every new commit, but a normal Atelier merge introduces ordinary code commits from its second parent as well as the merge commit; those commits satisfy neither allowed case. The adjacent ban on setting done while an item branch exists also needs to exclude the Atelier merge itself. Implementing this literally makes normal merge publication fail after local completion. Today check_base_commits_since explicitly excludes covered commits; that machinery is being deleted. Smallest fix: define the rule over new base first-parent entries, accept an Atelier merge and the history it integrates, and scope the done prohibition to direct planning commits. Add a normal merge-and-push case beside the direct-code refusal test.
