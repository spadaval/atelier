---
created_at: "2026-10-23T00:03:10.959489526+00:00"
id: "atelier-dueg"
evidence_type: "validation"
commit: "f3ee2581393146618dcb92e6bd5d971bdfb6110b"
result: "pass"
captured_at: "2026-10-23T00:03:10.959208653+00:00"
path: ".agents/skills/agent-factory/procedures/orchestrate.md"
agent_identity: "claude-validator"
target:
  kind: "issue"
  id: "atelier-k8h8"
  role: "validates"
criteria:
- slug: "S2"
  result: "pass"
criteria_mission: "atelier-k8h8"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-k8h8"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "S2 claim: Agent Factory orchestrate guidance says to update a waiting branch only when it is next to land and to check generated files mechanically, in vendored and upstream skill text. Method: read .agents/skills/agent-factory/procedures/orchestrate.md section 'Landing Parallel Branches' at f3ee2581: 'Leave waiting branches alone when the base moves', 'When a branch is next to land: 1. merge the base into it once; 2. regenerate generated files, such as lockfiles...', the merge_commit exception for clean merges whose merge checks run the proof, and 'Check generated files mechanically... run cargo metadata --locked in every workspace that has its own Cargo.lock'. Upstream: the local clone /home/supadava/projects/agents-skills (origin git@github.com:spadaval/.agents.git, last fetched 2026-10-22 19:27) has the same section byte-identical at origin/main 451c0c9 (skills/agent-factory/procedures/orchestrate.md). Limits: upstream checked only through that clone's last fetch, no network fetch; other vendored skill edits from this mission (atelier.md, SKILL.md, tracker-records.md, etc.) differ from upstream and are tracked by atelier-k4xg, outside S2's wording."
updated_at: "2026-10-23T00:03:10.960714369+00:00"
---

S2 claim: Agent Factory orchestrate guidance says to update a waiting branch only when it is next to land and to check generated files mechanically, in vendored and upstream skill text. Method: read .agents/skills/agent-factory/procedures/orchestrate.md section 'Landing Parallel Branches' at f3ee2581: 'Leave waiting branches alone when the base moves', 'When a branch is next to land: 1. merge the base into it once; 2. regenerate generated files, such as lockfiles...', the merge_commit exception for clean merges whose merge checks run the proof, and 'Check generated files mechanically... run cargo metadata --locked in every workspace that has its own Cargo.lock'. Upstream: the local clone /home/supadava/projects/agents-skills (origin git@github.com:spadaval/.agents.git, last fetched 2026-10-22 19:27) has the same section byte-identical at origin/main 451c0c9 (skills/agent-factory/procedures/orchestrate.md). Limits: upstream checked only through that clone's last fetch, no network fetch; other vendored skill edits from this mission (atelier.md, SKILL.md, tracker-records.md, etc.) differ from upstream and are tracked by atelier-k4xg, outside S2's wording.
