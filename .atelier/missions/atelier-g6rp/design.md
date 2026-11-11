---
status: draft
last_changed: "2026-11-08: created from Sumeet's direction"
---

# Shape

- **One source, moved, not vendored.** ADR 0023 rejected a second copy. This moves the single copy, so ADR 0023's reason still holds. A new ADR supersedes it, closes ADR 0020's deferred "ship the skill with `atelier setup`" (no: the `skills` CLI installs it, and the binary carries no skill), retires ADR 0020 Decision 3's sync rule (now one commit by construction) while keeping its concepts-versus-syntax boundary, and replaces Decision 1's plan-file fallback with "step aside outside Atelier".
- **Agent Factory owns process; Atelier owns records.** That boundary stays a directory boundary: `skills/agent-factory` says what to do and why, the CLI's help says how. Nothing under `crates/` reads the skill's files.
- **Freshness, not versioning.** This is an alpha hobby project. No version guarantee between skill and binary, and no skill built into the binary. A stale install warns; nothing blocks.
- **The skill is global.** Repository-specific guidance stays in the repository: `.atelier/`, `AGENTS.md` and the repository's own verify skill.
- **Wait for `atelier-l58v`.** Nothing edits the skill's files until `atelier-l58v` has merged into spadaval/.agents, so the skill is never edited in two places. Binary and doc work that does not touch the skill goes first.
- When qualities conflict: a fast `status` beats a fresh freshness result, and an unchanged skill on the move beats tidying it on the way.

# Risks and Unknowns

- A symlink to a checkout makes the live skill follow whatever that checkout has checked out, and `~/projects/atelier` switches branches during `atelier start`. Sumeet chose on 2026-11-08 to link a dedicated worktree detached at `master` (`~/projects/atelier-skill`). `atelier merge` does not move it, so it is moved forward with `git -C ~/projects/atelier-skill checkout --detach master`, and `doctor` warns when it is behind `master`.
- Merging spadaval/.agents history brings unrelated root commits into this repository. Whether `atelier land` and `doctor` accept that history is checked before the import merges; a plain copy that names the source commit is the fallback.
- `npx skills check` may be slow or need the network; `status` only reads its cached result.
