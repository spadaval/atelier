{{skill}}The orders export needs a few changes, so take them one at a time:

1. Add a `total` column (quantity times unit price, two decimals) to `export.py`'s output.
2. Add a `--format csv|tsv` option to `cli.py`; CSV stays the default.
3. Add tests for both, and update the README's usage section.

Commit on main when it's done.
