---
schema: atelier.mission
schema_version: 3
id: atelier-icv4
title: Let agents run Atelier without working out its commands by trial and error
status: dropped
labels:
- cli-output
- review
lead: null
ready: null
created_at: 2026-11-09T05:11:55.115425355Z
---

## Outcome

An agent in any role finds the exact Atelier commands its step needs in what it already reads (its brief, `atelier --help`, a refusal's Next line), and runs no Atelier command its role does not need. A reviewer reads its brief, judges the code, records findings and an approval, and stops: no `status`, `land`, `check` or `--help` on the way.

## Result

Dropped: Its five fixes targeted the old review brief, land, merge-ci-passed.sh and the old write commands, all removed in 0.5. The scorecard rerun (atelier-pwhn) measures command fumbling on the new tool.
