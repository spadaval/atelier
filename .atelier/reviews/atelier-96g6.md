---
schema: atelier.review
schema_version: 1
id: atelier-96g6
title: 'Reviewed git diff master...HEAD (master is an ancestor; c7eac73a..HEAD only adds records). Checked: with_article replaces RecordKind::with_article and is used by wrong_kind, work mission (work.rs:352) and the advances link-type error (workflow_policy.rs:61); grepped crates for format strings with a/an before a placeholder ('' a {'', '' an {'', ''is a '', ''not a {'') and found no other site that puts an article before a variable type or kind word; remaining ''is a mission/decision'' strings are fixed nouns. Issue types are lowercase identifiers, so the first-letter rule is an acceptable documented simplification (one no_action finding for u-initial words). Tests: unit test for the helper, unit test for the advances message with ''incident'', CLI test configuring a custom incident type and asserting ''is an incident, not a mission''; each would fail on the old ''a {type}'' wording. Built atelier-cli and ran the 16 record_ref/wrong_kind/scoped-view/advances tests with nextest -j 4: all pass. Did not rerun the full suite, fmt or diff-check (relied on evidence atelier-5zdf, atelier-a6id, atelier-pvs7 at c7eac73a), and did not mutate code to watch tests fail.'
target: atelier-jx7g
identity: Independent reviewer (Claude subagent)
commit: 1e29e392118f836aaa8903a906cb806568288517
created_at: 2026-10-17T08:55:24.734211809Z
event:
  event: review
---

Reviewed git diff master...HEAD (master is an ancestor; c7eac73a..HEAD only adds records). Checked: with_article replaces RecordKind::with_article and is used by wrong_kind, work mission (work.rs:352) and the advances link-type error (workflow_policy.rs:61); grepped crates for format strings with a/an before a placeholder (' a {', ' an {', 'is a ', 'not a {') and found no other site that puts an article before a variable type or kind word; remaining 'is a mission/decision' strings are fixed nouns. Issue types are lowercase identifiers, so the first-letter rule is an acceptable documented simplification (one no_action finding for u-initial words). Tests: unit test for the helper, unit test for the advances message with 'incident', CLI test configuring a custom incident type and asserting 'is an incident, not a mission'; each would fail on the old 'a {type}' wording. Built atelier-cli and ran the 16 record_ref/wrong_kind/scoped-view/advances tests with nextest -j 4: all pass. Did not rerun the full suite, fmt or diff-check (relied on evidence atelier-5zdf, atelier-a6id, atelier-pvs7 at c7eac73a), and did not mutate code to watch tests fail.
