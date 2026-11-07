---
schema: atelier.review
schema_version: 2
id: atelier-x5b7
title: 'references/strategic-plans.md lines 9-10 (unchanged context of an edited paragraph) still say ''For smaller work, put the same sections in the mission issue body instead of a separate file''. ''The same sections'' means the strategy template''s sections (Why, Shape, Scope, Authority, Risks and Unknowns), but Atelier accepts only Outcome, Success criteria and Background in a mission body. In a scratch repo, ''atelier issue create --issue-type mission'' with a ''## Shape'' section fails with: Unknown issue body section ''Shape''. The sentence also contradicts tracker-records.md#mission (''do not add those sections to the mission''). Reword it to say that for smaller work the mission''s Outcome and Success criteria carry the strategy, and point to tracker-records.md#mission.'
target: atelier-92mz
identity: Claude Reviewer (independent subagent)
commit: 45419cf2bc4bb16a08d685853f12c9f28cf33dd5
created_at: 2026-10-16T18:14:23.902555216Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Claude Reviewer (independent subagent)
    commit: 17f3f1b812d248e79c863923d8bbe04f4eba424f
    recorded_at: 2026-10-16T18:15:22.247369469Z
    resolution_commit: 341c358bc87e74472ad7176c1a86c3ade0ad4ed3
    reason: Verified. 341c358b changes only strategic-plans.md lines 8-11, which now say that for smaller work the mission's Outcome and Success criteria carry the strategy, with a link to tracker-records.md#mission (the anchor resolves). The vendored skill is identical (diff -r) to the agents-skills worktree and to a git archive of 3cda6b7. A fresh grep of the whole skill finds no other text telling agents to put non-mission sections in a mission body; plan.md and decide.md mention the mission issue but name no sections. atelier check passes at HEAD 17f3f1b8.
    migrated_from: atelier-08rf
---

references/strategic-plans.md lines 9-10 (unchanged context of an edited paragraph) still say 'For smaller work, put the same sections in the mission issue body instead of a separate file'. 'The same sections' means the strategy template's sections (Why, Shape, Scope, Authority, Risks and Unknowns), but Atelier accepts only Outcome, Success criteria and Background in a mission body. In a scratch repo, 'atelier issue create --issue-type mission' with a '## Shape' section fails with: Unknown issue body section 'Shape'. The sentence also contradicts tracker-records.md#mission ('do not add those sections to the mission'). Reword it to say that for smaller work the mission's Outcome and Success criteria carry the strategy, and point to tracker-records.md#mission.
