---
schema: atelier.review
schema_version: 2
id: atelier-9kwv
title: 'Item 1''s acceptance rationale in .atelier/issues/atelier-v44v.md (Design, item 1) is false: an uncommitted .atelier/config.toml does change a gate result. RecordIndex::validate_link_types (crates/atelier-app/src/record_index.rs:815-834) loads custom_context_types from the working-tree config via ProjectConfig::load, and land and check go through it. Reproduced in a scratch repo: on an owned work branch, add custom_context_types = ["inspired_by"] to config.toml without committing it, run ''atelier issue link <a> <b> --role inspired_by'', and commit only .atelier/issues. ''atelier land'' then prints ''Landing checks passed'' (with only an ''Unfinished checkout: 1 uncommitted path'' note) and ''atelier check'' passes. With the config change stashed, so the committed state is checked, both fail with workflow_issue_link_type_invalid. Prune retention (main.rs:896) also reads the working config. merge is protected only because read_record_files(repo, true) refuses a dirty tracked config.toml; an untracked, ignored config.toml (the layout the committed_snapshot test uses) would pass merge too. Fix: either make land validate link types against the committed config, or correct the Design record to say what is accepted (land''s link-type check and prune read local config; merge refuses a dirty tracked config) and drop the ''none of which bypass a gate'' claim.'
target: atelier-v44v
identity: Independent reviewer (Claude subagent)
commit: 656cef00ae542b1bad641b1c8392e5ad37dd5069
created_at: 2026-10-16T19:30:51.956055224Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Independent reviewer (Claude subagent)
    commit: 40a0de63cd3897f24acbfcf963d1311137a6cf33
    recorded_at: 2026-10-16T19:37:47.741759749Z
    resolution_commit: 496a92fb61d95e525061beedfa54fd43d187fdea
    reason: 'Resolved. require_committed_config (git.rs:114) refuses land (session.rs:301) and merge (merge.rs:100) when .atelier/config.toml raw bytes differ from HEAD or exist on only one side, before review_gate::evaluate. Config is always read from <repo_root>/.atelier/config.toml (StorageLayout::config_path), so paths.state_root does not relocate it. Raw-byte comparison matches the existing canonical-record convention (committed_snapshot bypasses filters), so a CRLF checkout fails closed consistently. Every None==None case (missing both sides, or non-UTF-8 on both) fails closed downstream: validate_link_types uses the empty custom set when the file is absent and errors on an unreadable one. start and check still read the working config, but neither integrates work, and land and merge re-check; prune reads only retention, which --retention-days already overrides, and its deletions are committed and reviewed. The test covers tracked-modified, untracked+ignored (HEAD without config) and restored; it passes.'
    migrated_from: atelier-lty8
---

Item 1's acceptance rationale in .atelier/issues/atelier-v44v.md (Design, item 1) is false: an uncommitted .atelier/config.toml does change a gate result. RecordIndex::validate_link_types (crates/atelier-app/src/record_index.rs:815-834) loads custom_context_types from the working-tree config via ProjectConfig::load, and land and check go through it. Reproduced in a scratch repo: on an owned work branch, add custom_context_types = ["inspired_by"] to config.toml without committing it, run 'atelier issue link <a> <b> --role inspired_by', and commit only .atelier/issues. 'atelier land' then prints 'Landing checks passed' (with only an 'Unfinished checkout: 1 uncommitted path' note) and 'atelier check' passes. With the config change stashed, so the committed state is checked, both fail with workflow_issue_link_type_invalid. Prune retention (main.rs:896) also reads the working config. merge is protected only because read_record_files(repo, true) refuses a dirty tracked config.toml; an untracked, ignored config.toml (the layout the committed_snapshot test uses) would pass merge too. Fix: either make land validate link types against the committed config, or correct the Design record to say what is accepted (land's link-type check and prune read local config; merge refuses a dirty tracked config) and drop the 'none of which bypass a gate' claim.
