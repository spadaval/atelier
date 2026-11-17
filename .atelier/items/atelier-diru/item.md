---
schema: atelier.item
schema_version: 3
id: atelier-diru
title: Start a T3 Code test instance on this host, with a thread, from one command
status: draft
mission: atelier-osel
labels: []
priority: medium
blocked_by:
- atelier-3dip
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-17T21:02:28.628244326Z
---

## Goal

Let an agent on this host see the extension in the real T3 Code, without
Legend being on and the human logged in. Five items of this mission end with
"seen in T3 Code", and each would otherwise wait on his Windows machine.

## Done when

- [ ] One command in `t3-extension/` starts a test instance and prints its debugging port, and one stops everything it started, by process ID and not by a pattern. Starting twice does not start two.
- [ ] It runs a named T3 Code Nightly for Linux, fetched once from `pingdotgg/t3code`'s releases into a folder outside the repository, on a virtual display it starts itself. It says in one line which program it needs when neither `Xvnc` nor `Xvfb` is installed.
- [ ] The instance cannot touch the T3 Code server this host runs: its home, T3 home, configuration, data and cache folders, its Chromium profile and its backend port are its own, and it does not update itself. The start command sets `HOME`, `T3CODE_HOME`, `XDG_CONFIG_HOME`, `XDG_DATA_HOME` and `XDG_CACHE_HOME` to folders of the instance, so that T3 Code and the OpenCode it runs read none of the human's. A check lists the real `~/.t3`, `~/.config/opencode` and `~/.local/share/opencode` before and after a start, a thread and a stop, and finds nothing changed; and it asks the instance's OpenCode for its credentials and finds none.
- [ ] The instance has a project and a thread with at least one assistant message, made inside T3 Code with a free OpenCode model. OpenCode runs from the instance's own home, with no folder of the human's linked or copied in, so it holds no credential and cannot reach a paid model. The command refuses, naming the model, when the model it is given is not a free one.
- [ ] The injector is attached with the built extension, reading a fixture Desk the command also starts, whose records name the instance's thread as the implementer of one item and the lead of one mission, and hold one running mission and one done one.
- [ ] The instance is left as `test-contract.mjs` needs it: T3's boring sidebar on, the thread above in a project, and a new-thread draft of that project.
- [ ] `node test-contract.mjs --port <port>` passes with every hook against the instance, with the `--thread` and `--draft` the start command prints.
- [ ] A command saves a screenshot of the instance in light and in dark.
- [ ] `t3-extension/README.md` says how to start, stop and look, that Chromium's sandbox is off in the test instance and why, and that Windows is still checked once before a release.

## Scope

In: `t3-extension/`.

Out: launching T3 Code on Linux from the loader; running the test instance in CI; signing in any provider that spends the human's allowance; fixing Chromium's sandbox on this host, which needs `sudo`.

## Context

The spike is in `~/t3-linux-spike` on this host, with the commands that worked in `run3.log`'s start line and in the memory note the planner left. What it found: `--ozone-platform=headless` crashes T3 Code's main process and leaves children holding the debugging port, so a real X display is needed, and `Xvnc` is installed; Chromium's SUID sandbox aborts here, so the instance runs with `--no-sandbox`; `HOME`, `T3CODE_HOME`, `XDG_CONFIG_HOME`, `XDG_DATA_HOME` and `XDG_CACHE_HOME` pointed at its own folders, with `T3CODE_PORT`, `T3CODE_DISABLE_AUTO_UPDATE` and `--user-data-dir`, isolate it, and leaving out the `XDG` ones would let its OpenCode read the human's `~/.local/share/opencode`; onboarding is passed with "Start without an agent"; and a project is added through T3's in-app palette (Add project, Local folder, the path, Enter), with no native dialog. `opencode run -m opencode/beam-free` answered from a fresh home with nothing linked in, so the free model needs no sign-in. That is OpenCode alone, not a T3 thread. Not tried: enabling OpenCode inside T3 Code, sending the first message, turning the boring sidebar on, and keeping a draft. An implementer who cannot make a thread that way asks the thread that started it.
