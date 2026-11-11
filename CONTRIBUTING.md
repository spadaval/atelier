# Contributing To Atelier

Atelier tracks its own work in its committed `.atelier/` records, which are
the source of truth for planned and active work. Do not open or treat GitHub
issues as the work queue for this repository.

Before changing code or docs:

1. Read [AGENTS.md](AGENTS.md), and run `atelier status`.
2. Find or create the item the change belongs to (`atelier list`, `atelier
   new`).
3. Start it with `atelier start <id>`, which makes its branch.
4. Tick its Done when lines, write its Report, and `atelier submit` it for an
   independent review.

`atelier submit` opens the pull request, and `atelier merge` merges it once
the review approves and CI passes. Pull requests are not opened or merged by
hand. Dependency update pull requests are welcome, but they still go through
an item before merge.

What the tool does is in the [product specification](docs/product.md).
