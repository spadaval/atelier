---
schema: atelier.item
schema_version: 3
id: atelier-wlyn
title: Install and update the extension from its release bundle
status: in_review
mission: atelier-m8ye
labels: []
priority: medium
blocked_by:
- atelier-n4g7
- atelier-451s
implementers:
- thread:delegated-task:command%3Amcp%3A932557e7-853d-4352-845a-e9951a7775d5%3Adelegate-task%3Af23868df-cea0-4817-82a5-b23acb7f3a9d
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

- [x] The installer in the bundle installs from the bundle's own files into a home directory, keeps an existing `config.json`, and records the installed version. Its Windows and macOS paths stay as they are; this item proves neither on a real machine, and its tests run on Linux with a stand-in for T3 Code's app, which the installer takes by the option atelier-451s documents. Installing from a checkout still works for development.
- [x] `t3-custom update` looks up this repository's releases, takes the newest whose tag is `t3-extension-v<version>` and is newer than the installed one, ignores every other release, downloads its bundle, verifies the checksum, installs it, and reloads. A bundle whose checksum does not match is refused and the installed version stays as it was. With `--check` it reports and changes nothing.
- [x] The injector checks for a newer release when it starts and once a day, and the Customizations section shows the installed version and whether a newer one exists. It applies an update by itself only when a setting in `config.json`, off by default, says so; otherwise the user runs the command or presses a button there.
- [x] A failed lookup or download is reported by name in `t3-custom status` and the Customizations section, and nothing else stops working.
- [x] Tests cover: choosing the release from a fixture list holding `atelier` releases, older and newer extension releases and a draft; the checksum refusal; an install from a bundle into an empty home; and the setting. They are files the one test command of atelier-7tg6 picks up, so the CI job runs them with no change to the workflow, and the Report links a run that shows them.
- [x] `t3-extension/README.md` describes installing on a new device from a release, and updating.

## Scope

In: `t3-extension/`.

Out: publishing releases (atelier-451s); restarting T3 Code or its injector after an update that changes the injector itself beyond what the extension already does, which the README states as a known limit.

## Context

`t3-extension/install.py` is the installer and `t3-extension/t3-custom.mjs` the loader and injector, which runs under T3 Code's own executable as Node, so it has `fetch` but no package manager. An update that replaces `t3-custom.mjs` needs the injector restarted; `reload` only reinjects the scripts.

## Report

A device can now install the extension from a release zip with no clone, and `t3-custom update` keeps it current from newer `t3-extension-v*` releases. The injector checks at start and daily, and installs by itself only when `settings.autoUpdate` is on (off by default). Nothing is published yet: the first release is atelier-451s's tag.

What changed, all in `t3-extension/`:

- `t3-custom.mjs` has `update` and `update --check`.
  - Finding the release: it reads `config.releases`, defaulting to `https://api.github.com/repos/spadaval/atelier/releases?per_page=100`. It keeps releases that are neither drafts nor prereleases and whose tag is exactly `t3-extension-v<x.y.z>`, then takes the highest version by numeric comparison.
  - Installing: when that release is newer than `VERSION`, it downloads the `.zip` and `.sha256` assets and refuses the zip unless the SHA-256 matches. It unpacks the zip with `python -m zipfile` into a temporary folder and runs the bundle's own `install.py --home <home>`, then asks a running injector to `reload`.
  - Versions: `development` and `0.0.0-dev.<sha>` count as 0.0.0, so a checkout install is always offered the newest release.
  - Errors: each one names its step and URL, for example `Looking up releases: GET <url>: HTTP 500 Internal Server Error` or `Checking t3-extension-0.2.0.zip: its SHA-256 is …, but t3-extension-0.2.0.zip.sha256 says …; nothing was installed`.
  - `update.json`: `update` writes the outcome there, and `--check` writes nothing.
