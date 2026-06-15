---
created_at: "2026-06-14T22:12:51.730780159+00:00"
id: "atelier-xz3g"
evidence_type: "validation"
captured_at: "2026-06-14T22:12:51.730745088+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-zbd4"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-zbd4"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Complete classification table for atelier-zbd4.\n\nClassification rule: dirty worktrees are preserve; clean branches not contained in master are fold into epic branch; clean branches contained in master are delete; missing/prunable registrations are investigate. origin/master containment is recorded separately because local master is 514 commits ahead of origin/master.\n\nCommand transcript evidence: atelier-ogi4 captures the inventory command script, including git worktree list --porcelain, du -sh, branch containment, duplicate heads, dirty counts, and stale-registration checks. This evidence record carries the complete classification table because command-backed stdout summaries are byte-limited.\n\n| branch | worktree | head | dirty_count | unique_vs_master | unique_vs_origin_master | contained_in_master | contained_in_origin_master | disk | classification |\n| --- | --- | --- | ---: | ---: | ---: | --- | --- | ---: | --- |\n| codex/atelier-10qm | present | 8594a8c | 0 | 0 | 313 | yes | no | 1.7G | delete |\n| codex/atelier-3iom | present | fff34e5 | 1 | 0 | 296 | yes | no | 1.7G | preserve |\n| codex/atelier-3z35 | present | 9b2a5a7 | 16 | 0 | 232 | yes | no | 863M | preserve |\n| codex/atelier-4eot | present | 97fd373 | 0 | 0 | 112 | yes | no | 5.9M | delete |\n| codex/atelier-4u5h | present | cf488fb | 0 | 0 | 334 | yes | no | 1.3G | delete |\n| codex/atelier-613f | present | aad3551 | 9 | 0 | 379 | yes | no | 1.3G | preserve |\n| codex/atelier-6w0u | present | be3f4a0 | 0 | 1 | 128 | no | no | 6.6M | fold into epic branch |\n| codex/atelier-7r55 | present | c05f8e2 | 0 | 0 | 144 | yes | no | 7.2M | delete |\n| codex/atelier-7yen | present | 3277a46 | 0 | 1 | 128 | no | no | 1.2G | fold into epic branch |\n| codex/atelier-9soq | present | e8ad446 | 0 | 0 | 494 | yes | no | 880M | delete |\n| codex/atelier-b2vi | present | 05c1533 | 0 | 0 | 208 | yes | no | 9.9M | delete |\n| codex/atelier-c4uz | present | 11c2d6e | 0 | 0 | 503 | yes | no | 871M | delete |\n| codex/atelier-c64h | present | ca0301f | 14 | 0 | 225 | yes | no | 12M | preserve |\n| codex/atelier-ca32 | present | 0badd6f | 0 | 2 | 77 | no | no | 1.1G | fold into epic branch |\n| codex/atelier-cbru | present | 05c1533 | 0 | 0 | 208 | yes | no | 9.9M | delete |\n| codex/atelier-exz1 | present | 76dbc4b | 0 | 0 | 124 | yes | no | 6.4M | delete |\n| codex/atelier-fx9r | present | 748a103 | 0 | 4 | 128 | no | no | 1.1G | fold into epic branch |\n| codex/atelier-ggls | present | 1eb1d74 | 0 | 0 | 335 | yes | no | 1.9G | delete |\n| codex/atelier-gzel | present | 5869298 | 1 | 0 | 296 | yes | no | 1.5G | preserve |\n| codex/atelier-h184 | present | be4171d | 0 | 0 | 389 | yes | no | 15M | delete |\n| codex/atelier-i9ob | present | a19109a | 0 | 0 | 314 | yes | no | 1.7G | delete |\n| codex/atelier-j01c | present | b28959e | 10 | 0 | 404 | yes | no | 1.1G | preserve |\n| codex/atelier-ja3o | present | ce1642e | 0 | 0 | 266 | yes | no | 12M | delete |\n| codex/atelier-jarw | present | 97c4851 | 0 | 2 | 71 | no | no | 736M | fold into epic branch |\n| codex/atelier-kpm8 | present | 2be83a1 | 0 | 0 | 334 | yes | no | 1.3G | delete |\n| codex/atelier-l0yk | present | c05f8e2 | 0 | 0 | 144 | yes | no | 7.2M | delete |\n| codex/atelier-liqk | present | b28959e | 6 | 0 | 404 | yes | no | 867M | preserve |\n| codex/atelier-lv4s | present | 5852a0d | 0 | 0 | 228 | yes | no | 1.3G | delete |\n| codex/atelier-mxug | present | b28959e | 0 | 0 | 404 | yes | no | 15M | delete |\n| codex/atelier-n0p4 | present | 95185d0 | 352 | 0 | 230 | yes | no | 1.3G | preserve |\n| codex/atelier-n1ys | present | 9c1a09c | 0 | 1 | 136 | no | no | 750M | fold into epic branch |\n| codex/atelier-nqp4 | present | 8070895 | 1 | 0 | 296 | yes | no | 1.3G | preserve |\n| codex/atelier-nwlx | present | 9206ec9 | 522 | 0 | 59 | yes | no | 2.4M | preserve |\n| codex/atelier-od8a | present | 35b2696 | 0 | 0 | 416 | yes | no | 15M | delete |\n| codex/atelier-of3h | present | 7ed85e9 | 195 | 0 | 274 | yes | no | 1.8G | preserve |\n| codex/atelier-oku1 | present | ca0301f | 9 | 0 | 225 | yes | no | 12M | preserve |\n| codex/atelier-oqtz | present | 35b2696 | 10 | 0 | 416 | yes | no | 1.2G | preserve |\n| codex/atelier-papa | present | 35b2696 | 6 | 0 | 416 | yes | no | 867M | preserve |\n| codex/atelier-qnxs | present | 12065e3 | 7 | 0 | 428 | yes | no | 1.3G | preserve |\n| codex/atelier-rc1v | present | 05c1533 | 0 | 0 | 208 | yes | no | 9.9M | delete |\n| codex/atelier-rgd1 | present | e15abfe | 0 | 0 | 312 | yes | no | 1.8G | delete |\n| codex/atelier-sxh8 | present | ccbc594 | 8 | 0 | 434 | yes | no | 1.7G | preserve |\n| codex/atelier-sxyy | present | e703f32 | 0 | 1 | 125 | no | no | 6.5M | fold into epic branch |\n| codex/atelier-u08r | present | fa9ca5d | 13 | 0 | 273 | yes | no | 1.1G | preserve |\n| codex/atelier-u4nx | present | 69724b1 | 0 | 0 | 149 | yes | no | 7.7M | delete |\n| codex/atelier-u6ax | present | 69724b1 | 2 | 0 | 149 | yes | no | 1.1G | preserve |\n| codex/atelier-uran | present | 004c1ba | 0 | 0 | 499 | yes | no | 17M | delete |\n| codex/atelier-ux3k | present | c8d9dd7 | 0 | 0 | 502 | yes | no | 873M | delete |\n| codex/atelier-v4u7 | present | ba6068e | 0 | 0 | 114 | yes | no | 6.0M | delete |\n| codex/atelier-v6nd | present | 68ed3dd | 0 | 2 | 137 | no | no | 6.8M | fold into epic branch |\n| codex/atelier-vau5 | present | d9a230d | 29 | 0 | 435 | yes | no | 939M | preserve |\n| codex/atelier-vj08 | present | d9a230d | 8 | 0 | 435 | yes | no | 868M | preserve |\n| codex/atelier-vu88 | present | ce1642e | 13 | 0 | 266 | yes | no | 1.3G | preserve |\n| codex/atelier-wj05 | present | abed86a | 0 | 0 | 333 | yes | no | 1.5G | delete |\n| codex/atelier-xcy9 | present | 389ae15 | 0 | 1 | 70 | no | no | 1.1G | fold into epic branch |\n| codex/atelier-y041 | present | 9287234 | 10 | 0 | 231 | yes | no | 866M | preserve |\n| codex/atelier-z80r | present | 05c1533 | 0 | 0 | 208 | yes | no | 9.9M | delete |\n| codex/atelier-zrmo | present | d58c2a1 | 0 | 0 | 491 | yes | no | 870M | delete |\n\nClassification totals:\n- delete: 27\n- preserve: 22\n- fold into epic branch: 9\n- investigate branch/worktree pairs: 0\n- stale/prunable Git registrations: 1\n\nStale/prunable Git registration:\n- /root/atelier/.atelier/.locks-cache | prunable gitdir file points to non-existent location\n\nDuplicate heads:\n- 05c1533: codex/atelier-b2vi codex/atelier-cbru codex/atelier-rc1v codex/atelier-z80r\n- 35b2696: codex/atelier-od8a codex/atelier-oqtz codex/atelier-papa\n- 69724b1: codex/atelier-u4nx codex/atelier-u6ax\n- b28959e: codex/atelier-j01c codex/atelier-liqk codex/atelier-mxug\n- c05f8e2: codex/atelier-7r55 codex/atelier-l0yk\n- ca0301f: codex/atelier-c64h codex/atelier-oku1\n- ce1642e: codex/atelier-ja3o codex/atelier-vu88\n- d9a230d: codex/atelier-vau5 codex/atelier-vj08\n\nNo codex/atelier-* branch was missing a matching .atelier-worktrees directory, and no .atelier-worktrees directory was missing its matching codex branch."
updated_at: "2026-06-14T22:12:53.323853750+00:00"
---

