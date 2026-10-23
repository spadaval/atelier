---
created_at: "2026-10-23T14:13:38.186773272+00:00"
id: "atelier-1o56"
issue_type: "validation"
labels:
- "cli-output"
workflow_branch:
  branch_base: master
  work_branch: validation/atelier-1o56
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-0uaj"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-buqu"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-dg3h"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-h1nc"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-imlh"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-l7nz"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-phfo"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-rc8j"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-23T17:59:58.678911018+00:00"
status: "done"
title: "Validate atelier-rc8j acceptance (S1-S6)"
updated_at: "2026-10-23T17:59:58.678911018+00:00"
---

## Goal

Check atelier-rc8j's success criteria S1-S6 independently against the merged master.

## Done when

Each of S1-S6 has an explicit result and linked evidence receipt; the handoff states failures, blocked checks and proof limits. S6 counts Claude Code transcripts recorded after the S1-S4 increments merged, by command and by whether output was piped through head, grep, sed or tail, and records the counts in a dated note.

## Context

Mission atelier-rc8j's Success criteria. Transcripts live under `~/.claude/projects/-home-supadava-projects-atelier*/` (subagent transcripts included); see atelier-o3lm's Background for the earlier transcript scripts. Run scenarios in a scratch repo under `~/.cache`, never `/tmp`.

## Notes

- 2026-10-23 [result] 2026-10-23 S6 transcript counts (receipt atelier-4n58). Source: Bash tool calls in ~/.claude/projects/-home-supadava-projects-atelier/ (the only matching project dir), main and subagent JSONL, extracted 13:50 -04:00 (latest call 13:48:49); this validator's own transcript contributed none. A run is one tool call invoking the command (atelier, target/*/atelier, or a binary variable such as $A/$B/$NEW); heredoc bodies and quoted strings (grep patterns, docs) are excluded; 'piped' = any pipeline stage after the command is head, grep, sed or tail; 'redirect' = output sent to a file that a later tail/head/grep/sed in the same call reads, counted only when not also piped. Strict window 13:13:22..13:48:49 (after S1's last merge, 0uaj): land 20 runs, 12 piped (60%), 1 redirect; merge --check 0; evidence record 6, 4 piped; review list 0; total 26, 16 piped (61.5%), 17 with redirect (65.4%); 3 transcripts (buqu reviewer 21 runs, buqu implementer 4, orchestrator 1). Per command after its own change: land since h1nc 11:17:22, 97 runs, 67 piped (69.1%), 2 redirect; merge --check since h1nc, 7 runs, 2 piped (28.6%), 1 piped to cut only; evidence record since l7nz 12:42:34, 9 runs, 5 piped (55.6%); review list since phfo 10:45:39, 0 runs. Baseline 2026-10-16..2026-10-22, excluding the 3 CLI-audit transcripts: land 151 runs / 85 piped (56.3%) / 9 redirect; merge --check 15 / 12 (80%) / 1; evidence record 93 / 31 (33.3%) / 6; review list 14 / 12 (85.7%); total 273 / 140 (51.3%), 156 with redirect (57.1%), 62 transcripts. No prompt or brief told agents to pipe (implementer and reviewer briefs checked); post-change runs are almost all this mission's own implementers and reviewers exercising the changed commands in scratch scenario scripts, where '| tail -3' compresses many scenarios and grep asserts one line. Verdict: blocked, sample too small and unrepresentative; at face value above one third. Scripts: ~/.cache/atelier-validate/s6/{extract,classify,report}.py.

## Close Reason

Independent acceptance validation of atelier-rc8j S1-S6 on master 5abb3651 (validation branch cd790a67), built from the worktree. Scenarios were built from scratch in ~/.cache/atelier-validate/{s1,s3,s5}: atelier init, a close transition gated on evidence.attached, a failing merge check, branches, an author-identity approval, a finding, a disposition, stale approvals, a base merge, newer schema versions, and Stop-hook payloads. Printed fixes were run verbatim. The full workspace suite with test-hooks passed, 953/953 with none skipped (atelier-wcxb).

| Criterion | Result | Receipt |
|---|---|---|
| S1 | pass | atelier-wu4e |
| S2 | pass | atelier-5hce |
| S3 | pass | atelier-iibq |
| S4 | pass | atelier-sf0e |
| S5 | pass | atelier-wink |
| S6 | blocked | atelier-4n58 |

S6 is blocked. In the strict window after the last S1 merge (13:13:22 to 13:48:49), land and evidence record have 26 runs, and merge --check and review list have none. 21 of the 26 runs come from one reviewer's scenario script. At face value 61.5% are piped through head, grep, sed or tail. The pre-mission baseline is 51.3% of 273 runs. Counts are in the dated note. Re-count after ordinary sessions have used the new binary.

### Deviations
The S1-S5 receipts carry `--criterion` tags so the mission rollup shows the results. A supporting full-suite capture was added (atelier-wcxb). Otherwise none.

### Decisions
S6 is marked blocked, not fail: the strict sample is too small and unrepresentative to judge the one-third threshold. S5 passes on its stated clauses, and the non-Atelier cwd case is reported as a finding because ft88's decision scoped adapter errors to cwds that cannot be entered.

### Surprises
`review list` had no runs at all after phfo merged. Almost every post-change run came from this mission's own agents.

### Open items
Findings for the accountable agent:
- Low-medium, defect in this change (S3, atelier-u0ig/3ekh). Freshness says "stale (pin not in HEAD's history)" when the pin is in HEAD's history but the item's record was uncommitted at the pin. Repro: create an item, start it without committing the records, commit code, run `atelier evidence record --target <id> --kind test -- true`, then commit the records. Expected: a reason naming the record. Actual: "pin not in HEAD's history" in record, list and show. The verdict (stale) agrees with the close gate. Cause: evidence_freshness.rs `staleness` maps every `first_mismatch` error to `NotInHistory`.
- Medium, gap in S5 scope (atelier-ft88). A Stop payload whose cwd exists but is outside any Atelier repo blocks the turn: `{"decision":"block","reason":"Error: Not an Atelier repository (or any parent). Run `atelier init` first.\nland: refused"}`. SessionStart with the same cwd says "Run `atelier init` first.. Run `atelier check` and repair the named records.", the double period and wrong advice ft88 set out to remove. ft88's decision covered only a cwd that cannot be entered.
- Low (S1). With the start transition recorded but uncommitted, merge --check prints "merge_owner_unstarted ... Next: atelier start <id>", and rerunning that does not clear the check. The records-commit line printed beside it does.

### Proof limits
- Base-merge staleness was shown for a hand-resolved record conflict. A hand-resolved code conflict and jw1n's child-to-owner review list are covered only by tests.
- Fixes that need a commit message or identity contain placeholders.
- S6 counting is regex-based. Spot checks found misses only inside heredocs, which are excluded by rule.

### Outside scope
- `merge <id> --sync` ends "merge: passed" when it merged nothing, even though merge --check would refuse.
- `review show` of an author-identity approval says only "not accepted on main", not why.
- `issue show atelier-rc8j --full` lists each Decision twice.
