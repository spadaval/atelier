---
schema: atelier.review
schema_version: 4
id: atelier-01wq
title: finding on atelier-zpmb @34b9970c
target: atelier-zpmb
identity: gpt-6.1-sol-r3
commit: 34b9970cc02ba2b133b420cd62a55e6eca0fd777
created_at: 2026-11-09T17:23:06.472750691Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: wont_fix
    identity: sumeet
    commit: 32f7112c47b6b8f72a3057594c523c239b72d9ad
    recorded_at: 2026-11-09T17:57:51.976810966Z
    reason: 'Real but not worth fixing. A transient Git failure on a local repository is rare: the reproduction needed chmod 000 on one object. The cache holds one view per repository at its base commit, so a wrong page lasts only until the base moves. Changing the ~23 history reads to tell a failed read from an absent record costs far more than that.'
---

R1 medium: crates/atelier-site/src/lib.rs:220-225 still caches incomplete debriefs after transient failures reading historical record blobs. record_history::issue_histories (crates/atelier-app/src/record_history.rs:94-97) suppresses issue_at errors, but ReadContext::failed_read is set only by file_text, so build_view returns 200 and retains a view with history missing. Reproduced at the pinned code in a scratch repository with mission versions draft, todo, in_progress: warm the mission summary, chmod 000 only the older todo-version blob, then GET /api/probe/mission/probe-m1/debrief. It returns 200 and timeline.started incorrectly advances from the todo commit to the in_progress commit. After restoring blob readability, the same server keeps the incorrect response; a fresh server returns the correct start commit at the unchanged base. Thus atelier-cuq9 is not fully resolved: Git availability still changes a supposedly commit-only cached result. Propagate or track failed reads of existing historical objects, including issue_at/blob paths, while retaining the distinction from absent or malformed historical records; return an uncached error and add a regression test proving recovery without moving the base. Reproducer: /home/supadava/.cache/atelier-tmp-zpmb/r3-history-probe.py; outputs: /home/supadava/.cache/atelier-tmp-zpmb/review-r3-k9r6i8pz/result.json.
