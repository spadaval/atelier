---
schema: atelier.review
schema_version: 2
id: atelier-0bba
title: finding on atelier-bg7w @f4175923
target: atelier-bg7w
identity: gpt-6.1-sol reviewer
commit: f41759232002d63dd6ff417fd4e7ded782faa78e
created_at: 2026-11-09T04:38:09.845300436Z
event:
  event: finding
  recommendation: fix_now
---

medium: .atelier/issues/atelier-bg7w.md:29 R6. 'It ignores code blocks, code spans and link targets' applies to the read-path word count as well as readability unless the plan distinguishes them. Existing command instructions and review/digest output templates are code-formatted text agents must read; excluding them lets the budget be met by moving instructions into backticks or fences, recreating the line-budget problem. Define a whole-path word-count convention that includes instructional command/template text, separately exclude code from sentence/readability scoring, and test that code formatting cannot make instruction words free. Publish the policy in maintainers/skill-evaluation.md as a new policy: that file currently contains no line budget to replace.
