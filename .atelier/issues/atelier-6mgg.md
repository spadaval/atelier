---
created_at: "2026-10-22T19:58:08.129685998+00:00"
id: "atelier-6mgg"
issue_type: "decision"
labels:
- "merge"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-22T20:10:10.348438211+00:00"
status: "done"
title: "Make merge_commit the default merge strategy when a test check exists?"
updated_at: "2026-10-22T20:10:10.348438211+00:00"
---

## Question

Should a policy without `branch_policy.merge_strategy` default to `merge_commit` when its `merge_checks` include a `shell.check`, and to `fast_forward_only` otherwise (atelier-2yl7)? It decides how new repositories behave by default, and how much of atelier-k8h8 S6 they get without configuration.

## Options

- **Conditional default, as atelier-2yl7 proposes (recommended).** Under `fast_forward_only`, every landing forces every other open branch to catch up, re-prove and re-approve. Over three days that came to 122 catch-up merges and 41 fresh-approval commits. Since this repository switched (atelier-qymu), its landed branches needed no last-mile catch-up. The risk is a textual merge that is clean but semantically broken: a clean merge in atelier-ihss broke a test. That is why the default applies only when a test check runs on the merged tree.
- **Always `merge_commit`.** This is simpler, but with no test check a clean textual merge reaches the base with nothing having run on it.
- **Keep `fast_forward_only`, and document how to switch.** Linear history and no surprises, but new repositories start in the mode that caused the churn this mission exists to remove.

## Decision

Make `merge_commit` the standard: a policy without `branch_policy.merge_strategy` defaults to `merge_commit` when its `merge_checks` include a `shell.check`, and to `fast_forward_only` otherwise, as atelier-2yl7 specifies. Decided by the human on 2026-10-22: Atelier should take headaches off the orchestrator wherever it can, and catch-up merges are one of them. The test-check condition keeps a clean but semantically broken merge from reaching the base with nothing run on it.

## Close Reason

merge_commit is the default when a test check exists (human, 2026-10-22)
