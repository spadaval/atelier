---
created_at: "2026-11-05T00:19:47.406255507+00:00"
id: "atelier-uqro"
evidence_type: "validation"
commit: "6200bbe6084d364f6dfa7233b2d6a6d9e95a4660"
result: "pass"
captured_at: "2026-11-05T00:19:47.406159908+00:00"
uri: "https://github.com/spadaval/atelier/actions/runs/41934968679"
target:
  kind: "issue"
  id: "atelier-l7gl"
  role: "validates"
criteria:
- slug: "S2"
  result: "pass"
criteria_mission: "atelier-vt8q"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-l7gl"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "S2 as reworded: v0.3.1's tag run 41931151380 published the release with all four platform archives, checksums and the installer, and its x86_64 archives report 0.3.1+d14d2e5 (validator receipt atelier-edle); this PR's Release run 41934968679 (linked) ran only the plan job, with build, host and announce skipped (reviewer atelier-3uww). Limit: the release build on the plan-mode workflow is first exercised by the next tag."
updated_at: "2026-11-05T00:19:47.406728349+00:00"
---

S2 as reworded: v0.3.1's tag run 41931151380 published the release with all four platform archives, checksums and the installer, and its x86_64 archives report 0.3.1+d14d2e5 (validator receipt atelier-edle); this PR's Release run 41934968679 (linked) ran only the plan job, with build, host and announce skipped (reviewer atelier-3uww). Limit: the release build on the plan-mode workflow is first exercised by the next tag.
