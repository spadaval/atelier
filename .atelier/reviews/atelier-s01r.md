---
schema: atelier.review
schema_version: 1
id: atelier-s01r
title: 'docs/product/human-cli-output.md:180 — stale after the merge (782e27b5): it still promises ''the top mission in `work`'' as a next step naming a real ID. Bare `work` now prints clap help, and no other work view names the top mission (`work missions` footer is only ''Browse mission records: atelier issue list --issue-type mission''). A reader of the governing output contract expects `atelier work` to point at `atelier work mission <top-id>` and gets none. Fix: drop that clause (or move the top-mission step to `work missions`''s footer and say so).'
target: atelier-od8b
identity: Independent reviewer (Claude subagent)
commit: 903423d35d443e1238acad167b032ccc50a7228f
created_at: 2026-10-17T08:07:03.456660601Z
event:
  event: finding
  recommendation: fix_now
---

docs/product/human-cli-output.md:180 — stale after the merge (782e27b5): it still promises 'the top mission in `work`' as a next step naming a real ID. Bare `work` now prints clap help, and no other work view names the top mission (`work missions` footer is only 'Browse mission records: atelier issue list --issue-type mission'). A reader of the governing output contract expects `atelier work` to point at `atelier work mission <top-id>` and gets none. Fix: drop that clause (or move the top-mission step to `work missions`'s footer and say so).
