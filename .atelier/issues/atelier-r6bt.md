---
created_at: "2026-10-16T23:30:44.986514082+00:00"
id: "atelier-r6bt"
issue_type: "mission"
labels:
- "review"
strategy:
  path: docs/strategies/review-and-introspection.md
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "draft"
title: "Make mission plans and outcomes reviewable by humans and agents"
updated_at: "2026-10-16T23:30:44.986514082+00:00"
---

## Outcome

Implementers record the decisions they make and a short report when they finish, and `atelier brief` and `atelier debrief` turn the records into complete Markdown reviews of a mission before it starts and after it completes. Humans make pre-work and post-mission calls from those documents; agents answer "why is it like this?" from the same records. The strategy owns success criteria, shape, and authority.

## Background

Atelier captures proof and review well but almost none of the reasoning: at the start of this mission, 79 issues had 425 review events and 181 evidence receipts, but one decision record, three handoff notes, and short Close Reasons. Human review of plans and outcomes currently depends on an agent running several commands and paraphrasing. The medium choice (CLI Markdown views plus conversation, no GUI or TUI) follows PRODUCT_INTENT.md and was agreed with Sumeet on 2026-10-16.
