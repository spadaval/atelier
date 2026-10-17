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
  relates:
  - kind: "issue"
    id: "atelier-bw0b"
    type: "related"
  - kind: "issue"
    id: "atelier-stsa"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "draft"
title: "Make mission plans and outcomes reviewable by humans and agents"
updated_at: "2026-10-17T00:12:05.366965251+00:00"
---

## Outcome

Implementers record the decisions they make and a short report when they finish. `atelier brief` and `atelier debrief` turn the records into complete reviews of a mission before it starts and after it completes: a self-contained HTML page a human absorbs quickly in T3 Code, and the same review as dense Markdown for agents. Humans make pre-work and post-mission calls by talking to an agent; agents answer "why is it like this?" from the same records. The strategy owns success criteria, shape, and authority.

## Background

Atelier captures proof and review well but almost none of the reasoning: at the start of this mission, 79 issues had 425 review events and 181 evidence receipts, but one decision record, three handoff notes, and short Close Reasons. Human review of plans and outcomes currently depends on an agent running several commands and paraphrasing. Sumeet chose the medium on 2026-10-16 after considering terminal Markdown, a T3 Code fork, and html_render: static HTML pages opened through T3's file panel for humans, dense Markdown for agents (ADR 0022, atelier-bw0b), no JSON, and no interactive UI.

## Notes

- 2026-10-16 [human] Strategy revised by human direction: brief and debrief may use full terminal styling (color, symbols, width-aware layout, links, paging) and emit plain Markdown when redirected. Interactive TUI remains out.
- 2026-10-17 [human] Strategy revised by human direction: humans read brief/debrief as static self-contained HTML pages under .atelier/runtime/, opened through T3 Code's file panel (no html_render re-emission, no T3 changes); agents get dense Markdown from the same view model per ADR 0022, with no JSON. Terminal-styled Markdown reviews and a T3 fork are dropped; progress views during a mission stay out of scope.
