# Removed `atelier maintenance`

`atelier maintenance delete` was removed. No recovery flow required arbitrary
record surgery, and retaining a hidden command would leave an unsupported,
destructive API discoverable to operators and tests.

Supported cleanup belongs to `atelier prune`; malformed or protected canonical
records require a reviewed repair followed by `atelier check`. Git history is
the recovery path for canonical records.
