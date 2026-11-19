"""Stock levels from a CSV file with the header sku,name,quantity.

    python3 inventory.py stock.csv

prints how many items the file holds and how many units in all.
"""
import argparse
import csv
import re
import sys
from dataclasses import dataclass

SKU = re.compile(r"[A-Z]{3}-[0-9]{4}")
QUANTITY = re.compile(r"[0-9]+")


class InventoryError(ValueError):
    pass


@dataclass(frozen=True)
class Item:
    sku: str
    name: str
    quantity: int


def load(path):
    """The items in the CSV file at `path`.

    This is the only place the tool makes Items. Every Item it returns has a
    SKU of the form ABC-1234, unique in the file, and a quantity that is a
    whole number of zero or more; a row that breaks any of this raises
    InventoryError naming its line, and nothing is returned.
    """
    items, seen = [], set()
    with open(path, newline="") as f:
        reader = csv.DictReader(f)
        if reader.fieldnames != ["sku", "name", "quantity"]:
            raise InventoryError("line 1: the header must be sku,name,quantity")
        for line, row in enumerate(reader, start=2):
            sku, name, quantity = row["sku"], row["name"], row["quantity"]
            if sku is None or name is None or quantity is None or None in row:
                raise InventoryError(f"line {line}: expected 3 fields")
            if not SKU.fullmatch(sku):
                raise InventoryError(f"line {line}: bad SKU {sku!r}")
            if sku in seen:
                raise InventoryError(f"line {line}: {sku} appears twice")
            if not QUANTITY.fullmatch(quantity):
                raise InventoryError(f"line {line}: bad quantity {quantity!r}")
            seen.add(sku)
            items.append(Item(sku, name, int(quantity)))
    return items


def main(argv=None):
    parser = argparse.ArgumentParser(prog="inventory.py")
    parser.add_argument("file")
    args = parser.parse_args(argv)
    try:
        items = load(args.file)
    except InventoryError as e:
        print(f"{args.file}: {e}", file=sys.stderr)
        return 1
    print(f"{len(items)} items, {sum(i.quantity for i in items)} units")
    return 0


if __name__ == "__main__":
    sys.exit(main())
