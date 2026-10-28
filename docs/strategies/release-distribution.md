---
status: active
last_changed: "2026-10-27: drafted; Sumeet directed filing and executing the mission"
---

# Why

Atelier can only be installed by building it from a source checkout with
`cargo install --path crates/atelier-cli`. Nobody outside the repository can
try it without a Rust toolchain and a clone, and nothing checks a commit
before it reaches `master` except an agent remembering to run the tests. The
stale-binary message that tells a user to upgrade assumes they have a source
checkout.

# Outcome

A tagged commit becomes a GitHub Release with prebuilt `atelier` binaries,
checksums and a shell installer, produced by CI from a commit that passed the
repository's checks. Someone without Rust installs or upgrades Atelier with
one command, and every message and doc that tells a user how to (re)install
names that command. Pull requests and `master` pushes run the repository's
validation commands on GitHub.

A partial outcome worth shipping: CI plus a release pipeline proven on a pull
request, before the first tag is cut.

# Success Criteria

- S1: Pull requests and pushes to `master` run `cargo fmt -- --check`,
  `cargo nextest run`, `cargo nextest run --features test-hooks`,
  `atelier check` and `git diff --check` on Linux and macOS, and fail when
  any of them fails. Shown by a passing GitHub Actions run on the work
  branch, plus a run that fails on a deliberately broken commit.
- S2: Pushing a `v<version>` tag publishes a GitHub Release with archives
  for x86_64 and aarch64 Linux and aarch64 and x86_64 macOS, their
  checksums, and a shell installer. A pull request touching the release
  setup builds the same artifacts without publishing. Shown by a PR run that
  uploads artifacts, and an archive from it whose `atelier --version`
  reports the tagged version and commit.
- S3: On a machine with no Rust toolchain, the installer one-liner installs
  `atelier` onto `PATH`, and rerunning it upgrades an older install. Shown
  by running the published installer in a clean `HOME`.
- S4: The stale-binary message, README and development-setup docs name the
  release installer as the way to install or upgrade, keeping
  `cargo install --path crates/atelier-cli --locked` for source checkouts.
  Package metadata names the real repository, and the changelog describes
  the rebuilt Markdown-record model rather than the old SQLite one. Shown by
  tests over the message and by inspection of the docs.

# Shape

- Use `dist` (axodotdev cargo-dist) to generate the release workflow and
  installer, rather than a hand-written workflow. It is maintained, and it
  produces checksums, the installer and PR dry runs from one config in
  `Cargo.toml`. Prefer its generated workflow unedited, so `dist init` can
  regenerate it.
- Upgrades go through the installer. Atelier itself gets no self-update
  command and no network code; it stays a local tool.
- The release workflow builds what CI tests. Release targets are only those
  CI also tests on (Linux and macOS).
- Simplicity wins over coverage: no Windows, crates.io, Homebrew or
  `cargo binstall` until someone needs them.

# Scope

In: CI workflow, `dist` config and generated release workflow, package
metadata, the reinstall message, README, development-setup docs, changelog,
and cutting the first release.
Out: Windows builds, crates.io publishing, Homebrew, package-manager
packaging, code signing and notarization, a self-update command.
Environment: the public `spadaval/atelier` GitHub repository and
GitHub-hosted runners.

# Authority

The orchestrator may change the increment breakdown, the CI layout, `dist`
settings and the targets within Linux and macOS, and may push work branches
to run CI. Pushing a release tag, choosing the first release's version,
adding a platform or distribution channel, or adding repository secrets
comes back to the human.

# Risks and Unknowns

- Atelier has never been built or tested on macOS. The CI increment learns
  this first; a macOS failure may need code changes before release targets
  are fixed.
- The test suite shells out to Git and may assume this machine's Git
  configuration (identity, default branch). CI will show what it needs.
- `build.rs` derives the version suffix from `git`; a shallow CI checkout
  must still give `atelier --version` the right commit.
