---
schema: atelier.review
schema_version: 2
id: atelier-njt9
title: finding on atelier-l5fu @ebc6e56f
target: atelier-l5fu
identity: Independent reviewer (Claude subagent)
commit: ebc6e56f46ff3ee8f0341a1a67a1a50bcd3fa804
created_at: 2026-11-07T21:34:57.578300604Z
event:
  event: finding
  recommendation: defer
---

low: spadaval/.agents d4db367 evals/agent-factory/harness/run.py:209-210. The foreign-file check fails open when a reported path is mangled. A reported path counts only if it names an existing file, so formatting that changes the string drops it silently. '**/home/u/.claude/CLAUDE.md**' parses as '/home/u/.claude/CLAUDE.md**', because rstrip does not remove '*'. '~/.claude/CLAUDE.md' parses as '/.claude/CLAUDE.md'. Neither is a file, so foreign stays empty, and the canary passes if the token also appears. I checked both by parsing the replies with run.py's expressions. Haiku used bold spontaneously in a separate probe reply, but every canary reply so far has been plain lines. A displacing CLAUDE.md is still caught by the positive control (once R1 is fixed); a leaked AGENTS.md or rules file, which displaces nothing, would get through. Not blocking: with the work root outside /home/supadava and the backstop, a leak needs a future Claude Code to load a new name. Fix: strip Markdown emphasis and expand ~, or fail closed by treating any reported absolute path outside the repo as foreign. A made-up path then costs a canary rerun. This could go in with R1's fix.
