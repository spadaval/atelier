---
schema: atelier.item
schema_version: 3
id: atelier-wlyn
title: Install and update the extension from its release bundle
status: ready
mission: atelier-m8ye
labels: []
priority: medium
blocked_by:
- atelier-n4g7
- atelier-451s
implementers: []
ready:
  by: 'sumeet, in conversation on 2026-11-16: do this, and the rest of the mission'
  at: 2026-11-16T19:26:24.749789035Z
trivial: null
caused_by: null
created_at: 2026-11-16T19:25:27.007787911Z
---

## Goal

Let a device run the extension from a release bundle, and find a newer one,
without a clone of this repository. Today the installer copies files from the
checkout it sits in, and updating means `git pull` and running it again.

## Done when

- [ ] The installer in the bundle installs from the bundle's own files into a home directory, keeps an existing `config.json`, and records the installed version. Its Windows and macOS paths stay as they are; this item proves neither on a real machine, and its tests run on Linux with a stand-in for T3 Code's app, which the installer takes by the option atelier-451s documents. Installing from a checkout still works for development.
- [ ] `t3-custom update` looks up this repository's releases, takes the newest whose tag is `t3-extension-v<version>` and is newer than the installed one, ignores every other release, downloads its bundle, verifies the checksum, installs it, and reloads. A bundle whose checksum does not match is refused and the installed version stays as it was. With `--check` it reports and changes nothing.
- [ ] The injector checks for a newer release when it starts and once a day, and the Customizations section shows the installed version and whether a newer one exists. It applies an update by itself only when a setting in `config.json`, off by default, says so; otherwise the user runs the command or presses a button there.
- [ ] A failed lookup or download is reported by name in `t3-custom status` and the Customizations section, and nothing else stops working.
- [ ] Tests cover: choosing the release from a fixture list holding `atelier` releases, older and newer extension releases and a draft; the checksum refusal; an install from a bundle into an empty home; and the setting. The CI job from atelier-7tg6 runs them.
- [ ] `t3-extension/README.md` describes installing on a new device from a release, and updating.

## Scope

In: `t3-extension/`.

Out: publishing releases (atelier-451s); restarting T3 Code or its injector after an update that changes the injector itself beyond what the extension already does, which the README states as a known limit.

## Context

`t3-extension/install.py` is the installer and `t3-extension/t3-custom.mjs` the loader and injector, which runs under T3 Code's own executable as Node, so it has `fetch` but no package manager. An update that replaces `t3-custom.mjs` needs the injector restarted; `reload` only reinjects the scripts.
