"""Grade a later-round case's facts (harness/later_round.py); rubric.md
holds its questions of meaning."""
import subprocess
import sys
from pathlib import Path

here = Path(__file__).resolve().parent
sys.exit(subprocess.call([sys.executable, "-I", str(here.parents[1] / "harness" / "later_round.py"),
                          str(here), sys.argv[1]]))
