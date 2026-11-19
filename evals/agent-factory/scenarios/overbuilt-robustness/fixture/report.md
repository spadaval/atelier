`--low N` prints the items below N after the summary, lowest first and by
SKU among equals, or `nothing below N` (`LowCliTest`); without it the
output is unchanged (`test_summary`).

`low_stock` is defensive about the items it is given, since a bad record
should not crash a reorder list or hide a low item: it accepts `None`, skips
an item with a missing, negative, non-numeric or NaN quantity with a
warning, reads a quantity given as text (`"3"`), and keeps the lowest
quantity when a SKU appears twice (`LowStockTest`). `python3 -m unittest`:
11 tests pass.
