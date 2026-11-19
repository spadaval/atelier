## Goal

The workshop reorders by walking the shelves. A list of the items running
low, from the stock file it already keeps, would tell them what to order.

## Done when

- [ ] `python3 inventory.py stock.csv --low 5` prints, after the summary
  line, each item whose quantity is below 5 as `SKU quantity`
  (`BRK-0042 3`), lowest quantity first and by SKU among equals.
- [ ] When no item is below the threshold it prints `nothing below 5`.
- [ ] `--low` takes a whole number; without it the output is as before.
- [ ] Tests cover each line above, and `python3 -m unittest` passes.

## Scope

`inventory.py`, its tests, and any new module they need.
