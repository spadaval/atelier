---
schema: atelier.review
schema_version: 1
id: atelier-t91g
title: 'Prior finding atelier-1ia7 is only half resolved. atelier-jjvc now exists and is blocked by sdb0, but its trigger ''Plan this into todo in the first release after the one that ships atelier-sdb0'' (and ADR:52 ''deprecated alias for one release'') depends on releases this project does not make: no git tags, a fixed workspace version 0.2.0, and CHANGELOG.md untouched since the initial commit, with legacy beads/SQLite entries. jjvc''s ''the CHANGELOG records the removal'' also points at that stale file. Failure: the alias persists indefinitely, the outcome the original finding warned about. Fix: give jjvc an observable trigger, e.g. this repo''s and astromech''s .claude/settings.json no longer call prime, or a date.'
target: atelier-bw0b
identity: Independent reviewer 2 (Claude subagent)
commit: 0db4b1845dab4da518fe334854b54f50a9a95345
created_at: 2026-10-16T23:50:25.813342300Z
event:
  event: finding
  recommendation: defer
---

Prior finding atelier-1ia7 is only half resolved. atelier-jjvc now exists and is blocked by sdb0, but its trigger 'Plan this into todo in the first release after the one that ships atelier-sdb0' (and ADR:52 'deprecated alias for one release') depends on releases this project does not make: no git tags, a fixed workspace version 0.2.0, and CHANGELOG.md untouched since the initial commit, with legacy beads/SQLite entries. jjvc's 'the CHANGELOG records the removal' also points at that stale file. Failure: the alias persists indefinitely, the outcome the original finding warned about. Fix: give jjvc an observable trigger, e.g. this repo's and astromech's .claude/settings.json no longer call prime, or a date.
