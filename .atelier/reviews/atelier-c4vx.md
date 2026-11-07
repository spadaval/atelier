---
schema: atelier.review
schema_version: 2
id: atelier-c4vx
title: finding on atelier-l5fu @ebc6e56f
target: atelier-l5fu
identity: Independent reviewer (Claude subagent)
commit: ebc6e56f46ff3ee8f0341a1a67a1a50bcd3fa804
created_at: 2026-11-07T21:34:49.945057081Z
event:
  event: finding
  recommendation: fix_now
---

medium: spadaval/.agents d4db367 evals/agent-factory/harness/run.py:198,211. The canary's positive control is guessable, so it does not prove what the second decision note says it proves. The random token is also part of the canary's directory name (work = work_root / f'{stamp}-canary-{token}'). own_seen = token in reply is then satisfied by any reply that prints a path inside the repo. The model sees that path as its working directory, and the prompt asks it for absolute paths. The note leans on this control to catch a CLAUDE.md leak even when the listing misses it ('an ancestor CLAUDE.md also hides the repository's AGENTS.md, so that check catches a leak too'). Scratch reproduction under /var/tmp, using the harness's own check_isolation with the backstop disabled: an ancestor CLAUDE.local.md is not loaded but still displaces the repo's AGENTS.md. Haiku answered NONE in 9 of 9 runs, so the canary correctly aborted. One of those replies added 'AGENTS.md shows as untracked in the git status snapshot, but its contents were not loaded'. The same sentence with the absolute path, '<repo>/AGENTS.md shows as untracked...', makes own_seen True and foreign empty, so the canary passes and the matrix runs without the scenario's AGENTS.md. I checked that by parsing the reply with run.py's own expressions. I have not seen Haiku produce that reply, in 12 runs with AGENTS.md absent or displaced. Fix (one line): keep the token out of the path by naming the directory with a separate random value, or match the heading, e.g. f'canary {token}' in reply. Then correct the note's 'unguessable' claim if the wording changes.
