---
schema: atelier.review
schema_version: 2
id: atelier-l6zy
title: 'The recorded diagnosis process counts are inflated about threefold by a strace counting artifact. .atelier/issues/atelier-83xi.md:37 and :45 and .atelier/issues/atelier-v5tt.md:29 say land on this repository starts 1,310 Git processes of which 617 are git log walks. Re-measured with the same installed binary 0.2.0+b9cdced8 in /home/supadava/projects/atelier (2,215 commits, 328 review records): strace -f -z (successful execve only) gives about 920 Git processes and 216 git log walks (68 introduced queries plus one append_only_history walk, 147 scope walks, so 3 per retained event over 68 retained events, not 617). merge-base 303 and rev-list 308 match the recorded 297/292 because those run sequentially. The log count is inflated because prefetch runs walks 8 at a time, so strace splits the failed PATH-lookup execve calls (ENOENT) of concurrent children into ''<unfinished ...>'' lines that a grep for non-ENOENT lines counts as walks. Without -z, my trace showed 501 log lines: 179 complete successes, 37 resumed successes and 285 resumed ENOENT. Failure scenario: atelier-v5tt''s Done-when is a walk count that must stop growing with review records, so its before baseline is wrong, and an after measurement that counts correctly would show a drop the change did not cause. The direction of the diagnosis holds (GIT_TRACE2 attribution: the log walks cover about 2.5 s of 5.5 s wall at about 40 ms each). Fix: correct the counts and the per-walk figure in the Design, the Close Reason and v5tt Context, and name the counting method (strace -z or GIT_TRACE2_EVENT).'
target: atelier-83xi
identity: Independent reviewer (Claude subagent)
commit: a32065d797cb83692ce9d2ac1c7478ac162cc6cd
created_at: 2026-10-16T19:08:34.720536855Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Independent reviewer (Claude subagent)
    commit: fee897184704969346ed2ef7d9f9b155d28c5497
    recorded_at: 2026-10-16T19:11:33.802594896Z
    resolution_commit: c9870e84dd0127e8e3c728d664b48eb7f490dc94
    reason: 'Resolved: the Design and Close Reason of atelier-83xi and the Goal and Context of atelier-v5tt now give about 920 Git processes, 216 git log walks, 302 merge-base and 306 rev-list, name the method (strace -f -z, successful execve only), and replace the 27 ms per-walk figure with the GIT_TRACE2 attribution (about 2.5 s of 5.5 s, about 40 ms each). Re-measured read-only in /home/supadava/projects/atelier with the same binary 0.2.0+b9cdced8 (now 2,224 commits, 331 review records): 944 execve, 220 log --root, 311 merge-base, 315 rev-list, land 5.3 s; consistent with the recorded figures given the small growth since.'
    migrated_from: atelier-lgvf
---

The recorded diagnosis process counts are inflated about threefold by a strace counting artifact. .atelier/issues/atelier-83xi.md:37 and :45 and .atelier/issues/atelier-v5tt.md:29 say land on this repository starts 1,310 Git processes of which 617 are git log walks. Re-measured with the same installed binary 0.2.0+b9cdced8 in /home/supadava/projects/atelier (2,215 commits, 328 review records): strace -f -z (successful execve only) gives about 920 Git processes and 216 git log walks (68 introduced queries plus one append_only_history walk, 147 scope walks, so 3 per retained event over 68 retained events, not 617). merge-base 303 and rev-list 308 match the recorded 297/292 because those run sequentially. The log count is inflated because prefetch runs walks 8 at a time, so strace splits the failed PATH-lookup execve calls (ENOENT) of concurrent children into '<unfinished ...>' lines that a grep for non-ENOENT lines counts as walks. Without -z, my trace showed 501 log lines: 179 complete successes, 37 resumed successes and 285 resumed ENOENT. Failure scenario: atelier-v5tt's Done-when is a walk count that must stop growing with review records, so its before baseline is wrong, and an after measurement that counts correctly would show a drop the change did not cause. The direction of the diagnosis holds (GIT_TRACE2 attribution: the log walks cover about 2.5 s of 5.5 s wall at about 40 ms each). Fix: correct the counts and the per-walk figure in the Design, the Close Reason and v5tt Context, and name the counting method (strace -z or GIT_TRACE2_EVENT).
