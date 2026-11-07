---
schema: atelier.review
schema_version: 2
id: atelier-u24r
title: finding on atelier-8bh4 @70b45d46
target: atelier-8bh4
identity: independent reviewer subagent (claude-opus-5-5), atelier-8bh4
commit: 70b45d464142a56eba268b9270dc764ea7e13de8
created_at: 2026-11-07T09:40:04.347162255Z
event:
  event: finding
  recommendation: fix_now
---

low: docs/product/work-model.md:149 still says `atelier work ready` lists only work an agent can start, and :173 that ready work must expose a direct transition into active work; loose ends and validations now show in work ready (as Done when 3 requires) while start refuses them, so the doc contradicts the behaviour and the start_done_on_base refusal, and Done when 7 (docs/product match) is not met. Fix: say that work ready also lists items done on the base (loose ends, validations), which are done by their done transition rather than started.
