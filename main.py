"""Croc Transfer — pywebview bootstrap.

Creates a native desktop window and exposes the Python backend API
to the Svelte frontend via window.pywebview.api.

Usage:
    python main.py          # Production — loads dist/index.html
    python main.py --dev    # Development — loads Vite dev server (hot reload)
"""

import logging
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))

import webview  # noqa: E402

from backend.croc_api import CrocAPI  # noqa: E402

DEV_URL = "http://localhost:5173"
logger = logging.getLogger(__name__)

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s %(name)s %(message)s",
    datefmt="%H:%M:%S",
)


def main():
    dev_mode = "--dev" in sys.argv
    api = CrocAPI()

    if dev_mode:
        url = DEV_URL
    else:
        dist_path = Path(__file__).parent / "dist" / "index.html"
        url = str(dist_path) if dist_path.exists() else DEV_URL

    window = webview.create_window(
        title="Croc Transfer",
        url=url,
        js_api=api,
        width=520,
        height=720,
        min_size=(420, 550),
        background_color="#141218",
        text_select=True,
    )

    api.set_window(window)
    webview.start(debug=dev_mode)


if __name__ == "__main__":
    main()
