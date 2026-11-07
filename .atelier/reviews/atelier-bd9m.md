---
schema: atelier.review
schema_version: 2
id: atelier-bd9m
title: 'Low: crates/atelier-cli/tests/cli_integration/debrief.rs No test feeds hostile text into the strings the new chips and cards embed (issue titles, summaries, Close Reason outcomes, parent titles, decision choices in the sidebar title attribute, producer identities, receipt summaries); only the mission Outcome is checked for escaping. I verified by hand in a scratch repo that <script>, <img>, quotes and & in those fields come out escaped and the page nests cleanly, but a regression would not be caught. Add a fixture with markup in an item title, Goal, Close Reason, decision choice and producer, and assert none survive unescaped.'
target: atelier-8bil
identity: Independent reviewer (Claude subagent)
commit: 6ee2a98cdea89ed3ac58ef59f139935c48c67fd3
created_at: 2026-10-18T07:00:30.276608869Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (implementer)
    commit: 8151a566626693a44fc6dd9c06568de0a78f6328
    recorded_at: 2026-10-18T07:07:56.811965582Z
    resolution_commit: 8151a566626693a44fc6dd9c06568de0a78f6328
    reason: Fixed in 8151a566
    migrated_from: atelier-9k8z
---

Low: crates/atelier-cli/tests/cli_integration/debrief.rs No test feeds hostile text into the strings the new chips and cards embed (issue titles, summaries, Close Reason outcomes, parent titles, decision choices in the sidebar title attribute, producer identities, receipt summaries); only the mission Outcome is checked for escaping. I verified by hand in a scratch repo that <script>, <img>, quotes and & in those fields come out escaped and the page nests cleanly, but a regression would not be caught. Add a fixture with markup in an item title, Goal, Close Reason, decision choice and producer, and assert none survive unescaped.
