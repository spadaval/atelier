---
created_at: "2026-10-17T09:35:34.551883060+00:00"
id: "atelier-pnha"
evidence_type: "validation"
commit: "13d7bb6f643217d7deefdb446677d3911b3922cb"
result: "fail"
captured_at: "2026-10-17T09:35:34.551634506+00:00"
agent_identity: "Independent validator (Claude subagent)"
target:
  kind: "issue"
  id: "atelier-o3lm"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-o3lm"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Validated at master 13d7bb6f; suite 755/755 pass. S1 pass: contract doc and human_output.rs helpers with rendering tests. S2 pass on behaviour: every command family piped to head ends with exit 0 or a silent SIGPIPE (141) and empty stderr, via a global SIG_DFL; the proof is one piped test (issue show), not one test per family. S3 fail: (a) a refused start or issue transition with -q prints the whole transition report on stdout (atelier -q start <blocked-id> 2>/dev/null), and quiet_tokens.rs only checks refusals for check, land and merge; (b) invalid-value errors use several forms: issue create/update --issue-type nope gives a workflow-policy sentence while issue list --issue-type nope gives Invalid issue type; issue link --role nope lists validates and evidenced_by, which it rejects, and leaves out blocks, which it accepts; an unknown transition says Unknown transition ... available from; parser-checked values (--result, --recommendation) use clap's form with exit 2; --limit 0 gives its own sentence. Not-found, wrong-kind, short IDs, real IDs in hints, and P0-P3 with words all pass (priority is case-sensitive). S4 fail (minor): status prints Unassigned Work (7) and no label (4) where work missions prints Unassigned: 1 open issue; status prints an Other Local Work Branches section after Next; brief issue show lists close --reason as allowed while --full Transition Readiness says close: blocked; with COLUMNS set, redirected output still wraps. Timestamps pass outside review show. S5 pass: root help is generated and lifecycle-ordered, every argument has help, and the log flags are removed. Not covered by any criterion: the verdict line, the mutation confirmation, and the review show, review list and evidence list views (review show still prints JSON); all are in backlog surface items."
updated_at: "2026-10-17T09:35:34.553725559+00:00"
---

Validated at master 13d7bb6f; suite 755/755 pass. S1 pass: contract doc and human_output.rs helpers with rendering tests. S2 pass on behaviour: every command family piped to head ends with exit 0 or a silent SIGPIPE (141) and empty stderr, via a global SIG_DFL; the proof is one piped test (issue show), not one test per family. S3 fail: (a) a refused start or issue transition with -q prints the whole transition report on stdout (atelier -q start <blocked-id> 2>/dev/null), and quiet_tokens.rs only checks refusals for check, land and merge; (b) invalid-value errors use several forms: issue create/update --issue-type nope gives a workflow-policy sentence while issue list --issue-type nope gives Invalid issue type; issue link --role nope lists validates and evidenced_by, which it rejects, and leaves out blocks, which it accepts; an unknown transition says Unknown transition ... available from; parser-checked values (--result, --recommendation) use clap's form with exit 2; --limit 0 gives its own sentence. Not-found, wrong-kind, short IDs, real IDs in hints, and P0-P3 with words all pass (priority is case-sensitive). S4 fail (minor): status prints Unassigned Work (7) and no label (4) where work missions prints Unassigned: 1 open issue; status prints an Other Local Work Branches section after Next; brief issue show lists close --reason as allowed while --full Transition Readiness says close: blocked; with COLUMNS set, redirected output still wraps. Timestamps pass outside review show. S5 pass: root help is generated and lifecycle-ordered, every argument has help, and the log flags are removed. Not covered by any criterion: the verdict line, the mutation confirmation, and the review show, review list and evidence list views (review show still prints JSON); all are in backlog surface items.
