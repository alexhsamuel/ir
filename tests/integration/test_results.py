import ir
from   pathlib import Path
import platform

SCRIPTS_DIR = Path(__file__).parent / "scripts"

#-------------------------------------------------------------------------------

def test_general():
    proc = ir.run1({
        "argv": [
            str(SCRIPTS_DIR / "general"),
            "--allocate", "1073741824",  # 1 GB
            "--work", "0.25",
            # FIXME: Record start/stop/elapsed time.
            # "--sleep", "0.5", 
            "--exit-code", "42",
        ],
    })

    assert proc["exit_code"] == 42
    rusage = proc["rusage"]
    utime = rusage["ru_utime"]
    utime = utime["tv_sec"] + 1e-6 * utime["tv_usec"]
    assert 0.25 < utime < 0.5
    if platform.system() == "Darwin":
        rset_size_bytes = rusage["ru_maxrss"]
    elif platform.system() == "Linux":
        rset_size_bytes = rusage["ru_maxrss"] * 1024
    else:
        assert False, f"Unsupported system {platform.system()}"
    assert 1073741824 < rset_size_bytes < 1100000000


