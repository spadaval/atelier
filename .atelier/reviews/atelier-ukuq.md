---
schema: atelier.review
schema_version: 1
id: atelier-ukuq
title: 'Re-reviewed git diff 50adccc2 HEAD (excluding .atelier) at e9be31ca. Fixes hold: use_cases.rs doc comment now matches the code (4h7j); clap''s SuggestedValue becomes the hint ''Did you mean pass?'' (qdot); the unknown-transition error ends ''These are the transitions out of status todo.'' (defz); a malformed --target (''bad'', ''issue/'') is an InvalidValue naming issue/<id>, evidence/<id> and exits 2, and the Errors section covers wrong-form values (ry08). All are asserted in every_invalid_value_takes_the_shared_form_and_exits_2. Merged fnak/4775 code does not conflict with them: transition_issue still resolves the name (InvalidValue, exit 2) before ensure_transition_available (''not available from status'', exit 1) and fnak''s reworked ensure_transition_allowed (blocked, exit 1, with validator hints as Next:). start.rs only calls ensure_transition_allowed, so no InvalidValue reaches start. In a fixture, -q unknown-transition, -q blocked and -q --limit 0 refusals all leave stdout empty. cargo nextest run -j 4 at HEAD: 757/757 passed. Not run: --features test-hooks.'
target: atelier-pyfr
identity: Independent reviewer (Claude subagent)
commit: e9be31ca573fdcb4113559d5b14ac674c69531b7
created_at: 2026-10-17T09:57:07.284939306Z
event:
  event: approval
---

Re-reviewed git diff 50adccc2 HEAD (excluding .atelier) at e9be31ca. Fixes hold: use_cases.rs doc comment now matches the code (4h7j); clap's SuggestedValue becomes the hint 'Did you mean pass?' (qdot); the unknown-transition error ends 'These are the transitions out of status todo.' (defz); a malformed --target ('bad', 'issue/') is an InvalidValue naming issue/<id>, evidence/<id> and exits 2, and the Errors section covers wrong-form values (ry08). All are asserted in every_invalid_value_takes_the_shared_form_and_exits_2. Merged fnak/4775 code does not conflict with them: transition_issue still resolves the name (InvalidValue, exit 2) before ensure_transition_available ('not available from status', exit 1) and fnak's reworked ensure_transition_allowed (blocked, exit 1, with validator hints as Next:). start.rs only calls ensure_transition_allowed, so no InvalidValue reaches start. In a fixture, -q unknown-transition, -q blocked and -q --limit 0 refusals all leave stdout empty. cargo nextest run -j 4 at HEAD: 757/757 passed. Not run: --features test-hooks.
