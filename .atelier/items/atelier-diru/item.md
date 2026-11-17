---
schema: atelier.item
schema_version: 3
id: atelier-diru
title: Start a T3 Code test instance on this host, with a thread, from one command
status: changes_requested
mission: atelier-osel
labels: []
priority: medium
blocked_by:
- atelier-3dip
implementers:
- thread:delegated-task:command%3Amcp%3A3eadca8a-0dea-4bef-b682-3d92ff9eca06%3Adelegate-task%3A536bbdf4-e2c6-44ea-86d8-2829b3ce9b07
ready:
  by: 'sumeet, in conversation on 2026-11-17: looks good, start up a lead'
  at: 2026-11-17T21:31:50.834606783Z
trivial: null
caused_by: null
created_at: 2026-11-17T21:02:28.628244326Z
---

## Goal

Let an agent on this host see the extension in the real T3 Code, without
Legend being on and the human logged in. Five items of this mission end with
"seen in T3 Code", and each would otherwise wait on his Windows machine.

## Done when

- [x] One command in `t3-extension/` starts a test instance and prints its debugging port, and one stops everything it started, by process ID and not by a pattern. Starting twice does not start two.
- [x] It runs a named T3 Code Nightly for Linux, fetched once from `pingdotgg/t3code`'s releases into a folder outside the repository, on a virtual display it starts itself. It says in one line which program it needs when neither `Xvnc` nor `Xvfb` is installed.
- [x] The instance cannot touch the T3 Code server this host runs: its home, T3 home, configuration, data and cache folders, its Chromium profile and its backend port are its own, and it does not update itself. The start command sets `HOME`, `T3CODE_HOME`, `XDG_CONFIG_HOME`, `XDG_DATA_HOME` and `XDG_CACHE_HOME` to folders of the instance, so that T3 Code and the OpenCode it runs read none of the human's. A check lists the real `~/.t3`, `~/.config/opencode` and `~/.local/share/opencode` before and after a start, a thread and a stop, and finds nothing changed; and it asks the instance's OpenCode for its credentials and finds none.
- [x] The instance has a project and a thread with at least one assistant message, made inside T3 Code with a free OpenCode model. OpenCode runs from the instance's own home, with no folder of the human's linked or copied in, so it holds no credential and cannot reach a paid model. The command refuses, naming the model, when the model it is given is not a free one.
- [x] The injector is attached with the built extension, reading a fixture Desk the command also starts, whose records name the instance's thread as the implementer of one item and the lead of one mission, and hold one running mission and one done one.
- [x] The instance is left as `test-contract.mjs` needs it: T3's boring sidebar on, the thread above in a project, and a new-thread draft of that project.
- [x] `node test-contract.mjs --port <port>` passes with every hook against the instance, with the `--thread` and `--draft` the start command prints.
- [x] A command saves a screenshot of the instance in light and in dark.
- [x] `t3-extension/README.md` says how to start, stop and look, that Chromium's sandbox is off in the test instance and why, and that Windows is still checked once before a release.

## Scope

In: `t3-extension/`.

Out: launching T3 Code on Linux from the loader; running the test instance in CI; signing in any provider that spends the human's allowance; fixing Chromium's sandbox on this host, which needs `sudo`.

## Context

The spike is in `~/t3-linux-spike` on this host, with the commands that worked in `run3.log`'s start line and in the memory note the planner left. What it found: `--ozone-platform=headless` crashes T3 Code's main process and leaves children holding the debugging port, so a real X display is needed, and `Xvnc` is installed; Chromium's SUID sandbox aborts here, so the instance runs with `--no-sandbox`; `HOME`, `T3CODE_HOME`, `XDG_CONFIG_HOME`, `XDG_DATA_HOME` and `XDG_CACHE_HOME` pointed at its own folders, with `T3CODE_PORT`, `T3CODE_DISABLE_AUTO_UPDATE` and `--user-data-dir`, isolate it, and leaving out the `XDG` ones would let its OpenCode read the human's `~/.local/share/opencode`; onboarding is passed with "Start without an agent"; and a project is added through T3's in-app palette (Add project, Local folder, the path, Enter), with no native dialog. `opencode run -m opencode/beam-free` answered from a fresh home with nothing linked in, so the free model needs no sign-in. That is OpenCode alone, not a T3 thread. Not tried: enabling OpenCode inside T3 Code, sending the first message, turning the boring sidebar on, and keeping a draft. An implementer who cannot make a thread that way asks the thread that started it.

