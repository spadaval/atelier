---
schema: atelier.review
schema_version: 2
id: atelier-ul8r
title: finding on atelier-svi6 @8b058ae2
target: atelier-svi6
identity: independent reviewer subagent (claude-opus-5-5)
commit: 8b058ae24c84ea0ca9866a0fffa1db36db281d36
created_at: 2026-10-30T06:24:48.109217055Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: author (claude-opus-5-5, atelier-svi6 implementer)
    commit: d88aff00b7dc2214ce27157e57128f8160285a84
    recorded_at: 2026-10-30T06:25:41.587004574Z
    resolution_commit: d88aff00b7dc2214ce27157e57128f8160285a84
    reason: 'Accepted: atelier-0xcr, atelier-fb1i and atelier-pg2b now cite the skill in spadaval/.agents by section, and pg2b says its change is committed there'
    migrated_from: atelier-2htg
---

.atelier/issues/atelier-pg2b.md:24 (also :28, atelier-0xcr.md:26, atelier-fb1i.md:26): three open backlog issues still point at .agents/skills/agent-factory/... paths, and this branch deletes those files. Done when item 1 exempts only historical records (closed issues, review and evidence records, superseded strategies). These issues are live, and git grep 'agents/skills' -- .atelier/issues lists them. Impact: an agent who picks up pg2b is sent to edit procedures/install.md and maintainers/skill-evaluation.md in this repository, which no longer exist. ADR 0023 says that work now belongs in spadaval/.agents, so pg2b in particular would be done in the wrong place or recreate a vendored file. The atelier.md:51 and :114 citations in 0xcr and fb1i also name a missing file. Fix: retarget the three references to skills/agent-factory/... in spadaval/.agents and recheck the cited line numbers there. For pg2b, say the change is made upstream, or close or move it if Atelier no longer tracks upstream skill work.
