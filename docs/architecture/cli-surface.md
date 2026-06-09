# CLI Surface Tiers

Atelier's public CLI should present the agent-native workflow first. Inherited
Chainlink utilities may remain callable while migration work is in progress, but
they should not appear as peer commands in primary help unless they are
explicitly promoted by a follow-up issue or ADR.

## Core

Core commands are stable enough to appear in `atelier --help` and are expected
in normal Agent Factory workflows:

- `atelier init`
- `atelier issue ...`
- `atelier dep add/remove/list`
- `atelier export` and `atelier export --check`
- `atelier rebuild`
- `atelier import-beads`
- `atelier lint`
- `atelier doctor`

Future first-class record and workflow commands such as `mission`, `plan`,
`link`, `work`, `evidence`, and workflow validation commands become core only
when their owning issues define storage, JSON, export/rebuild, and validation
contracts.

## Hidden Compatibility

Compatibility commands remain callable for existing scripts, imported habits, or
migration paths, but docs should prefer their core replacements and primary help
should hide them:

- Flat issue aliases: `create`, `show`, `list`, `ready`, `close`, `update`,
  `block`, `unblock`, `search`, `relate`, `related`, `tree`, and similar aliases
  for `atelier issue ...`.
- `timer` and flat `start`/`stop`, pending a future run/session model.
- `archive`, because archive state is inherited issue lifecycle behavior rather
  than a target workflow primitive.
- `milestone`, until the first-class milestone record contract is updated to
  match the target domain model.
- `session`, because durable run/session accounting is deferred.
- `agent`, `locks`, and `sync`, until the claim/worktree/lock policy is decided.
- Backup `import` and `export --format json|markdown`, which are not canonical
  projection/rebuild.

Compatibility commands may be removed after their replacement path and migration
window are documented.

## Integration Or Experimental

Integration commands can remain implemented, but must not define the product's
default mental model:

- `cpitd`, because it depends on external clone-detection behavior and creates
  issues directly.
- `usage`, because token accounting belongs to future run/session or Mission
  Control policy.
- Assumption-specific impact commands such as `cascade` and `falsify`.

Generic replacements should use domain language. `atelier issue impact <id>` is
the visible relation-impact command. Until first-class `atelier link` commands
define directed typed-link semantics, impact follows hierarchy plus the
impact-bearing relation types `derived`, `caused-by`, and `falsifies`
transitively, and `assumption` one hop from the source. `cascade` is a hidden
compatibility alias for `impact`; `falsify` remains a hidden mutating legacy
command that labels and closes the source issue and should be removed or
reworked when typed-link workflow policy replaces assumption-specific behavior.

## Removed Or Deferred Behavior

The daemon surface and changelog-on-close behavior are not part of the target
public workflow. Issue closure records close state, close time, and optional
reason in tracker state; it does not mutate `CHANGELOG.md`.

The inherited backup export/import path is preserved only as compatibility.
Canonical state is `.atelier-state/`, checked with `atelier export --check` and
rebuilt with `atelier rebuild`.
