# Repository Contribution

Committed Atelier records are this repository's coordination source of truth.
[AGENTS.md](../../AGENTS.md) routes durable guidance; use
[Agent Factory](../../.agents/skills/agent-factory/SKILL.md) for process and
[development setup](development-setup.md) for tools.

Commit `.atelier/` changes with the implementation they describe. Keep work on
reviewable branches. Non-trivial executable changes require independent review
before local integration into `master`. Agents may integrate reviewed work
locally under repository policy; pushing any remote requires the human.

Atelier's merge command checks the reviewed base policy and fast-forwards
committed work. It never commits or pushes. GitHub collaboration surfaces may
support discussion and dependency updates, but do not create a second tracker
or substitute for in-repo approval and evidence.

[CONTRIBUTING.md](../../CONTRIBUTING.md) is the contribution entry point. The
[pull request template](../../.github/PULL_REQUEST_TEMPLATE.md) asks for linked
work and validation; [Dependabot](../../.github/dependabot.yml) proposes Cargo
updates. Do not add issue templates or placeholder CODEOWNERS that invent
ownership absent from durable repository guidance.
