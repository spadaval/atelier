---
schema: atelier.review
schema_version: 2
id: atelier-p4v4
title: finding on atelier-5zre @8c68c66c
target: atelier-5zre
identity: GPT-6.1 Sol independent reviewer, round 3
commit: 8c68c66cf7d7c1a0f98af37fd78c2b38f13918b2
created_at: 2026-11-11T04:30:30.805905011Z
event:
  event: finding
  recommendation: fix_now
---

R1 low: .atelier/issues/atelier-pu2i.md:46 The (b) drop proves open-only listing but loses the first Done when line's independent requirement that the footer count hidden done records. In a valid converted scratch clone, after making atelier-fpxc done and committing that fixture, the supplied binary check passes; list omits it and ends immediately after the final open row, with no hidden count or command to see the omitted records. list --done separately shows atelier-fpxc, confirming there is one hidden done item. The product specification's list entry specifies the open/default and --done selection, but does not say to remove this footer requirement. Restore and rewrite the still-unmet footer requirement for list and list --done, preserve it explicitly in another retained item, or leave it for Sumeet, and update the fate and mission counts. Do not implement the footer in this records-only sort. Reproduction: /home/supadava/.cache/atelier-qr5s/5zre-review3/proof.txt, section between the fixture commit and the first start atelier-orb2; runnable probe.py and full probe-output.txt are beside it.

Verdict: merge after R1 is fixed; the sort is worth integrating, but no approval is recorded while this contract miss remains. Sizes: complexity medium (181 per-record judgments), value high (a usable cutover inventory), risk high (a mistaken drop silently disappears from conversion). Contract: fail on the surviving requirement above. Fitness: fail until that work is retained; otherwise the change stays within the records-only boundary and adds no unnecessary mechanism. The earlier rounds' findings have corresponding retained rewrites and corrected counts. Independently checked all 181 fate lines, the 65 protected records (only atelier-5zre changed), 76 terminal transitions and matching Report reasons, mission membership/fate counts, the explained remaining links to drops, and the cutover facts: 11 ready items in draft missions, 3 ready items without checkbox lines, 126 deferred findings on done records with zero missing follow-ups (73 done, 11 backlog, 42 dropped). Fresh proof passes: whole-branch git diff --check, the brief's record-excluded diff check, installed atelier check, conversion of pinned head 8c68c66c, --verify (119 files), and the supplied binary check (97 items, 8 missions). Behavioral probes ran only in scratch clones. No Cargo build, install, push, merge or other-repository read ran. Open questions: none needed to retain the requirement. Residual risk: master changes after the baseline still need reconciliation, and the remaining protected proposals need Sumeet's judgment. Code-only Cargo checks were not run, as this branch changes records only.
