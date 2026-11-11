---
schema: atelier.review
schema_version: 2
id: atelier-jaa4
title: finding on atelier-5zre @9b9af447
target: atelier-5zre
identity: GPT-6.1 Sol independent reviewer, round 7
commit: 9b9af44768378ed0141c0bd4a3c8077067f2e975
created_at: 2026-11-11T05:29:49.939464461Z
event:
  event: finding
  recommendation: fix_now
---

R1 low: .atelier/issues/atelier-yzuv.md:47 Shrink: keep the surviving mission-show document pointer instead of dropping this record as already implemented. Its first Done when at line 36 requires mission show to print the strategy path. On a fresh conversion of the pinned head, .atelier/missions/atelier-qr5s/design.md exists, but the supplied new binary's show atelier-qr5s prints no path to that document or its directory. Running show on atelier-4pzj, the proof cited for this drop, cannot establish that requirement because that mission has no strategy document. The round 6 fix to atelier-lkuy correctly preserves the pointer in list's mission headings, but that item's Scope covers list, not show. This (b) drop therefore still loses a surviving requirement at conversion. Restore atelier-yzuv as a rewrite narrowed to the unmet show <mission> document pointer, replacing the old strategy path with the converted mission document path or directory, and update the fate, mission and conversion counts. Alternatively, explicitly retain that show requirement in another open item and name it in a (c) duplicate reason. This is a record-only correction within Scope; no CLI change is requested here.

Verdict: shrink this one drop and re-review. Sizes: complexity medium for per-record judgments; value high because the cutover needs an accurate open-work set; risk high because a wrong drop silently loses work. Contract: fail on this surviving requirement; the census, protection, counts, links and executable conversion proof otherwise pass. Fitness: pass apart from the contract failure; no code or unnecessary moving parts were added. Round 6's atelier-lkuy correction preserves its document-pointer requirement.

Proof: independently counted all 181 open records at fc0fe395: 68 drop (43 declined, 25 superseded), 27 rewrite, 10 as is, 76 left for Sumeet, with no missing or duplicate report lines. Checked metadata, needs-human labels, explicit IDs, protected mission membership and file-by-file Git history: all 72 protected records remain unchanged except the permitted atelier-5zre updates, and all eight missions remain unchanged. Drop Reports match all 68 stated reasons. The Report explains the four open records linked to dropped ones. Independently confirmed 28 ready items, 11 in draft missions, three ready records without checkbox lines, and 126 deferred findings on done records with zero missing follow-ups (73 done, 11 backlog, 23 superseded, 19 declined). Mission membership and fate counts match. Read baseline dropped and kept records, rewrite diffs, protected drop proposals, governing specification and branch changes including the round 6 fix. Installed atelier 0.4.0 check passes with existing checkbox warnings. Both git diff --check fc0fe395 9b9af447 and the brief's non-record whitespace check pass. Converted 9b9af447 in a fresh scratch clone under ~/.cache/atelier-qr5s/5zre-review7/converted: 105 items (76 draft, 27 ready, two ready from decisions), eight missions (six draft, two ready); --verify matches 127 files and the supplied new binary's check passes. Scratch probes also confirm the mission-document omission, multi-error check, missing-ID hint, refused draft and blocked starts, retained multi-label and cycle needs, redirected rows and the removed command/flag forms relevant to the drops.

Open questions: none. Residual risk: fate decisions remain judgments, and protected-record proposals remain for Sumeet. No cargo builds, installs, cross-repository reads, pushes or merges were run. Approval withheld for R1.
