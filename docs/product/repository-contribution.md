# Repository Contribution Policy

Atelier's committed tracker records are the repository's coordination source of
truth. GitHub collaboration files may support review and dependency hygiene,
but they must not introduce a competing issue workflow or imply human/team
ownership that is not recorded elsewhere.

## Classification

| Surface | Decision | Rationale |
| --- | --- | --- |
| `CONTRIBUTING.md` | Add | A root contribution entry point reduces setup and workflow mistakes by routing contributors to `AGENTS.md`, `AGENTFACTORY.md`, and Atelier issue evidence. |
| Pull request template | Add | Pull requests are a useful review boundary when they require linked Atelier work, evidence, and validation commands. |
| GitHub issue templates | Not applicable | Atelier issues are canonical. GitHub issue templates would create a parallel intake path and weaken tracker-first coordination. |
| `CODEOWNERS` | Defer | The repository does not have durable human or team owner identifiers to encode. A placeholder owner file would create false review expectations. |
| Dependency update automation | Add | Cargo dependency update pull requests reduce drift without changing the work queue. The pull request template still requires Atelier ownership before merge. |

## Added Surfaces

- [CONTRIBUTING.md](../../CONTRIBUTING.md) is the contributor entry point.
- [Pull request template](../../.github/PULL_REQUEST_TEMPLATE.md) requires a
  linked Atelier issue or mission, evidence, and validation commands.
- [Dependabot configuration](../../.github/dependabot.yml) checks Cargo
  dependencies weekly.

## Deferred Surfaces

Do not add `CODEOWNERS` until the repository has a real owner identifier to
name. Do not add GitHub issue templates unless the product decision changes and
GitHub intake is intentionally reconciled back into committed Atelier records.
