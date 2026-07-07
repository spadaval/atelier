# Retired `atelier abandon`

`atelier abandon` is removed. It cleaned up a hidden active-work pointer that
is not part of the current work model.

When durable work state must change, record context with `atelier issue note`
and use the configured `atelier issue transition <id>` path. Otherwise no
Atelier cleanup command is required.
