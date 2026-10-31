---
created_at: "2026-10-30T06:52:05.478210610+00:00"
id: "atelier-nmvv"
issue_type: "feature"
labels: []
priority: "P1"
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
status: "backlog"
title: "Show when a command-backed fail is cleared by anything but a pass of the same command"
updated_at: "2026-10-30T06:52:05.478210610+00:00"
---

## Goal

A failed measurement can be passed by assertion. Checked on master 2f1eea77 in a scratch repository: `evidence record --target <id> -- false` records a captured fail, a later `evidence record --target <id> --result pass "looks fine"` with no command supersedes it (`evidence list` shows the fail "superseded by newer pass" and `Proof: ✓ fresh pass`), and `close` then succeeds. The Agent Factory eval fixtures hit the same path on 0.2.0. An agent may have a good reason for that, so it stays allowed, but it should be plain to the reviewer and the human that a measured fail was cleared by something other than rerunning the measurement.

## Done when

Tests show, on one item: a captured fail of command C followed by a newer manual pass still clears as today (`close` succeeds, verdict and exit status unchanged), and `evidence list`, `merge --check` and `atelier debrief <mission>` each show one line naming the fail, the pass and the pass's summary. The same line appears when the newer pass is a captured pass of a different command (`-- true`). A newer captured pass of C, and a manual fail followed by a manual pass, show no line. Reverting the detection makes the first two tests fail. A receipt on atelier-ll3p judges criterion assertion-shown with a captured run of these tests.

## Scope

`evidence list`, `merge --check` and the debrief, and docs/product/validation.md. Gate decisions do not change, so this needs no new flag: a manual receipt already requires a summary, and that summary is the reason shown.

## Design

Show, do not refuse. Atelier trusts agents and makes shortcuts visible (PRODUCT_INTENT.md); a refusal with no override would make an item whose check cannot be rerun impossible to close. Decided by the human on 2026-10-30. Related atelier-4urj unifies the supersession rule; this item does not depend on it.
