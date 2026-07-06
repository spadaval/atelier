---
created_at: "2026-07-06T18:51:19.406282400+00:00"
id: "atelier-9adj"
evidence_type: "validation"
captured_at: "2026-07-06T18:51:19.301762925+00:00"
command: "bash -c '\nset -eu\nfor name in abandon graph mission note plan repair search start worktree; do\n  file=\"docs/product/command-audit/${name}.md\"\n  lines=$(wc -l < \"$file\")\n  test \"$lines\" -le 10\n  heading_count=$(rg \"^## \" \"$file\" | wc -l)\n  test \"$heading_count\" -eq 0\n  printf \"%s %s lines\\n\" \"$name\" \"$lines\"\ndone\ndecision_count=$(rg -l \"^## Decision Record$\" docs/product/command-audit/*.md | wc -l)\ntest \"$decision_count\" -eq 11\nrg -n \"Product/cognitive cost.*Architecture/code cost\" docs/product/command-audit/{bundle,check,evidence,history,init,issue,man,prune,review,status,work}.md\nrg -n \"Keep:|Simplify:|Fold:|Hide:|Remove:\" docs/product/command-audit/complexity-budget.md\nrg -n \"Product complexity is|Architecture complexity is|Do not optimize one kind\" docs/product/zen.md\n'"
exit_status: "0"
agent_identity: "independent-validator"
target:
  kind: "issue"
  id: "atelier-vqhi"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-vqhi"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "PASS corrected command-audit structure audit (supersedes failed harness evidence atelier-41db): retired pages are compact tombstones and active decision pages separate operator/product cost from architecture/code cost with Keep/Simplify/Fold/Hide/Remove verdicts."
updated_at: "2026-07-06T18:51:26.272004166+00:00"
---

## Summary

PASS corrected command-audit structure audit (supersedes failed harness evidence atelier-41db): retired pages are compact tombstones and active decision pages separate operator/product cost from architecture/code cost with Keep/Simplify/Fold/Hide/Remove verdicts.

## Command

```console
bash -c '
set -eu
for name in abandon graph mission note plan repair search start worktree; do
  file="docs/product/command-audit/${name}.md"
  lines=$(wc -l < "$file")
  test "$lines" -le 10
  heading_count=$(rg "^## " "$file" | wc -l)
  test "$heading_count" -eq 0
  printf "%s %s lines\n" "$name" "$lines"
done
decision_count=$(rg -l "^## Decision Record$" docs/product/command-audit/*.md | wc -l)
test "$decision_count" -eq 11
rg -n "Product/cognitive cost.*Architecture/code cost" docs/product/command-audit/{bundle,check,evidence,history,init,issue,man,prune,review,status,work}.md
rg -n "Keep:|Simplify:|Fold:|Hide:|Remove:" docs/product/command-audit/complexity-budget.md
rg -n "Product complexity is|Architecture complexity is|Do not optimize one kind" docs/product/zen.md
'
```

Exit status: 0

## Stdout

Bytes: 2283
Truncated: no

```text
abandon 8 lines
graph 8 lines
mission 9 lines
note 6 lines
plan 7 lines
repair 7 lines
search 7 lines
start 6 lines
worktree 8 lines
docs/product/command-audit/work.md:11:| Operator question | Role | Product/cognitive cost | Architecture/code cost | Verdict | Next action |
docs/product/command-audit/status.md:9:| Operator question | Role | Product/cognitive cost | Architecture/code cost | Verdict | Next action |
docs/product/command-audit/review.md:10:| Operator question | Role | Product/cognitive cost | Architecture/code cost | Verdict | Next action |
docs/product/command-audit/prune.md:10:| Operator question | Role | Product/cognitive cost | Architecture/code cost | Verdict | Next action |
docs/product/command-audit/man.md:9:| Operator question | Role | Product/cognitive cost | Architecture/code cost | Verdict | Next action |
docs/product/command-audit/issue.md:10:| Operator question | Role | Product/cognitive cost | Architecture/code cost | Verdict | Next action |
docs/product/command-audit/init.md:10:| Operator question | Role | Product/cognitive cost | Architecture/code cost | Verdict | Next action |
docs/product/command-audit/history.md:9:| Operator question | Role | Product/cognitive cost | Architecture/code cost | Verdict | Next action |
docs/product/command-audit/evidence.md:9:| Operator question | Role | Product/cognitive cost | Architecture/code cost | Verdict | Next action |
docs/product/command-audit/check.md:10:| Operator question | Role | Product/cognitive cost | Architecture/code cost | Verdict | Next action |
docs/product/command-audit/bundle.md:10:| Operator question | Role | Product/cognitive cost | Architecture/code cost | Verdict | Next action |
63:- Keep: the command owns a distinct, valuable operator job.
64:- Simplify: the job is real, but the command or output exceeds its budget.
65:- Fold: the capability should move into an existing owner command.
66:- Hide: the command is useful only for admin, migration, recovery, or debug.
67:- Remove: the command preserves vocabulary, plumbing, or alternate paths with
67:Product complexity is what an operator has to understand: which command to use,
71:Architecture complexity is what the system has to carry: code paths,
75:Do not optimize one kind by blindly increasing the other.
```

## Stderr

Bytes: 0
Truncated: no

```text
```

