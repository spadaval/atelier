---
schema: atelier.review
schema_version: 1
id: atelier-e91j
title: 'docs/product/work-model.md:39-44: says work started with the bare ''issue transition <id> start'' on the base or an unowned branch ''fails atelier land with land_work_untraceable''. That is only true while the item is still active (session.rs check_traceability filters on the active category). Repro: on main, create a spike, ''issue transition start'', commit, ''issue transition close'', commit, then ''atelier land'' prints ''Landing checks passed''. That todo->in_progress->done on the base is exactly the astromech repro behind item 6, so the doc promises a safety net that is not there. Fix: say land refuses only while the item is still active, and that work driven to done on the base is not caught (or make land/check catch it). Also docs/product/cli-surface.md:60 ''start prepares Git context'' sits in the transitions paragraph and reads as if the transition does it; say ''atelier start''.'
target: atelier-kaor
identity: Independent reviewer (Claude subagent)
commit: 3f41d55973b5f9469c0ed8066ded2673c0be2259
created_at: 2026-10-15T23:46:43.852795905Z
event:
  event: finding
  recommendation: fix_now
---

docs/product/work-model.md:39-44: says work started with the bare 'issue transition <id> start' on the base or an unowned branch 'fails atelier land with land_work_untraceable'. That is only true while the item is still active (session.rs check_traceability filters on the active category). Repro: on main, create a spike, 'issue transition start', commit, 'issue transition close', commit, then 'atelier land' prints 'Landing checks passed'. That todo->in_progress->done on the base is exactly the astromech repro behind item 6, so the doc promises a safety net that is not there. Fix: say land refuses only while the item is still active, and that work driven to done on the base is not caught (or make land/check catch it). Also docs/product/cli-surface.md:60 'start prepares Git context' sits in the transitions paragraph and reads as if the transition does it; say 'atelier start'.
