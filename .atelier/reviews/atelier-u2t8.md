---
schema: atelier.review
schema_version: 1
id: atelier-u2t8
title: 'crates/atelier-app/src/lint.rs:210-218: a strategy whose front matter has an empty key (`mission:` or `mission: ~`) is reported as ''front matter names mission null'', so check fails. Scenario: following the new mission guide advice to write the strategy first, an author leaves `mission:` blank until the mission exists (the strategic-plans template lists the key); check, which runs in gates, fails with a message naming a mission ''null''. A blank key names no mission and should be treated like an absent key. Fix: return None for Value::Null (and an empty string). Lists and maps currently print as YAML (''names mission - id''); report them as ''mission: must be a mission ID'' instead of serializing. Add a test case for the empty key.'
target: atelier-9m4k
identity: Independent reviewer (Claude subagent)
commit: 5ea0b021f0cd536f14bcbb311766fd499f2f023d
created_at: 2026-10-15T23:56:14.600984852Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-app/src/lint.rs:210-218: a strategy whose front matter has an empty key (`mission:` or `mission: ~`) is reported as 'front matter names mission null', so check fails. Scenario: following the new mission guide advice to write the strategy first, an author leaves `mission:` blank until the mission exists (the strategic-plans template lists the key); check, which runs in gates, fails with a message naming a mission 'null'. A blank key names no mission and should be treated like an absent key. Fix: return None for Value::Null (and an empty string). Lists and maps currently print as YAML ('names mission - id'); report them as 'mission: must be a mission ID' instead of serializing. Add a test case for the empty key.
