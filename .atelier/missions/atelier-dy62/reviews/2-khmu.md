---
schema: atelier.review
schema_version: 3
round: 2
commit: e3cfd2a467e794be46eaf4747f0ee26dc9f033b9
reviewers:
- thread:delegated-task:command%3Amcp%3A7528a691-d081-4f17-89b8-d45bea97b6e3%3Adelegate-task%3A18100410-e9da-471c-ba56-f732efd989e6
model: gpt-6-astra
verdict: approve
created_at: 2026-11-14T06:07:37.685603293Z
---

## Summary

Complexity: medium. The merge touches three library crates, two callers and
shared documentation, but adds no state or subsystem; the three items separate
the file moves, process-call consolidation and terminology rename.

Value: medium. Both programs use this library today, and the change removes
misleading crate names and the dependency barrier behind duplicate Git calls.

Risk of merging it: medium. Implementation reaches the CLI and Desk's Git and
forge operations; behavior preservation and the existing tests must constrain
those moves.

The value warrants the scope. The plan keeps unrelated cleanup out, orders the
forge move through atelier-yclc, and explicitly makes the terminology rename
droppable. A smaller change would leave part of the requested consolidation
undone.

Reviewed the plan at e3cfd2a467e794be46eaf4747f0ee26dc9f033b9, all three
mission items, the changes to atelier-yclc and atelier-hj6b, and all six carried
findings. Compared the round-one corrections with the manifests, module
exports, test and fixture paths, dependency test, Git and forge spawn sites,
Desk request types, design rules and the related atelier-to83 mission. Also
checked atelier-o91z: its old path is historical context, not a requirement to
implement against a deleted crate.

`atelier check` passed for 164 items, 10 missions and config.toml.
`git diff --check c22b1377b^..HEAD` passed. This is a plan review: no
implementation exists here to compile or exercise, and I did not run the Rust
or browser test suites. Their preservation remains an implementation check.

All six carried findings are fixed. No additional findings.

## Finding 1: Compare behavior against each item's base, allowing the required open-mission stop feature
Recommendation: fix now
Was: 1-atz2.1
Now: fixed

The mission's nothing-changed criterion now compares each item's tests with
its own base and explicitly excludes atelier-yclc's intervening feature changes
(mission.md:73-80). atelier-x5al therefore preserves the post-yclc behavior.

## Finding 2: Allow mechanical crate-path changes inside test bodies
Recommendation: fix now
Was: 1-atz2.2
Now: fixed

Both nothing-changed (mission.md:75-78) and atelier-ni3d's Done when allow
crate and module path substitutions wherever they occur in tests. The item
explicitly includes the fully qualified call in a test body.

## Finding 3: one-git still fails after x5al because the Desk build script spawns git
Recommendation: fix now
Was: 1-kh04.1
Now: fixed

The mission explains the Desk build script's compile-time stamp, limits the
Outcome to runtime Git calls, and excludes build.rs in one-git
(mission.md:38-40, 49, 83-85). atelier-x5al applies the same exception in
its acceptance criterion and Scope.

## Finding 4: nothing-changed and ni3d's test-diff rule forbid edits the crate merge must make
Recommendation: fix now
Was: 1-kh04.2
Now: fixed

nothing-changed and atelier-ni3d now allow moved fixture paths and the crate
list in crates/atelier-desk/tests/dependencies.rs. The item names both concrete
cases from the finding, while retaining the unchanged-test-behavior rule.

## Finding 5: atelier-yclc still tells its implementer to write atelier-app after ni3d deletes it
Recommendation: fix now
Was: 1-kh04.3
Now: fixed

atelier-yclc's Done when and Scope now name atelier-core, and its Context
explains the blocking crate merge. atelier-hj6b names the core before and after
that merge without imposing an unnecessary ordering. atelier-o91z's old path
only describes where the earlier failure was found.

## Finding 6: dropping nl43 leaves Authority's frozen module name `actions` unmet
Recommendation: fix now
Was: 1-kh04.4
Now: fixed

Outcome now calls the module acts until atelier-nl43 renames it. Authority
freezes only model, records, git and forge, and explicitly records that dropping
nl43 leaves acts and an unmet one-word criterion (mission.md:49-51, 122-130).
