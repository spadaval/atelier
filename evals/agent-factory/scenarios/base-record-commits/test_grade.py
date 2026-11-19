"""Fixtures for a replay case: harness/replay_tests.py, run on this case.
Run with `python3 -m unittest` here, with EVAL_ATELIER naming the binary."""
import importlib.util
from pathlib import Path

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location(
    "replay_tests", HERE.parents[1] / "harness" / "replay_tests.py")
replay_tests = importlib.util.module_from_spec(spec)
spec.loader.exec_module(replay_tests)


class ReplayTest(replay_tests.ReplayCase):
    case = HERE
