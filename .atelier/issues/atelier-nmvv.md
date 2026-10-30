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
    id: "atelier-4urj"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-ll3p"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Keep a command-backed fail failing until a command-backed pass"
updated_at: "2026-10-30T06:52:05.478210610+00:00"
---

## Goal

A failed measurement can be passed by assertion. Checked on master 2f1eea77 in a scratch repository: `evidence record --target <id> -- false` records a captured fail, a later `evidence record --target <id> --result pass "looks fine"` with no command supersedes it (`evidence list` shows the fail "superseded by newer pass" and `Proof: ✓ fresh pass`), and `close` then succeeds. The Agent Factory eval fixtures hit the same path on 0.2.0. A receipt whose result comes from running the check should be cleared only by running the check again.

## Done when

Tests show, on one item: a captured fail followed by a newer manual pass leaves the item's proof failing in `evidence list` (naming the captured fail and saying a manual pass cannot clear it), and `close` and `merge --check` refuse with a Next line that reruns the check with `evidence record ... -- <command>`; the same fail followed by a newer captured pass clears it; a manual fail followed by a manual pass still clears as today. Reverting the rule makes the first test fail.

## Scope

The one supersession function atelier-4urj introduces, which this item is blocked by, and docs/product/validation.md. Manual receipts stay valid proof where no captured receipt has failed.

## Unknowns

Whether "a newer captured pass" must match the failing receipt's kind or command, or any captured pass on the item counts. Decide when 4urj has settled how gates filter by kind.

## Design

Refuse rather than warn: a measured fail is the strongest evidence the tracker holds, and an assertion is the weakest, so the gate keeps the order. No override flag. A check that truly cannot be rerun is the human's call, made as a decision record, not a receipt.
