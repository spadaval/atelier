# Transcripts

How to read earlier agent runs, for `audit` retrospectives and `garden`.

- **Start from raw transcripts**; memory, summaries and handoffs are hypotheses.
  Claude Code keeps a JSONL file per session in `~/.claude/projects/<dir>/`,
  `<dir>` being the working directory with `/` and `.` as `-` (one per
  worktree); Codex in `~/.codex/sessions/<yyyy>/<mm>/<dd>/`. For another
  harness, find its store first, and say so if you cannot.
- **Search first, then read around hits**: refusals, errors, user corrections.
- **Follow lineage to the root.** Trace subagent spawns to the root session and
  judge the run against the root task; a child alone answers only a question
  about that child. A created or forked thread is a peer, not a parent.
- **Check each claim in context.** Read the transcript around a failure,
  recovery or handoff claim before stating it: a child's report or a detected
  signal is a claim, and a gap between events does not prove idleness. Take Git
  history only from what the transcript recorded.
