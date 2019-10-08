import json
from   pathlib import Path
import subprocess
import sys
import tempfile

#-------------------------------------------------------------------------------

IR_EXE = Path(__file__).parents[2] / "target/debug/ir"
TEST_EXE = Path(__file__).parent / "test.py"


def run(spec):
    with tempfile.NamedTemporaryFile(mode="w+") as tmp_file:
        json.dump(spec, tmp_file)
        tmp_file.flush()
        res = subprocess.run(
            [str(IR_EXE), tmp_file.name],
            stdout=subprocess.PIPE,
            check=True,
        )
    res = json.loads(res.stdout)
    json.dump(res, sys.stderr, indent=2)
    return res


