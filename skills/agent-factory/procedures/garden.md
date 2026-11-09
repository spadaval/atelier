# Garden

Use this subskill to tend the conditions agents work in, not the product: find
where agents trip and fix the ground. Change how agents work here (code
structure without behaviour change, checks, docs, error and help text, skills,
tooling), never what the product does; send a product question to the plan's
owner as an issue. The human gardens too: you do the bulk reading and the
routine upkeep, and bring them the rest.

A run covers one repository and never touches a branch in flight. Its window is
what is new since the last run (the latest digest, label `garden`), or the last
7 days on a first run. Read agent [transcripts](../references/transcripts.md),
review findings, reverts, refused checks and failed CI runs, and the code and
docs agents read: dead code, residue of finished migrations, docs the code
contradicts, broken links, and the [verify skill](../references/verify-skill.md)
where agents tripped on it.

A trip is where an agent went wrong or wasted effort: a misread refusal, a rule
broken or worked around, a retry loop, a wrong claim review caught. A trip seen
in transcripts earns a new rule, check or gate only once it has happened at
least twice in the last 30 days; mess found by reading the code needs no repeat.

## Fixing at the Right Level

Fix each trip at the highest level that works: architecture that makes it
impossible, then a check, then a sign where the agent trips (an error message or
help text, before a rule in a distant doc). Guardrails are nudges: a better
default or message comes before a new gate. Removing a rule whose mistake can no
longer happen counts as much as adding one, so look for those too.

Routine upkeep is a change whose correctness is evident from the diff, that adds
or removes no rule, check, gate, interface or default, in at most about 100
changed lines: deleting dead code or a finished migration's residue, fixing a
doc line the code contradicts or a broken link. Make it on one branch, have it
independently reviewed like any change and merge it; a run opens at most one
pull request. Everything else is a recommendation, as is anything you are unsure
of: a new check, a removed rule, reworded guidance or messages, a changed
default, a restructure.

## The Digest

File one digest per run and repository (`atelier issue create "Garden <repo>:
<n> recommendations" --issue-type decision -l needs-human -l garden`), or add to
the open one while any entry is unanswered; a run with none files none. Number
its entries R1, R2 and so on, each giving the trip and how often it happened,
its evidence (sessions and events, or paths and lines), the fix and its level
(architecture, check, sign or removal), its size (lines changed, and complexity,
value and risk) and the alternatives, leaving it included. The human answers
each in a note: do it, do it differently, or no. Start each run with those
answers: file each accepted entry as an issue, planned and reviewed as `plan`
and [Planning on the Base](../references/atelier.md#planning-on-the-base) say,
and name it in the digest; propose a declined one again only with new evidence,
and name that. Once every entry is answered and filed, `decide` the digest.
Report the window, the trips found, the routine fixes merged, the
recommendations made, and the rules and checks added and removed.
