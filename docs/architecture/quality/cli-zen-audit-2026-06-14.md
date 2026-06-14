# CLI Zen Audit, 2026-06-14

This audit compares Atelier's visible CLI surface with `docs/product/zen.md`.
It uses current command help, the product CLI/work-model docs, and the command
definitions in `src/main.rs`.

## Summary

Atelier's CLI mostly exposes the right domain nouns: missions, issues, proof,
history, worktrees, plans, and repository state. The remaining friction is not
that concepts are missing. It is that normal operator paths, recovery mechanics,
diagnostics, and older relationship mechanisms still sit too close together.

The main Zen gap is therefore information hierarchy. Operators should first see
how to understand current work, choose next work, record proof, and close loops.
Implementation mechanics such as projection rebuilds, exports, diagnostics JSON,
and destructive maintenance should stay available, but they should read as
advanced recovery tools rather than routine workflow.

## Findings

### Root Help Still Undersells The Product

Current root help opens with:

```text
A simple, lean issue tracker CLI
```

That is no longer the product Atelier is trying to be. The Zen says missions give
work a durable reason to exist, proof must stand on its own, and distinct
concepts should not collapse into one generic issue tree. The visible root
identity should therefore describe Atelier as a repository-owned work,
mission, and proof system, not just as an issue tracker.

Tracked by `atelier-4yrt`.

### Root Command Hierarchy Makes Recovery Look Routine

The root help is already grouped, but it gives `export`, `rebuild`,
`import-beads`, `integrations`, `maintenance`, and `diagnostics` the same
top-level weight as everyday commands such as `status`, `start`, `mission`,
`issue`, and `evidence`.

This conflicts with:

- Information hierarchy matters.
- Product concepts over implementation mechanics.
- Guidance at friction points.

The commands can remain available, but the first screen should make the normal
operator loop obvious: orient, pick work, inspect records, coordinate blockers,
record proof, and close or hand off. Repair and local diagnostics should be
clearly framed as recovery or advanced maintenance.

Tracked by `atelier-4yrt` and `atelier-j01c`.

### Relationship Management Is Fragmented

The CLI has several relationship paths:

- `issue create --parent`
- `mission add-work` and `mission add-blocker`
- `dep add`, `dep remove`, and `dep list`
- `link add`, `link remove`, and `link list`
- `graph hierarchy`, `graph related`, and `graph impact`
- `plan link`
- `evidence attach`

Some of this is good domain separation: blockers, mission membership, proof
links, and plan links are different concepts. The problem is that `link` also
exists as a generic typed issue-link mechanism, while record-specific commands
own other relationships. During mission setup, `link remove mission ...` looked
plausible but was not supported.

The product should choose one rule and make it obvious:

- Either centralize relationship creation and removal through one typed link
  surface.
- Or keep relationship operations on the owning record commands and remove or
  sharply scope generic `link`.

Tracked by `atelier-sxh8` and `atelier-od8a`.

### `workflow check` Has Split Semantics

`atelier workflow` is hidden from root help and labels itself as
advanced/internal diagnostics, but repository guidance still refers to workflow
checks as part of operator validation.

That creates a contract mismatch. If workflow policy validation is a normal
gate, it should be reachable through normal surfaces such as `status`, `doctor`,
`lint`, or mission status. If it is an advanced diagnostic, docs and agent
guidance should stop presenting it as a normal handoff step.

Tracked by `atelier-gh3m`.

### Mission Creation Help Hides The Section Model

`atelier mission create --help` lists `--body`, `--constraint`, `--risk`, and
`--validation`, but it does not say which Markdown sections those flags populate.
In practice, `--body` populates Intent, not an arbitrary full mission body.

That is a small help-text issue with a large workflow effect: it caused manual
canonical Markdown repair during mission setup. The CLI should say exactly what
each flag means and include an example for a complete mission seed.

Tracked by `atelier-xbr0`.

### Evidence Commands Still Invite Untargeted Proof

`evidence record --target` remains optional in help, and `evidence attach` is a
visible sibling command. Optional untargeted proof can be useful for recovery,
but the Zen says proof must stand on its own. Normal proof capture should
encourage a target at creation time so future operators can inspect claims
without reconstructing intent from chat or shell history.

The existing transcript-derived tasks already cover this:

- `atelier-papa`: require corrective guidance for mistargeted evidence.
- `atelier-uu2o`: require validation evidence to name the claim it proves.

### Projection Repair Still Leaks Into Operator Thinking

`rebuild` and `export --check` are visible state-management commands. They are
valid tools, but the Zen says product concepts should outrank implementation
mechanics. The operator should not need to remember when projection state is
stale or how to refresh it after canonical Markdown changes.

Command-driven changes already refresh the projection. Manual canonical edits
and read/check paths should get the same transparent recovery behavior where
safe.

Tracked by `atelier-j01c`.

### Diagnostics JSON Needs A Sharper Boundary

`atelier diagnostics slow` advertises stable JSON. That is reasonable for local
telemetry and tooling, but it should not leak into normal planning or validation
guidance. AGENTFACTORY guidance already says not to plan or validate work by
parsing command-result JSON. The CLI should keep that boundary clear: diagnostics
JSON is for local performance analysis, not for ordinary work-state decisions.

Covered by the broader documentation and Agent Factory update epics; no new
tracker item is required unless diagnostics starts appearing in normal workflow
recipes.

### Hidden Legacy Surfaces Remain A Residual Risk

Several hidden issue subcommands still exist in the command enum for tests,
migration, or compatibility residue. Hidden commands are less urgent than
visible help drift, but they are still a maintenance risk because docs, tests, or
agents can rediscover and normalize them.

This should be handled through the existing compatibility-cleanup principle:
legacy aliases and patterns should be temporary, and removal should be direct
when the replacement is clearer.

Existing cleanup work under `atelier-4p7q` is sufficient unless a hidden command
becomes documented or recommended again.

## Recommended Direction

1. Make root help describe Atelier as a mission/proof-oriented repository work
   system.
2. Put the normal operator loop first: `prime`, `status`, `mission`, `issue`,
   `start`, `evidence`, `history`, `doctor`, and closeout-oriented commands.
3. Move rebuild/export/import/diagnostics/maintenance language toward recovery,
   migration, or advanced local operation.
4. Resolve relationship ownership: generic `link` should either become the
   central relationship API or disappear behind record-specific commands.
5. Treat workflow policy validation as either a normal gate or an advanced
   diagnostic, not both.
6. Make command help explain record section semantics wherever flags generate
   canonical Markdown sections.

## Tracker Coverage Added By This Audit

- `atelier-4yrt`: align root CLI identity and command hierarchy with Zen.
- `atelier-xbr0`: clarify mission create section semantics in help.
- `atelier-gh3m`: resolve workflow check visibility and operator contract.
