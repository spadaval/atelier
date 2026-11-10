---
schema: atelier.review
schema_version: 2
id: atelier-6a6v
title: finding on atelier-mal9 @e9b51789
target: atelier-mal9
identity: sol-mal9-r1
commit: e9b517897c4be5ce3f4c7904bbbcabec201110da
created_at: 2026-11-10T15:19:09.545271532Z
event:
  event: finding
  recommendation: fix_now
---

medium: next/crates/atelier-records/src/body.rs:134 Fenced Markdown examples are interpreted as real record fields. The finding loop at line 67 has the same problem. Reproduced a deferred finding containing a fenced text example Taken: atelier-bbbb by example-thread: validate returns no problems and is_settled becomes true. A pending Decisions entry containing only a fenced example of Finding: 1-aaaa.1, Answer by example-thread: Example only, Outcome: no action similarly validates, acquires an answer and makes the actual fix-now finding disappear from open_findings. The section splitter ignores fenced headings, but these raw content.lines loops still consume fenced field lines, so documenting command output or an answer format can silently settle work or close an objection. Ignore fenced literal content when extracting structured fields, consistently for findings and Decisions, and add regression tests proving these examples leave the finding unsettled/open and the question unanswered while rendering preserves the body unchanged.
