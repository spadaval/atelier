# Braid

## Source

- Crate/docs: `https://docs.rs/crate/braid/latest/source/README.md`
- Repository referenced by docs.rs: `https://github.com/alextes/braid`
- Local clone: `/root/atelier-references/braid`
- Local clone commit: `a101e7f8ccf873fb9fdec1fe0b8953db64a74e07`
- Local analysis source: `/root/AGENT_ISSUE_TRACKER.md`

These notes are based on Braid `0.9.0` docs and the local clone above as read
on 2026-06-08.

## Relevant Ideas

- Repo-local issue state is the main product lesson. Braid stores issues as
  Markdown files under `.braid/issues/`, making review, merge, and audit feel
  like normal Git work.
- The command surface is intentionally small: `brd add`, `brd ls`,
  `brd ready`, `brd start`, `brd done`, `brd dep add`, and `brd doctor` are
  easy for agents to learn and script.
- Ready/start/done is a good default loop for agent work. `brd ready` exposes
  unblocked work, while `brd start` claims work to avoid duplicate parallel
  execution.
- Agent worktrees are first-class. `brd agent init <name>` creates per-agent
  Git worktrees; `brd agent merge` handles rebase and fast-forward merge back
  to main.
- Braid separates workflow storage modes from auto-sync policy:
  issues-with-code, a dedicated issues branch, and an external issues repo are
  distinct choices. Atelier should preserve that explicitness when designing
  `.atelier-state/` and worktree behavior.
- `design` issues are a useful precedent for human-in-the-loop decisions.
  Atelier's decision beads and ADR flow should keep this spirit, but with
  richer links to decisions, evidence, and workflow validation.
- `meta` issues with rollup counts are useful as a lightweight grouping model.
  Atelier should translate that into first-class missions and milestones rather
  than overloading dependency edges for membership.
- Scheduled issues are a simple, practical affordance. Atelier should keep
  scheduled/deferred work as a core workflow primitive.
- The terminal UI is useful mainly as proof that repo-local structured state can
  drive operator surfaces. Atelier's Mission Control projection should aim for
  the same scan-friendly status overview before any rich UI.

## Do Not Copy Blindly

- Do not make Markdown parsing the hot path. Atelier's spec keeps SQLite as
  state in motion and deterministic exports as state at rest.
- Do not encode hierarchy only through dependencies. Braid's meta/dependency
  pattern is useful, but Atelier needs explicit mission, milestone, plan, and
  typed-link concepts.
- Do not limit durable issue kinds to regular, design, and meta. Atelier needs
  configurable issue/work item types plus first-class evidence, workflow
  validators, plans, runs, and decisions.
- Do not assume Git-only synchronization is enough for all runtime behavior.
  Atelier should use Git for mergeable durable state, while local SQLite indexes
  support fast queries, locks, workflow checks, and Mission Control projection.
- Do not copy editor-driven mutation as the main agent interface. Agents need
  stable noninteractive commands with focused human output and durable
  projection files.

## Follow-Up Decisions

- `atelier-nxxa`: decide the Atelier CLI binary and alias naming before
  adopting Braid-inspired command names.
- `atelier-xrzs`: define how Braid-like Markdown records map into
  `.atelier-state/`.
- `atelier-eqmn` and `atelier-x88e`: adapt Braid's agent/worktree flow for
  Atelier's `work start`, `work finish`, and worktree commands.
- `atelier-v72a`: decide how Braid's design/meta ideas map to first-class
  missions, milestone checkpoint records, plans, evidence, workflow validators,
  and runs.
- `atelier-kitl`: decide how much of the ready/start/done loop becomes
  configurable workflow behavior.
- Future ADR: storage mode policy for issues-with-code, dedicated state branch,
  or external state repo if Atelier supports more than the default
  `.atelier-state/` layout.
