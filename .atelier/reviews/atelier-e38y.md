---
schema: atelier.review
schema_version: 2
id: atelier-e38y
title: finding on atelier-l457 @bc807963
target: atelier-l457
identity: independent reviewer subagent (claude-opus-5-5), atelier-l457
commit: bc8079639a940804bbdf8528a72e098c84f45e62
created_at: 2026-11-07T11:17:30.958543247Z
event:
  event: finding
  recommendation: fix_now
---

low: docs/product/workflow-configuration.md:36 says "Until that commit, `status`, `check`, `land` and the Stop hook say there are no commits yet and name it." On an unborn branch `check` prints only "Check passed." (the test an_unborn_head_is_reported_and_holds_nothing_up asserts exactly that; reproduced in a scratch `git init -b trunk`), and `doctor`, which does name the install commit, is left out. sessions-and-hooks.md:47 has the right list, so the two product docs this change edits now disagree. Fix: "`status`, `land`, `doctor` and the Stop hook say there are no commits yet and name it; `check` passes."
