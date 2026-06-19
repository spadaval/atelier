# `atelier bundle`

Primary role: Manager/orchestrator.

Primary question: "How do I review and apply a prepared graph of missions,
issues, links, blockers, and evidence references in one controlled operation?"

## Assessment

- Name: Correct. `bundle` signals authored input and avoids pretending bulk
  planning is ordinary single-record CRUD.
- Documentation: Visible in root help, but needs stronger manager guidance that
  `preview` is the first move and `apply --yes` is the deliberate mutation.
- Design: Correct as the bulk graph creation path. It should not become a
  private shell-loop alternative to `mission create`, `issue create`, and
  relationship commands.
- Output hierarchy: Input path, parse/validation result, records and links that
  would change, then exact apply or correction command.

## Operator Assessment

| Form | Persona | Likely use cases | Information wanted | Likely next action | Guidance/orientation |
| --- | --- | --- | --- | --- | --- |
| `bundle preview <file>` | Manager/orchestrator | Review a planned mission graph before mutation; check an agent-authored backlog bundle; validate relationships and blockers. | Whether the file parses, which records/links/evidence references it would create, and which errors block application. | Fix the bundle or run `atelier bundle apply <file> --yes`. | Good command split. Help should emphasize preview-first and name `issue create`/`mission add-work` for one-off work. |
| `bundle apply <file> --yes` | Manager/orchestrator | Create a reviewed work graph; import a planned mission slice; turn an approved artifact update into tracker state. | What was created, generated IDs, relationship results, and validation/lint follow-up. | Run `atelier graph tree --compact`, `atelier mission show <id>`, or `atelier lint`. | Mutation requires `--yes`, which is good. Output should never leave the next inspection command implicit. |

## Guidance Finding

`bundle` is a high-leverage manager command. Per `zen.md`, it should make
coordination visible by showing the graph effect and next inspection command,
not just success. It should not appear in worker/reviewer core loops except when
reviewing the authored bundle as evidence.
