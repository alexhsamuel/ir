import ir
from   pathlib import Path
import platform

SCRIPTS_DIR = Path(__file__).parent / "scripts"

#-------------------------------------------------------------------------------

def test_general():
    expected_exit_code = 42
    min_utime = 0.25
    max_utime = 2 * min_utime
    min_rset_size_bytes = 1024 * 1024 * 1024  # 1 GB
    max_rset_size_bytes = int(min_rset_size_bytes * 1.03)  # 3% margin
    proc = ir.run1({
        "argv": [
            str(SCRIPTS_DIR / "general"),
            "--allocate", str(min_rset_size_bytes),  # 1 GB
            "--work", str(min_utime),
            # FIXME: Record start/stop/elapsed time.
            # "--sleep", "0.5", 
            "--exit-code", str(expected_exit_code),
        ],
    })

    assert proc["exit_code"] == expected_exit_code
    rusage = proc["rusage"]
    utime = rusage["ru_utime"]
    utime = utime["tv_sec"] + 1e-6 * utime["tv_usec"]
    assert min_utime <= utime < max_utime
    rset_size_bytes = rusage["ru_maxrss"] * 1024
    assert min_rset_size_bytes <= rset_size_bytes < max_rset_size_bytes


