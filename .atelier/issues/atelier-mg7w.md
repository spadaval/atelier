---
created_at: "2026-10-17T20:10:21.682260002+00:00"
id: "atelier-mg7w"
issue_type: "task"
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
  - kind: "issue"
    id: "atelier-r6bt"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-04T23:31:35.700179187+00:00"
status: "done"
title: "Offer plan approval in the brief footer only when none is recorded"
updated_at: "2026-11-04T23:31:35.700179187+00:00"
---

## Goal

The brief's Next footer suggests recording a plan approval even after one exists (independent validation of atelier-r6bt, S5).

## Done when

With an approval recorded, `atelier brief` offers a different next step, shown by a CLI test.

## Notes

- 2026-11-04 [decision] After an approval, offer 'atelier issue transition <mission>' while the mission is in its workflow's initial status, else 'atelier work ready'; the HTML footer names the same step
  Reason: Listing the transitions works under any policy, where naming 'ready' would assume the starter workflow; once the mission has left draft the next move is its ready work

## Close Reason

The brief's next step now depends on whether a plan approval is recorded. Without one, the Markdown footer still offers `atelier review approve <mission>`. With one, it offers `atelier issue transition <mission>` while the mission is in its workflow's initial status (draft), and `atelier work ready` after that. One helper, `brief_next_step`, builds the step for the Markdown footer and the HTML page's footer, so the page no longer always says to record an approval. CLI tests in brief.rs cover every state in both forms. Before the change, two of them failed. docs/product/mission-reviews.md describes the footer.

### Deviations
The HTML footer changed as well as the Markdown footer. Done when names only `atelier brief`, but the page had the same always-approve text.

### Decisions
One decision note on this item: the step after approval lists the mission's transitions instead of naming `ready`.

### Surprises
None.

### Open items
None.
