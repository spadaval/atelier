#!/usr/bin/env bash
# A small Python repository on main with no tracker: no .atelier/, no AGENTS.md
# process, no remote. `atelier status` in it fails with "Not an Atelier
# repository".
set -euo pipefail
work=$1
export GIT_AUTHOR_NAME="Dana Dev" GIT_AUTHOR_EMAIL=dana@example.com
export GIT_COMMITTER_NAME="Dana Dev" GIT_COMMITTER_EMAIL=dana@example.com
git init -q -b main "$work/repo"
cd "$work/repo"
cat > export.py <<'PY'
"""Export orders as delimited text."""
import csv
import io

FIELDS = ["id", "customer", "quantity", "unit_price"]


def export(orders, delimiter=","):
    out = io.StringIO()
    writer = csv.writer(out, delimiter=delimiter, lineterminator="\n")
    writer.writerow(FIELDS)
    for order in orders:
        writer.writerow([order[f] for f in FIELDS])
    return out.getvalue()
PY
cat > cli.py <<'PY'
"""Print the orders in a JSON file as CSV."""
import argparse
import json
import sys

from export import export


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("orders", help="path to a JSON list of orders")
    args = parser.parse_args(argv)
    with open(args.orders) as f:
        orders = json.load(f)
    sys.stdout.write(export(orders))


if __name__ == "__main__":
    main()
PY
cat > test_export.py <<'PY'
import unittest

from export import export


class ExportTest(unittest.TestCase):
    def test_header_and_row(self):
        rows = export([{"id": 1, "customer": "Ann", "quantity": 2, "unit_price": 3.5}])
        self.assertEqual(rows.splitlines()[0], "id,customer,quantity,unit_price")
        self.assertEqual(rows.splitlines()[1], "1,Ann,2,3.5")


if __name__ == "__main__":
    unittest.main()
PY
cat > README.md <<'MD'
# orders-export

Export the shop's orders for the accountants.

## Usage

```bash
python3 cli.py orders.json > orders.csv
```

`orders.json` is a list of objects with `id`, `customer`, `quantity` and
`unit_price`.

## Tests

```bash
python3 -m unittest
```
MD
printf '__pycache__/\n' > .gitignore
git add . && git commit -qm "Export orders as CSV"

# Grader inputs, outside the repository.
git rev-parse HEAD > "$work/.setup_head"
git ls-files > "$work/.setup_files"
