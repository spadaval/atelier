---
created_at: "2026-06-16T17:17:24.891945259+00:00"
id: "atelier-amnr"
evidence_type: "docs"
captured_at: "2026-06-16T17:17:24.891823288+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-8jaf"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-8jaf"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Branch workflow guidance demoted explicit branch helpers from routine paths. Proof: rg over docs/product, command audits, AGENTFACTORY.md, /root/.agents/skills/agent-factory, and CLI fixture leaves only advanced/diagnostic branch for-epic references; target/debug/atelier --help shows branch as Inspect and repair under Advanced work and Common commands list atelier start plus atelier issue close; target/debug/atelier man worker Normal Loop lists atelier start and atelier issue close; squash default/config field documented in .atelier/workflow.yaml, SPEC.md, docs/product/cli-surface.md, docs/product/work-model.md, and docs/product/workflow-configuration.md. Validation passed: cargo fmt -- --check; cargo build -p atelier-cli; focused cli_integration help/man/branch lifecycle tests; target/debug/atelier lint atelier-8jaf; target/debug/atelier export --check; git diff --check. Broad setup_guidance:: suite was also sampled and had 4 unrelated pre-existing failures in ready/status/temp-git expectations."
updated_at: "2026-06-16T17:17:28.426849261+00:00"
---

Branch workflow guidance demoted explicit branch helpers from routine paths. Proof: rg over docs/product, command audits, AGENTFACTORY.md, /root/.agents/skills/agent-factory, and CLI fixture leaves only advanced/diagnostic branch for-epic references; target/debug/atelier --help shows branch as Inspect and repair under Advanced work and Common commands list atelier start plus atelier issue close; target/debug/atelier man worker Normal Loop lists atelier start and atelier issue close; squash default/config field documented in .atelier/workflow.yaml, SPEC.md, docs/product/cli-surface.md, docs/product/work-model.md, and docs/product/workflow-configuration.md. Validation passed: cargo fmt -- --check; cargo build -p atelier-cli; focused cli_integration help/man/branch lifecycle tests; target/debug/atelier lint atelier-8jaf; target/debug/atelier export --check; git diff --check. Broad setup_guidance:: suite was also sampled and had 4 unrelated pre-existing failures in ready/status/temp-git expectations.
