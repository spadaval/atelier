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
closed_at: "2026-10-31T00:04:51.688766956+00:00"
status: "done"
title: "Tie review events to the review that produced them?"
updated_at: "2026-10-31T00:04:51.688766956+00:00"
---

## Context

Source: /home/supadava/.t3/scratch/2026-10-30-the-agent-factory-skill-has-0de9a47e/af-analysis/REPORT.md pattern 3, synth/S1.md and S3.md. This decision advances atelier-ll3p; the work it calls for is filed after it is decided.

## Question

Should Atelier connect an approval or finding to a dispatched review, so that an approval recorded under a reviewer's label for a head no reviewer saw is visible? Identity is self-declared and compared only with Git author names and emails (docs/product/reviews-and-merge.md). On this host every agent commits as the human, so any other string passes: the 2026-10-15 to 10-29 sessions show invented identities (`claude2`, "orchestrator (not a code author)"), an orchestrator re-pinning approvals under the reviewer's label ("approval re-pinned … substance as in …"), and authors accepting defer or no_action on their own findings under a new label. Building eval fixtures on 2026-10-30 confirmed the disposition side: `review disposition --identity` accepts any reviewer name, so an agent can record a defer as a reviewer it never dispatched. The skill now says who may decide; this asks what the tool should show.

## Options

Atelier is not access control. It trusts agents and nudges them toward the right thing; its checks catch honest mistakes and make shortcuts visible, and none of them tries to stop an agent determined to lie. Self-declared identity stays (PRODUCT_INTENT.md). Options are judged by how much they help a well-meaning agent and a human reading the record, for how little rule and ceremony.

- **Roles as a naming convention.** A delegation states the agent's role and the identity to record under, written `<role>/<instance>` such as `reviewer/af97d9a0`, with roles taken from Agent Factory's subskills. It is a convention: Atelier keeps no list of roles and refuses nothing for an unfamiliar or missing one, and the free-text `--identity` stays. `atelier start --identity` records who implemented the work, and the existing non-author check compares against that as well as Git authors, so it catches an agent accidentally approving its own work, which today it cannot because every agent commits as the human. Small: one optional field and one comparison.
- **Show who recorded what.** `merge --check` and the debrief show when one identity both recorded author-side events on an owner and approved or disposed of its findings. A fact for the reader; refuses nothing. Works better on top of the convention.
- **Review requests become records.** `review request` writes a request event that approvals name. More machinery, and an orchestrator that runs the request at a new head still passes. Rejected in spirit: it builds an audit trail for a problem better handled by trust and visibility.
- **Identity from Git, one identity per agent.** Changes how every agent commits and breaks `--identity`. Rejected in spirit: too heavy for what it shows.
- **Leave it entirely to the skill.** No product work; the non-author check stays unable to catch an honest mistake.

Recommendation: roles as a naming convention, which the human proposed on 2026-10-30, with the display added if the shortcuts persist. No role-based gates.

## Decision

Roles as a naming convention, decided by the human on 2026-10-30. A delegation states the agent's role and identity, written `<role>/<instance>` with roles taken from Agent Factory's subskills. Atelier keeps no list of roles and has no role-based gates; `--identity` stays free text and self-declared. `atelier start --identity` records who implemented the work and the existing non-author check compares against it, so an honest slip is caught. Existing review records keep their labels. Work: atelier-jzh9. The display of who recorded what is added later only if the shortcuts persist.

## Close Reason

Roles as a naming convention with no role gates, by the human's direction; work is atelier-jzh9.
