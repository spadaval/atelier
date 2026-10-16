---
schema: atelier.review
schema_version: 1
id: atelier-x5b7
title: 'references/strategic-plans.md lines 9-10 (unchanged context of an edited paragraph) still say ''For smaller work, put the same sections in the mission issue body instead of a separate file''. ''The same sections'' means the strategy template''s sections (Why, Shape, Scope, Authority, Risks and Unknowns), but Atelier accepts only Outcome, Success criteria and Background in a mission body. In a scratch repo, ''atelier issue create --issue-type mission'' with a ''## Shape'' section fails with: Unknown issue body section ''Shape''. The sentence also contradicts tracker-records.md#mission (''do not add those sections to the mission''). Reword it to say that for smaller work the mission''s Outcome and Success criteria carry the strategy, and point to tracker-records.md#mission.'
target: atelier-92mz
identity: Claude Reviewer (independent subagent)
commit: 45419cf2bc4bb16a08d685853f12c9f28cf33dd5
created_at: 2026-10-16T18:14:23.902555216Z
event:
  event: finding
  recommendation: fix_now
---

references/strategic-plans.md lines 9-10 (unchanged context of an edited paragraph) still say 'For smaller work, put the same sections in the mission issue body instead of a separate file'. 'The same sections' means the strategy template's sections (Why, Shape, Scope, Authority, Risks and Unknowns), but Atelier accepts only Outcome, Success criteria and Background in a mission body. In a scratch repo, 'atelier issue create --issue-type mission' with a '## Shape' section fails with: Unknown issue body section 'Shape'. The sentence also contradicts tracker-records.md#mission ('do not add those sections to the mission'). Reword it to say that for smaller work the mission's Outcome and Success criteria carry the strategy, and point to tracker-records.md#mission.
