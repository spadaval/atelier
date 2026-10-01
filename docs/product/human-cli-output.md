# Human CLI Output

Normal output should let an operator identify the work, understand its state,
and choose the next concrete action. Markdown records are the durable machine
boundary; human output is a focused view over them.

Detail views show the item title, exact configured status, intent, blockers,
proof, and ownership. Queues use compact rows with an ID, title, one state cue,
and a blocker count when needed. Rollups can use workflow categories; rows
should not repeat redundant category/status tokens.

Use specific labels for evidence, reviews, hierarchy, blockers, and Git facts.
Never present an attached receipt as passing proof without checking its result
and freshness. Branch summaries must name the branch and remain separate from
current-checkout state.

Next actions come from actual state: inspect a blocker, select ready work,
record proof, resolve a finding, or commit record changes. Avoid generic command
menus in every row. Errors name the failed invariant or path and give a useful
recovery step. Optional verbosity may show check details and command context.

`--quiet` limits output to essential IDs, counts, and result tokens. It does not
hide failures or change behavior. Diagnostic logging uses `--log-level` and
`--log-format`; logging is not a second work-state store. Color and terminal
width may improve scanning without changing meaning or breaking redirected
output.

See [inventory and mission views](issue-inventory-and-mission-overview.md) and
[work ordering](work-view-ordering.md) for view semantics.