Complete classification table for atelier-zbd4.

Classification rule: dirty worktrees are preserve; clean branches not contained in master are fold into epic branch; clean branches contained in master are delete; missing/prunable registrations are investigate. origin/master containment is recorded separately because local master is 514 commits ahead of origin/master.

Command transcript evidence: atelier-ogi4 captures the inventory command script, including git worktree list --porcelain, du -sh, branch containment, duplicate heads, dirty counts, and stale-registration checks. This evidence record carries the complete classification table because command-backed stdout summaries are byte-limited.

| branch | worktree | head | dirty_count | unique_vs_master | unique_vs_origin_master | contained_in_master | contained_in_origin_master | disk | classification |
| --- | --- | --- | ---: | ---: | ---: | --- | --- | ---: | --- |
| codex/atelier-10qm | present | 8594a8c | 0 | 0 | 313 | yes | no | 1.7G | delete |
| codex/atelier-3iom | present | fff34e5 | 1 | 0 | 296 | yes | no | 1.7G | preserve |
| codex/atelier-3z35 | present | 9b2a5a7 | 16 | 0 | 232 | yes | no | 863M | preserve |
| codex/atelier-4eot | present | 97fd373 | 0 | 0 | 112 | yes | no | 5.9M | delete |
| codex/atelier-4u5h | present | cf488fb | 0 | 0 | 334 | yes | no | 1.3G | delete |
| codex/atelier-613f | present | aad3551 | 9 | 0 | 379 | yes | no | 1.3G | preserve |
| codex/atelier-6w0u | present | be3f4a0 | 0 | 1 | 128 | no | no | 6.6M | fold into epic branch |
| codex/atelier-7r55 | present | c05f8e2 | 0 | 0 | 144 | yes | no | 7.2M | delete |
| codex/atelier-7yen | present | 3277a46 | 0 | 1 | 128 | no | no | 1.2G | fold into epic branch |
| codex/atelier-9soq | present | e8ad446 | 0 | 0 | 494 | yes | no | 880M | delete |
| codex/atelier-b2vi | present | 05c1533 | 0 | 0 | 208 | yes | no | 9.9M | delete |
| codex/atelier-c4uz | present | 11c2d6e | 0 | 0 | 503 | yes | no | 871M | delete |
| codex/atelier-c64h | present | ca0301f | 14 | 0 | 225 | yes | no | 12M | preserve |
| codex/atelier-ca32 | present | 0badd6f | 0 | 2 | 77 | no | no | 1.1G | fold into epic branch |
| codex/atelier-cbru | present | 05c1533 | 0 | 0 | 208 | yes | no | 9.9M | delete |
| codex/atelier-exz1 | present | 76dbc4b | 0 | 0 | 124 | yes | no | 6.4M | delete |
| codex/atelier-fx9r | present | 748a103 | 0 | 4 | 128 | no | no | 1.1G | fold into epic branch |
| codex/atelier-ggls | present | 1eb1d74 | 0 | 0 | 335 | yes | no | 1.9G | delete |
| codex/atelier-gzel | present | 5869298 | 1 | 0 | 296 | yes | no | 1.5G | preserve |
| codex/atelier-h184 | present | be4171d | 0 | 0 | 389 | yes | no | 15M | delete |
| codex/atelier-i9ob | present | a19109a | 0 | 0 | 314 | yes | no | 1.7G | delete |
| codex/atelier-j01c | present | b28959e | 10 | 0 | 404 | yes | no | 1.1G | preserve |
| codex/atelier-ja3o | present | ce1642e | 0 | 0 | 266 | yes | no | 12M | delete |
| codex/atelier-jarw | present | 97c4851 | 0 | 2 | 71 | no | no | 736M | fold into epic branch |
| codex/atelier-kpm8 | present | 2be83a1 | 0 | 0 | 334 | yes | no | 1.3G | delete |
| codex/atelier-l0yk | present | c05f8e2 | 0 | 0 | 144 | yes | no | 7.2M | delete |
| codex/atelier-liqk | present | b28959e | 6 | 0 | 404 | yes | no | 867M | preserve |
| codex/atelier-lv4s | present | 5852a0d | 0 | 0 | 228 | yes | no | 1.3G | delete |
| codex/atelier-mxug | present | b28959e | 0 | 0 | 404 | yes | no | 15M | delete |
| codex/atelier-n0p4 | present | 95185d0 | 352 | 0 | 230 | yes | no | 1.3G | preserve |
| codex/atelier-n1ys | present | 9c1a09c | 0 | 1 | 136 | no | no | 750M | fold into epic branch |
| codex/atelier-nqp4 | present | 8070895 | 1 | 0 | 296 | yes | no | 1.3G | preserve |
| codex/atelier-nwlx | present | 9206ec9 | 522 | 0 | 59 | yes | no | 2.4M | preserve |
| codex/atelier-od8a | present | 35b2696 | 0 | 0 | 416 | yes | no | 15M | delete |
| codex/atelier-of3h | present | 7ed85e9 | 195 | 0 | 274 | yes | no | 1.8G | preserve |
| codex/atelier-oku1 | present | ca0301f | 9 | 0 | 225 | yes | no | 12M | preserve |
| codex/atelier-oqtz | present | 35b2696 | 10 | 0 | 416 | yes | no | 1.2G | preserve |
| codex/atelier-papa | present | 35b2696 | 6 | 0 | 416 | yes | no | 867M | preserve |
| codex/atelier-qnxs | present | 12065e3 | 7 | 0 | 428 | yes | no | 1.3G | preserve |
| codex/atelier-rc1v | present | 05c1533 | 0 | 0 | 208 | yes | no | 9.9M | delete |
| codex/atelier-rgd1 | present | e15abfe | 0 | 0 | 312 | yes | no | 1.8G | delete |
| codex/atelier-sxh8 | present | ccbc594 | 8 | 0 | 434 | yes | no | 1.7G | preserve |
| codex/atelier-sxyy | present | e703f32 | 0 | 1 | 125 | no | no | 6.5M | fold into epic branch |
| codex/atelier-u08r | present | fa9ca5d | 13 | 0 | 273 | yes | no | 1.1G | preserve |
| codex/atelier-u4nx | present | 69724b1 | 0 | 0 | 149 | yes | no | 7.7M | delete |
| codex/atelier-u6ax | present | 69724b1 | 2 | 0 | 149 | yes | no | 1.1G | preserve |
| codex/atelier-uran | present | 004c1ba | 0 | 0 | 499 | yes | no | 17M | delete |
| codex/atelier-ux3k | present | c8d9dd7 | 0 | 0 | 502 | yes | no | 873M | delete |
| codex/atelier-v4u7 | present | ba6068e | 0 | 0 | 114 | yes | no | 6.0M | delete |
| codex/atelier-v6nd | present | 68ed3dd | 0 | 2 | 137 | no | no | 6.8M | fold into epic branch |
| codex/atelier-vau5 | present | d9a230d | 29 | 0 | 435 | yes | no | 939M | preserve |
| codex/atelier-vj08 | present | d9a230d | 8 | 0 | 435 | yes | no | 868M | preserve |
| codex/atelier-vu88 | present | ce1642e | 13 | 0 | 266 | yes | no | 1.3G | preserve |
| codex/atelier-wj05 | present | abed86a | 0 | 0 | 333 | yes | no | 1.5G | delete |
| codex/atelier-xcy9 | present | 389ae15 | 0 | 1 | 70 | no | no | 1.1G | fold into epic branch |
| codex/atelier-y041 | present | 9287234 | 10 | 0 | 231 | yes | no | 866M | preserve |
| codex/atelier-z80r | present | 05c1533 | 0 | 0 | 208 | yes | no | 9.9M | delete |
| codex/atelier-zrmo | present | d58c2a1 | 0 | 0 | 491 | yes | no | 870M | delete |

Classification totals:
- delete: 27
- preserve: 22
- fold into epic branch: 9
- investigate branch/worktree pairs: 0
- stale/prunable Git registrations: 1

Stale/prunable Git registration:
- /root/atelier/.atelier/.locks-cache | prunable gitdir file points to non-existent location

Duplicate heads:
- 05c1533: codex/atelier-b2vi codex/atelier-cbru codex/atelier-rc1v codex/atelier-z80r
- 35b2696: codex/atelier-od8a codex/atelier-oqtz codex/atelier-papa
- 69724b1: codex/atelier-u4nx codex/atelier-u6ax
- b28959e: codex/atelier-j01c codex/atelier-liqk codex/atelier-mxug
- c05f8e2: codex/atelier-7r55 codex/atelier-l0yk
- ca0301f: codex/atelier-c64h codex/atelier-oku1
- ce1642e: codex/atelier-ja3o codex/atelier-vu88
- d9a230d: codex/atelier-vau5 codex/atelier-vj08

No codex/atelier-* branch was missing a matching .atelier-worktrees directory, and no .atelier-worktrees directory was missing its matching codex branch.