## Report

`node t3-extension/t3-instance.mjs start` now gives an agent on this host a
T3 Code test instance with everything `test-contract.mjs` needs, in about a
minute. Against it, `node test-contract.mjs --port 9333 --thread <…> --draft
<…>` with the printed values passed all 35 probes.

**What it does.** It fetches T3 Code Nightly `v0.1.11-nightly.20261117.4851`
once into `/var/tmp/atelier-t3-test-<user>/apps/` (tested from scratch by
moving the unpacked copy aside), starts `Xvnc` (else `Xvfb`) on a free display, and
starts T3 with `HOME`, `T3CODE_HOME` and all five `XDG_*` folders inside the instance
folder beside it, backend port 47331, debugging port 9333
and auto-update off. It seeds T3's `settings.json` (only OpenCode enabled,
the free model for threads and titles) and `client-settings.json`
(onboarding done, boring sidebar on), then drives T3's own screens: Add
project, Local folder, the composer, Enter, and the folder's New thread for
the draft. It then starts the fixture Desk with the new thread as lead of
`atelier-fm01` and implementer of `atelier-fi01` (`/api/ties` confirmed both),
and the injector with the built `dist/custom.js`. `status`, `shot` (light and
dark PNGs under `artifacts/test-instance/`) and `stop` complete it.

**Found on the way, each handled in the script:**

- The caller's environment carries `T3CODE_TAILSCALE_SERVE=true` and the
  host server's `T3CODE_*`. My first hand launch inherited them; no Tailscale
  entry was made for the test port, but the script now passes T3 an
  allowlist (`PATH`, locale, user, and the instance's folders), which also
  keeps API keys away from OpenCode.
- Any `opencode` command without `--standalone` leaves a detached background
  service (`opencode serve --service`) behind, and T3's own `opencode serve`
  runs in its own session, so killing T3's process group misses both. `stop`
  signals the PIDs in `state.json`, their descendants, and any process whose
  `HOME` is the instance's, each checked against its start time so a reused
  PID is never signalled. Every stop above ended 12 to 15 processes and left
  none.
- T3 ignores `--user-data-dir`; its Chromium profile is
  `XDG_CONFIG_HOME/t3code-v2`, inside the instance, so the flag is not passed.
- About one start in three never exited: Chromium left the DevTools
  WebSocket's close frame unanswered and the open socket (to 9333, found with
  `process._getActiveHandles()`) kept Node alive. Node's WebSocket cannot be
  closed without that answer, so the script calls `process.exit()` when a
  command is done. That is a stopgap and the comment says what it hides; the
  same can happen to any tool using `devtools.mjs`.

**Free models.** A model counts as free when the instance's credential-less
OpenCode lists it (`opencode models --standalone`): 12 OpenCode Zen models
today. `start --model opencode/claude-fable-5-2` refused by name, listing
them, before starting anything.

**The isolation check differs from the Done when line, on purpose.** "Finds
nothing changed" cannot hold: the host's T3 server rewrites `~/.t3` (its
logs, `statev2.sqlite`, PR caches) every few seconds, 7 to 20 entries per
run. `node t3-instance.mjs isolation` instead:

1. checks, while the instance runs, that none of its processes has a file
   under `~/.t3`, `~/.config/opencode` or `~/.local/share/opencode` open (fds,
   working directory, mappings) and that each runs with the instance's `HOME`,
   `T3CODE_HOME` and `XDG_*`. Chromium's `--type=` children rewrite their
   environ and glycin's image loader runs under `bwrap` with `HOME=/tmp-home`,
   so only their open files are checked;
