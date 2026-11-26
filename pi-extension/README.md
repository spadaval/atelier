# Atelier's extension for Pi

Lets a thread run in the [Pi coding agent](https://pi.dev) under the same
rules as a Codex or Claude Code thread, so one model can be compared across
harnesses. It is an experiment: whether it stays depends on that comparison.

In a checkout with no `.atelier` directory it does nothing but offer the
background-command tools.

## What it does

- **Says who the thread is.** Codex and Claude Code run `atelier status` from
  a session-start hook. Pi reads neither hook file, so the first run of a
  session is given `atelier status` here, with the goal and unticked Done when
  lines of the item whose branch is checked out, or the outcome and success
  criteria of the mission the thread leads.
- **Checks handoffs.** A call to a delegation tool (`delegate_task`,
  `t3_thread_send`, `t3_thread_launch` and Codex's own, matched by suffix) is
  sent to `handoff-guard` in the shape of Codex's `PreToolUse` event, and a
  denial blocks the call with the guard's reason. Calls made from inside a
  `codemode` script are checked too. Like Codex's hook, a guard that fails to
  answer does not stop the call; the failure is written to the session.
- **Keeps a reviewer from editing.** While `atelier status` says the thread is
  an item's reviewer, `edit` and `write` are blocked. This is a nudge, not a
  wall: `bash` can still write files.
- **Repeats the records.** Every 25 turns, and after each compaction, the
  goal and what is left are appended at the tail of the conversation, without
  asking the model for another answer. Pi's
  own summary is kept; the records follow it and win where they disagree. The
  system prompt is never changed, so the cached prefix holds.
- **Runs long commands.** `background_start`, `background_read` and
  `background_stop` keep a dev server, an emulator or a watch build running in
  `tmux` while the agent works, which Pi's `bash` cannot do.

It exposes no subagent tool. Delegation goes through T3 Code's `delegate_task`,
whose children are ordinary threads with readable transcripts.

## Install

Pi 1.6 or later, `atelier`, `tmux`, and `handoff-guard` in `~/.local/bin`.
Name this directory in `~/.pi/agent/settings.json`:

```json
{
  "defaultTools": ["+codemode"],
  "extensions": ["/path/to/atelier/pi-extension"]
}
```

It must be a global extension: in RPC mode, which T3 Code uses, Pi skips a
project's own extensions in a folder it has not been told to trust.

## Settings

| Variable | Default | Meaning |
| --- | --- | --- |
| `ATELIER_PI_REMIND_EVERY` | `25` | Turns between reminders; `0` turns them off. |
| `ATELIER_PI_GUARD` | `~/.local/bin/handoff-guard-logged pi`, else `handoff-guard hook` | The guard command; `off` disables the check. |

## Test

```sh
node --test test.mjs
```

Needs Node 22.18 or later, `git` and `tmux`. It runs the extension against a
stand-in for Pi, so it needs neither Pi nor a model.
