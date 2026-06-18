# Repository Shape

This document describes the intended shape of an agent-ready repository.

A fresh agent must be able to answer these questions without private chat
history:

- **What is this repository for?** Product intent names users, purpose, and
  observable target behaviors.
- **What words mean what?** Domain context defines core nouns and records
  resolved terminology/model choices.
- **Where do I start reading?** A docs map routes agents to product,
  architecture, quality, validation, operations, and agent-process sources.
- **Which choices are durable?** ADRs record choices that are costly,
  surprising, or repeatedly relevant.
- **How do I prove work?** A validation router maps checks and proof methods to
  the behavior or quality they own.
- **How is work tracked?** The repository instructions name the tracker. The
  tracker holds scope, Outcome, Evidence, dependencies, status, and handoff.
- **What repo-specific sources exist?** Repository instructions and docs map the
  generic operating model to concrete files, checks, and product-specific
  skills.

## File Structure

The expected shape is:

```text
.
├── AGENTS.md
├── CONTEXT.md
├── SPEC.md
├── docs/
│   ├── index.md
│   ├── adr/
│   └── architecture/
│       ├── index.md
│       └── quality/
│           ├── index.md
│           ├── architecture-quality.md
│           ├── standards.md
│           └── validation.md
├── .agents/
│   └── skills/
│       └── <repo-local product-specific skills>
└── <tracker state named by repository instructions>
```

Exact names differ only when repository instructions name the equivalent source
clearly. Missing equivalents are gaps, not harmless omissions.

## File Expectations

### `AGENTS.md`

- **Purpose**: Entry point for agents. Acts as a table of contents, not an
  encyclopedia.
- **Quality**: ≤150 lines. Names the tracker and durable repo sources. Does not
  duplicate subskill procedure or tracker command manuals.
- **Anti-pattern**: A 500-line monolith that tries to encode all repository
  knowledge; stale rules that no one maintains.

### `CONTEXT.md`

- **Purpose**: Domain language. Defines the core nouns agents must use
  consistently and records resolved terminology/model choices.
- **Quality**: Concrete domain terms with definitions, not placeholders.
  Records real choices that prevent repeated confusion.
- **Anti-pattern**: Generic terms that could apply to any repository; empty
  TODO lists that never get filled.

### `SPEC.md`

- **Purpose**: Product intent. States what the repository is for, who the users
  are, and what observable behaviors the product must support.
- **Quality**: Named users, clear purpose statement, concrete target behaviors.
- **Anti-pattern**: Vague aspirational language; no named users; behaviors that
  are not observable.

### `docs/index.md`

- **Purpose**: Documentation map. Routes agents to durable sources.
- **Quality**: Lists all primary docs with one-line descriptions. Updated when
  new durable sources are added.
- **Anti-pattern**: A grab-bag of links with no structure; missing references to
  ADRs or quality docs.

### `docs/adr/`

- **Purpose**: Durable choices. Records choices that are costly, surprising, or
  repeatedly relevant.
- **Quality**: Each ADR explains the trade-off, not just the choice.
  Superseded ADRs are clearly marked, not deleted. Archive hygiene is
  maintained: current docs do not rely on historical text for target-state
  instructions.
- **Anti-pattern**: Choices without rationale; deleted old ADRs that remove
  important historical context; stale ADRs that contradict current code.

### `docs/architecture/index.md`

- **Purpose**: Architecture index. Maps domains, ownership, and dependency
  direction.
- **Quality**: Names owners and boundaries. Cross-links to ADRs and quality
  docs.
- **Anti-pattern**: A single file that tries to document everything; no
  ownership assignments.

### `docs/architecture/quality/`

- **Purpose**: Quality, standards, and validation vocabulary.
- **Contents**:
  - `index.md`: quality index.
  - `architecture-quality.md`: vocabulary for evaluating architecture
    (abstraction, coupling, cohesion, etc.).
  - `standards.md`: code standards and conventions.
  - `validation.md`: validation router mapping checks to behaviors, proof
    methods, and result states.
- **Quality**: Uses concrete repository nouns, not generic `module` language.
  Validation router names runnable commands.
- **Anti-pattern**: Generic advice copy-pasted from the internet; validation
  commands that do not actually exist.

### `.agents/skills/`

- **Purpose**: Product-specific repository-local skills. Generic reusable
  skills such as `agent-factory` should live at the root/global skill level,
  not be copied into each repository.
- **Quality**: Each skill is loadable and has a clear `SKILL.md`. Skills are
  discoverable from `AGENTS.md`.
- **Anti-pattern**: Skills that duplicate subskill references; skills that are
  not discoverable from repository instructions; copying global skills into the
  repository when a root/global installation is available.

### Tracker State

- **Purpose**: Durable tracker state named by repository instructions.
- **Quality**:
  - Tracker is initialized, inspectable, and syncable or export-checkable using
    the tracker's own help and health surfaces.
  - Repository-specific item types, labels, or templates exist when the tracker
    supports them and the workflow needs them.
- **Anti-pattern**: Tracker not initialized; no sync/check command; no backup
  or export path; instructions that contradict the repository tracker.

## Basic Hygiene

- **`.gitignore`**: Excludes build artifacts, editor files, local secrets, and
  dependency lockfiles that should not be committed.
- **Secrets**: No committed secrets or credentials. Local secrets live in
  ignored files or a secrets manager.
- **CODEOWNERS**: If the repository has multiple owners, `.github/CODEOWNERS` or
  equivalent documents code ownership.
- **No competing target states**: Docs and code agree on the target design.
  Historical ADRs may preserve old rationale, but current docs should not rely
  on them for target-state instructions.
