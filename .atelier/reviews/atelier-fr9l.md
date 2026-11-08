---
schema: atelier.review
schema_version: 2
id: atelier-fr9l
title: finding on atelier-dkul @0fb7f857
target: atelier-dkul
identity: claude-code-reviewer (opus-5.5)
commit: 0fb7f8579e2767ceaa92a50ee1d7a603d0a613ed
created_at: 2026-11-08T22:38:05.934148040Z
event:
  event: finding
  recommendation: defer
---

medium: atelier land base check (pre-existing, reported in this item's Report > Surprises, not filed) - in the author's scratch trial a plain git merge --no-ff of the unreviewed import branch into master passed atelier land as '2 unpushed record-only commits outside review', while a direct code commit was refused. Impact: unreviewed code can reach master through a merge commit without land noticing, against AGENTS.md's merge policy. Not caused or worsened by this branch, which goes through review and CI. Fix: file a bug to make land classify a base merge commit by the files it brings in, not as record-only.
