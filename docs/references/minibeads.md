# Minibeads

## Source

- Crate/docs: `https://docs.rs/crate/minibeads/0.13.1`
- Repository referenced by docs.rs: `https://github.com/rrnewton/minibeads`
- Local clone: `/root/atelier-references/minibeads`
- Local clone commit: `f21ef616b512808f57bf71453143f5a4c619e0a0`
- Local analysis source: `/root/AGENT_ISSUE_TRACKER.md`

These notes are based on Minibeads `0.13.1` docs and the local clone above as
read on 2026-06-08.

## Relevant Ideas

- Markdown-only storage under `.beads/issues/` is the strongest reference
  point. Issues are ordinary `.md` files with YAML front matter, which makes
  records human-readable, Git-friendly, and easy to inspect without a daemon.
- The issue document shape is worth borrowing: front matter for structured
  fields, then Markdown sections for Description, Design, Acceptance Criteria,
  and Notes.
- Dependency-aware ready work is central. Minibeads supports blockers and a
  ready query without requiring a database.
- Parent and typed dependency concepts are useful precedents. Minibeads names
  dependency types including `blocks`, `related`, `parent-child`, and
  `discovered-from`.
- The Beads-like command surface is useful for migration and agent familiarity:
  create, list, show, update, dependency add, ready, and stats.
- MCP compatibility is a useful integration target. Atelier should expose
  stable machine-readable commands first, and can add MCP integration once the
  core CLI contracts are stable.
- Coarse-grained file locking is a useful simplicity benchmark. Atelier may need
  stronger coordination for multi-agent work, but the lock model should remain
  understandable and observable.

## Do Not Copy Blindly

- Do not adopt Markdown-only runtime state. Atelier intentionally uses SQLite
  for fast local queries, workflow checks, locks, and Mission Control
  projections.
- Do not keep fixed issue types as the long-term model. Minibeads' documented
  fixed types are `bug`, `feature`, `task`, `epic`, and `chore`; Atelier needs
  configurable types and first-class non-issue records.
- Do not make `.beads/` the target state directory. Atelier's storage contract
  is `.atelier/` for runtime state and `.atelier-state/` for deterministic
  exported state.
- Do not depend on a drop-in Beads compatibility layer as the product
  architecture. Compatibility can help migration, but Atelier's domain model is
  broader than issue CRUD plus dependencies.
- Do not treat parent-child as just another dependency edge in the long term.
  Membership and sequencing need different link semantics.

## Follow-Up Decisions

- `atelier-xrzs`: use Minibeads' front matter plus Markdown body pattern as an
  input to the `.atelier-state` record format decision.
- `atelier-ywow` and `atelier-fq9y`: ensure deterministic export/rebuild keeps
  the human-readable benefits of Minibeads without making Markdown the runtime
  source.
- `atelier-8tf0`: decide how Minibeads' dependency type vocabulary maps to
  Atelier typed links.
- `atelier-v72a`: decide which issue fields become shared record fields across
  missions, milestones, plans, evidence, gates, and runs.
- `atelier-ttp5`: compare Minibeads' coarse-grained file lock approach with
  inherited Chainlink lock/sync behavior before choosing Atelier coordination
  semantics.
- Future ADR: canonical record format for `.atelier-state` Markdown files.
