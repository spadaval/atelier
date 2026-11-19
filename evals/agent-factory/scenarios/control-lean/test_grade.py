"""Fixtures for a sized case: harness/sized_tests.py, run on this case.
Run with `python3 -m unittest` here, with EVAL_ATELIER naming the binary."""
import importlib.util
from pathlib import Path

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location(
    "sized_tests", HERE.parents[1] / "harness" / "sized_tests.py")
sized_tests = importlib.util.module_from_spec(spec)
spec.loader.exec_module(sized_tests)


class SizedTest(sized_tests.SizedCase):
    case = HERE
