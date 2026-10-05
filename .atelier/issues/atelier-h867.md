---
created_at: "2026-10-05T18:50:59.959468442+00:00"
id: "atelier-h867"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Remove unconditional executable permission scans from merge"
updated_at: "2026-10-05T18:50:59.959468442+00:00"
---

## Goal

Remove unconditional tracked executable-permission scanning from ordinary merge validation. Protect the approved committed tree and relevant refs without enforcing local filesystem permission equivalence beyond Git's configured cleanliness checks.

## Done when

- A local executable-bit change hidden by core.filemode=false does not independently block an otherwise valid merge.
- Ordinary Git cleanliness checks still apply with normal permission tracking. Approved commit/ref protections, canonical tracker integrity, and separately justified hidden-content checks remain effective.
- Ordinary merge no longer scans every tracked regular file's metadata solely to compare execute bits. Measure the removed overhead on representative repositories, including separate source/base checkouts; its current latency contribution is unmeasured.
- Assess shared evidence-capture/session callers before changing shared code. Keep stricter permission equivalence only where a concrete supported workflow requires it.
- Revise or remove tests and documentation that require this broader merge guarantee.

## Context

committed_snapshot::flagged_paths independently compares Unix executable permissions against the pinned Git tree even when core.filemode=false. Merge runs snapshot cleanliness checks around validation, potentially in both source and base checkouts, adding work proportional to tracked file count.

A local chmod does not modify the approved commit or its recorded permissions. The test demonstrates a local checkout discrepancy, not an incorrect committed tree being merged. No concrete supported workflow requiring this extra merge restriction has been established. Respect Git's configured cleanliness checks unless an explicit product requirement justifies overriding them.

Relevant sources:
- crates/atelier-app/src/committed_snapshot.rs: flagged_paths and its Unix metadata loop.
- crates/atelier-cli/src/commands/merge.rs: snapshot checks around validation.
- crates/atelier-cli/tests/merge_gates.rs: normal_index_hidden_modes_refuse_merge_before_and_after_checks_in_both_checkouts.
- Commit 6c6457dbd28cb8315f0185899492f3460a5a0951 added the scan/tests following finding atelier-ye9h.

A CSIRT notice for chmod +x /tmp/.tmplWNJ30/base.rs prompted investigation. This regression test generates that command pattern on dummy text; exact alert attribution remains unconfirmed. Merge semantics and cost motivate this task; avoiding incidental alerts is secondary.
