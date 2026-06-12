---
created_at: "2026-06-12T05:10:04.339881237+00:00"
id: "atelier-wws5"
issue_type: "task"
labels:
- "agent-factory"
- "process"
- "rework"
- "validation"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-hah9"
  - kind: "issue"
    id: "atelier-n1ys"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Update Agent Factory skill for evidence-backed mission reliability"
updated_at: "2026-06-12T05:10:04.339881237+00:00"
---

## Description

Update the Agent Factory skill itself so coordinated agents cannot complete
missions through vague notes, stale tests, or unrelated green checks. The skill
must teach orchestrators, implementers, validators, reviewers, and docs agents
how to use Outcome/Evidence sections, attach proof, and run adversarial closeout
audits.

## Outcome

- Agent Factory planning guidance requires executable work to include
  Description, Outcome, Evidence, and optional Notes sections once the section
  parser lands.
- Agent Factory guidance explains how to write good mission, epic, executable
  issue, validation item, Outcome, Evidence, and Notes text.
- The orchestrate procedure requires a mission contract audit before closeout:
  each mission Outcome line maps to linked work and attached evidence.
- The implement procedure requires agents to update evidence or record exact
  validation output before closing assigned work.
- The validate subskill requires line-by-line verification of Outcome claims,
  command transcript capture, ignored-test review, docs/help consistency checks,
  and evidence attachment.
- The review subskill treats missing Evidence, vague Outcome, stale tests,
  ignored tests, and public-help/docs mismatch as findings.
- The docs subskill updates Agent Factory bindings and repository docs when
  command surfaces change.
- Skill guidance stops recommending raw internal workflow-validator commands as
  normal closeout proof.
- The skill distinguishes new repair issues from reopening misleading closed
  work.

## Evidence

- Patch the relevant Agent Factory skill procedure and standards files under
  `/root/.agents/skills/agent-factory/`, including reusable work-item authoring
  guidance.

- Add or update repository binding guidance in `AGENTFACTORY.md` only where it
  reflects the shared skill behavior.

- Run a docs/process review against the updated skill text and capture evidence.

- Demonstrate the new authoring and closeout guidance on mission `atelier-tcmr`
  before that mission closes.

## Notes

This task intentionally changes global skill behavior, not only this

repository's local instructions. Keep the guidance general enough for Agent

Factory, but use Atelier-specific command examples where the tracker binding

requires them.
