# Validation

Validation is how Atelier keeps autonomous work useful without turning work into
paperwork. The product goal is simple: an operator should know what must be
true, know what proof is expected, produce that proof while doing the work, and
close only when another operator can inspect the result.

This document owns the product language and user-visible behavior. Detailed
proof-routing policy lives in
[Architecture Validation](../architecture/quality/validation.md). The broader
work graph is described in [Work Model](work-model.md).

## Core Terms

Outcome is the desired observable result of a work item. It says what should be
true when the work is done. Outcomes should describe product, command, file,
workflow, or coordination behavior, not the private implementation path.

Evidence requirement is the expected way to prove an outcome. On issue-shaped
work this normally lives in the issue `Evidence` section. It should name an
observable check: command output, test result, help text, file content, review
artifact, screenshot, migration transcript, or another inspectable record.

Proof is the relationship between a claim and supporting facts. A passing test
run is not automatically proof for every claim; it is proof when it supports the
specific outcome being closed.

Evidence record is the durable proof envelope. It records what was observed,
the result, the accountable target, and any command transcript or artifact
reference. Evidence records let future operators inspect proof without relying
on chat history.

Validation is the act of checking a claim against evidence. Ordinary
implementation work can validate its own local outcome with focused proof.
Risky, parent-level, public-contract, migration, workflow, or completion claims
may need separate validation work.

Workflow validator is a configured transition gate. It answers whether a record
may move to another status. Validators should produce actionable guidance such
as missing proof, open blockers, stale tracker state, dirty worktree, or pending
validation work. Validator names are diagnostic detail, not the main product
language for normal operators.

External-state validator is a workflow validator that reads a system outside
Atelier without mutating it. `linked_pr_merged` is the product model for
provider-backed review artifact gates: it reads the issue's active structured
`review` artifact link, verifies the remote PR-equivalent artifact against
configured repo and branch policy, then reports whether the required review
artifact is merged. It must not create, comment on, review, merge, or close the
artifact, and it must not transition Atelier workflow. Review commands perform
review-artifact actions; validators only decide whether the configured workflow
transition is currently allowed. In the starter policy, this is an epic close
gate only; validation issues and ordinary implementation issues close from
attached proof and local workflow checks.

Review validators check the local workflow facts Atelier depends on: the active
review kind, native room merge state or normalized provider-local review
number, configured review provider remote, expected source and target branches,
merged state, and review-complete state when the workflow names that validator.
They do not duplicate provider branch-protection, required-approval, merge
strategy, or merge-authorization policy.

Completion is the final completion judgment. For an ordinary issue, completion means
the issue outcome is done and the required proof is attached. For an epic or
mission, completion synthesizes child work, blockers, proof, review, and
validation rather than duplicating every child proof record.

## Product Rule

Proof requirements must be visible before completion.

Agents should not discover required proof only after `atelier issue transition
<id> close` or `atelier mission close` fails. The normal workflow surfaces
should show the expected proof early, summarize whether it is satisfied, and
provide the next command that moves the work forward.

## Ordinary Issue Flow

For normal implementation work, validation should stay light:

1. Read the issue outcome and evidence requirement.
2. Implement the feature or fix.
3. Run the focused check that proves the outcome.
4. Record the check as evidence attached to the issue.
5. Close when transition checks are satisfied.

The happy path should be one command for proof capture when possible:

```text
atelier evidence record --target issue/<id> --kind test -- <command>
```

or a higher-level convenience command that does the same capture and attachment.

Ordinary issues should not require manual claim IDs, independent reviewer
metadata, residual-risk forms, or parent completion tables. The evidence section
and command transcript are enough when the proof is specific and inspectable.

## Escalated Validation

Additional validation is justified when the work changes a boundary that future
agents or users depend on:

- public CLI behavior, help text, or command contracts;
- workflow policy, transition gates, or proof behavior;
- external review-artifact gates such as linked PR merge state;
- canonical records, projection rebuild, migration, or runtime repair;
- mission, epic, milestone, or other parent-level completion claims;
- security, data-loss, irreversible, or hard-to-reproduce behavior;
- broad claims where one green test run could easily miss the real outcome.

Escalated validation should still minimize ceremony. The system should infer
what it can from commands, targets, status, actor identity, and linked work.
Manual detail is useful only when it explains risk, judgment, or a failed,
blocked, deferred, or not-applicable result.

## Parent Completion

Missions and epics coordinate work; they are not proof dumping grounds.

An epic completion should answer whether the reviewable branch is coherent, child
work is closed, child proof exists, and any explicit review or validation work
has approved the branch. The configured epic close transition also requires the
linked provider-backed review artifact to be merged.

A mission completion should answer whether linked work is closed, blockers are
clear, configured health gates pass, and any explicit mission-level validation
or validation work is complete.

Parent completion should cite child issue IDs and evidence IDs. It should not ask
operators to restate proof that already belongs on closed child work.

## Visibility Surfaces

`atelier issue show <id>` should show a compact proof summary near transition
readiness:

```text
Proof
-----
Status: missing
Required for close: 1 passing evidence record attached to issue/<id>
Expected proof:
  - focused CLI test proves the changed completion behavior
Next:
  atelier evidence record --target issue/<id> --kind test -- <command>
```

When proof is present, the same block should name the evidence IDs and results.

`atelier issue transition <id> start` should remind the worker what proof will
be needed before close, because this is where the operator forms the work plan.

`atelier issue transition <id> --options` should preview completion blockers in
operator language:

```text
close -> done: blocked
  missing proof: no passing evidence attached to issue/<id>
  next: atelier evidence record --target issue/<id> --kind test -- <command>
```

`atelier mission status <id>` should show mission proof gaps by accountable
work item, not as raw validator names:

```text
Proof Gaps
----------
  atelier-task3: missing passing issue evidence
  atelier-validation2: validation work still in progress
  atelier-epic1: child proof incomplete
```

`atelier mission status <id> --verbose` may show deeper terminal-check detail, including
unmapped parent outcome lines, failed proof, blocked proof, deferred proof, and
residual risks.

## Notes Versus Evidence

Notes are for handoff context, caveats, observations, and coordination.
Evidence is for completion proof.

A note can explain why a command was skipped, where follow-up should start, or
what an operator learned. A note should not satisfy an evidence requirement for
a behavior change, validation issue, validation issue, workflow gate, or parent
claim.

## Results

Evidence results use product-level meanings:

- `pass`: the observed proof supports the claim.
- `fail`: the observed proof disproves the claim.
- `blocked`: proof cannot complete until another blocker is resolved.
- `deferred`: proof is intentionally postponed and follow-up is named.
- `not-applicable`: the claim or check does not apply, with rationale.

Non-pass evidence is not a nuisance record. It is how agents avoid silently
turning uncertainty into fake completion.

## Anti-Red-Tape Test

Every validation feature should pass this test:

- Does it help the agent know what to prove earlier?
- Does it make the correct proof capture path shorter?
- Does it prevent a common premature-close or missing-proof failure?
- Does it keep ordinary feature work light?

If the answer is no, the feature is probably process theater.
