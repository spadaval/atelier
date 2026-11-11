## Work Tracking

Work is tracked with Atelier, in `.atelier/`: missions in
`missions/<id>/mission.md`, items in `items/<id>/item.md` with their
reviews beside them, and the rules in `config.toml`.

- Run `atelier status` first; Claude Code and Codex run it at session start
  through `.claude/settings.json` and `.codex/hooks.json`. It says what you
  hold and the next command, and `atelier <command> --help` says what each
  does. Agent Factory, loaded as `/agent-factory`, says how to decide.
- An item, in order: `atelier start <id>`, implement, tick every Done when
  line and write the `## Report`, `atelier submit`. A reviewer, in a thread
  that did not write the change, runs `atelier review <id>` and `atelier
  verdict`. Once approved, the implementer runs `atelier merge`, which
  marks the item done on `@BASE@`.
- Planning happens on `@BASE@` with `atelier new`, `set` and `ready`;
  `ready`, `drop`, `close` and `lead` make their own commit. A mission is
  read with `atelier show <mission>`.
- This repository's config names no forge, so nothing pushes for you.
  After `atelier merge`, or after planning on `@BASE@`, push it with
  `git push origin @BASE@`. The pre-push hook checks every commit the push
  brings to `@BASE@`: code arrives only through an Atelier merge. A rule
  can be passed on purpose with `ATELIER_SKIP="<reason>"` on the command;
  use it only when you mean to.
- `atelier setup` installs the Git hooks in a new clone.
