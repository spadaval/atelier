# Zen Of Atelier

Atelier facilitates long-running mostly-autonomous multi-agent workflows to
reliably complete ambitious work.

## Principles

The repository is the source of truth.
Work, policy, and history must be reviewable from committed state.
No hidden state, no magic.

Proof must stand on its own.
Someone who did not do the work should be able to inspect the record,
check the claim, and continue.

Value and outcomes first.
Missions exist for a reason.
Issues are accountable steps toward that reason.

Model the domain, don't flatten it.
Missions, epics, issues, and evidence are different shapes because they
answer different questions.

Domain truth is not UI.
Domain services say what is true, allowed, blocked, or required.
Renderers decide wording, layout, emphasis, and command spelling.
Do not smuggle UI annotations into domain code, and do not reimplement domain
rules in renderers.

One truth can have many views.
Status, work queues, issue detail, transitions, history, and checks may show
different slices of the same facts.
They must not maintain parallel interpretations of those facts.

Coordination must be visible.
Blockers, ownership, and agreements live in records and policy, not in
someone's head or chat history.

Agents make mistakes. The system must detect errors and guide recovery.
Workflow catches what agents forget.
Status surfaces make uncertainty visible and explain how to move forward.

Show the right layer first.
Default output should name the state, the cause, and the next useful step.
Do not dump implementation machinery by default.
Do not hide it behind vague failure messages.

Commands answer operator questions.
Do not print data just because it exists.
Do not invent generic next actions.
Every visible line should help the operator understand state, choose work, or
recover from a problem.

Workflow enforces the minimum. Agents handle the rest.
Add checks only where agents skip evidence or close too early.
Give agents freedom for everything else.

Every feature must justify its cost.
Less is more. Curation is a virtue.

Resilience over perfection.
Keep useful paths open.
Make uncertainty visible.
Treat blocked or failed outcomes as information, not dirt to sweep away.

Delete the old path once the new one is clear.
Do not preserve obsolete commands, aliases, or fallback behavior.
