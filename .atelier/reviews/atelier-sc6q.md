---
schema: atelier.review
schema_version: 2
id: atelier-sc6q
title: finding on atelier-5zre @ef433b8e
target: atelier-5zre
identity: GPT-6.1 Sol independent reviewer, round 8
commit: ef433b8e6bd3270ac129c7aa494151a4eabf8b22
created_at: 2026-11-11T05:47:51.704534354Z
event:
  event: finding
  recommendation: fix_now
---

R1 low: .atelier/issues/atelier-7ebi.md:35 Shrink: preserve the surviving relationship-confirmation requirements in this rewrite. At fc0fe395, the third Done when line requires link/unlink confirmations to describe the stored relationship with titles and, for blocked_by, its readiness effect. The rewrite removes that line because issue link is gone, but the relationships survive as set --mission and set --blocked-by (product specification, Commands: Planner). The Report itself maps issue link --role advances to set --mission in atelier-8bve. On the pinned-head conversion, atelier-next-7b06ed12 set atelier-fpxc --mission atelier-qr5s and set atelier-fpxc --blocked-by atelier-orb2 both succeed and print only Set the fields of item atelier-fpxc plus the file/commit advice; neither names the linked title or the blocking effect. The remaining Done when asks only for field old/new values and a bare blocked_by ID example, so an implementation could satisfy it while still losing these independent requirements. Keep the surviving link-confirmation clauses and their tests, mapped to set --mission and set --blocked-by, and correct the rewrite explanation in this record and atelier-5zre's Report. Removing issue note and parent-only cases is still appropriate. This asks only for preserving the work in the records, not for implementing CLI changes on this branch.

Verdict: shrink this rewrite and re-review. Sizes: complexity medium for per-record judgments; value high because cutover needs an accurate open-work set; risk high because a wrong drop or narrowing silently loses work. Contract: fail on preservation of atelier-7ebi's surviving need. Fitness: pass apart from that contract failure; no code or unnecessary moving parts were added. The round 7 mission-document fix retains that requirement, and all 26 restored class (b) records are byte-identical to fc0fe395 under the recorded rescope.

Proof: independently counted all 181 open records at fc0fe395: 41 drop (32 declined, nine superseded), 28 rewrite, 36 as is, 76 left for Sumeet; no missing, duplicate or extra fate lines. Checked metadata, needs-human labels, explicit IDs, protected mission membership and file-by-file Git history: all 72 protected records remain unchanged except the permitted atelier-5zre updates; all eight missions are unchanged. All drop Reports match the stated reasons. Mission membership/fate counts match. The Report explains the four open records blocked by or children of drops. Confirmed 28 ready items, 11 in draft missions, three ready records without checkbox lines, and 126 deferred findings on done records with zero missing follow-ups (73 done, 23 backlog, 14 superseded, 16 declined). Read baseline drop records, rewrite diffs, the governing specifications, branch history and replacement records. Installed atelier 0.4.0 check passes with checkbox warnings; git diff --check fc0fe395 ef433b8e and the brief's non-record whitespace check pass. A fresh conversion under ~/.cache/atelier-qr5s/5zre-review8/conv converts 132 items (103 draft, 27 ready, two ready from decisions) and eight missions (six draft, two ready); --verify matches 154 files; check passes on both supplied binaries. Latest-binary probes reproduce the missing mission-document path, multi-label limitation, blocked start refusal, mission-start error, cycle diagnostic gap, and multi-error check output; the set probes above establish R1.

Open questions: none. Residual risk: fate decisions remain judgments, and protected-record/mission proposals remain for Sumeet. Cargo checks were not run: the branch changes only records and the host instructions require no builds. No installs, other-repository reads, pushes or merges were run. Approval withheld for R1.
