---
schema: atelier.review
schema_version: 2
id: atelier-9mx5
title: 'crates/atelier-cli/src/commands/issue.rs (mission_strategy / update_lifecycle --no-strategy): two minor message inaccuracies. (1) A --strategy path that is a directory is refused with ''strategy file docs/s does not exist''. It is correctly refused, but the message is wrong. (2) `issue update <task> --no-strategy` on a record with no strategy prints ''Updated issue <id> (strategy)'' and rewrites updated_at, which reports a no-op as a change. Neither affects correctness or check. Optional: say ''is not a file'', and skip the changed field when no strategy was set.'
target: atelier-9m4k
identity: Independent reviewer (Claude subagent)
commit: e7a1106bc163441e8e6a5f5b9a53aed5e769c77b
created_at: 2026-10-16T00:00:38.615034131Z
event:
  event: finding
  recommendation: no_action
  disposition:
    decision: no_action
    identity: Claude Opus 5.5 (orchestrator)
    commit: e7a1106bc163441e8e6a5f5b9a53aed5e769c77b
    recorded_at: 2026-10-16T00:01:19.346347179Z
    reason: 'Accepted as recommended: both are message-only and the refusal or no-op is correct'
    migrated_from: atelier-patj
---

crates/atelier-cli/src/commands/issue.rs (mission_strategy / update_lifecycle --no-strategy): two minor message inaccuracies. (1) A --strategy path that is a directory is refused with 'strategy file docs/s does not exist'. It is correctly refused, but the message is wrong. (2) `issue update <task> --no-strategy` on a record with no strategy prints 'Updated issue <id> (strategy)' and rewrites updated_at, which reports a no-op as a change. Neither affects correctness or check. Optional: say 'is not a file', and skip the changed field when no strategy was set.
