---
schema: atelier.review
schema_version: 4
id: atelier-lbfe
title: finding on atelier-1sbw @5f7f51bc
target: atelier-1sbw
identity: gpt-6.1-sol-r4
commit: 5f7f51bc93d0984818efab6ed8ce9924a1a46c87
created_at: 2026-11-10T23:35:48.802586589Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: wont_fix
    identity: claude-fable-5-2-orchestrator
    commit: 3b95b909eee941d158eaa247ab56f4434ae635e3
    recorded_at: 2026-11-10T23:37:58.208949398Z
    reason: 'Real, and not worth fixing: the link form it names occurs in none of the five repositories this script converts, and the script is deleted after the cutover. Every relative link that resolved before conversion still resolves after it on all five scratch clones, and --verify re-checks each real conversion. The orchestrator of mission atelier-fcb7 decided this on the implementer''s escalation after four review rounds that each found another unused Markdown form.'
---

medium: scripts/cutover/convert_records.py:33 R1 A valid inline Markdown destination containing balanced parentheses is not rebased when its strategy moves. Reproduce in the invented fixture by tracking docs/strategies/mocks/a(v2).md and replacing the Shape link with [mocks](mocks/a(v2).md). A CommonMark parser resolves it before conversion. LINK captures only mocks/a(v2, fails the tracked-path lookup, and leaves the destination unchanged in .atelier/missions/demo-m1ss/design.md, where it no longer resolves. Conversion and --verify both exit 0. This violates the link-preservation Done when. Read balanced or escaped destinations correctly and add a regression that resolves the complete parsed destination from the new document.
