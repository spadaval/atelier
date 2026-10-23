---
created_at: "2026-10-17T01:10:57.977408887+00:00"
id: "atelier-093h"
issue_type: "feature"
labels:
- "cli-output"
- "review"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-rc8j"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-23T14:35:52.585690861+00:00"
status: "done"
title: "Say what review event was recorded and warn on an author identity"
updated_at: "2026-10-23T14:35:52.585690861+00:00"
---

## Goal

`review record`, `approve`, `finding`, `trivial` and `disposition` all print the same block (crates/atelier-cli/src/commands/review.rs:78):

```
Review event: atelier-45rc
Identity: Reviewer R
Commit: fed2d57190dbd081a7aa6437a97ce2680f34c1c0
proof-sample.txt says hello, should say goodbye
Commit this record with the work it describes.
```

It doesn't say whether the event was a finding or an approval, which owner it targets, or what a disposition settled or superseded. A new finding gives no reminder that it needs a disposition. Records that can never pass a gate are accepted silently: `review approve --identity author@example.com` (a Git author), and `disposition --decision no_action` by the author, both succeed, and `merge --check` only rejects them later. `DEFER requires a follow-up issue` doesn't name the flag. In transcripts, 44 of 132 review appends were piped or redirected.

## Done when

- Confirmations name the kind, owner, short pin and next step, for example:
  `Finding atelier-45rc (fix_now) on atelier-nqgr @fed2d571. Dispose: atelier review disposition atelier-45rc --identity … --decision …`
  `Disposition atelier-6o75: no_action on atelier-rm9e "Name could be better" (supersedes atelier-ul3r)`
  `Approval atelier-hleo on atelier-nqgr @c24af21a`
- An approval, or a defer/no_action disposition, whose identity matches a substantive Git author is still recorded but prints `⚠ identity matches a Git author; this will not satisfy review.current` (or `findings.disposed`). Use the same author matching as review_gate.rs.
- The defer error reads `DEFER requires --follow-up <issue-id>`.
- `--quiet` still prints only the ID. Tests cover the warning and each kind's line.

## Notes

- 2026-10-23 [note] 2026-10-23 regroom against master ff7cc896: still needed. Every kind prints 'Review event: … Commit this record…' (review.rs:68); no author warning; defer error doesn't name --follow-up. Use review_gate::author_identities (:645) and make independent (:688) pub rather than copying.
- 2026-10-23 [decision] Review events confirm in one line, '<Kind> <id> [<decision>] on <owner> @<pin8>' plus what a disposition settled or a finding's disposition command, without brackets around decisions.
  Reason: A new event changes no existing value, so the '<id> <field>: <value>' form has nothing to list; the output contract bars brackets around a decision, so the item's '(fix_now)' example is written 'fix_now'.
  Rejected: One '<id> <field>: <value>' line per field
- 2026-10-23 [decision] The author warning is best-effort: when the policy or the base branch cannot be read, no authors are counted and no warning prints; under --quiet it goes to stderr.
  Reason: It only warns; recording must not newly fail on a read the record does not need, and quiet stdout carries only the ID.

## Close Reason

Each recording command confirms in one line naming the kind, ID, owner and short pin: `Approval <id> on <owner> @<pin>`, `Finding <id> fix_now on ... Dispose: atelier review disposition <id> --identity "..." --decision fix_now --resolution-commit ... "..."`, `Disposition <id> no_action on ...: finding <id> "<first line>", supersedes <id>`. An approval, or a defer/no_action disposition, by a Git author is still recorded and warns `⚠ identity matches a Git author; this will not satisfy review.current` (or `findings.disposed`), using review_gate::author_identities and review_gate::independent. The defer error reads `DEFER requires --follow-up <issue-id>`. `--quiet` still prints only the ID, with the warning on stderr.

### Deviations
Decisions are not bracketed (`fix_now`, not `(fix_now)`), per the output contract. The old `Commit this record with the work it describes.` reminder is gone. The defer error exits 1, not 2.

### Decisions
See the decision notes on this item (one-line form; the warning is best-effort and goes to stderr under quiet).

### Surprises
None.

### Open items
None.
