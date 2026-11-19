"""Exporters turn report rows into a text format.

Each exporter registers itself under a format name, so `report --format`
can offer every registered format and a new one is one class:

    @register("yaml")
    class YamlExporter(Exporter):
        def write(self, columns, rows, out): ...
"""
import csv
import json
from abc import ABC, abstractmethod

_EXPORTERS = {}


def register(name):
    """Class decorator: make an Exporter available under `name`."""
    def wrap(cls):
        if name in _EXPORTERS:
            raise ValueError(f"an exporter is already registered as {name!r}")
        _EXPORTERS[name] = cls
        cls.format_name = name
        return cls
    return wrap


def available_formats():
    return sorted(_EXPORTERS)


def get_exporter(name, **options):
    try:
        cls = _EXPORTERS[name]
    except KeyError:
        raise ValueError(f"unknown format {name!r}; "
                         f"choose one of {', '.join(available_formats())}") from None
    return cls(**options)


class Exporter(ABC):
    """Writes rows, each a tuple of strings in `columns`' order, to `out`."""
    format_name = None

    @abstractmethod
    def write(self, columns, rows, out):
        ...


@register("csv")
class CsvExporter(Exporter):
    def __init__(self, delimiter=",", header=True):
        self.delimiter = delimiter
        self.header = header

    def write(self, columns, rows, out):
        writer = csv.writer(out, delimiter=self.delimiter, lineterminator="\n")
        if self.header:
            writer.writerow(columns)
        writer.writerows(rows)


@register("tsv")
class TsvExporter(CsvExporter):
    def __init__(self, header=True):
        super().__init__(delimiter="\t", header=header)


@register("json")
class JsonExporter(Exporter):
    def __init__(self, indent=2):
        self.indent = indent

    def write(self, columns, rows, out):
        json.dump([dict(zip(columns, row)) for row in rows], out, indent=self.indent)
        out.write("\n")


@register("markdown")
class MarkdownExporter(Exporter):
    def write(self, columns, rows, out):
        out.write("| " + " | ".join(columns) + " |\n")
        out.write("|" + "|".join(" --- " for _ in columns) + "|\n")
        for row in rows:
            out.write("| " + " | ".join(cell.replace("|", "\\|") for cell in row) + " |\n")
