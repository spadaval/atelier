---
schema: atelier.mission
schema_version: 3
id: atelier-icv4
title: Let agents run Atelier without working out its commands by trial and error
status: draft
labels:
- cli-output
- review
lead: null
ready: null
created_at: 2026-11-09T05:11:55.115425355Z
---

## Outcome

An agent in any role finds the exact Atelier commands its step needs in what it already reads (its brief, `atelier --help`, a refusal's Next line), and runs no Atelier command its role does not need. A reviewer reads its brief, judges the code, records findings and an approval, and stops: no `status`, `land`, `check` or `--help` on the way.
