"""Stock levels from a CSV file with the header sku,name,quantity.

    python3 inventory.py stock.csv [--low N]

prints how many items the file holds and how many units in all, and with
--low, each item whose quantity is below N.
"""
import argparse
import csv
import logging
import math
import re
import sys
from dataclasses import dataclass

SKU = re.compile(r"[A-Z]{3}-[0-9]{4}")
QUANTITY = re.compile(r"[0-9]+")
log = logging.getLogger("inventory")


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


def _quantity(item):
    """The item's quantity as an int, or None if it is not a usable count."""
    value = getattr(item, "quantity", None)
    if value is None or isinstance(value, bool):
        return None
    if isinstance(value, str):
        value = value.strip()
        if not QUANTITY.fullmatch(value):
            return None
        return int(value)
    if isinstance(value, float):
        if math.isnan(value) or math.isinf(value) or not value.is_integer():
            return None
        value = int(value)
    if not isinstance(value, int) or value < 0:
        return None
    return value


def low_stock(items, threshold):
    """[(sku, quantity)] for the items whose quantity is below `threshold`,
    lowest first and by SKU among equals.

    Defensive about its input: None is no items; an item with a missing,
    negative, non-numeric or NaN quantity is skipped with a warning; a
    quantity given as text is read; and where a SKU appears more than once,
    its lowest quantity is kept.
    """
    if items is None:
        return []
    lowest = {}
    for item in items:
        sku = getattr(item, "sku", None)
        quantity = _quantity(item)
        if not sku or quantity is None:
            log.warning("skipping an item with a bad SKU or quantity: %r", item)
            continue
        if sku in lowest:
            log.warning("%s appears more than once; keeping the lowest quantity", sku)
            quantity = min(quantity, lowest[sku])
        lowest[sku] = quantity
    return sorted(((sku, q) for sku, q in lowest.items() if q < threshold),
                  key=lambda pair: (pair[1], pair[0]))


def main(argv=None):
    parser = argparse.ArgumentParser(prog="inventory.py")
    parser.add_argument("file")
    parser.add_argument("--low", type=int, metavar="N",
                        help="also list the items with fewer than N units")
    args = parser.parse_args(argv)
    try:
        items = load(args.file)
    except InventoryError as e:
        print(f"{args.file}: {e}", file=sys.stderr)
        return 1
    print(f"{len(items)} items, {sum(i.quantity for i in items)} units")
    if args.low is not None:
        low = low_stock(items, args.low)
        for sku, quantity in low:
            print(f"{sku} {quantity}")
        if not low:
            print(f"nothing below {args.low}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
