---
schema: atelier.review
schema_version: 3
id: atelier-7r5a
title: finding on branch desk-plan-docs @07418b1b
branch: desk-plan-docs
identity: GPT-6.1 Sol (xhigh), independent Desk re-review
commit: 07418b1b66757d2b6f02ff7ba53b3504d10faf7e
created_at: 2026-11-11T02:36:03.308455795Z
event:
  event: finding
  recommendation: fix_now
---

R2 (medium) docs/strategies/redesign-architecture.md:1067-1068: Transition/Prepare still commands that the new workspace builds and serves no site. Its corrected Site section at 1012-1023 explicitly puts atelier-desk in that workspace and preserves its web app and CI. The same blanket deletion remains in redesign-3-cutover.md:83 (nothing in next/ builds or serves a site; all web jobs and the Node build leave CI) and atelier-oci2.md:34. If the Desk lands first, following those live cutover instructions removes or disables its independent build, contradicting the fixed direction that both missions run in parallel. Scope every preparation instruction to the old atelier-site and the tracker binary/release, and explicitly preserve atelier-desk, desk/ and their jobs through the move. This leaves atelier-rfpw and atelier-7kw4 only partially fixed. Recorded once on the branch because the conflicting architecture specification is a branch document.

Branch review: Verdict merge after corrections, since separating the human's product serves the fixed direction; approval withheld. Sizes: complexity medium (several specifications and a durable decision must agree), value high (the cutover and Desk need a consistent boundary), risk medium (wrong instructions can remove the parallel product, visibly and reversibly). Contract fail and Fitness fail for the contradiction above. Reviewed the own commits 63af769d, the merge f6111503 and fixes 07418b1b, the net diff, linked plan and repository contracts. Proof passed: git diff --check HEAD^ HEAD -- . ':!.atelier'; cargo fmt -- --check; cargo metadata --locked --format-version 1 > /dev/null; and git diff --check ad41a7a7 07418b1b -- . ':!.atelier'. No implementation builds, browser validation, phone-link test or T3 launch was run: these are planned evidence boundaries, not completed claims.
