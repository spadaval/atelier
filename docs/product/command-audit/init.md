# `atelier init`

Primary role: Admin.

Primary question: "How do I create or reconcile the Atelier project state in
this repository?"

## Decision Record

| Operator question | Role | Product/cognitive cost | Architecture/code cost | Verdict | Next action |
| --- | --- | --- | --- | --- | --- |
| How do I establish trustworthy project state? | Admin | Low as a distinct setup step; import must remain explicit. | Scaffolding, configuration, and optional migration input. | Keep | Keep setup bounded; use `check` after setup and hide standalone migration mechanics. |

## Assessment

- Name: Correct. `init` is the expected setup verb.
- Documentation: Mostly correct. It should stay in setup/admin guidance and
  should not be presented as ordinary workflow after a repository is initialized.
- Design: Correct if it writes only tracker scaffolding, workflow config, local
  runtime storage, and ignore rules. `--import-beads` is acceptable as explicit
  migration input, but the command must not silently import predecessor state.
- Output hierarchy: Created/reused paths first, migration detection second,
  verification commands last. It should point to `atelier check`,
  `atelier man admin`, and `atelier status` before issue creation.

## Role Use

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `atelier init` | Admin | Create first-class Atelier state. | Good. |
| `atelier init --force` | Admin | Reconcile existing setup. | Good, but should clearly say what was reused or repaired. |
| `atelier init --import-beads` | Admin | Explicit predecessor import during setup. | Good as a migration option, not as default setup behavior. |
