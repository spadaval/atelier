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

Should Atelier connect an approval or finding to a dispatched review, so that an approval recorded under a reviewer's label for a head no reviewer saw is visible? Identity is self-declared and compared only with Git author names and emails (docs/product/reviews-and-merge.md). On this host every agent commits as the human, so any other string passes: the 2026-10-15 to 10-29 sessions show invented identities (`claude2`, "orchestrator (not a code author)"), an orchestrator re-pinning approvals under the reviewer's label ("approval re-pinned … substance as in …"), and authors accepting defer or no_action on their own findings under a new label. Building eval fixtures on 2026-10-30 confirmed the disposition side: `review disposition --identity` accepts any reviewer name, so an agent can record a defer as a reviewer it never dispatched. The skill now says who may decide; this asks what the tool should show.

## Options

No option makes fabrication impossible: an agent can set any label, Git identity or request. They differ in which of the three patterns (invented identities, re-pinned approvals, authors disposing of their own findings) becomes visible without extra effort.

- **Review requests become records.** `review request` writes a request event pinning the head it briefed; approvals and findings name the request they answer; `merge --check` shows, without refusing, an approval whose pin no request covers. Covers an approval recorded at a head nobody was briefed on. Does not cover re-pinning when the orchestrator also runs `review request` at the new head, invented identities, or self-disposition. Costs one record kind and one flag.
- **Show who recorded what, no new records.** `merge --check` and the debrief show, per approval and per defer or no_action disposition, whether the same identity also recorded author-side events on the owner (notes, transitions, fix-now resolutions). Covers an orchestrator or author that writes notes and approves or disposes under one label. Misses a label used only once, so invented identities pass.
- **Identity from Git, one identity per agent.** The dispatcher gives each agent its own Git author identity, and Atelier takes a review event's identity from the author of the commit that adds it, refusing approvals and defer or no_action dispositions whose committing author also authored the code. Covers invented labels and self-disposition when agents keep to their own identity; re-pinning still passes if the orchestrator commits as the reviewer. Changes how every agent commits on this host and breaks the current `--identity` flag.
- **Leave identity to process.** Keep self-declared identity and rely on the skill. No product work, and none of the patterns becomes visible.

Recommendation: the second option first. It needs no new record kind or commit discipline and shows the most common form seen (one agent both writing and approving); revisit the third if invented labels persist.
