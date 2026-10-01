# Chainlink Provenance

Atelier began as a fork of `dollspace-gay/chainlink`. Chainlink supplied the
initial Rust CLI, command handling, identity, locking, and test foundation.
Its SQLite-backed operational model is historical provenance, not Atelier's
current persistence contract.

[ADR 0019](../adr/0019-verified-repo-native-tracker.md) defines the current
verified repo-native tracker. Records are tracked Markdown, commands parse them
directly, reviews live in-repo, transitions have no side-effecting actions, and
Atelier never creates commits. The rebuild deliberately removes inherited
utilities and later machinery that do not serve that model.

Use Chainlink for source history and Atelier for the current product. Preserve
useful behavior and tests when changing current contracts; do not preserve
retired commands or database behavior through compatibility shims.
