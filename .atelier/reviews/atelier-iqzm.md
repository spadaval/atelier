---
schema: atelier.review
schema_version: 1
id: atelier-iqzm
title: '.agents/skills/agent-factory/references/atelier.md:69-71 and crates/atelier-cli/src/main.rs:561: the agent guidance still says ''Target the mission or an item advancing it'' and marks come ''from the newest receipt'', and the --criterion help says ''Needs --target in the mission''s scope''. After the dqty fix, an item in several missions is refused, the deciding receipt must be in HEAD''s history, and a blocked receipt leaves a criterion unproven. An agent following the reference will be refused when it targets a shared item, with no hint why. Fix: match validation.md''s wording (''the mission, or an item only that mission holds''; newest receipt in HEAD''s history).'
target: atelier-ho7o
identity: Independent reviewer 2 (Claude subagent)
commit: 97f4dc47ec4ddf96e4da406883b0780a6da947d8
created_at: 2026-10-17T18:47:46.773529999Z
event:
  event: finding
  recommendation: defer
---

.agents/skills/agent-factory/references/atelier.md:69-71 and crates/atelier-cli/src/main.rs:561: the agent guidance still says 'Target the mission or an item advancing it' and marks come 'from the newest receipt', and the --criterion help says 'Needs --target in the mission's scope'. After the dqty fix, an item in several missions is refused, the deciding receipt must be in HEAD's history, and a blocked receipt leaves a criterion unproven. An agent following the reference will be refused when it targets a shared item, with no hint why. Fix: match validation.md's wording ('the mission, or an item only that mission holds'; newest receipt in HEAD's history).
