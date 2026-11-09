# Transcripts

How to read earlier agent runs, for `audit` retrospectives and `garden`.

- **Start from raw transcripts**; memory, summaries and handoffs are hypotheses.
  Claude Code keeps them in `~/.claude/projects/<cwd, / and . as ->/`, one
  directory per worktree, and Codex in `~/.codex/sessions/<yyyy>/<mm>/<dd>/`;
  for another harness, find its store. Say what the window's stores lacked.
- **Search first, then read around hits**: refusals, errors, user corrections.
- **Follow lineage to the root.** Trace subagent spawns to the root session and
  judge the run against the root task; a child alone answers only a question
  about that child. A created or forked thread is a peer, not a parent.
- **Check each claim in context.** Read the transcript around a failure,
  recovery or handoff claim before stating it: a child's report or a detected
  signal is a claim, and a gap between events does not prove idleness. Take Git
  history only from what the transcript recorded.
