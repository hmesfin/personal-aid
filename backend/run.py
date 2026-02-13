"""Sidecar entry point for the Personal Aid Python backend.

This script is the target for both:
- Development: `uv run python run.py`
- Production: PyInstaller bundles this into an .exe
"""

import sys

import uvicorn

from personal_aid.main import app


def main() -> None:
    port = 18008
    if len(sys.argv) > 1:
        port = int(sys.argv[1])

    uvicorn.run(
        app,
        host="127.0.0.1",
        port=port,
        log_level="info",
    )


if __name__ == "__main__":
    main()
