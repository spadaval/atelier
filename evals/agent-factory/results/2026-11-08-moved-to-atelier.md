# Moved to spadaval/atelier (atelier-dkul), 2026-11-08

The harness runs from this repository. It found the repository root, staged
`skills/agent-factory` from it with `git archive`, and the canary and one
`trivial-typo` run passed.

- **Skill:** `2fd7620f` in spadaval/atelier, whose `skills/agent-factory`
  tree is identical to spadaval/.agents `c75a9b1`.
- **Harness:** `harness/run.py`, Claude Code 2.1.318, `--effort high`, skill
  arm only, Opus 5.5, work directory under `/var/tmp/af-evals`. The Haiku
  canary passed and reported only its repository's own `AGENTS.md`.
- **Binary:** `atelier 0.3.1+44cbe69c`.
- **Runs:** 1. Its `final.md` and tool calls were read by hand, and its init
  event is valid (skill loaded, no MCP servers). Transcripts are in
  `~/.cache/af-evals/runs/2026-11-08-dkul-move/`.

## Result

| Scenario | Skill `2fd7620f` |
| --- | --- |
| trivial-typo | 1/1 |

The agent loaded the skill, read the staged `references/atelier.md`, fixed the
word, recorded the trivial exemption with `atelier review trivial --base`,
checked that `atelier land` passed, and pushed. It opened no issue, branch or
subagent. This is a smoke test of the move, not a measure of the skill.
