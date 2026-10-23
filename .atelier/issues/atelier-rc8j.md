---
created_at: "2026-10-17T01:44:17.482012585+00:00"
id: "atelier-rc8j"
issue_type: "mission"
labels:
- "cli-output"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-093h"
    type: "advances"
  - kind: "issue"
    id: "atelier-2tux"
    type: "advances"
  - kind: "issue"
    id: "atelier-3ekh"
    type: "advances"
  - kind: "issue"
    id: "atelier-3u2u"
    type: "advances"
  - kind: "issue"
    id: "atelier-6rh7"
    type: "advances"
  - kind: "issue"
    id: "atelier-7ts2"
    type: "advances"
  - kind: "issue"
    id: "atelier-9jr8"
    type: "advances"
  - kind: "issue"
    id: "atelier-b05y"
    type: "advances"
  - kind: "issue"
    id: "atelier-bphy"
    type: "advances"
  - kind: "issue"
    id: "atelier-bvl3"
    type: "advances"
  - kind: "issue"
    id: "atelier-cisa"
    type: "advances"
  - kind: "issue"
    id: "atelier-ft88"
    type: "advances"
  - kind: "issue"
    id: "atelier-grsi"
    type: "advances"
  - kind: "issue"
    id: "atelier-h6o4"
    type: "advances"
  - kind: "issue"
    id: "atelier-kiry"
    type: "advances"
  - kind: "issue"
    id: "atelier-mp6v"
    type: "advances"
  - kind: "issue"
    id: "atelier-ns04"
    type: "advances"
  - kind: "issue"
    id: "atelier-o8ni"
    type: "advances"
  - kind: "issue"
    id: "atelier-u0ig"
    type: "advances"
  - kind: "issue"
    id: "atelier-uv5d"
    type: "advances"
  - kind: "issue"
    id: "atelier-yrlj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-23T21:11:03.107815375+00:00"
status: "done"
title: "Make every gate end in a verdict, with each failure's cause and fix"
updated_at: "2026-10-23T21:11:03.107815375+00:00"
---

## Outcome

`land`, `merge --check`, `review` and `evidence` tell their reader:
- what was checked or recorded;
- which checks fail, against which record, and why;
- the exact command that fixes each failure;
- a final verdict.

Evidence keeps the tail of a failing command and can be listed per item. Review state can be read per item without opening record files. Stop-hook output stays separate from landing failures.

## Success criteria

- S1: `land` and `merge --check` list failing checks first, one line each with the record, the cause and a runnable fix using real IDs, then end with a verdict. Every gate is evaluated even when a precondition fails. A passing land says what it checked (atelier-9jr8, atelier-6rh7, atelier-kiry, atelier-grsi, atelier-7ts2, atelier-ns04, atelier-yrlj, atelier-bvl3, atelier-2tux). Shown by tests over the land and merge refusal paths.
- S2: When an approval is not current, the gate names the approval it considered and why (atelier-cisa). Shown by tests for stale-by-code, stale-by-base-merge and author-identity cases.
- S3: `evidence record` confirms in one line with the result. Captured output keeps the failure tail. `evidence list` filters by item. `evidence show` is brief. A passing receipt stores a summary of its output, not the full log, and records end without a trailing blank line (atelier-3ekh, atelier-bphy, atelier-57wj, atelier-u0ig, atelier-o8ni, atelier-b05y, atelier-auca). Shown by CLI tests.
- S4: `review list <id>` shows review state, with findings under their dispositions. `review show` is brief. Recording a review event confirms what was recorded and warns on an author identity. Review titles do not repeat their body (atelier-uv5d, atelier-h6o4, atelier-3u2u, atelier-093h, atelier-hulj). Shown by CLI tests.
- S5: Stop-hook adapter errors are reported apart from landing failures. Uncommitted records alone on the base do not block a turn. A review still pending on the branch's own item is reported without blocking. A record that needs a newer binary says to reinstall Atelier (atelier-ft88, atelier-mp6v, atelier-i80r, atelier-u4ql). Shown by hook tests.
- S6: Claude Code transcripts recorded after S1-S4 land show `land`, `merge --check`, `evidence record` and `review list` piped through head, grep, sed or tail in under a third of runs. Shown by a dated note with the counts.

## Background

Split out of atelier-o3lm on 2026-10-16 at the human's direction. Its items come from that mission's six-surface CLI audit; see atelier-o3lm's Background for the method and the transcript scripts. The output contract (atelier-o3lm S1, atelier-i2qq) and ADR 0022 (atelier-bw0b) govern how output looks. Where an item here rewrites the same renderer as an atelier-o3lm or atelier-1w8c item, it is blocked by that item. The 2026-10-16 transcript retrospective found gate refusals that named no fix to be the most expensive friction after review churn.

## Notes

- 2026-10-23 [decision] Pending review (S5, atelier-i80r): the Stop hook reports instead of blocking when it runs on the owner's own work branch and every failed check is review.current or claims.reviewed with no open finding; land itself still refuses.
  Reason: Needs no new record, and an agent that has requested review has nothing to do but wait; findings or any other failure still need action now.
  Rejected: A review-request record (record-format change); the owner's review status (close moves the owner past it).
- 2026-10-23 [decision] Records-only relaxation (S5, atelier-mp6v) applies on the base branch only.
  Reason: S5 scopes it to the base; on a work branch, uncommitted records are part of reviewed work and should be committed with it.
  Rejected: Relaxing it on work branches too.
