---
schema: atelier.review
schema_version: 2
id: atelier-nsp7
title: finding on atelier-8ep4 @f4175923
target: atelier-8ep4
identity: gpt-6.1-sol reviewer
commit: f41759232002d63dd6ff417fd4e7ded782faa78e
created_at: 2026-11-09T04:38:08.694415858Z
event:
  event: finding
  recommendation: fix_now
---

medium: .atelier/issues/atelier-8ep4.md:39 R2. The mandatory vacuous-test-review comparison does not reliably exercise the current skill: its setup.sh creates no Atelier repository, so SKILL.md tells the agent to stop using the skill when atelier status fails. atelier-wl6u already records this defect; earlier passes depended on reading review.md before stepping aside. Equal pass counts can therefore approve the role split without testing its reviewer instructions. Require the wl6u fixture repair before this comparison, or exclude that cell with an explicit limitation and use a valid role-scoped reviewer probe. Confirm each counted run actually uses the skill.
