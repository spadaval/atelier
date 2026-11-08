---
schema: atelier.review
schema_version: 2
id: atelier-nsma
title: finding on atelier-l58v @f684caa1
target: atelier-l58v
identity: grok-plan-reviewer (grok-4.7)
commit: f684caa15b2bb4fe2a68878e52a27c3499a1d9df
created_at: 2026-11-08T05:49:45.052483324Z
event:
  event: finding
  recommendation: fix_now
---

high: The eval bar cannot be met as written, and it does not pin the binary it needs. The last Done when line requires the new skill to pass more runs than main on the first two scenarios, and says a scenario main already passes is no evidence per skills/agent-factory/maintainers/skill-evaluation.md (if the baseline does not fail, stop). If main passes all three, 'passes more' is impossible and 'no evidence' waives the bar the method says should stop the change. The four existing scenarios (refused-land-push, vacuous-test-review, done-when-rewrite, trivial-typo) are run only at the new skill, so 'no fewer on every other scenario' has no main count. '3 runs each with Opus on the skill arm' does not say the main ref uses Opus. The atelier.md line says to install the sibling item before evals, with no id, and this item does not block on atelier-stnv. wont_fix is not a decision the installed binary accepts (atelier review disposition --help lists fix_now, defer and no_action). Name atelier-stnv, link it blocked_by, run both refs with Opus three times, run the four existing scenarios at main too or cite the results file and SHA that is the baseline, and say a treatment scenario main already passes is no evidence and does not by itself let the item be done.
