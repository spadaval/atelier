---
schema: atelier.review
schema_version: 2
id: atelier-d6fh
title: finding on atelier-5fsj @f35e6e42
target: atelier-5fsj
identity: gpt-6.1-sol-5fsj-r1
commit: f35e6e4206d454b2355d188a33aa4e7689f5c005
created_at: 2026-11-10T14:57:18.257901889Z
event:
  event: finding
  recommendation: fix_now
---

low: scripts/scorecard/commands.py:44 R3. is_process only looks for atelier or a literal .atelier path in the command, so a bare git commit after an earlier staging command is counted as non-process even when it commits only tracker records. This contradicts the strategy definition, which explicitly includes commits only there; the Report acknowledges this exclusion as a decision without escalating the changed definition. I verified one such successful command in a life-on-rails thread tied to a merged item: its output commit hash resolves to a manifest-history commit whose changed files are all under .atelier, but is_process returns false. Use frozen command/output and manifest Git history to recognize proven record-only commits, add a fixture that stages records separately and then commits them, and regenerate both shell and token process shares. Do not infer these from the live index.
