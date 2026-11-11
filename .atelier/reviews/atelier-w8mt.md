---
schema: atelier.review
schema_version: 2
id: atelier-w8mt
title: finding on atelier-5zre @f24fefd6
target: atelier-5zre
identity: GPT-6.1 Sol independent reviewer, round 6
commit: f24fefd6a574e28f6ca5e76a4ca4f4d27772619d
created_at: 2026-11-11T05:23:48.448487383Z
event:
  event: finding
  recommendation: fix_now
---

R1 low: .atelier/issues/atelier-lkuy.md:46 Shrink: preserve the surviving document pointer when rewriting the mission overview. At fc0fe395 the Goal explicitly reports that a linked strategy is not shown, and the first Done when includes its path in the entry. This rewrite removes that requirement because the documents now live in the mission directory. That relocates the document; it does not meet the need to find it from the overview. On the pinned-head conversion, .atelier/missions/atelier-qr5s/design.md exists, but the supplied new binary prints no path to it in either list --mission atelier-qr5s or show atelier-qr5s, so the restored show pointer does not supply it either. Keep the requirement, replacing the old strategy path with the mission document path or directory, and update this item’s rewrite explanation. This is a trivial record-only fix within the existing Scope; it needs no CLI change here.

Verdict: shrink the removal from atelier-lkuy, retain its document pointer, and re-review. Sizes: complexity medium for per-record judgments; value high because cutover needs an accurate open-work set; risk high because incorrect drops or narrowed requirements silently lose work. Contract: fail on this one surviving requirement; the census, protection and executable proof otherwise pass. Fitness: pass apart from that contract miss; no added code or unjustified moving parts.

Proof: independently counted all 181 open records at fc0fe395: 68 drop, 27 rewrite, 10 as is, 76 left for Sumeet. The 72-record protected set was checked by metadata, labels, mission membership, explicit IDs and file-by-file Git history; only the permitted atelier-5zre record changed. All eight missions are unchanged. Independently confirmed 28 ready items, 11 in draft missions, 3 ready records without checkbox lines, and 126 deferred findings on done records with zero missing follow-ups (73 done, 11 backlog, 23 superseded, 19 declined). Mission membership and fate counts match the Report. Read the baseline drop records, rewrite diffs, kept records, governing specifications and branch history. Installed atelier check passes with existing checkbox warnings; git diff --check fc0fe395 f24fefd6 and the brief’s non-record whitespace check pass. Converted the pinned head in a fresh scratch clone under ~/.cache/atelier-qr5s/5zre-review6: 105 items (76 draft, 27 ready, 2 ready from decisions), 8 missions (6 draft, 2 ready); --verify matches 127 files and the supplied new binary’s check passes. Additional scratch commands confirm the mission-document omission, multi-error check, refused draft and blocked starts, missing-ID hint, redirected rows and removed command/flag forms.

Open questions: none. Residual risk: the fate decisions remain judgments and protected-record proposals still require Sumeet. No builds or installs, cargo checks, cross-repository smoke tests, pushes or merges were run, as required for this record-only review. Approval withheld for R1.
