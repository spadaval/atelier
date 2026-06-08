# Worktrunk

## Source

- Repository: `https://github.com/max-sixty/worktrunk`
- Documentation: `https://worktrunk.dev`
- Local clone: `/root/atelier-references/worktrunk`
- Local clone commit: `ba6a3c2b56c34ef60ae8ca85dadcaf46524401fd`

These notes are based on the public repository and local clone above as read on
2026-06-08.

## Relevant Ideas

- Worktrees are addressed by branch name, while paths are computed from a
  configurable template. Atelier should treat branch/worktree naming as policy,
  not hard-coded string assembly.
- The core loop is intentionally small: switch/create, list/status, merge, and
  remove. Atelier should keep Git worktree helpers similarly compact.
- `wt list` is a strong status precedent. A useful Atelier status surface should
  show current worktree, dirty state, ahead/behind, unpushed commits, base
  relationship, and stale exported state.
- Hooks are project workflow configuration, not hidden one-off scripts.
  Worktrunk's create, pre-merge, post-merge, and post-start hooks point toward
  an Atelier workflow config that can prepare worktrees and run validation.
- Cache and local ignored-file sharing matter. Copying `target/`,
  `node_modules/`, generated env files, or other configured ignored files can
  remove most of the cost of opening parallel worktrees.
- Per-worktree dev server settings are an important operator detail. A stable
  hash-derived port or equivalent policy keeps parallel worktrees usable.
- Merge/remove flows should be one command when the policy allows it, but still
  remain transparent Git operations with clear failure recovery.

## Do Not Copy Blindly

- Do not turn Atelier into a generic worktree manager. Worktree helpers should
  serve tracked work, exported state freshness, and workflow gates.
- Do not launch or supervise direct agent processes as part of the current
  milestone. Worktrunk's `-x` execution pattern is useful inspiration for hooks,
  but direct agent-run management is deferred.
- Do not make generated commit messages part of the first design. Evidence,
  export freshness, and validation gates are higher-value primitives for
  Atelier.
- Do not hide Git. Operators and agents should still be able to understand the
  branch, worktree path, merge base, and cleanup action.

## Follow-Up Beads

- `atelier-x88e`: incorporate branch naming, path templates, and rebuild-after
  worktree creation.
- `atelier-msmg.1`: add scan-friendly worktree status, configured hook/cache
  setup, per-worktree local settings, and cleanup flows.
- `atelier-kitl`: externalized workflow config should own hooks and validation
  policy instead of scattering them through CLI flags.
