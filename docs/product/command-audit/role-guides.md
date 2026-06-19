# Role-Specific Guides

Atelier should keep product-oriented commands, but provide role-specific guide
pages that filter the command surface for the operator's current job.

## Proposed Surface

Add a guide command such as:

```text
atelier man worker
atelier man reviewer
atelier man validator
atelier man manager
atelier man admin
```

`man` is intentionally a guide layer, not a new command namespace. It should not
create commands such as `atelier worker start` or `atelier orchestrator issue
create`. Role guides answer "which existing commands matter to me right now?"

## Guide Contract

Each guide should include:

- The role's default first command.
- A short set of commands that matter most for the role.
- The role's common orientation loop.
- The commands that are intentionally not part of normal work.
- Recovery guidance when the first command reports stale, invalid, blocked, or
  missing-proof state.
- A reminder that workflow/status/transition output owns the next process step.
  Role guides should not decide whether to open review artifacts, which review
  provider to use, or which branch/merge action completes work.

## Worker Guide

Primary job: implement assigned or ready work.

Default first command: `atelier status`.

Orientation loop:

```text
atelier man worker
atelier status
atelier issue list --ready
atelier issue show <id>
atelier issue note <id> "..."
atelier evidence record --target issue/<id> --kind test -- <command>
atelier issue transition <id> --options
```

After that, the worker follows the transition command and recovery guidance
Atelier prints for the current issue. Review, branch, provider, and completion
steps are not chosen from the role guide.

Worker guide should hide or de-emphasize setup, maintenance, raw diagnostics,
bundle apply, branch merge, and destructive record deletion.

## Reviewer Guide

Primary job: check proof, review outputs, and validate transitions.

Default first command: `atelier mission status`.

Orientation loop:

```text
atelier issue show <id>
atelier issue transition <id> --options
atelier evidence show <evidence-id>
atelier evidence record --target issue/<id> --kind validation -- <command>
atelier history --issue <id>
atelier lint <id>
atelier mission status <id> --verbose
```

Reviewer guide should explain that `workflow check` is raw admin diagnostics;
normal readiness inspection uses `issue transition --options`, `lint`, and
`mission status`. Any review-artifact action should come from Atelier's current
workflow or recovery guidance, not from static reviewer policy.

## Validator Guide

Primary job: run explicit validation work and record validation proof.

Default first command: `atelier issue show <id>`.

Orientation loop:

```text
atelier issue show <id>
atelier issue transition <id> --options
atelier evidence show <evidence-id>
atelier evidence record --target issue/<id> --kind validation -- <command>
```

Validator guide should not encode review-provider or review-artifact policy.
Validation follows the issue, mission, and transition guidance Atelier prints
for the current work item.

## Manager Guide

Primary job: create and coordinate work.

`manager` is the broad CLI role class. `orchestrator` remains a specific Agent
Factory agent type within that class.

Default first command: `atelier mission status`.

Core loop:

```text
atelier mission list
atelier mission show <id>
atelier mission start <id> --switch
atelier bundle preview <file>
atelier bundle apply <file> --yes
atelier issue create "..."
atelier mission add-work <mission-id> <issue-id>
atelier issue block <blocked-id> <blocker-id>
atelier graph tree --compact
atelier worktree for-mission <mission-id>
```

Manager guidance should make `bundle preview <file>` and
`bundle apply <file> --yes` the expected path for bulk graph creation, such as a
mission with many epics, issues, blockers, mission links, or evidence links.
Manual `issue create`, `mission add-work`, and `issue block` remain appropriate
for one-off edits, not for shell loops that recreate bundle behavior. Explicit
branch commands belong in advanced repair and diagnostic guidance when Atelier
routes the operator there.

## Admin Guide

Primary job: configure, migrate, repair, and maintain Atelier state.

Default first command: `atelier doctor`.

Core loop:

```text
atelier init
atelier lint
atelier doctor
atelier doctor --fix
atelier maintenance delete <kind> <id> --force
```

Admin guide is the only role guide that should teach local tracker machinery.
It should name hidden diagnostics only when they are explicitly useful:
`workflow check`, `diagnostics slow`, `import-beads`, hidden/admin `export`,
and hidden/admin `rebuild`.

## Resolved Design

`atelier man` is the role-guide command. It is visible in root help, uses static
Rust-rendered text plus a brief live state snapshot, and replaces the removed
prime command. It accepts only `worker`, `reviewer`, `validator`, `manager`, and
`admin`.
