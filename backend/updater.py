"""
GitHub-based auto-updater.
Checks for new releases, downloads .exe updates, launches installer.
"""

import json
import logging
import os
import tempfile
import urllib.request

from backend.version import APP_VERSION, GITHUB_REPO

logger = logging.getLogger(__name__)

API_URL = f"https://api.github.com/repos/{GITHUB_REPO}/releases/latest"


def _parse_version(tag: str) -> tuple[int, ...]:
    """Parse 'v1.2.3' or '1.2.3' into (1, 2, 3)."""
    clean = tag.lstrip("vV").strip()
    parts = []
    for p in clean.split("."):
        try:
            parts.append(int(p))
        except ValueError:
            break
    return tuple(parts) or (0,)


def check_for_updates() -> dict:
    """Check GitHub for a newer release.

    Returns dict with keys:
      - update_available: bool
      - current_version: str
      - latest_version: str (if available)
      - download_url: str (if available)
      - release_notes: str (if available)
      - error: str (if failed)
    """
    result = {"update_available": False, "current_version": APP_VERSION}

    try:
        req = urllib.request.Request(
            API_URL,
            headers={
                "Accept": "application/vnd.github.v3+json",
                "User-Agent": "Crude-Updater",
            },
        )
        with urllib.request.urlopen(req, timeout=10) as resp:
            data = json.loads(resp.read().decode())

        latest_tag = data.get("tag_name", "")
        result["latest_version"] = latest_tag

        body = data.get("body", "")
        if body:
            result["release_notes"] = body[:500]

        current = _parse_version(APP_VERSION)
        latest = _parse_version(latest_tag)

        if latest > current:
            result["update_available"] = True

            # Find .exe asset
            for asset in data.get("assets", []):
                name = asset.get("name", "")
                if name.endswith(".exe"):
                    result["download_url"] = asset["browser_download_url"]
                    result["file_name"] = name
                    result["file_size"] = asset.get("size", 0)
                    break

            if "download_url" not in result:
                result["download_url"] = data.get("html_url", "")

    except Exception as e:
        logger.error("Update check failed: %s", e)
        result["error"] = str(e)

    return result


def download_update(url: str, progress_callback=None) -> str | None:
    """Download the update .exe to a temp folder.

    Args:
        url: Direct download URL for the .exe
        progress_callback: Optional callable(percent: int) for progress updates

    Returns:
        Full path to downloaded file, or None on failure.
    """
    try:
        req = urllib.request.Request(url, headers={"User-Agent": "Crude-Updater"})
        with urllib.request.urlopen(req, timeout=300) as resp:
            total = int(resp.headers.get("Content-Length", 0))
            file_name = url.split("/")[-1]
            if not file_name.endswith(".exe"):
                file_name = "CrocTransfer-update.exe"

            dest = os.path.join(tempfile.gettempdir(), file_name)
            downloaded = 0
            chunk_size = 64 * 1024  # 64KB chunks

            with open(dest, "wb") as f:
                while True:
                    chunk = resp.read(chunk_size)
                    if not chunk:
                        break
                    f.write(chunk)
                    downloaded += len(chunk)
                    if total > 0 and progress_callback:
                        pct = min(100, int(downloaded * 100 / total))
                        progress_callback(pct)

            if progress_callback:
                progress_callback(100)

            logger.info("Update downloaded to: %s", dest)
            return dest

    except Exception as e:
        logger.error("Download failed: %s", e)
        return None