- The injector checks when it starts and every 24 hours, one check at a time. It installs only with `settings.autoUpdate`, then reloads.
  - Status: both status paths (the control socket and the page's `status` request) return `update` (from `update.json`) and `restartNeeded`. `restartNeeded` is true when `t3-custom.mjs` on disk differs from the copy the injector started with.
  - Failures are kept out of `lastError`, because `launch` treats that field as fatal. An offline lookup therefore does not stop T3 Custom from launching.
  - A new `update` request lets the page apply an update.
- `custom.js` adds a Release block to the Customizations section. It shows the installed release, the newest release, when the injector last checked, any error by name, the restart note, an **Update to <version>** button when a newer release exists, and the `autoUpdate` toggle. `checkSettings` accepts `autoUpdate` as a boolean. `config.example.json` has `"autoUpdate": false`.
- `install.py` records `sys.executable` as `python` in `config.json`, and `update` runs the next installer with that Python. Its Windows and macOS branches are unchanged. It already installed from its own folder, kept `config.json` and copied `VERSION` (atelier-451s), so line 1 needed only tests.
- A separate commit (`d3b82ca13`) changes how `install.py` keeps `custom.js.previous`. Before, it saved the old `custom.js` as an edit whenever it differed from the new one. Every routine update would then have left a stale `custom.js.previous` and printed "Replaced a modified". It now records the installed file's SHA-256 in `custom.js.sha256` and keeps the old file only when it differs from that record.
- `README.md` has new parts:
  - Under Install: how to install on a new device from a release, and how to install from a checkout for development.
  - A new `## Updates` section: how a release is chosen, the checksum, `update.json`, the setting and its default, and the known limit. An update that changes `t3-custom.mjs` needs `t3-custom stop` and a relaunch, and nothing restarts T3 Code or the injector by itself.
  - Small additions to Settings, Edit and control, and Development.

Tests:

- `test-update.mjs` is new, and `run-tests.mjs` picks it up with no workflow change. It needs `python3`, no browser and no Desk. It builds 0.1.0 and 0.2.0 bundles from the working tree and serves them from a local server, with a releases list holding these entries:
  - `atelier` `v9.0.0` and `v0.7.0`
  - a draft `t3-extension-v0.9.0`
  - a prerelease `0.8.0`
  - a `1.0.0-rc.1` tag, which is not x.y.z
  - out of order, `0.1.0`, `0.2.0` and `0.0.9`

  It checks:
  - An install from the 0.1.0 bundle into an empty home with a stand-in `T3 Code.app` (`--app`) records `VERSION`, `python` and `appPath`, with `autoUpdate` false.
  - `update --check` picks 0.2.0 and writes and downloads nothing.
  - A bad checksum is refused by name, and `VERSION` and `custom.js` stay as they were.
  - A failed lookup is named.
  - `update` installs 0.2.0, keeps the edited `config.json` keys, leaves no `custom.js.previous`, and does nothing when run again.
  - An edited `custom.js` is kept as `custom.js.previous`.
  - The injector reports a failed lookup in `status.update.error` while `lastError` stays null.
  - With `autoUpdate` off, the injector reports 0.2.0 and installs nothing. With it on, the injector installs 0.2.0 at start and status reports `release` 0.2.0.

  Two mutations fail it as they should: dropping the draft filter fails the `--check` assertion, and skipping the checksum comparison fails the refusal assertion.
- `test-extension.mjs` now points `releases` at its fixture server, which answers 503, so the test no longer reaches GitHub. It checks that the Customizations section shows `Release check failed: Looking up releases: GET <base>/releases: HTTP 503 Service Unavailable` with no Update button. It then writes an `update.json` reporting 0.5.0 and checks that `Update to 0.5.0` appears and that the toggle is off.
- Locally, `node run-tests.mjs` passed both files: test-extension in 42s, run against the `atelier-desk` binary and `desk/dist` from the `atelier-spike-t3` checkout, and test-update in 2s. `scripts/t3-extension-release.sh bundle` then `verify` at `d3b82ca13` installed into an empty home, and `t3-custom check` printed `release` `0.0.0-dev.test`.
- CI on pull request #186 at `10dbd7cfa` (code as of `d3b82ca13`): https://github.com/spadaval/atelier/actions/runs/43952390974. Its `extension` job runs `node run-tests.mjs`, which runs `test-extension.mjs` and `test-update.mjs`. The release workflow's bundle check on the same commit: https://github.com/spadaval/atelier/actions/runs/43952390934. Both were still running when this was written; the reviewer reads their outcome there.

Not proven:

- Nothing has run on Windows or macOS. That includes running Python from the injector there (`py` is the fallback when `config.json` has no `python`) and `python -m zipfile -e` on those systems.
- Nothing has run against a real published release, since none exists yet.
- The GitHub API's unauthenticated limit is 60 requests an hour per IP address. A daily check is far under it.
- The list is read with `per_page=100` and no paging. That holds while the newest extension release is among the 100 newest releases.

