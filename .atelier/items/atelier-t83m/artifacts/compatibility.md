# Review the correction before consumer use

No consumer application is authorized by this item. The parent coordinates
independent review of both the submitted correction and this compatible patch.
The normal review and merge gates still apply.

The installed v0.6.0 baseline refusal is retained in
`installed-v0.6.0-baseline.txt`. Its synthetic fixture uses an actual conflict,
resolves it, and attempts `git commit` with a pre-commit hook invoking the
installed binary. It imports an already-approved base record unchanged.

The compatible candidate is the release tag v0.6.0, commit
`c121ced0037874ec7fc1e3f928c3e01cdee026f8`, plus
`v0.6.0-verified-base.patch` (SHA-256
`90430ea86695155de07b8803768daf7d9702492509e0c2e35ec69af9c79285e3`).
The patch changes only `crates/atelier-cli/src/check.rs`; it uses v0.6.0's
existing imports and names. Current master moved the core APIs into one crate
and renamed local variables, so the current source file cannot simply replace
the release file. The helper and lifecycle exception have identical logic.

To reproduce without installing anything, extract that pinned commit with
`git archive` into an owned temporary directory, apply the patch with
`git apply`, and copy `crates/atelier-cli/tests/base_imports.rs` from this
item's reviewed commit into the extracted source's test directory. Run
`cargo test -p atelier-cli --test base_imports`, `cargo fmt --all -- --check`
and `cargo clippy --all-targets -- -D warnings` there. The debug binary is
`target/debug/atelier`; no `cargo install` is needed.

The local prepared source is `/tmp/atelier-t83m-v060-nkbdxpa9` and its debug
binary SHA-256 is
`49fe3302694438a97e07bf1d6fbfa0f05e3e5849dbe792011d2075e9465783cb`.
The source pin and patch hash, rather than a temporary directory or a
machine-specific binary hash, define the reproducible candidate. Its five
real-hook tests cover the same acceptance and adversarial cases as master.

No guard skip or commit-plumbing bypass is used by the regression fixtures.
They seed committed synthetic histories before installing their hooks; every
commit whose acceptance or refusal is asserted runs the real pre-commit hook.
No consumer records, refs, index, hooks, runtime or installations were changed.
