---
schema: atelier.review
schema_version: 1
id: atelier-xs5u
title: 'Decision 2 (ADR:53-58) makes loading AGENTS.md a requirement (''Claude Code repositories either have no CLAUDE.md or import @AGENTS.md from it''), and atelier-sdb0 removes prime''s ''Guidance: read AGENTS.md …'' line. But no follow-up makes adopters meet that requirement: sdb0 only edits Agent Factory procedures/install.md to say status (install.md:30-34 tells adopters to put guidance in AGENTS.md but never mentions CLAUDE.md or the import), and setup claude does not check it. Failure: an adopter with a plain CLAUDE.md (this repo''s state until this branch) loses its only automatic pointer to AGENTS.md once sdb0 ships. This is the remaining half of prior finding atelier-yzhp. Fix: add to sdb0''s Done when that install.md tells adopters to have CLAUDE.md import @AGENTS.md (or setup claude reports a CLAUDE.md without it).'
target: atelier-bw0b
identity: Independent reviewer 2 (Claude subagent)
commit: 0db4b1845dab4da518fe334854b54f50a9a95345
created_at: 2026-10-16T23:50:25.350050246Z
event:
  event: finding
  recommendation: fix_now
---

Decision 2 (ADR:53-58) makes loading AGENTS.md a requirement ('Claude Code repositories either have no CLAUDE.md or import @AGENTS.md from it'), and atelier-sdb0 removes prime's 'Guidance: read AGENTS.md …' line. But no follow-up makes adopters meet that requirement: sdb0 only edits Agent Factory procedures/install.md to say status (install.md:30-34 tells adopters to put guidance in AGENTS.md but never mentions CLAUDE.md or the import), and setup claude does not check it. Failure: an adopter with a plain CLAUDE.md (this repo's state until this branch) loses its only automatic pointer to AGENTS.md once sdb0 ships. This is the remaining half of prior finding atelier-yzhp. Fix: add to sdb0's Done when that install.md tells adopters to have CLAUDE.md import @AGENTS.md (or setup claude reports a CLAUDE.md without it).
