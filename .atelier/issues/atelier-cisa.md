---
created_at: "2026-10-17T01:10:57.975874283+00:00"
id: "atelier-cisa"
issue_type: "feature"
labels:
- "cli-output"
- "merge"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-o3lm"
    type: "advances"
  - kind: "issue"
    id: "atelier-tciw"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Say which approval merge --check considered and why it is not current"
updated_at: "2026-10-17T01:10:57.975874283+00:00"
---

## Goal

`FAIL review.current: no current independent approval or explicit trivial exemption; authors: author a, author a <author@example.com>, author@example.com` is the same message whether there is no approval, the only approval came from an author, or the approval went stale after a later commit. `approval_is_current_to` (crates/atelier-app/src/review_gate.rs:169) finds the offending edge and then reduces it to a bool. The authors list also repeats one person in three normalized forms.

## Done when

- Every rejected approval or trivial-exemption candidate gets one line under the gate, for example:
  `⚠ atelier-x16l @e2562bd6 stale: base merge 4f1e2a3b changed in-scope record .atelier/issues/atelier-bw0b.md beyond status`
  `⚠ atelier-7l79 @4c79d30b stale: 1a2b3c4d "Tweak after approval" changed proof-sample.txt`
  `⚠ atelier-i3ay identity author@example.com matches a Git author`
- The FAIL line ends with the fix: `atelier review approve <id> --identity "<non-author>" "…"`.
- Authors are listed once per person (`Author A <author@example.com>`).
- The `merge_diverged` advice (crates/atelier-cli/src/commands/merge.rs:72) and docs/product/reviews-and-merge.md agree with the gate. They say a clean merge keeps the approval unless the base changed an in-scope record beyond status, and the stale line names that case. If the investigation shows the gate is wrong instead, fix the gate.
- Tests cover stale-by-code, stale-by-base-merge-of-in-scope-record and author-identity. `land` shows the same lines.

## Context

The real case: a clean `git merge master` into task/atelier-bw0b (master had edited .atelier/issues/atelier-bw0b.md) made approval atelier-x16l stale with no explanation. `allowed_base_merge` (review_gate.rs:194) correctly lets in-scope records change only in status. But the `merge_diverged` advice shown just before says "a clean merge keeps the approval", so the reader was told the opposite of what happened.