2. after the stop, lists everything in those folders modified since the
   check began (`find -newer`; `~/.t3` holds 3.4 million entries, too many to
   hold two full listings) and anything removed from their top three levels,
   and fails if any such path names, or any such file holds, the instance's
   fresh thread or environment UUID. It does not print those IDs until it is
   done, since the agent running it is itself logged by the host's T3. It
   leaves out this checkout, which sits under `~/.t3/worktrees`;
3. asks the instance's OpenCode `opencode auth list --standalone`: "No
   authenticated integrations".

It passes. To show it can fail, a throwaway copy that guarded the instance's
own `.t3` reported 19 problems.

**Checks run:** `node run-tests.mjs` (all 3 pass, after the
`fixture-desk.mjs` change); `start` twice (the second printed the running
instance and started nothing); the contract (35 ok); `shot`; `isolation`;
`start` with no `Xvnc` or `Xvfb` on `PATH` (one line naming both, before
anything starts); `stop` with nothing running.

**Not done:** Windows is still checked once before a release (README says
so). The spike's `~/t3-linux-spike` (1.2 GB) is the planner's and is left in
place; it can be deleted.

### Round 1 fixes

Both findings held, and both are fixed.

1. **OpenCode read the human's folders.** The cause was where the instance
   lived, not its environment: OpenCode looks for `.opencode`,
   `.claude/skills`, `.agents/skills` and `opencode.json` in every folder
   above the one it runs in, and both the instance (under `~/.cache`) and the
   model probe (run in the caller's folder, here this checkout under
   `~/.t3/worktrees`) sat below the real home. The instance now lives in
   `/var/tmp/atelier-t3-test-<user>`, `start` refuses a `T3_TEST_ROOT` inside
   the home folder and says why, and the probes run in the instance's home.
   The isolation check now also fails if OpenCode's log names any folder in
   the human's home it watched or ran in. Against the old layout (the home
   check removed in a throwaway copy) it named `~/.claude/skills`,
   `~/.agents/skills` and `~/.opencode/skill(s)`. At the new location it
   passes, and OpenCode's log names only `/`, `/etc`, `/var/tmp` and the
   instance. The reviewer's repro, an `opencode.json` with a costed fake
   provider in the caller's folder, now gets `Refusing model
   reviewfake/paid-test-model`, while `opencode models` run from that folder
   still lists the fake model.
2. **Simultaneous starts raced.** `start` now takes `start.lock` in the root,
   linked into place whole and naming its owner by PID and start time, before
   it reads state or clears the folder, and holds it until the state is
   complete. A lock whose owner has exited is removed. Two starts launched
   one second apart: the second printed "Another start (PID …) is making the
   instance; waiting for it", then "Already running" with the first one's
   thread and draft, and both exited 0. The contract then passed all 35
   probes against it.

`test-extension.mjs` failing for the reviewer on port 19222 was another
browser holding that port, not this change; it passed here before round 1.

### Round 2 fix

My last edit before round 2 put an apostrophe in a single-quoted string and I
did not rerun `node --check` after it, so the submitted script did not parse.
Fixed in cc9d69645. At that head: `node --check` passes; `isolation` passes (start, a
thread, stop, no credentials, OpenCode used no folder in the home); two starts
a second apart give one instance, the second waiting and reusing it; the
contract passes all 35 probes with the printed values; `shot` saves both
screenshots; `stop` ended 16 processes; and `start --model
opencode/claude-fable-5-2` is refused by name.

### Responses

1-d4r6.1: fixed in 538d83e04: the instance moved to /var/tmp outside the home folder, probes run in its home, and isolation reads OpenCode's log.
1-d4r6.2: fixed in 538d83e04: start takes start.lock before reading state or clearing the folder; a second start waits and reuses.
2-4ito.3: fixed in cc9d69645: the string is double-quoted; node --check passes and every command was rerun at that head.
