---
created_at: "2026-10-17T19:00:36.353441447+00:00"
id: "atelier-rc8p"
issue_type: "feature"
labels:
- "retro-2026-10-17"
- "review"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Add atelier review request to generate the reviewer brief"
updated_at: "2026-10-22T21:35:09.857437426+00:00"
---

## Goal

Orchestrators hand-wrote the reviewer prompt about 113 times in the adoption run (about 12% of its output tokens): identity, TMPDIR, target dir, scope commit, Done when, prior findings.

## Done when

`atelier review request <id>` prints a dense reviewer brief: the item's Goal and Done when, the commit range to review, prior open findings and dispositions, the commands for recording findings and approval, and the reviewer's working rules from policy. A rendering test pins it; the Agent Factory skill uses it.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168).

## Notes

- 2026-10-22 [decision] The reviewer's working rules are built-in generic rules, the base policy's merge_checks commands, and the text of a repository file named by [review] rules_file in .atelier/config.toml
  Reason: Host-specific rules (scratch directories, build limits) must not live in code, and the generic review procedure is the same everywhere. A repo-relative Markdown file keeps them versioned with the repository, and merge_checks already says what proof the repository runs.
  Rejected: Hard-coding this repository's paths and build limits in the command
  Rejected: A key in workflow.yaml (its schema is strict and base-trusted for gates, and a rules text is not a gate)
- 2026-10-22 [decision] This repository's .atelier/config.toml is not given a [review] table in this change
  Reason: config.toml rejects unknown tables, so an installed atelier older than this change could not read it (see the reinstall-after-policy-change risk); the human or orchestrator sets rules_file after reinstalling, pointing at a file holding the host rules now kept outside the repository.
- 2026-10-22 [decision] The brief omits notes and the Close Reason, and the review range is the commits since the last review or approval, with the whole branch also shown (--from overrides)
  Reason: Agent Factory orchestrate says not to hand a reviewer the author's rationale; Done when and findings are requirements and history. A re-review after fixes needs the delta, while the first review needs the whole branch, so the brief gives both and leads with the delta only when a prior review exists.
  Rejected: Always the whole branch (hides what changed since an earlier review)
- 2026-10-22 [decision] Quiet output of review request is the range <from>..<head> in full commits
  Reason: The brief itself is the human and agent output; scripts need the range, and the quiet-token table requires every leaf command to document one.
