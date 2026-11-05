---
created_at: "2026-10-17T18:07:05.623651933+00:00"
id: "atelier-ypks"
issue_type: "bug"
labels:
- "review"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-r6bt"
    type: "advances"
  - kind: "review"
    id: "atelier-hmta"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-05T01:23:47.858901794+00:00"
status: "done"
title: "Clear or confirm a reused Close Reason when a reopened item closes again"
updated_at: "2026-11-05T01:23:47.858901794+00:00"
---

## Goal

A record that leaves done keeps its Close Reason, and the close transition with no flag reuses it, so a reopened item closed again silently carries its old Outcome and report; `--reason` on supersede also keeps report parts written for a close. Found by review atelier-hmta on atelier-sfsq.

## Done when

A transition out of a done status clears Close Reason (or the close transition says which Close Reason it used), and `--reason` on supersede does not keep report parts written for a close, shown by transition tests that reopen and reclose an item and that supersede an item whose record holds a report.

## Notes

- 2026-11-04 [decision] The close names the Close Reason it kept instead of a transition clearing it
  Reason: Policy validation refuses any transition out of a terminal status (workflow_config_invalid_transition), so an item reopens only by a hand edit, which no transition sees; clearing on leaving done would be dead code
- 2026-11-04 [decision] A --reason Outcome keeps report parts only when the transition requires a report part
  Reason: required_fields is the policy's own signal for which transitions carry a report; naming supersede in code would not follow a custom policy

## Close Reason

A close with no flag now prints `Close Reason: kept from the record: <Outcome>`, so a reclose after a reopen names the old Close Reason it reused, and `--reason` on a transition that requires no report part (supersede) replaces the whole Close Reason instead of keeping report parts written for a close.

### Deviations
Done when offered clearing Close Reason on a transition out of done, or having the close say which Close Reason it used. I took the second only: policy validation refuses any transition out of a terminal status, so the reopen test reopens by a hand edit, which no transition can see.

### Decisions
Two decision notes on this item: the close reports what it kept rather than a transition clearing it, and `--reason` keeps report parts only when the transition's `required_fields` name a report part. docs/product/work-model.md now says both.

### Surprises
No workflow can define a transition out of a done status (`workflow_config_invalid_transition`), so the reviewer's "policy transition out of done" path does not exist.

### Open items
None.
