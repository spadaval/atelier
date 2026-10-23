---
created_at: "2026-10-23T17:59:10.851139283+00:00"
id: "atelier-wink"
evidence_type: "validation"
commit: "cd790a67a0fd34914afe25337408d4b88a439af4"
result: "pass"
captured_at: "2026-10-23T17:59:10.850836217+00:00"
agent_identity: "independent validator (Claude)"
target:
  kind: "issue"
  id: "atelier-1o56"
  role: "validates"
criteria:
- slug: "S5"
  result: "pass"
criteria_mission: "atelier-rc8j"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-1o56"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "S5 pass: Stop hook (echo payload | atelier land --hook claude): bad JSON, empty input, SessionStart payload and missing cwd give non-blocking systemMessage 'Atelier hook misconfigured: <error>; check .claude/settings.json'. Records-only dirty on base: systemMessage, no block (land still refuses); on a work branch it blocks. Pending review on own branch (no approval, or resolved fix_now awaiting approval) reports without blocking; an open finding, or a sibling branch, blocks. Newer record schema_version (land, hook, status, SessionStart) and newer policy say 'needs a newer atelier. Next: ... reinstall it with cargo install --path crates/atelier-cli --locked'. Finding (gap, medium): a payload cwd that exists but is outside any Atelier repo blocks the turn with 'Not an Atelier repository ... Run atelier init first. land: refused', and SessionStart gives 'Run atelier init first.. Run atelier check and repair the named records.' (double period, wrong advice ft88 meant to remove); ft88's decision covered only a cwd that cannot be entered."
updated_at: "2026-10-23T17:59:10.852295150+00:00"
---

S5 pass: Stop hook (echo payload | atelier land --hook claude): bad JSON, empty input, SessionStart payload and missing cwd give non-blocking systemMessage 'Atelier hook misconfigured: <error>; check .claude/settings.json'. Records-only dirty on base: systemMessage, no block (land still refuses); on a work branch it blocks. Pending review on own branch (no approval, or resolved fix_now awaiting approval) reports without blocking; an open finding, or a sibling branch, blocks. Newer record schema_version (land, hook, status, SessionStart) and newer policy say 'needs a newer atelier. Next: ... reinstall it with cargo install --path crates/atelier-cli --locked'. Finding (gap, medium): a payload cwd that exists but is outside any Atelier repo blocks the turn with 'Not an Atelier repository ... Run atelier init first. land: refused', and SessionStart gives 'Run atelier init first.. Run atelier check and repair the named records.' (double period, wrong advice ft88 meant to remove); ft88's decision covered only a cwd that cannot be entered.
