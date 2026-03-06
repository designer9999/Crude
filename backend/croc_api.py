"""
Croc Transfer — Python backend API.
Exposed to the Svelte frontend via window.pywebview.api.
"""

import json
import logging
import os
import shutil
import subprocess
import sys
import threading

import webview

from backend.updater import check_for_updates, download_update
from backend.version import APP_VERSION

logger = logging.getLogger(__name__)

# Avoid console windows popping up on Windows
_NO_WINDOW = getattr(subprocess, "CREATE_NO_WINDOW", 0)


def _file_size_str(size_bytes: int) -> str:
    """Human-readable file size."""
    for unit in ("B", "KB", "MB", "GB"):
        if size_bytes < 1024:
            return f"{size_bytes:.1f} {unit}" if unit != "B" else f"{size_bytes} B"
        size_bytes /= 1024
    return f"{size_bytes:.1f} TB"


class CrocAPI:
    """pywebview js_api — each public method becomes window.pywebview.api.<name>()

    IMPORTANT: All non-API attributes MUST start with _ to prevent pywebview
    from recursively enumerating them (which causes infinite recursion on
    WinForms .NET AccessibilityObject properties).
    """

    def __init__(self) -> None:
        self._window: webview.Window | None = None
        self._process: subprocess.Popen | None = None
        self._transfer_active = False
        self._croc_path = self._find_croc()

    def set_window(self, window: webview.Window) -> None:
        self._window = window

    # ── Croc discovery ──

    @staticmethod
    def _find_croc() -> str | None:
        found = shutil.which("croc")
        if found:
            return found
        candidates = [
            os.path.join(
                os.environ.get("LOCALAPPDATA", ""),
                "Microsoft",
                "WinGet",
                "Links",
                "croc.exe",
            ),
            os.path.expanduser("~/.local/bin/croc"),
            "/usr/local/bin/croc",
        ]
        for p in candidates:
            if p and os.path.isfile(p):
                return p
        return None

    # ── Public API (called from JS) ──

    def get_status(self) -> dict:
        if not self._croc_path:
            return {
                "ok": False,
                "error": "croc not found. Install: winget install schollz.croc",
            }
        try:
            r = subprocess.run(
                [self._croc_path, "--version"],
                capture_output=True,
                text=True,
                timeout=5,
                creationflags=_NO_WINDOW,
            )
            return {
                "ok": True,
                "version": r.stdout.strip(),
                "app_version": APP_VERSION,
            }
        except Exception as e:
            return {"ok": False, "error": str(e)}

    def check_update(self) -> dict:
        """Check GitHub for a newer app version."""
        return check_for_updates()

    def download_update(self, url: str) -> None:
        """Download update .exe in background, report progress to frontend."""

        def _run():
            def on_progress(pct):
                self._js_event("update_progress", {"percent": pct})

            path = download_update(url, progress_callback=on_progress)
            if path:
                self._js_event(
                    "update_ready", {"path": path, "file_name": os.path.basename(path)}
                )
            else:
                self._js_event("update_failed", {})

        threading.Thread(target=_run, daemon=True).start()

    def launch_update(self, path: str) -> None:
        """Launch the downloaded update and exit the app."""
        if os.path.isfile(path):
            os.startfile(path)
            # Give a moment for the process to start, then exit
            if self._window:
                self._window.destroy()
            sys.exit(0)

    def pick_files(self) -> list[str] | None:
        if not self._window:
            return None
        result = self._window.create_file_dialog(
            webview.FileDialog.OPEN, allow_multiple=True
        )
        return [str(p) for p in result] if result else None

    def pick_folder(self) -> list[str] | None:
        if not self._window:
            return None
        result = self._window.create_file_dialog(webview.FileDialog.FOLDER)
        return [str(p) for p in result] if result else None

    def pick_save_folder(self) -> str | None:
        """Pick a folder to save received files into."""
        if not self._window:
            return None
        result = self._window.create_file_dialog(webview.FileDialog.FOLDER)
        if result and len(result) > 0:
            return str(result[0])
        return None

    def get_file_info(self, path: str) -> dict | None:
        """Return file/folder name, size, type for display."""
        if not os.path.exists(path):
            return None
        name = os.path.basename(path) or path
        is_dir = os.path.isdir(path)
        if is_dir:
            total = 0
            count = 0
            for root, dirs, files in os.walk(path):
                for f in files:
                    try:
                        total += os.path.getsize(os.path.join(root, f))
                    except OSError:
                        pass
                    count += 1
            return {
                "name": name,
                "size": _file_size_str(total),
                "type": "folder",
                "count": count,
            }
        else:
            size = os.path.getsize(path)
            ext = os.path.splitext(name)[1].lower()
            return {"name": name, "size": _file_size_str(size), "type": ext or "file"}

    def send_files(self, paths: list, options: dict | None = None) -> None:
        """Send one or more files/folders with optional croc flags."""
        if not self._croc_path:
            self._js_log("error", "croc not found!")
            return
        valid = [p for p in paths if os.path.exists(p)]
        if not valid:
            self._js_log("error", "No valid files selected")
            return

        self._transfer_active = True
        names = [os.path.basename(p) for p in valid]
        self._js_event("transfer_start", {"mode": "send", "files": names})
        threading.Thread(
            target=self._run_send, args=(valid, options or {}), daemon=True
        ).start()

    def send_text(self, text: str, options: dict | None = None) -> None:
        """Send text content via croc."""
        if not self._croc_path:
            self._js_log("error", "croc not found!")
            return
        text = (text or "").strip()
        if not text:
            self._js_log("error", "No text provided")
            return

        self._transfer_active = True
        self._js_event("transfer_start", {"mode": "send", "files": ["(text)"]})
        threading.Thread(
            target=self._run_send_text, args=(text, options or {}), daemon=True
        ).start()

    def receive_file(self, code: str, options: dict | None = None) -> None:
        if not self._croc_path:
            self._js_log("error", "croc not found!")
            return
        code = (code or "").strip()
        if not code:
            self._js_log("error", "No code provided")
            return

        self._transfer_active = True
        self._js_event("transfer_start", {"mode": "receive", "code": code})
        threading.Thread(
            target=self._run_receive, args=(code, options or {}), daemon=True
        ).start()

    def stop_transfer(self) -> bool:
        if self._process:
            self._process.terminate()
            self._transfer_active = False
            self._js_log("warn", "Transfer stopped")
            self._js_event("transfer_done", {"success": False, "stopped": True})
            return True
        return False

    # ── Internal: build CLI args from options ──

    def _build_global_args(self, opts: dict) -> list[str]:
        """Build global croc flags from options dict.
        Note: 'sourceFolder' is a UI-only key, ignored here.
        """
        args = [self._croc_path, "--yes"]

        if opts.get("noCompress"):
            args.append("--no-compress")
        if opts.get("overwrite"):
            args.append("--overwrite")
        if opts.get("curve") and opts["curve"] != "p256":
            args += ["--curve", opts["curve"]]
        if opts.get("relay"):
            args += ["--relay", opts["relay"]]
        if opts.get("relayPass"):
            args += ["--pass", opts["relayPass"]]
        if opts.get("throttleUpload"):
            args += ["--throttleUpload", opts["throttleUpload"]]
        if opts.get("local"):
            args.append("--local")

        return args

    def _build_send_args(self, opts: dict) -> list[str]:
        """Build send subcommand flags from options dict."""
        args = ["send"]

        if opts.get("code"):
            args += ["--code", opts["code"]]
        if opts.get("hash") and opts["hash"] != "xxhash":
            args += ["--hash", opts["hash"]]
        if opts.get("zip"):
            args.append("--zip")
        if opts.get("git"):
            args.append("--git")
        if opts.get("noLocal"):
            args.append("--no-local")
        if opts.get("noMulti"):
            args.append("--no-multi")
        if opts.get("exclude"):
            args += ["--exclude", opts["exclude"]]

        return args

    # ── Internal: run croc subprocess ──

    def _run_send(self, paths: list[str], opts: dict) -> None:
        try:
            cmd = self._build_global_args(opts) + self._build_send_args(opts) + paths
            logger.info("Running: %s", " ".join(cmd))
            self._process = subprocess.Popen(
                cmd,
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                text=True,
                bufsize=1,
                creationflags=_NO_WINDOW,
            )
            for line in self._process.stdout:
                line = line.rstrip()
                if not line:
                    continue
                if "Code is:" in line:
                    code = line.split("Code is:")[-1].strip()
                    self._js_event("code_ready", {"code": code})
                self._js_log("info", line)
            self._process.wait()
            success = self._process.returncode == 0
            self._js_event("transfer_done", {"success": success})
        except Exception as e:
            logger.error("Send error: %s", e)
            self._js_log("error", str(e))
            self._js_event("transfer_done", {"success": False})
        finally:
            self._process = None
            self._transfer_active = False

    def _run_send_text(self, text: str, opts: dict) -> None:
        try:
            cmd = (
                self._build_global_args(opts)
                + self._build_send_args(opts)
                + ["--text", text]
            )
            logger.info("Running: croc send --text ...")
            self._process = subprocess.Popen(
                cmd,
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                text=True,
                bufsize=1,
                creationflags=_NO_WINDOW,
            )
            for line in self._process.stdout:
                line = line.rstrip()
                if not line:
                    continue
                if "Code is:" in line:
                    code = line.split("Code is:")[-1].strip()
                    self._js_event("code_ready", {"code": code})
                self._js_log("info", line)
            self._process.wait()
            success = self._process.returncode == 0
            self._js_event("transfer_done", {"success": success})
        except Exception as e:
            logger.error("Send text error: %s", e)
            self._js_log("error", str(e))
            self._js_event("transfer_done", {"success": False})
        finally:
            self._process = None
            self._transfer_active = False

    def _run_receive(self, code: str, opts: dict) -> None:
        try:
            cmd = self._build_global_args(opts)

            if opts.get("outFolder"):
                cmd += ["--out", opts["outFolder"]]

            cmd.append(code)
            logger.info("Running: %s", " ".join(cmd))
            self._process = subprocess.Popen(
                cmd,
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                text=True,
                bufsize=1,
                creationflags=_NO_WINDOW,
            )
            for line in self._process.stdout:
                line = line.rstrip()
                if line:
                    self._js_log("info", line)
            self._process.wait()
            success = self._process.returncode == 0
            self._js_event("transfer_done", {"success": success})
        except Exception as e:
            logger.error("Receive error: %s", e)
            self._js_log("error", str(e))
            self._js_event("transfer_done", {"success": False})
        finally:
            self._process = None
            self._transfer_active = False

    # ── JS bridge helpers ──

    def _js_log(self, level: str, msg: str) -> None:
        if self._window:
            safe = json.dumps(msg)
            self._window.evaluate_js(f'window.onLog && window.onLog("{level}", {safe})')

    def _js_event(self, event: str, data: dict) -> None:
        if self._window:
            safe = json.dumps(data)
            self._window.evaluate_js(
                f'window.onEvent && window.onEvent("{event}", {safe})'
            )
