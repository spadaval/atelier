---
created_at: "2026-10-23T00:20:14.947840090+00:00"
id: "atelier-sig1"
evidence_type: "validation"
commit: "5b2efd7428984a236704c4af58d7019dd4f4c150"
result: "pass"
captured_at: "2026-10-23T00:20:14.947767104+00:00"
agent_identity: "claude-validator"
target:
  kind: "issue"
  id: "atelier-k8h8"
  role: "validates"
criteria:
- slug: "S8"
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
title: "Re-validated S8 on master 5b2efd74. Skill text read in full (SKILL.md core rules, scaling table, Independent Review, Disagreement; constitution.md; orchestrate.md; implement.md; review.md; finding-disposition.md): all say every non-trivial change, code or prose (docs, ADRs, plans), needs independent review; trivial exemption is the only exception; implement.md says docs changes need review; orchestrator's own prose needs review; no text limits review to code. Minor non-blocking wording: SKILL.md frontmatter description says 'agent-written code'; trivial examples list 'prose wording' (bounded by 'if you have to think, it is not trivial'); references/atelier.md says strategy edits may go on the base without a work branch, which concerns branching and does not state a review exemption, though a reader could take it as one. Command behaviour in scratch repo under TMPDIR revalidate-s8 with installed atelier: 3 low-priority children started onto one parent branch task/r-ojdt, closed, one review approve on the parent, one merge into main succeeded; atelier review request on the parent printed the reviewer brief listing each child's Done when; issue create --discovered-from showed 'Discovered from: issue r-ojdt'. Limits: did not re-run the repo CLI test suite; local scratch only, nothing pushed."
updated_at: "2026-10-23T00:20:14.948395004+00:00"
---

Re-validated S8 on master 5b2efd74. Skill text read in full (SKILL.md core rules, scaling table, Independent Review, Disagreement; constitution.md; orchestrate.md; implement.md; review.md; finding-disposition.md): all say every non-trivial change, code or prose (docs, ADRs, plans), needs independent review; trivial exemption is the only exception; implement.md says docs changes need review; orchestrator's own prose needs review; no text limits review to code. Minor non-blocking wording: SKILL.md frontmatter description says 'agent-written code'; trivial examples list 'prose wording' (bounded by 'if you have to think, it is not trivial'); references/atelier.md says strategy edits may go on the base without a work branch, which concerns branching and does not state a review exemption, though a reader could take it as one. Command behaviour in scratch repo under TMPDIR revalidate-s8 with installed atelier: 3 low-priority children started onto one parent branch task/r-ojdt, closed, one review approve on the parent, one merge into main succeeded; atelier review request on the parent printed the reviewer brief listing each child's Done when; issue create --discovered-from showed 'Discovered from: issue r-ojdt'. Limits: did not re-run the repo CLI test suite; local scratch only, nothing pushed.
