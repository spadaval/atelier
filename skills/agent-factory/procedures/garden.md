# Garden

Use this subskill to tend the conditions agents work in: find where agents trip
and fix the ground. Change how agents work here (code structure without
behaviour change, checks, docs, error and help text, skills, tooling), not what
the product does. A fix that would change the product (as when a repository
builds the tools agents trip on) or the host's setup is a recommendation; send a
product question to the plan's owner as an item. You do the bulk reading and
routine upkeep, and bring the human, who gardens too, the rest.

A run covers one repository and never touches a branch in flight. Its window
starts where the last run's ended, as the latest digest (label `garden`) records
it, or 7 days back if none does. Read agent
transcripts except gardener runs, other
repositories' too where they trip on the same tool, review findings, reverts,
refused checks and failed CI runs, and the code and docs agents read: dead code,
residue of finished migrations, docs the code contradicts, broken links, and the
verify skill where agents tripped on it.

A trip is where an agent went wrong or wasted effort: a misread refusal, a rule
broken or worked around, a retry loop, a kind of mistake review keeps catching
(a refusal followed, or one finding fixed, is a gate working). A trip seen in
transcripts earns a new rule, check or gate only once it has happened at least
twice in the last 30 days; mess found by reading the code needs no repeat.

## Fixing at the Right Level

Fix each trip at the highest level that works: architecture that makes it
impossible, then a check, then a sign where the agent trips (an error message or
help text, before a rule in a distant doc). Guardrails are nudges: a better
default or message comes before a new gate. Removing a rule whose mistake can no
longer happen counts as much as adding one, so look for those too.

Routine upkeep is a change whose correctness is evident from the diff, that adds
or removes no rule, check, gate, interface or default, in at most about 100
changed lines: deleting dead code or a finished migration's residue, fixing a
doc line the code contradicts or a broken link. Make it on one branch, reviewed
and merged like any change, in at most one pull request per run. Everything else
is a recommendation, as is anything you are unsure of: a new check, a removed
rule, reworded guidance or messages, a changed default, a restructure.

## The Digest

File one digest per run and repository (`atelier new "Garden <repo>: <n>
recommendations"`, then `atelier set <id> --labels needs-human,garden`), or add
to the open one while any entry is unanswered, and record the run's window in
its Goal; a run with no entries files one that says "no entries" and closes it
(`atelier close <id> "no entries"`). Its Goal asks which entries go ahead, and
gives each as `### R1 <title>`: the trip and
how often it happened, its evidence (sessions and events, or paths and lines),
the fix and its level (architecture, check, sign or removal), its size (lines,
and complexity, value and risk) and the alternatives, including leaving it.

The human answers each entry in conversation or in the digest's body, and the
agent writes the answer under its entry: do it, do it differently, or no. Start
each run with those answers: file each accepted entry as an item, or point it
at the item that already holds it, planned and reviewed as
[roles/planner.md](../roles/planner.md) says; propose a
declined one again only with new evidence, and name that. Once every entry is
answered and filed, close the digest (`atelier close <id> "<outcome>"`).
Report the window, the trips, the
routine fixes merged, the recommendations, and the rules and checks added and
removed.
