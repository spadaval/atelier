---
created_at: "2026-11-09T05:11:55.115425355+00:00"
id: "atelier-icv4"
issue_type: "mission"
labels:
- "cli-output"
- "review"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "draft"
title: "Let agents run Atelier without working out its commands by trial and error"
updated_at: "2026-11-09T05:11:55.115425355+00:00"
---

## Outcome

An agent in any role finds the exact Atelier commands its step needs in what it already reads (its brief, `atelier --help`, a refusal's Next line), and runs no Atelier command its role does not need. A reviewer reads its brief, judges the code, records findings and an approval, and stops: no `status`, `land`, `check` or `--help` on the way.

## Background

GPT-6.1 Sol reviews run on ChatGPT Plus, which holds about $50 a week at API prices; 48 reviews on 2026-11-08 used 31% of it in about six hours. Every turn resends the whole context, so turns are the cost.

An audit of agent transcripts from 2026-11-04 to 2026-11-09 (Claude Code sessions in atelier, life-on-rails and astromech, main checkouts and T3 worktrees: 82 orchestrators, 93 implementers, 173 reviewers, 8 validators, 11 planners; Codex: 50 Sol reviews; cold-read and eval runs left out) parsed 8,123 Atelier invocations. Each command's cost is its share of the turn that ran it plus its output carried through every later turn of the session. The patterns overlap a little, so the savings are marginal estimates.

Atelier errors are rare (about 120 of 8,100 calls). The cost is in commands that succeed but should not be needed. A Sol review averages 14 turns, and Atelier commands take 6.2 of them:

- `status`, `land` and `check`, which a reviewer does not need: 1.8 turns, 14% of a review's tokens.
- Re-reading review history one record at a time: 1.1 turns, 14%.
- `--help` on record commands the brief already prints: 1.2 turns, 12%.

Claude reviewers do the same at lower rates. Orchestrators, implementers, planners and validators ran 541 `--help` lookups for write-command flags. Agents ran `scripts/merge-ci-passed.sh` 76 times where it cannot run.

Approvals outnumber reviews because planning reviews record one approval per item (up to 8), not because of retries: one of 31 Sol reviews approved the same item twice, and 2 record commands errored.

Each item below gives its pattern's counts, cause and fix. The loose end re-counts once they ship.
