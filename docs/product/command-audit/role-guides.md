# Role-Specific Guides

Atelier should keep product-oriented commands, but provide role-specific guide
pages that filter the command surface for the operator's current job.

## Proposed Surface

Add a guide command such as:

```text
atelier man worker
atelier man reviewer
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
- The role's common loop.
- The commands that are intentionally not part of normal work.
- Recovery guidance when the first command reports stale, invalid, blocked, or
  missing-proof state.

## Worker Guide

Primary job: implement assigned or ready work.

Default first command: `atelier status`.

Core loop:

```text
atelier man worker
atelier status
atelier issue list --ready
atelier issue show <id>
atelier start <id>
atelier issue note <id> "..."
atelier evidence record --target issue/<id> --kind test --result pass -- <command>
atelier issue transition <id> --options
atelier issue close <id> --reason "..."
```

Worker guide should hide or de-emphasize setup, maintenance, raw diagnostics,
bulk plan apply, branch merge, and destructive record deletion.

## Reviewer Guide

Primary job: check proof, review outputs, and validate transitions.

Default first command: `atelier mission status`.

Core loop:

```text
atelier issue show <id>
atelier issue transition <id> --options
atelier evidence show <evidence-id>
atelier evidence record --target issue/<id> --kind validation --result pass -- <command>
atelier history --issue <id>
atelier lint <id>
atelier mission status <id> --closeout --verbose
```

Reviewer guide should explain that `workflow check` is raw admin diagnostics;
normal readiness inspection uses `issue transition --options`, `lint`, and
`mission status`.

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
atelier issue create "..."
atelier mission add-work <mission-id> <issue-id>
atelier issue block <blocked-id> <blocker-id>
atelier graph tree --compact
atelier worktree for-mission <mission-id>
atelier branch for-epic <epic-id>
```

Manager guide should include `plan apply --dry-run` and
`plan apply --validate-only` as advanced bulk-planning commands, not as the
ordinary way to make every issue.

## Admin Guide

Primary job: configure, migrate, repair, and maintain Atelier state.

Default first command: `atelier doctor`.

Core loop:

```text
atelier init
atelier lint
atelier doctor
atelier doctor --fix
atelier export --check
atelier rebuild
atelier maintenance delete <kind> <id> --force
```

Admin guide should name hidden diagnostics only when they are explicitly useful:
`workflow check`, `diagnostics slow`, and `import-beads`.

## Resolved Design

`atelier man` is the role-guide command. It is visible in root help, uses static
Rust-rendered text plus a brief live state snapshot, and replaces the removed
prime command. It accepts only `worker`, `reviewer`, `manager`, and `admin`.
