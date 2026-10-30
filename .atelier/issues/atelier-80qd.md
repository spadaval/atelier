---
created_at: "2026-10-30T06:40:37.260576853+00:00"
id: "atelier-80qd"
issue_type: "decision"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-ll3p"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Tie review events to the review that produced them?"
updated_at: "2026-10-30T06:40:37.260576853+00:00"
---

## Context

Source: /home/supadava/.t3/scratch/2026-10-30-the-agent-factory-skill-has-0de9a47e/af-analysis/REPORT.md pattern 3, synth/S1.md and S3.md. This decision advances atelier-ll3p; the work it calls for is filed after it is decided.

## Question

Should Atelier connect an approval or finding to a dispatched review, so that an approval recorded under a reviewer's label for a head no reviewer saw is visible? Identity is self-declared and compared only with Git author names and emails (docs/product/reviews-and-merge.md). On this host every agent commits as the human, so any other string passes: the 2026-10-15 to 10-29 sessions show invented identities (`claude2`, "orchestrator (not a code author)"), an orchestrator re-pinning approvals under the reviewer's label ("approval re-pinned … substance as in …"), and authors accepting defer or no_action on their own findings under a new label. The skill now says who may decide; this asks what the tool should show.

## Options

- **Review requests become records.** `review request` writes a request event pinning the head it briefed. Approvals and findings name the request they answer, and `merge --check` shows, without refusing, an approval whose pin no request covers or whose identity differs from the request's. Catches re-pinning and label swaps. Costs one record kind and one flag, and an orchestrator that fabricates both the request and the approval still passes.
- **Show who recorded what, no new records.** `merge --check` and the debrief show, per approval and per defer or no_action disposition, whether the same identity also recorded author-side events on the owner (notes, transitions, fix-now resolutions). Cheaper, catches an orchestrator that writes notes and approves under one label, misses a label used only once.
- **Leave identity to process.** Keep self-declared identity and rely on the skill. No product work, and the patterns stay invisible to the human.

Recommendation: the first, shown as a fact rather than enforced. Re-pinning was the most consequential pattern, and only a record of what was briefed can show it.
