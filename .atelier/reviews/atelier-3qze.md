---
schema: atelier.review
schema_version: 2
id: atelier-3qze
title: finding on atelier-l58v @b7b60ea6
target: atelier-l58v
identity: claude-code-rereviewer (opus-5.5)
commit: b7b60ea65035801761ab6e402448991190f4697e
created_at: 2026-11-08T22:24:17.857125967Z
event:
  event: finding
  recommendation: fix_now
---

medium: skills/agent-factory/procedures/orchestrate.md:76-77 (agents-skills 12c8a15) against Done when line 3 as rewritten at 93a797a6. The ticked line says 'fix_now means only that the work cannot merge without the fix and never records a scope choice; a deferred finding fixed in passing stays deferred or is recorded as fixed'. No file in the skill at 12c8a15 says either: orchestrate.md now says only 'Fix trivial issues. For larger ones, decide based on how much they grow the change.', and references/atelier.md:33-38 lists dispositions defer (with a follow-up), wont_fix and no_action, with no way to record a finding as fixed. The binary (0.3.1+44cbe69c) has none either: review disposition --help offers fix_now, defer, no_action, wont_fix, and a defer-recommended finding needs one before merge. So an orchestrator that fixes a trivial deferred finding must pick one: relabel it fix_now (the help lists it, and 'needs no disposition' makes it the easy path), which is what Sumeet forbids; run the one-step 'atelier review defer', which the same paragraph tells it to use and which files a backlog follow-up for work already done, the backlog growth this item's Context names; or call a real, fixed finding wont_fix or no_action, which misstates it. Secondary: 'Fix trivial issues' and the override rule two sentences later ('a new finding on the fix of a low-severity finding is deferred') disagree on a trivial new finding on such a fix; say which wins. Fix: one or two sentences in orchestrate.md or atelier.md's step 5 saying fix_now is only for 'cannot merge without it', and exactly what to record for a deferred finding fixed in passing with today's binary (e.g. defer to a follow-up then close it done with the fixing commit, or whatever Sumeet picks), plus a follow-up if a 'fixed' disposition belongs in the binary.
