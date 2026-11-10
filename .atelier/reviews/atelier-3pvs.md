---
schema: atelier.review
schema_version: 2
id: atelier-3pvs
title: finding on atelier-yptz @8426a78b
target: atelier-yptz
identity: gpt-6.1-sol
commit: 8426a78b209e2e384efbbfb36984fece4a45fee1
created_at: 2026-11-10T19:38:05.651998522Z
event:
  event: finding
  recommendation: fix_now
---

Planning review of 8426a78b. Verdict: merge after the FIX NOW findings are corrected and freshly reviewed; the three-item split serves step B without needing another command or record kind. Sizes: complexity high (mission state plus three workers sharing lifecycle and record contracts), value high (the mission requires this planning slice), risk low (next/ is uninstalled and the old workspace remains untouched). Contract: fail. Fitness: fail pending the shared persisted-format decision. R1, low severity: .atelier/issues/atelier-yptz.md:34 promises that set changes a draft or ready record's priority, mission and blocked-by, restricting only caused-by to items. But mission.md's fixed schema has none of priority, mission or blocked_by (docs/strategies/redesign-architecture.md, Records). A worker satisfying this literally must add mission fields the specifications do not name, or fail the Done when line; the reader also refuses unknown keys. Limit mission set to its existing editable fields (title and labels), keep priority/membership/blockers/caused-by item-only, and require unsupported mission field flags to refuse without writing. Recommendation: FIX NOW, because the ready item's acceptance contract contradicts the fixed record schema.
