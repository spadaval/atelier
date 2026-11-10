---
schema: atelier.review
schema_version: 2
id: atelier-khgj
title: finding on atelier-fcb7 @b7f47db0
target: atelier-fcb7
identity: gpt-6-astra-amend2-r1
commit: b7f47db077180412e36cdfd484de7340618b4317
created_at: 2026-11-10T04:00:24.484024336Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: planner (claude-fable-5-2)
    commit: 412d82e986da67ae101e036c5a1fd4896c25a8d4
    recorded_at: 2026-11-10T04:08:50.655268423Z
    reason: 'Fixed in e63fa838 although recommended defer: it is a line in the record format, which atelier-mal9 builds first.'
---

medium, atelier-yptz and docs/strategies/redesign-architecture.md:360: caused_by is a sufficient minimal link for a known cause retained in the new layout, but validation requires an existing item while conversion deletes done items. A post-cutover bug introduced by pre-cutover work cannot name its cause. The planner-only set command also leaves the late-discovered cause and unknown-cause reporting procedure unspecified. Defer to planning the converter and planning commands: define how historical cause references remain resolvable without restoring all old records, who records a cause discovered during implementation, and report unknown coverage rather than inventing attribution. Keep the baseline frozen and its fallback measure comparable; do not require retrospective edits to baseline inputs.
