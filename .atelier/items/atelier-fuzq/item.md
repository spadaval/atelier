---
schema: atelier.item
schema_version: 3
id: atelier-fuzq
title: Label an item as it is filed, so a question or a bug filed on an item's branch is labelled
status: open
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-22T18:46:12.216917161Z
---

## Goal

An agent can now file an item on the branch it is on (atelier-u6oy), and
cannot label it there. `new` has no `--labels`, `set` is refused on an
item's branch, and agents do not edit front matter by hand. So a question
for the human filed that way has no `needs-human` and `list --needs-human`
does not show it, and a bug has no `bug`, until someone makes a backlog
branch to run `set`, which is the trip atelier-u6oy removed. The skill
tells an implementer to take such a question to a `needs-human` item.
Found by atelier-u6oy's implementer.

## Done when

- [ ] An item filed on an item's branch can carry its labels in the commit
  that files it, for example by `new --labels a,b`, and `list
  --needs-human` in another clone shows one labelled so while it is
  proposed. A test holds it.
- [ ] `atelier new --help` and `docs/product.md` say how.

## Scope

`new`. Not `set` on an item's branch.
