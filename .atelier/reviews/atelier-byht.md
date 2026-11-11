---
schema: atelier.review
schema_version: 2
id: atelier-byht
title: finding on atelier-94pv @1592c02a
target: atelier-94pv
identity: independent-planning-reviewer
commit: 1592c02ae0255e694e3183ef3ebdaff49975a9ef
created_at: 2026-11-11T05:25:48.782482757Z
event:
  event: finding
  recommendation: fix_now
---

Low: .atelier/issues/atelier-94pv.md:34 and :52 leave work outside missions ambiguous. The carried-over spike rail links to /r/:repo/loose, and the referenced atelier-1ie1 Report says that page is changed to use list groups with no mission. But docs/architecture/desk.md specifies loose as only a count, no route enumerates those items, and the four-page acceptance test never opens the rail link. A repository with only ordinary loose work can therefore get a nonzero count and a dead or empty destination while all listed tests pass. FIX NOW: state whether this increment offers only a non-clickable count or an actual loose-work destination. For the latter, explicitly require its item rows from the existing list view in the API (the app scope already permits adding fields), its route and a fixture test opening the rail link; align the API document. This is a bounded clarification within the approved page-layout authority, not a request for another subsystem.
