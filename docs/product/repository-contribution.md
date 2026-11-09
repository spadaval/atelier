# Repository Contribution

Committed Atelier records are this repository's coordination source of truth.
[AGENTS.md](../../AGENTS.md) routes durable guidance; use
[Agent Factory](../../skills/agent-factory/SKILL.md) for process and
[development setup](development-setup.md) for tools.

Commit `.atelier/` changes with the implementation they describe. Keep work on
reviewable branches. Non-trivial executable changes require independent review
before local integration into `master`. Agents may integrate reviewed work
locally and push it, including `master` after a gated merge, under repository
policy; force-pushing `master` or deleting remote branches requires the human.
A branch that changes more than tracker records also needs a passing CI
run on its pull request before it merges; record-only changes merge without
one ([AGENTS.md](../../AGENTS.md)).

Atelier's merge command evaluates the reviewed base policy's gates and integrates
committed work with one merge commit, which marks the work done. Plain `merge`
never pushes; in a repository whose policy names a forge, `merge --push`
pushes the branch, opens its pull request and publishes the base
([CI on a forge](reviews-and-merge.md#ci-on-a-forge)). This repository's
policy names no forge yet, so the routine in AGENTS.md still applies. GitHub collaboration surfaces may
support discussion and dependency updates, but do not create a second tracker
or substitute for in-repo approval and evidence.

[CONTRIBUTING.md](../../CONTRIBUTING.md) is the contribution entry point. The
[pull request template](../../.github/PULL_REQUEST_TEMPLATE.md) asks for linked
work and validation; [Dependabot](../../.github/dependabot.yml) proposes Cargo
updates. Do not add issue templates or placeholder CODEOWNERS that invent
ownership absent from durable repository guidance.
