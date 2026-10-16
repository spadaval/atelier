---
schema: atelier.review
schema_version: 1
id: atelier-4cid
title: 'The .claude/settings.json Stop hook (''atelier land --hook claude'', ADR Consequence 1) runs at the end of every Claude Code turn in this repo, not once per session. Measured with ~/.cargo/bin/atelier (0.2.0+8e3b6c67): about 5.8 s per turn on a clean master or the bw0b worktree, within the 30 s timeout. In a scratch clone of master with one uncommitted .atelier/issues record, it returns decision:block ''land_records_uncommitted … commit canonical records before continuing''. So any turn that files an issue without committing is forced to continue, which pushes the agent to commit on master unasked (only a repeated stop gets the warning instead). It writes nothing and corrupts nothing (land is read-only), and this is the documented product behaviour (sessions-and-hooks.md). Enabling it repo-wide is the dogfooding decision, and the human should accept it knowingly. Follow-up: decide whether a Stop failure on the base with only uncommitted records should warn rather than block, and note the per-turn cost.'
target: atelier-bw0b
identity: Independent reviewer 2 (Claude subagent)
commit: 0db4b1845dab4da518fe334854b54f50a9a95345
created_at: 2026-10-16T23:50:25.697604966Z
event:
  event: finding
  recommendation: defer
---

The .claude/settings.json Stop hook ('atelier land --hook claude', ADR Consequence 1) runs at the end of every Claude Code turn in this repo, not once per session. Measured with ~/.cargo/bin/atelier (0.2.0+8e3b6c67): about 5.8 s per turn on a clean master or the bw0b worktree, within the 30 s timeout. In a scratch clone of master with one uncommitted .atelier/issues record, it returns decision:block 'land_records_uncommitted … commit canonical records before continuing'. So any turn that files an issue without committing is forced to continue, which pushes the agent to commit on master unasked (only a repeated stop gets the warning instead). It writes nothing and corrupts nothing (land is read-only), and this is the documented product behaviour (sessions-and-hooks.md). Enabling it repo-wide is the dogfooding decision, and the human should accept it knowingly. Follow-up: decide whether a Stop failure on the base with only uncommitted records should warn rather than block, and note the per-turn cost.
