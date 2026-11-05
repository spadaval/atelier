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
    id: "atelier-43m1"
    type: "advances"
  - kind: "issue"
    id: "atelier-bw0b"
    type: "related"
  - kind: "issue"
    id: "atelier-et6q"
    type: "advances"
  - kind: "issue"
    id: "atelier-stsa"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-05T05:18:11.675287493+00:00"
status: "done"
title: "Make mission plans and outcomes reviewable by humans and agents"
updated_at: "2026-11-05T05:18:11.675287493+00:00"
---

## Outcome

Implementers record the decisions they make and a short report when they finish. `atelier brief` and `atelier debrief` turn the records into complete reviews of a mission before it starts and after it completes: a self-contained HTML page a human absorbs quickly in T3 Code, and the same review as dense Markdown for agents. Humans make pre-work and post-mission calls by talking to an agent; agents answer "why is it like this?" from the same records. The strategy owns success criteria, shape, and authority.

## Background

Atelier captures proof and review well but almost none of the reasoning: at the start of this mission, 79 issues had 425 review events and 181 evidence receipts, but one decision record, three handoff notes, and short Close Reasons. Human review of plans and outcomes currently depends on an agent running several commands and paraphrasing. Sumeet chose the medium on 2026-10-16 after considering terminal Markdown, a T3 Code fork, and html_render: static HTML pages opened through T3's file panel for humans, dense Markdown for agents (ADR 0022, atelier-bw0b), no JSON, and no interactive UI.

## Notes

- 2026-10-16 [human] Strategy revised by human direction: brief and debrief may use full terminal styling (color, symbols, width-aware layout, links, paging) and emit plain Markdown when redirected. Interactive TUI remains out.
- 2026-10-17 [human] Strategy revised by human direction: humans read brief/debrief as static self-contained HTML pages under .atelier/runtime/, opened through T3 Code's file panel (no html_render re-emission, no T3 changes); agents get dense Markdown from the same view model per ADR 0022, with no JSON. Terminal-styled Markdown reviews and a T3 fork are dropped; progress views during a mission stay out of scope.
- 2026-10-17 [human] 2026-10-17 Strategy revised by human direction after a backlog review: the implementer report extends Close Reason (Close Reason stays the required Outcome; other parts are policy-required and get the same approval-freshness exemption; input comes from the record body or a file, not a flag). Decisions need only choice and reason, reversible unless marked one-way, as a typed note. S6 reads status per ADR 0022 and limits the brief view to a count plus one-way decisions. Criterion naming is the mission's first decision (a decision record blocking S4 and S8). atelier-stsa joins the mission as S8. S7's candidate is the new orientation mission. One Git-derived record-history query serves debrief, S8 and atelier-majk. The HTML renderer uses its medium fully; a fixture test keeps the two forms' facts equal. Close Reason's content is now defined in docs/product/work-model.md.
- 2026-10-17 [plan] 2026-10-17: Sumeet directed implementation. Increments: atelier-fip8 (S1, S3), atelier-sfsq (S2), atelier-ho7o (et6q criteria and receipts) then atelier-stsa (S8), atelier-1mk4 (S6), atelier-rx22 (S4), atelier-acrt (S5), atelier-lfs7 (S7, validation). S7's named candidate atelier-1w8c had 5 of 7 items done before capture could land, so S7 may need a different mission (human's call).
- 2026-10-17 [result] 2026-10-17: all increments merged (fip8, sfsq, ho7o with stsa, 1mk4, rx22, acrt). Independent validation passed S1-S6 and S8 on master 0edf25bb (receipts atelier-syrp, qkk8, 8ctc, 9mk4, n3o5, 82x5, ms6z). S7 waits on the human's review of a real HTML debrief (atelier-lfs7); the mission stays open until then.
- 2026-11-04 [note] 2026-11-04 handoff: every item but atelier-lfs7 (S7, human) is done. Acceptance validation at 0a2ec1a7 passed S1-S3, S5, S6, S8 and failed S4 (atelier-obcr): (1) done children's reports were missing, fixed by atelier-ov97; (2) the debrief has no separate decision log with decision records first, by design since atelier-8bil, which left to the human whether timeline decisions satisfy S4. Needs Sumeet: reword S4 or ask for the log back; then S4 needs a fresh receipt.
- 2026-11-04 [human] 2026-11-04 Sumeet: S4 reworded to match the debrief's design (each decision shown where it was made, done children's reports included; work decisions lists the whole log). S7 moved from human to independent agent review of atelier-vt8q's debrief (atelier-lfs7). Human review comes after agent review and after merge, judges taste rather than detail, and gates nothing; Sumeet may still read the page once the mission closes.

## Close Reason

Implementers record typed decision notes and an implementer report in Close Reason, decision records link ADRs and supersede earlier ones, `status` and `issue show` surface current decisions, `work mission` marks each criterion from evidence, and `atelier brief` and `atelier debrief` render a mission's plan and outcome as dense Markdown and a self-contained HTML page. All eight criteria are proven: S1-S3, S5, S6 and S8 by independent validation at 0a2ec1a7 (atelier-5pz3, m265, hvtl, 7izq, hkm6, rbc1), S4 at 7518f516 (atelier-bdbd) and S7 by an independent agent review of atelier-vt8q's debrief at 7518f516 (atelier-btnc). Of the eight commits `work mission` counts after the 0a2ec1a7 receipts, three change only docs and five change only the debrief and its page (atelier-ov97, atelier-mgl0): decision notes, Close Reason, decision records, brief, status, issue show and work mission criterion parsing are untouched, and CI passed the full suite on them.

### Deviations
On 2026-11-04 Sumeet reworded S4 to match the debrief's design (each decision shown where it was made) and moved S7 from a human judgement to an agent review followed by his non-blocking read (atelier-3km4); the candidate became atelier-vt8q (atelier-s2r4 supersedes atelier-9y66). S4 failed at 0a2ec1a7 (atelier-obcr) and again at c73d55e8 (atelier-dn84), and the first S7 review failed (atelier-g8sg), adding atelier-mgl0 to the scope.

### Decisions
Decision records atelier-et6q, atelier-6306 and atelier-s2r4, and the item decision notes (`atelier work decisions atelier-r6bt`).

### Surprises
Agent review of the debrief page found accuracy and accessibility defects that the earlier human read of an orientation debrief had not: a planned item counted as filed by review, criteria cut to their first sentence, a report part never shown, and AA contrast failures in both palettes.

### Open items
Sumeet's read of the vt8q page (atelier-3km4). Debrief follow-ups in the backlog: 930x, j79t, idx3, okfy, wg6p, k7vp, r8om, les9, ztuq, 1hbk, lozn, rdsa, g21n, j0b9 and cagr.
