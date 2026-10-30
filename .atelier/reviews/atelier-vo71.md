---
schema: atelier.review
schema_version: 1
id: atelier-vo71
title: finding on atelier-svi6 @e2f061b0
target: atelier-svi6
identity: independent reviewer subagent 2 (claude-opus-5-5)
commit: e2f061b03733d715bb2a88b970569eb139f6b614
created_at: 2026-10-30T06:28:57.947718106Z
event:
  event: finding
  recommendation: fix_now
---

low: .atelier/issues/atelier-fb1i.md:31 (open, backlog) still says 'references/atelier.md:114 states the actual behaviour. Agent Factory is vendored, so the edit goes upstream per ADR 0020.' d88aff00 repointed only the Goal line (:26). Finding atelier-ul8r asked to recheck the cited line numbers, and disposition atelier-2htg says the three issues now cite the skill by section. That is not true of this Done when line. The claim that Agent Factory is vendored contradicts ADR 0023, and :114 matches nothing in the canonical file: the check sentence is at references/atelier.md:168 in the local spadaval/.agents main and :128 on origin/main, under Administration. Impact: the agent implementing fb1i is told a vendored copy exists and given a dead line anchor, so atelier-ul8r is only partly resolved and the close reason's Deviations overstates the fix. Fix (one line): cite the skill's references/atelier.md Administration section in spadaval/.agents, drop 'is vendored' and say the edit is committed in spadaval/.agents per ADR 0023.
