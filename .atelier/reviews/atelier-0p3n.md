---
schema: atelier.review
schema_version: 2
id: atelier-0p3n
title: finding on atelier-z5bp @c96db9f5
target: atelier-z5bp
identity: sol-reviewer-r1
commit: c96db9f52afd084701076406cabce5f4ba7bebbf
created_at: 2026-11-10T14:37:16.373966174Z
event:
  event: finding
  recommendation: fix_now
---

low: docs/architecture/quality/scorecard-baseline/defect-method.md:90 R2: Step 3 searches only the file changed by the fix, but that procedure does not reproduce row 6. For 7fb74f89, the documented expression search on the changed file returns 1ad98b21 and 8d31ef4f, not the claimed cause ab6ad12c. The earlier check can be found under a different file, which the method never tells a tracer to inspect. Add an explicit way to follow moved or copied logic across files and document the cross-file step for row 6, then rerun it. Otherwise a fresh tracer can stop at a different introducing item despite following the written instructions. The attribution may be reasonable; the missing step is the finding. Reviewed branch-only commits 649aafc8..c96db9f5, repeated all 30 blame/date checks locally within ae4d18ba, and inspected fix/cause changes. No runtime code or unnecessary implementation layer was added.
