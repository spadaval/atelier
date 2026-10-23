---
created_at: "2026-10-23T00:03:44.239918616+00:00"
id: "atelier-om3m"
evidence_type: "validation"
commit: "f3ee2581393146618dcb92e6bd5d971bdfb6110b"
result: "fail"
captured_at: "2026-10-23T00:03:44.239444141+00:00"
path: "/home/supadava/.cache/atelier-tmp/validate-k8h8/scripts/s8.sh"
agent_identity: "claude-validator"
target:
  kind: "issue"
  id: "atelier-k8h8"
  role: "validates"
criteria:
- slug: "S8"
  result: "fail"
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
title: "S8 claim: review costs no more ceremony than the change needs (low-priority follow-ups share one branch and one review; review request writes the brief; a follow-up records where it was found), and review keeps one standard for code and prose, shown by CLI tests and the Agent Factory review guidance. Behaviour passes: s8.sh (exit 0) puts three low-priority children with discovered_from on one parent branch (task/<parent>), 'atelier review request <parent>' prints a brief with each child's Done when, range, findings and approve command, one approval on the parent merges all three and land on main passes; issue show prints 'Discovered From'; a missing source is refused. Atelier enforces one standard (s9.sh: an unexempted prose-only README commit on main is refused with land_base_unreviewed) and docs/product/reviews-and-merge.md states it. FAIL, first concrete failure: the Agent Factory review guidance still exempts prose. .agents/skills/agent-factory/SKILL.md 'Independent Review' (lines 70-80): 'Every non-trivial code change is reviewed...' and 'Prose-only documentation is not code.'; constitution.md:39 and procedures/orchestrate.md:62,175 likewise require review only for code. Expected (human decision 2026-10-22, atelier-1mn2 supersede reason): prose gets independent review like code and only the trivial exemption skips it. Actual: an agent following the skill concludes a non-trivial prose-only change needs no review, which Atelier's land then refuses. Repro: sed -n 70,85p .agents/skills/agent-factory/SKILL.md at f3ee2581. Cause: defect in this change (S8 was reworded on 2026-10-22 but the vendored guidance was not updated; upstream origin/main 451c0c9 has the same text)."
updated_at: "2026-10-23T00:03:44.240994160+00:00"
---

S8 claim: review costs no more ceremony than the change needs (low-priority follow-ups share one branch and one review; review request writes the brief; a follow-up records where it was found), and review keeps one standard for code and prose, shown by CLI tests and the Agent Factory review guidance. Behaviour passes: s8.sh (exit 0) puts three low-priority children with discovered_from on one parent branch (task/<parent>), 'atelier review request <parent>' prints a brief with each child's Done when, range, findings and approve command, one approval on the parent merges all three and land on main passes; issue show prints 'Discovered From'; a missing source is refused. Atelier enforces one standard (s9.sh: an unexempted prose-only README commit on main is refused with land_base_unreviewed) and docs/product/reviews-and-merge.md states it. FAIL, first concrete failure: the Agent Factory review guidance still exempts prose. .agents/skills/agent-factory/SKILL.md 'Independent Review' (lines 70-80): 'Every non-trivial code change is reviewed...' and 'Prose-only documentation is not code.'; constitution.md:39 and procedures/orchestrate.md:62,175 likewise require review only for code. Expected (human decision 2026-10-22, atelier-1mn2 supersede reason): prose gets independent review like code and only the trivial exemption skips it. Actual: an agent following the skill concludes a non-trivial prose-only change needs no review, which Atelier's land then refuses. Repro: sed -n 70,85p .agents/skills/agent-factory/SKILL.md at f3ee2581. Cause: defect in this change (S8 was reworded on 2026-10-22 but the vendored guidance was not updated; upstream origin/main 451c0c9 has the same text).
