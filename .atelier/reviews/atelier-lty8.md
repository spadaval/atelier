---
schema: atelier.review
schema_version: 1
id: atelier-lty8
title: 'Resolved. require_committed_config (git.rs:114) refuses land (session.rs:301) and merge (merge.rs:100) when .atelier/config.toml raw bytes differ from HEAD or exist on only one side, before review_gate::evaluate. Config is always read from <repo_root>/.atelier/config.toml (StorageLayout::config_path), so paths.state_root does not relocate it. Raw-byte comparison matches the existing canonical-record convention (committed_snapshot bypasses filters), so a CRLF checkout fails closed consistently. Every None==None case (missing both sides, or non-UTF-8 on both) fails closed downstream: validate_link_types uses the empty custom set when the file is absent and errors on an unreadable one. start and check still read the working config, but neither integrates work, and land and merge re-check; prune reads only retention, which --retention-days already overrides, and its deletions are committed and reviewed. The test covers tracked-modified, untracked+ignored (HEAD without config) and restored; it passes.'
target: atelier-v44v
identity: Independent reviewer (Claude subagent)
commit: 40a0de63cd3897f24acbfcf963d1311137a6cf33
created_at: 2026-10-16T19:37:47.741759749Z
event:
  event: disposition
  finding: atelier-9kwv
  decision: fix_now
  supersedes: null
  resolution_commit: 496a92fb61d95e525061beedfa54fd43d187fdea
  follow_up: null
---

Resolved. require_committed_config (git.rs:114) refuses land (session.rs:301) and merge (merge.rs:100) when .atelier/config.toml raw bytes differ from HEAD or exist on only one side, before review_gate::evaluate. Config is always read from <repo_root>/.atelier/config.toml (StorageLayout::config_path), so paths.state_root does not relocate it. Raw-byte comparison matches the existing canonical-record convention (committed_snapshot bypasses filters), so a CRLF checkout fails closed consistently. Every None==None case (missing both sides, or non-UTF-8 on both) fails closed downstream: validate_link_types uses the empty custom set when the file is absent and errors on an unreadable one. start and check still read the working config, but neither integrates work, and land and merge re-check; prune reads only retention, which --retention-days already overrides, and its deletions are committed and reviewed. The test covers tracked-modified, untracked+ignored (HEAD without config) and restored; it passes.
