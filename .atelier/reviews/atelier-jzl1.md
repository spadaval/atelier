---
schema: atelier.review
schema_version: 1
id: atelier-jzl1
title: finding on atelier-jzh9 @963c75a7
target: atelier-jzh9
identity: reviewer/planning-c (claude-opus-5-5)
commit: 963c75a7d7fc19c6e66faec4bb493bd9acc7e4ac
created_at: 2026-10-31T00:10:31.496873196Z
event:
  event: finding
  recommendation: fix_now
---

R3: Done when contradicts itself for a non-convention identity. Goal and Scope say free text with no validation, so `start --identity Rae` then `review approve --identity Rae` should be caught, but "for identities such as Rae or reviewer that do not follow the convention, behaviour is unchanged" reads as passing, and invites parsing the role, which Scope forbids. Fix: say the comparison is exact (normalized) text whatever the shape, and that the unchanged case is a reviewer identity differing from the recorded implementer.
