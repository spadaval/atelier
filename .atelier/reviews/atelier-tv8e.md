---
schema: atelier.review
schema_version: 2
id: atelier-tv8e
title: finding on atelier-l58v @9ea0a19b
target: atelier-l58v
identity: claude-code-reviewer (opus-5.5)
commit: 9ea0a19b4fb5a0f3488d5efd40e58bbaeff0d91b
created_at: 2026-11-08T21:49:01.579323337Z
event:
  event: finding
  recommendation: fix_now
---

low: records accuracy. (1) evals/agent-factory/results/2026-11-08-saying-no.md:161 says the eb5fffd plan-defect rerun used 'the same installed binary'; the installed atelier was rebuilt at 44cbe69c at 13:34 and the rerun ran at 16:06, so it used 0.3.1+44cbe69c, not 4910be45 (Done when line 9 asks for the same binary; the result likely stands, but the claim is false). (2) The item Report's Deviations say the cuts were 'older rules already stated in another file', but review.md's without-Atelier record format (R1 FIX NOW: <reason> ...) is stated nowhere else in the skill after 23a6bed; say it was dropped. (3) Merged with agents-skills main c2ed148 (+3 lines in references/atelier.md), the paths are review 322/300, orchestrate 465/450 and plan 306/300; the Report's counts and overrun reasons cover only the branch (319/462/303). Give the merged counts. Fix the sentences in the results file and the Report alongside the R1 rerun.
