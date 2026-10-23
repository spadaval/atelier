---
schema: atelier.review
schema_version: 1
id: atelier-hen4
title: finding on atelier-0uaj @d0752d08
target: atelier-0uaj
identity: Independent reviewer 2 (Claude Sonnet subagent)
commit: d0752d08c118347563e06c6c7d38857b2cdad47e
created_at: 2026-10-23T16:09:55.395164234Z
event:
  event: finding
  recommendation: defer
---

low: crates/atelier-cli/src/commands/session.rs land_base_mismatch fix: the perl one-liner works verbatim for bases with / + . ( ) # (release/1.x, a+b, x.y, v(1), a#b: gate clears, only the branch_base line changes, commit succeeds) but fails for legal git branch names containing | (delimiter, syntax error), $ or @ (perl interpolation inside \Q..\E and the replacement, so no change and commit finds nothing) or ' (breaks the shell quoting). Loud failure, not corruption; such base names are rare. Suggested: pass the values through the environment (BASE=.. perl -pi -e 's/^  branch_base: .*\n/  branch_base: $ENV{TRUSTED}\n/' with shell-quoted values) or shell-quote and use a non-interpolating form.
