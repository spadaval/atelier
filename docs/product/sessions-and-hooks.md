# Sessions and Hooks

`atelier prime` gives a fresh session concise context: checkout and branch,
active work and its owner, relevant mission, blockers, proof, and repository
guidance. It reads records and Git without changing them.

`atelier land` checks handoff invariants. Record and policy changes must be
committed; the graph must be valid and active work traceable. Completion claims
must satisfy their evidence, dependency, and review requirements. A legitimate
handoff may retain active work; landing does not mean every item is done and
never creates a commit.

`atelier setup claude` installs Claude Code lifecycle hooks so session start
calls prime and stop checks land. Claude is the supported harness. Consult
command help for hook protocol options and installation details; hooks are
adapters to these explicit commands, not an agent runner.

`atelier status` includes labeled summaries for relevant local work branches.
A branch can contain active or terminal claims absent from the base. These
summaries preserve each branch's identity and never overlay its records onto
the current checkout. Invalid or unavailable branch state should be visible.
