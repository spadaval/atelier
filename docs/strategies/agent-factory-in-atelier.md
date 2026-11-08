---
status: draft
last_changed: "2026-11-08: created from Sumeet's direction"
---

# Why

Agent Factory (the skill) and Atelier are one product split across two repositories. The skill mentions Atelier 52 times, ADR 0020 says it targets Atelier only, and ADR 0020 Decision 3 exists only to keep the two repositories in step. A command change and the guidance that describes it land in separate commits, reviews and CI runs, and the guidance drifts. Moving the skill here means both land in one commit, one review and one CI run.

The skill is also global, and its description matches almost any multi-step software work, so it loads in repositories that have nothing to do with Atelier.

# Outcome

- The skill's only copy is `skills/agent-factory` in this repository, with its git history, and its evals are `evals/agent-factory`. spadaval/.agents no longer holds it.
- Anyone installs it with `npx skills add spadaval/atelier --skill agent-factory -g` (Vercel's `skills` CLI), unpinned, and the skill's first lines give that line and atelier's release installer.
- `atelier doctor` warns, without failing, when the binary is older than the latest GitHub release or the installed skill is behind, and prints the update command. `atelier status` shows the same warning from a cached result, so it stays fast. A skill symlinked to a checkout is reported as linked, not stale.
- `atelier setup --link` points the installed skill at this checkout for editing it, and `npx skills update` leaves the link alone.
- The skill's first step is `atelier status`. Outside an Atelier repository, or without atelier installed, it steps aside.
- spadaval/.agents loses the Codex-only skills (`artifact-hub`, `generate-html-plan`, `pr-review-explorer`) and `codex-reflect`, after anything useful from `codex-reflect` is folded into Agent Factory's `audit.md`.

# Shape

- **One source, moved, not vendored.** ADR 0023 rejected a second copy. This moves the single copy, so ADR 0023's reason still holds. A new ADR supersedes it, closes ADR 0020's deferred "ship the skill with `atelier setup`" (no: the `skills` CLI installs it, and the binary carries no skill), retires ADR 0020 Decision 3 (now one commit by construction), and replaces Decision 1's plan-file fallback with "step aside outside Atelier".
- **Agent Factory owns process; Atelier owns records.** That boundary stays a directory boundary: `skills/agent-factory` says what to do and why, the CLI's help says how. Nothing under `crates/` reads the skill's files.
- **Freshness, not versioning.** This is an alpha hobby project. No version guarantee between skill and binary, and no skill built into the binary. A stale install warns; nothing blocks.
- **The skill is global.** Repository-specific guidance stays in the repository: `.atelier/`, `AGENTS.md` and the repository's own verify skill.
- **Wait for `atelier-l58v`.** Nothing edits the skill's files until `atelier-l58v` has merged into spadaval/.agents, so the skill is never edited in two places. Binary and doc work that does not touch the skill goes first.
- When qualities conflict: a fast `status` beats a fresh freshness result, and an unchanged skill on the move beats tidying it on the way.

# Scope

In: this repository's code, docs, ADRs and records; `skills/` and `evals/` once moved; spadaval/.agents (deleting what moved and the Codex-only skills); the symlinks `~/.claude/skills/agent-factory` and `~/.agents/skills/agent-factory` on this host.

Out: review verdicts, which `atelier-l58v` records as `fix_now` findings (`atelier-9mdj` and `atelier-7iq0`, a separate verdict record, were superseded on Sumeet's choice); the `firecrawl*` skills and retiring the spadaval/.agents repository, which Sumeet decides; moving `file-pr` (decided against; see the mission's decision note).

Target environment: Linux and macOS, Claude Code and Codex reading `~/.claude/skills` and `~/.agents/skills`, `npx` from Node available on the machines that install the skill.

# Authority

The orchestrator may reorder and split items, and change flag names and cache details, without asking. It asks Sumeet before any push (to either repository), before deleting anything from spadaval/.agents beyond the skills named in the Outcome, and before changing the Outcome.

# Risks and Unknowns

- A symlink to a checkout makes the live skill follow whatever that checkout has checked out, and `~/projects/atelier` switches branches during `atelier start`. Sumeet chose on 2026-11-08 to link a dedicated worktree kept on `master` (for example `~/projects/atelier-skill`), pulled after merges.
- Merging spadaval/.agents history brings unrelated root commits into this repository. Whether `atelier land` and `doctor` accept that history is checked before the import merges; a plain copy that names the source commit is the fallback.
- `npx skills check` may be slow or need the network; `status` only reads its cached result.
