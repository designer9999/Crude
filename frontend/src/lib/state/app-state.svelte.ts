/**
 * Global app state — transfer status, logs, selected files, persisted options.
 */

import type { FileInfo, SendOptions, ReceiveOptions } from "$lib/api/bridge";

export interface LogEntry {
  level: "info" | "warn" | "error" | "success";
  text: string;
  time: string;
}

export interface SelectedFile {
  path: string;
  info: FileInfo | null;
}

// ── localStorage helpers ──

const SEND_OPTS_KEY = "crude-send-options";
const RECV_OPTS_KEY = "crude-receive-options";

function loadJson<T>(key: string, fallback: T): T {
  try {
    const raw = localStorage.getItem(key);
    return raw ? { ...fallback, ...JSON.parse(raw) } : fallback;
  } catch {
    return fallback;
  }
}

function saveJson(key: string, data: unknown): void {
  try {
    localStorage.setItem(key, JSON.stringify(data));
  } catch { /* quota exceeded — ignore */ }
}

// ── Default options ──

const DEFAULT_SEND_OPTS: SendOptions = {};
const DEFAULT_RECV_OPTS: ReceiveOptions = {};

class AppState {
  activeTab = $state<"send" | "receive" | "guide">("send");
  files = $state<SelectedFile[]>([]);
  transferActive = $state(false);
  transferMode = $state<"send" | "receive" | null>(null);
  transferCode = $state<string | null>(null);
  crocVersion = $state<string>("...");
  crocOk = $state(true);
  logs = $state<LogEntry[]>([]);

  // Persisted options (loaded from localStorage)
  sendOptions = $state<SendOptions>(loadJson(SEND_OPTS_KEY, DEFAULT_SEND_OPTS));
  receiveOptions = $state<ReceiveOptions>(loadJson(RECV_OPTS_KEY, DEFAULT_RECV_OPTS));

  get hasFiles(): boolean {
    return this.files.length > 0;
  }

  get filePaths(): string[] {
    return this.files.map(f => f.path);
  }

  addFile(path: string, info: FileInfo | null) {
    if (this.files.some(f => f.path === path)) return;
    this.files = [...this.files, { path, info }];
  }

  removeFile(path: string) {
    this.files = this.files.filter(f => f.path !== path);
  }

  clearFiles() {
    this.files = [];
  }

  addLog(level: LogEntry["level"], text: string) {
    const time = new Date().toLocaleTimeString("en-GB", { hour12: false });
    this.logs = [...this.logs, { level, text, time }];
  }

  clearLogs() {
    this.logs = [];
  }

  resetTransfer() {
    this.transferActive = false;
    this.transferMode = null;
    this.transferCode = null;
  }

  // Persist options on change
  saveSendOptions() {
    saveJson(SEND_OPTS_KEY, this.sendOptions);
  }

  saveReceiveOptions() {
    saveJson(RECV_OPTS_KEY, this.receiveOptions);
  }

  updateSendOption<K extends keyof SendOptions>(key: K, value: SendOptions[K]) {
    this.sendOptions = { ...this.sendOptions, [key]: value };
    this.saveSendOptions();
  }

  updateReceiveOption<K extends keyof ReceiveOptions>(key: K, value: ReceiveOptions[K]) {
    this.receiveOptions = { ...this.receiveOptions, [key]: value };
    this.saveReceiveOptions();
  }
}

let instance: AppState | null = null;

export function getAppState(): AppState {
  if (!instance) instance = new AppState();
  return instance;
}