- 2026-10-23 [decision] Review titles (S4, atelier-hulj) are generated from event kind, owner and short pin; existing records are not rewritten.
  Reason: Stops repeating the body without a schema change or a migration.
  Rejected: Dropping the title (schema change); capping the summary (still repeats it).
- 2026-10-23 [decision] A recorded fail from evidence record exits 0; its verdict line names the result (S3, atelier-3ekh).
  Reason: Recording succeeded; the result is data, like merge --sync's 'synced — approval needs fresh review'. The gate that cares (land/merge) refuses on the failure.
  Rejected: Exit non-zero on a recorded fail.
- 2026-10-23 [decision] Existing evidence receipts are not rewritten when passing captures change to a summary (S3, atelier-57wj).
  Reason: Receipts are committed proof; rewriting them changes history for ~2 MB of savings that new receipts stop growing anyway.
  Rejected: Migrating old pass receipts.
- 2026-10-23 [note] 2026-10-23 plan: regroomed all 31 items against master ff7cc896 (dated notes on each). atelier-6wes superseded (done in 1a9d6b9c); the rest still needed, several rescoped. Success criteria unchanged. Increments: atelier-h1nc (S1 verdicts and renderer: 9jr8, 6rh7, bvl3, 2tux), atelier-dg3h (S3 receipt shape: bphy, 57wj, auca), atelier-phfo (S4 review views: h6o4, 3u2u, 093h, jw1n, hulj, uv5d, d8ku) run first in parallel on disjoint files. Then atelier-imlh (S2: cisa) and atelier-0uaj (S1 preconditions: kiry, yrlj, ns04, 7ts2, grsi, y0en) after h1nc, atelier-l7nz (S3 views: 3ekh, o8ni, u0ig, b05y) after dg3h, and atelier-buqu (S5 hook: ft88, 467y, u4ql, mp6v, i80r) after 0uaj. Acceptance and S6 count: atelier-1o56.
- 2026-10-23 [decision] Pending review (refines the i80r decision): a findings.disposed failure counts as pending review too when every failing finding is a fix_now already resolved by a later commit and waiting only for fresh independent approval.
  Reason: That state is also waiting on a reviewer, with nothing for the agent to do; it was part of the turns the retrospective counted. Undisposed findings and unresolved fix_now still block.
  Rejected: Keeping findings.disposed always blocking.
- 2026-10-23 [decision] Close atelier-rc8j with S6 unproven; atelier-p1vz recounts it after about a week of ordinary sessions. The criterion text is unchanged.
  Reason: Human decision on 2026-10-23 after acceptance validation atelier-1o56 proved S1-S5 and found S6's post-landing sample too small (26 runs, mostly reviewer scripts).
  Rejected: Keeping the mission open for the recount; revising S6 first.

## Close Reason

### Outcome
Every gate ends in a verdict with each failure's cause and fix. `land`, `merge --check` and `merge` list failed checks first, one line each with the record, the cause and a fix using real IDs that tests run verbatim, keep evaluating every gate when a precondition fails, and end with a verdict that counts every failed check; a passing land says what it checked (S1: atelier-h1nc, atelier-0uaj). An approval that does not count is named with why: the commit and path or base merge that staled it, or an author identity (S2: atelier-imlh). Evidence receipts keep the failure tail, summarize passes instead of storing logs, and end cleanly; `evidence record` confirms in one line, `evidence list <item>` filters with freshness marks that agree with the gates, and `evidence show` is brief (S3: atelier-dg3h, atelier-l7nz). `review list <id>` shows approval state and findings under their dispositions, `review show` is brief, recording confirms and warns on an author identity, and titles no longer repeat the body (S4: atelier-phfo). The Stop hook blocks only on failures the agent must act on now (S5: atelier-buqu). S1-S5 are proven by independent acceptance validation atelier-1o56 (receipts atelier-wu4e, atelier-5hce, atelier-iibq, atelier-sf0e, atelier-wink). S6 is unproven: only 26 runs fell after S1-S4 landed, almost all from this mission's reviewers' scratch scripts (receipt atelier-4n58, blocked).

### Deviations
Closed with S6 unproven by the human's decision on 2026-10-23; atelier-p1vz recounts it after about a week of ordinary sessions. atelier-6wes was superseded at the regroom (done before the mission). Follow-ups added to scope along the way: atelier-jiz0, atelier-3xq4, atelier-r3oy, atelier-zegg. atelier-4urj was unlinked as low priority and keeps its discovered_from source.

### Decisions
Six decisions on this record's notes (pending-review rule and its refinement, records-only relaxation on the base only, generated review titles, recorded-fail exit status, no rewrite of old receipts, closing with S6 deferred); increment decisions are on their items (`atelier work decisions atelier-rc8j`).

### Surprises
Review rounds kept finding printed fixes that did not run verbatim or led into worse states (a roleless unlink, start under a closed ancestor, a commit fix that made review records permanently append-only-broken, a reset that dropped a just-made commit), until a test helper ran every printed fix and reran the gate. A `merge --sync` bug nested old receipt bodies in rerun summaries (atelier-3xq4). On the S6 sample, piping looked worse than the 51% pre-mission baseline (61.5%), though the output contract itself recommends `| tail -1` to read the verdict line.

### Open items
atelier-p1vz (S6 recount). Follow-ups with their sources: atelier-gg0m (hook cwd outside a repository), atelier-wu7t (wrong stale reason), atelier-wir7 (three misleading lines), atelier-e8l6 (inline backtick fence), atelier-x0vs (decisions listed twice), atelier-4urj (one supersession rule). Whether S6 should count `| tail -1` verdict reads is a question for the recount.
