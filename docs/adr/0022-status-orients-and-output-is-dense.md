# ADR 0022: Status Orients Sessions, and Output Is Dense Markdown

## Status

Proposed (2026-10-16, from discussion with Sumeet; atelier-bw0b).

Amends [ADR 0020](0020-agent-factory-targets-atelier-only.md) Decision 3:
Agent Factory takes session context from `status`, not `prime`.

## Context

`prime` and `status` both describe the current checkout, and neither is
complete. `prime` is the SessionStart hook's view. Without an owner it shows
the branch, checkout state, policy diagnostics, up to eight active items, the
needs-a-human count, and other branches, but not the ready count or missions
that `status` shows. With an owner it adds the owner's body, mission, strategy
path, recent reviews, and evidence freshness, which `status` does not show.
Agents have to guess which view to run.

`prime` also prints two static lines. "Guidance: read AGENTS.md, the
repository strategy, and …SKILL.md" points at files the harness can load
itself: Claude Code reads AGENTS.md when the repository has no `CLAUDE.md`, or
through an `@AGENTS.md` import in `CLAUDE.md`. This repository had a
`CLAUDE.md` without that import, so AGENTS.md did not load, and on 2026-10-16
AGENTS.md still named a finished mission as current. "Before handing off: …"
restates what the Stop hook enforces by running `land`. Guidance on using
Atelier already lives in `atelier help` and the Agent Factory skill's
`references/atelier.md`; ADR 0019 removed `atelier man`.

Output was also judged against the wrong reader. A recurring proposal was
JSON output for every view so agents "don't scrape text". Agents are not
parsers: they read Markdown, labels, and symbols such as ✓ fluently, and JSON
costs more tokens and buries the point. Scripts have `--quiet`, and two
commands have `--json` (`review list` and `merge --check`), used today only by
`scripts/bench_read_commands.py`.

What agents do instead shows where the views fall short. Counting Bash tool
calls in this repository's Claude Code transcripts up to 2026-10-16 (commands
that run `atelier work`, `status`, `prime`, `issue show`, or `issue list`,
excluding scripts that only quote them), agents ran about 60 and piped about
a third to `head`, `sed`, `grep`, or `tail`. In astromech, an adopter on an
earlier model, they ran about 220 and piped about half, mostly to `sed` to cut
one section or line out of `issue show`. The exact shares depend on the
counting rule; the pattern does not.
Agents also looped over IDs extracting the Hierarchy section to rebuild a
tree, and grepped `prime` for "human".

## Decision

1. **`status` is the one orientation view.** It shows live state only:
   checkout and owner (with what `prime` shows for an owner today), missions,
   the ready and needs-a-human counts with the top ready items, and other
   local branches. The SessionStart hook runs `atelier status --hook claude`.
   `prime` becomes a deprecated alias until known adopters' hooks no longer
   call it, then is removed.
2. **Static guidance stays out of command output.** Repository instructions
   belong in AGENTS.md, which the harness must load: Claude Code repositories
   either have no `CLAUDE.md` or import `@AGENTS.md` from it. How to use
   Atelier belongs in `atelier help` and the Agent Factory reference. No `man`
   command returns. Commands print state and next actions, not instructions
   an agent has already loaded.
3. **Output is dense Markdown for every reader.** One line per item, short
   headers, the status word with a symbol where it speeds scanning (✓, ⚠),
   and no repeated tokens a header already states. Detail views are brief by
   default, with a flag for the full record. Views gain no JSON or YAML mode
   for agents. `--quiet` and the existing `--json` flags on `review list` and
   `merge --check` remain interfaces for scripts. On a terminal, color,
   emphasis, and width-aware layout may decorate the same content, as
   `human-cli-output.md` already allows; redirected output, which is what
   agents read, is the plain Markdown.
4. **Piping is a missing view.** When agents routinely pipe atelier output
   through `head`, `grep`, `sed`, or loops, treat it as a defect in the
   views and file it, rather than adding a machine format.

## Consequences

- This repository's `CLAUDE.md` imports `@AGENTS.md`, and its
  `.claude/settings.json` runs the installed CLI's Atelier hooks instead of a
  leftover `bd prime`.
- atelier-sdb0 folds `prime` into `status` and updates every current
  reference: `PRODUCT_INTENT.md`, `AGENTS.md`,
  `docs/product/{sessions-and-hooks,cli-surface,work-model,index}.md`, root
  help, the `setup claude` message, this repository's `.claude/settings.json`,
  `scripts/bench_read_commands.py`, and Agent Factory's
  `references/atelier.md` and `procedures/install.md` (upstream per ADR 0020).
  `setup claude` warns when a `CLAUDE.md` would keep AGENTS.md from loading.
  atelier-jjvc removes the alias once this repository's and astromech's hooks
  no longer call it.
- The draft strategy `docs/strategies/review-and-introspection.md` (mission
  atelier-r6bt) names `prime` in S6 and its Scope. Accepting this ADR means
  those read as `status`; editing the strategy needs Sumeet's authority.
- atelier-deyy writes Decisions 3 and 4 into
  `docs/product/human-cli-output.md` and applies them to the `work` views;
  atelier-7u4o makes `issue show` brief and multi-ID.
- Other follow-ups, labelled `orientation`: atelier-fpxc (text search),
  atelier-ihns (between-missions state), atelier-7n5u (in-flight marks on
  ready work), and in backlog atelier-stsa (mission criteria and proof),
  atelier-fdxj (unblock ranking), and atelier-majk (recent activity).
- Making `status` the hook view raises its bar: it must answer the session's
  first questions without a second command.
