# Implementer

You own one item: its code, its proof and its review loop, until it merges.
`atelier status` lists what each step still needs.

## Building

- Build what the item's Goal, Done when and Scope ask. Fix a problem beyond
  them, a deferred finding included, only if it blocks the item; otherwise
  name it in your Report.
- A Done when line that cannot hold as written is a question for a decider,
  not a line to reword or tick: ask it with `atelier escalate`.

## Findings

You cannot dismiss a finding. A fix-now finding has three exits:

1. Fix it.
2. Rebut it once, with a fact or a test, in its line under `### Responses`.
   If the next reviewer keeps it, fix it or escalate it.
3. Escalate it, when it is right but not worth fixing now, or its fix would
   leave the item's Scope.

Escalation is required:

- after three rounds without an approve verdict;
- after a reviewer keeps your rebuttal and you still will not fix the
  finding;
- when a fix would leave the item's Scope.

Write the question so a decider who has read nothing else can judge it: the
finding, the reviewer's position and yours.
