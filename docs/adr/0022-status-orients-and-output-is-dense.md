# ADR 0022: Status Orients Sessions, and Output Is Dense Markdown

## Status

Proposed (2026-10-16, from discussion with Sumeet; atelier-bw0b).

## Context

`prime` and `status` both describe the current checkout, and neither is
complete. `prime` is the SessionStart hook's view. Without an owner it shows
the branch, one line of static guidance, and other branches, and less than
`status` (no counts, no missions, no ready work). With an owner it adds the
owner's body, mission, strategy path, recent reviews, and evidence freshness,
which `status` does not show. Agents have to guess which view to run.

The static guidance in `prime` ("read AGENTS.md, the repository strategy, and
the Agent Factory skill") repeats what the harness already loads from
AGENTS.md, and on 2026-10-16 it pointed agents at a finished mission. Guidance
about how to use Atelier has homes already: `atelier help` and the Agent
Factory skill's `references/atelier.md`. ADR 0019 removed `atelier man`.

Output was also judged against the wrong reader. A recurring proposal was
JSON output for every view so agents "don't scrape text". Agents are not
parsers: they read Markdown, labels, and symbols such as ✓ fluently, and JSON
costs more tokens and buries the point. Scripts have `--quiet`, and two
commands have `--json` (`review list` and `merge --check`), used today only by
`scripts/bench_read_commands.py`.

What agents do instead shows where the views fall short. Across about 1,600
orientation commands in this repository's Claude Code transcripts, agents
piped atelier output through `head` (59), `grep` (21), `sed` (18), and `tail`
(9). They trimmed `issue show`, grepped `issue list` for a word, extracted the
Hierarchy section in loops to rebuild a tree, and grepped `prime` for
"human".

## Decision

1. **`status` is the one orientation view.** It shows live state only:
   checkout and owner (with what `prime` shows for an owner today), missions,
   ready and needs-a-human work, and other local branches. The SessionStart
   hook runs `atelier status --hook claude`. `prime` becomes a deprecated
   alias for it and is removed once adopters have re-run `atelier setup
   claude`.
2. **Static guidance stays out of command output.** Repository instructions
   belong in AGENTS.md; how to use Atelier belongs in `atelier help` and the
   Agent Factory reference. No `man` command returns. Commands print state
   and next actions, not instructions an agent has already loaded.
3. **Output is dense Markdown for every reader.** One line per item, short
   headers, the status word with a symbol where it speeds scanning (✓, ⚠),
   and no repeated tokens a header already states. Detail views are brief by
   default, with a flag for the full record. Views gain no JSON or YAML
   mode for agents. `--quiet` and the existing `--json` flags on `review
   list` and `merge --check` remain interfaces for scripts.
4. **Piping is a missing view.** When agents routinely pipe atelier output
   through `head`, `grep`, `sed`, or loops, treat it as a defect in the
   views and file it, rather than adding a machine format.

## Consequences

- `docs/product/human-cli-output.md` gains the dense-output and piping rules;
  `docs/product/sessions-and-hooks.md` describes `status` as the hook view.
- Agent Factory's `references/atelier.md` says `status` instead of `prime`
  (upstream per ADR 0020).
- Follow-up work, labelled `orientation`: atelier-sdb0 (fold `prime` into
  `status`), atelier-7u4o (brief, multi-ID `issue show`), atelier-deyy (dense
  work views), atelier-fpxc (text search), atelier-ihns (between-missions
  state), and atelier-7n5u (in-flight marks on ready work); in backlog,
  atelier-stsa (mission criteria and proof), atelier-fdxj (unblock ranking),
  and atelier-majk (recent activity).
- Making `status` the hook view raises its bar: it must answer the session's
  first questions without a second command.
