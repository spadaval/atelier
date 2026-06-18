# Install

Use this subskill to install agent-factory in a repository. Installation creates
the minimum durable sources that agents need
to plan, execute, review, validate, and hand off work.

Do not make the operating model optional. The install subskill connects it to
concrete repository paths and tracker surfaces; it does not negotiate the
operating model.

## Target Repository Shape

An agent-factory-ready repository has durable places for intent, scope,
choices, proof, and handoff. See [repo-shape.md](../standards/repo-shape.md) for the
complete intended shape, file expectations, and quality heuristics.

A fresh agent must be able to answer these questions without private chat
history:

- **What is this repository for?**
- **What words mean what?**
- **Where do I start reading?**
- **Which choices are durable?**
- **How do I prove work?**
- **How is work tracked?**
- **What repo-specific sources exist?**

Exact names differ only when repository instructions name the equivalent source
clearly. Missing equivalents are installation gaps, not harmless omissions.

## Start Gate

Inspect the repository before writing. Follow
[repository workflow](../standards/repo-workflow.md) for git worktree checks,
and [tracker.md](../standards/tracker.md) for tracker workflow. Then
run the repository discovery commands below:

```bash
find . -maxdepth 3 -type f \( -name AGENTS.md -o -name CONTEXT.md -o -name SPEC.md \)
find docs -maxdepth 3 -type f 2>/dev/null
find . -maxdepth 3 -type d \( -name adr -o -name .atelier-state \)
```

If the worktree is dirty, preserve unrelated changes. If existing docs conflict
with the expected scaffolding, adapt to the existing structure
rather than duplicating sources.

## Required Outputs

Create or verify these repository sources:

- Agent instructions file: usually `AGENTS.md`.
- Docs map: usually `docs/index.md`.
- Domain context: usually `CONTEXT.md`.
- Product intent: usually `SPEC.md` or a clearly named equivalent.
- ADR directory: usually `docs/adr/`.
- Architecture index: usually `docs/architecture/index.md`.
- Quality index: usually `docs/architecture/quality/index.md`.
- Architecture quality vocabulary: usually
  `docs/architecture/quality/architecture-quality.md`.
- Code standards: usually `docs/architecture/quality/standards.md`.
- Validation router: usually `docs/architecture/quality/validation.md`.
- Durable tracker available. The repository chooses a tracker in
  `AGENTS.md`; Atelier is a supported first-class tracker.

Use existing equivalent files when they already exist. Otherwise create concise
starter files with useful headings and explicit TODO markers.

## Agent Instructions

`AGENTS.md` should be a concise repository entry point. It names the tracker and
durable repository sources without duplicating tracker help or subskill
procedure.

Delete entries only after recording a follow-up item to add the missing source.
After tracker setup exists, that follow-up should be a tracker item. During
initial bootstrap, it may be a concrete gap in the install handoff. A missing
repository instruction source is a defect.

## Starter Files

When creating starter files, keep them small and useful.

`CONTEXT.md` defines domain language:

```md
# Context

## Domain Terms

- TODO: define the core nouns agents must use consistently.

## Ambiguities

- TODO: record terminology/model choices that prevent repeated confusion.
```

The product intent file defines purpose and user-visible target behavior:

```md
# Product Intent

## Purpose

TODO: state what this repository is for.

## Users

TODO: name the users or operators.

## Target Behaviors

- TODO: list observable behaviors the product must support.
```

The docs map routes agents to durable sources:

```md
# Documentation Map

- `AGENTS.md`: agent instructions and repository reference map.
- `CONTEXT.md`: domain language.
- `SPEC.md`: product intent.
- `docs/adr/`: durable choices.
- `docs/architecture/`: architecture and ownership.
- `docs/architecture/quality/`: quality, standards, and validation.
```

The validation router defines check ownership:

```md
# Validation

## Commands

| Command            | Owns                                                    |
| ------------------ | ------------------------------------------------------- |
| `git diff --check` | whitespace and patch hygiene                            |
| `atelier lint`     | tracker record structure                               |
| TODO               | project tests, type checks, build, lint, or docs checks |

## Result States

- `pass`
- `fail`
- `blocked`
- `deferred`
- `not-applicable`
```

## Tracker Setup

Verify the repository tracker is available. For an Atelier setup:

```bash
atelier issue list --ready
atelier lint
atelier doctor
```

If the repository tracker is not initialized, run the repository-appropriate setup
command or stop with the exact missing command/tool. Do not invent a parallel
tracker.

After tracker setup, ensure tracker changes are committed with related work.

## AGENTS.md Update

Ensure agent instructions say:

- use the repository tracker for task tracking;
- use the `agent-factory` skill for coordinated agent work;
- orchestrators assign one subskill per subagent;
- do not use interactive tracker commands (see the agent-factory tracker reference for conventions).

Keep this short.

## Verification

Before handoff:

```bash
test -f AGENTS.md
test -f CONTEXT.md
test -d docs/adr
<tracker lint>
git diff --check
<mapped markdown check, if available>
```

If any required source is intentionally deferred, create a follow-up item that
names the missing source, why it matters, and which subskill creates it. Once
the tracker is available, record that follow-up in the tracker.

## Readiness Verification

After creating or updating sources, spot-check against
[repo-shape.md](../standards/repo-shape.md):

- `AGENTS.md` exists and is ≤150 lines.
- Every source listed in `AGENTS.md` points to an existing file.
- `docs/index.md` routes to all primary durable sources.
- At least one validation gate runs without error.
- No committed secrets or credentials.

If a criterion fails, name the gap in the handoff report. Do not block
installation on non-critical gaps, but do create a follow-up tracker item for any
gap that would mislead a fresh agent.

## Handoff

Report:

- agent instructions created or updated;
- required sources created, reused, or deferred;
- tracker setup status and health command;
- checks run and failures;
- readiness gaps found and follow-up tracker item IDs.
