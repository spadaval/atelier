# Symphony

## Source

- Repository: `https://github.com/openai/symphony`
- Spec: `https://github.com/openai/symphony/blob/main/SPEC.md`
- Local clone: `/root/atelier-references/symphony`
- Local clone commit: `b4ccf7b55327821ca6d9b1b8b9e4ab5ac7f30e15`

These notes are based on the public repository and local clone above as read on
2026-06-08.

## Relevant Ideas

- Repository-owned workflow configuration is the strongest lesson. Symphony's
  `WORKFLOW.md` combines YAML front matter for runtime settings with a Markdown
  body for task instructions.
- Config should be self-contained, versioned, parseable, and strict. Missing
  files, invalid front matter, unknown template variables, and bad values should
  produce explicit configuration errors rather than silently falling back.
- Dynamic reload is valuable. Future dispatches, checks, hooks, and guidance
  should use the latest valid workflow config without requiring a restart where
  practical.
- Environment indirection should be explicit. A value such as `$VAR` can opt in
  to environment resolution, but environment variables should not invisibly
  override repository policy.
- Hooks belong in the workflow contract with defined timeout and failure
  semantics. This maps cleanly to Atelier gates and worktree setup hooks.
- Strict prompt/template rendering is a good precedent for action-aware
  guidance. Unknown variables or filters should fail close to the command that
  needs the guidance.
- Symphony draws a useful boundary: the scheduler reads tracker state, while
  ticket writes and workflow-specific success are defined by the workflow. For
  Atelier, workflow state should be explicit without forcing direct agent-run
  tracking into the first implementation.

## Do Not Copy Blindly

- Do not import Symphony's daemon or Linear-specific scheduler model. Atelier's
  first surface is a local CLI with deterministic exports.
- Do not make `WORKFLOW.md` the final filename by assumption. Atelier should
  decide the repository-owned config path and schema deliberately.
- Do not add direct coding-agent run supervision yet. A later design can add
  run attempts, retry state, and live session metrics once the durable workflow
  and evidence model exists.
- Do not couple correctness to a dashboard. JSON projection and CLI validation
  must remain sufficient.

## Follow-Up Beads

- `atelier-saii.1`: decide the config path, schema, strict parsing rules,
  environment indirection, hook semantics, and dynamic reload behavior.
- `atelier-kitl`: implement configurable workflows and gates from that
  repository-owned contract.
- `atelier-9naj`: use the same strict rendering and error-surface expectations
  for action-aware guidance.
- `atelier-9h7g`: project workflow/config health and gate results without
  requiring direct agent-run rows.
