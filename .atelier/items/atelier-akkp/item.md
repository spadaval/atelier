---
schema: atelier.item
schema_version: 3
id: atelier-akkp
title: Re-measure record bookkeeping in adopting repositories after the lighter workflow ships
status: draft
mission: null
labels: []
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-06T22:08:25.310487900Z
---

## Goal

Check that the lighter workflow (mission atelier-n4mj) cut bookkeeping in real use, not only in the scenario test: measure the agent effort it was meant to cut on repositories using the released version: per merged item, the commands agents ran to record proof and review, the receipts recorded and re-recorded, and the refusals they had to clear; plus the findings-to-files count from the mission's Why section.

## Done when

- [ ] A note on this issue gives those measures, from records, git history and agent transcripts, for a window of at least three weeks after the repositories adopted the release, next to the same measures for 2026-10-15 to 2026-11-06 (the evidence audit in atelier-s66u's Background is the method for receipts and refusals), and files a bug for any that did not fall. Record-only commit counts may be reported but are not a target.

## Context

Pick this up three or more weeks after the release containing atelier-n4mj's changes has been adopted in atelier, astromech, demiurge, cosplay-studio or life-on-rails. life-on-rails adopted Atelier on 2026-11-06 and follows the evidence rule voluntarily, so it is also a comparison point from before the release. It is outside the mission because it waits on calendar time and adoption. The original counts came from a temporary script; atelier-5fus's and atelier-rkxm's receipts record reusable methods. 

## Links

Relationships of the old record that the new format has no field for.

- related: atelier-n4mj (Record what proves work, when it can be proven, and nothing more)
