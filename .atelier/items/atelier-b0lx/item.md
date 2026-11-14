---
schema: atelier.item
schema_version: 3
id: atelier-b0lx
title: Resolve a relative path to the Desk's notify program beside its config, as the document says
status: draft
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-14T03:10:30.843839575Z
---

## Goal

Taken from deferred finding atelier-v958/8-h9fb.2, in .atelier/items/atelier-v958/reviews/8-h9fb.md:

> Finding 2: Resolve relative notification helpers beside the config as documented
> Recommendation: defer
>
> `crates/atelier-desk/src/config.rs:127-133` resolves `notify.command[0]`
> only when it starts with `~/`. A path such as `./notify-helper` stays
> relative, and `news.rs` passes it to `Command::new` without changing the
> working directory. It therefore resolves from the server's process
> directory. `docs/architecture/desk.md`, under Running the server, explicitly
> says relative paths, including the program of `notify.command`, resolve
> from the configuration file's directory.
>
> I reproduced this with a temporary configuration naming
> `command = ["./notify-helper"]`, an executable helper beside that config,
> and the server launched from the repository. A newly proposed mission
> appeared in `news.json`, but the server logged ENOENT for the helper and
> sent nothing. This can silently lose a configured notification beyond the
> log, since that proposal has already been recorded as seen.
>
> Resolve relative program paths containing a directory component against
> the config directory while preserving PATH lookup for bare command names,
> or narrow the documented contract to match the implementation. Add a
> configuration regression for that distinction. Deferred because the
> installed configuration's `~/.../notify-mail` path is resolved correctly;
> this affects an alternate configuration and does not block the required
> host behavior or the carried fix.

## Done when

## Scope
