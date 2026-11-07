---
schema: atelier.review
schema_version: 2
id: atelier-bzic
title: finding on atelier-ml3r @0ac85467
target: atelier-ml3r
identity: independent reviewer subagent (claude-opus-5-5), atelier-ml3r
commit: 0ac8546728725631c173eec494bc745cfe33fc37
created_at: 2026-11-07T14:01:15.998896752Z
event:
  event: finding
  recommendation: fix_now
---

low: spadaval/.agents@fc5e3977 skills/agent-factory/constitution.md:45 and procedures/docs.md:25: the blanket check-to-test rewrite goes past the tracker terms the Words section scopes and changes meaning. The Constitution now says independent review and 'the automated tests are the only gate' where the owner reviews after merge, though that gate is CI and the merge checks (formatting, lockfile, CI status in this repo), the very sense of check the Words section keeps; docs.md asks for 'link tests' where a link check is the standard tool. Fix: 'the merge checks and CI are the only gate' and 'link checks'.
