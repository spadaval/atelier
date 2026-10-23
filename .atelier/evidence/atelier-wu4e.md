---
created_at: "2026-10-23T17:59:06.451375874+00:00"
id: "atelier-wu4e"
evidence_type: "validation"
commit: "cd790a67a0fd34914afe25337408d4b88a439af4"
result: "pass"
captured_at: "2026-10-23T17:59:06.450913501+00:00"
agent_identity: "independent validator (Claude)"
target:
  kind: "issue"
  id: "atelier-1o56"
  role: "validates"
criteria:
- slug: "S1"
  result: "pass"
criteria_mission: "atelier-rc8j"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-1o56"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "S1 pass: in scratch repos (~/.cache/atelier-validate/s1, s5) land and merge --check printed failing checks first, one line each '⚠ <id> <check>: <cause>. Next: <fix>' with real IDs, then '✓ N checks pass (...)' and a 'land:/merge: refused — N checks (...)' verdict, exit 1. Preconditions (uncommitted records, dirty completion, diverged base, branch mismatch, newer policy) were reported alongside review/evidence/findings gates judged on HEAD; where gates cannot run (owner unstarted, branch mismatch) a 'Not checked: ...' line names them. Passing land listed its 8 checks plus 'Next: atelier merge <id>'. Printed fixes run verbatim: review request, evidence record rerun, git add .atelier && git commit, merge --sync, git switch && land, and the merge went through. Limits: fixes needing a commit or identity use placeholders; merge_owner_unstarted offers 'atelier start <id>' even when start is recorded but uncommitted, and rerunning it does not clear the check (low finding; the records-commit line beside it does). Full suite atelier-wcxb."
updated_at: "2026-10-23T17:59:06.452541159+00:00"
---

S1 pass: in scratch repos (~/.cache/atelier-validate/s1, s5) land and merge --check printed failing checks first, one line each '⚠ <id> <check>: <cause>. Next: <fix>' with real IDs, then '✓ N checks pass (...)' and a 'land:/merge: refused — N checks (...)' verdict, exit 1. Preconditions (uncommitted records, dirty completion, diverged base, branch mismatch, newer policy) were reported alongside review/evidence/findings gates judged on HEAD; where gates cannot run (owner unstarted, branch mismatch) a 'Not checked: ...' line names them. Passing land listed its 8 checks plus 'Next: atelier merge <id>'. Printed fixes run verbatim: review request, evidence record rerun, git add .atelier && git commit, merge --sync, git switch && land, and the merge went through. Limits: fixes needing a commit or identity use placeholders; merge_owner_unstarted offers 'atelier start <id>' even when start is recorded but uncommitted, and rerunning it does not clear the check (low finding; the records-commit line beside it does). Full suite atelier-wcxb.
