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

Tests show, on one item: a captured fail of command C followed by a newer manual pass leaves the item's proof failing in `evidence list` (naming the captured fail and saying only a newer pass of the same command clears it), and `close` and `merge --check` refuse with a Next line that reruns C with `evidence record ... -- C`; the same fail followed by a newer captured pass of a different command (`-- true`) still refuses; a newer captured pass of C clears it; a manual fail followed by a manual pass still clears as today. Reverting the rule makes the first two tests fail. A receipt on atelier-ll3p judges criterion no-pass-by-assertion with a captured run of these tests.

## Scope

The one supersession function atelier-4urj introduces, which this item is blocked by, and docs/product/validation.md. Manual receipts stay valid proof where no captured receipt has failed.

## Design

Refuse rather than warn: a measured fail is the strongest evidence the tracker holds, and an assertion is the weakest, so the gate keeps the order. A captured fail is cleared only by a newer captured pass of the same command string; kind does not matter. No override flag. An item whose check can no longer be run, or whose failing receipt captured the wrong command, cannot reach done: it is superseded (with a reason) and replaced by an item with a runnable check. Failures never expire (crates/atelier-app/src/evidence_freshness.rs), so editing a Done when does not clear one.
