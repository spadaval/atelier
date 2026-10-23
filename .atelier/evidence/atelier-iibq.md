---
created_at: "2026-10-23T17:59:08.653486112+00:00"
id: "atelier-iibq"
evidence_type: "validation"
commit: "cd790a67a0fd34914afe25337408d4b88a439af4"
result: "pass"
captured_at: "2026-10-23T17:59:08.653172657+00:00"
agent_identity: "independent validator (Claude)"
target:
  kind: "issue"
  id: "atelier-1o56"
  role: "validates"
criteria:
- slug: "S3"
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
title: "S3 pass: evidence record prints one confirmation line '<id> ✓ pass test for issue/<id> @<pin>: <cmd> (exit 0)' plus the verdict line; a fail adds the stderr tail line and 'recorded fail — <id> cannot close or merge until a later pass' (exit 0, per decision). A 150 KB failing capture kept the first 1 KiB and last 3 KiB with '… N bytes omitted …', including the final stdout line and stderr; a 134 KB passing capture stored bytes, SHA-256 and the test-summary line only (1 KB receipt). evidence list <id> filters by item with freshness marks and a Proof line; evidence show is 3-4 lines. git diff --check clean on scratch receipts; all 66 repo receipts created after the dg3h merge end in one newline (the 13 with a trailing blank were made before it on parallel branches). Finding (defect in this change, low-medium): when the item's record was uncommitted at the pin, list/show/record say 'stale (pin not in HEAD's history)' though the pin is HEAD~1; gate verdict agrees it is stale, only the reason is wrong (evidence_freshness.rs staleness maps any first_mismatch error to NotInHistory)."
updated_at: "2026-10-23T17:59:08.654885273+00:00"
---

S3 pass: evidence record prints one confirmation line '<id> ✓ pass test for issue/<id> @<pin>: <cmd> (exit 0)' plus the verdict line; a fail adds the stderr tail line and 'recorded fail — <id> cannot close or merge until a later pass' (exit 0, per decision). A 150 KB failing capture kept the first 1 KiB and last 3 KiB with '… N bytes omitted …', including the final stdout line and stderr; a 134 KB passing capture stored bytes, SHA-256 and the test-summary line only (1 KB receipt). evidence list <id> filters by item with freshness marks and a Proof line; evidence show is 3-4 lines. git diff --check clean on scratch receipts; all 66 repo receipts created after the dg3h merge end in one newline (the 13 with a trailing blank were made before it on parallel branches). Finding (defect in this change, low-medium): when the item's record was uncommitted at the pin, list/show/record say 'stale (pin not in HEAD's history)' though the pin is HEAD~1; gate verdict agrees it is stale, only the reason is wrong (evidence_freshness.rs staleness maps any first_mismatch error to NotInHistory).
