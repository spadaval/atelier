"""Grade a sized case's facts (harness/sized.py); rubric.md holds its
questions of meaning."""
import subprocess
import sys
from pathlib import Path

here = Path(__file__).resolve().parent
sys.exit(subprocess.call([sys.executable, "-I", str(here.parents[1] / "harness" / "sized.py"),
                          str(here), sys.argv[1]]))
