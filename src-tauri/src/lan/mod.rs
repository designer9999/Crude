pub mod discovery;
pub mod protocol;
pub mod transfer;

use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;
use tauri::AppHandle;

pub struct LanPeer {
    pub handle: AppHandle,
    running: Arc<Mutex<bool>>,
    /// (peer_ip, last_seen_time) — updated by beacon listener
    peer_last_seen: Arc<Mutex<Option<(String, Instant)>>>,
    code_hash: Arc<Mutex<Vec<u8>>>,
    out_folder: Arc<Mutex<String>>,
}

impl LanPeer {
    pub fn new(handle: AppHandle) -> Self {
        Self {
            handle,
            running: Arc::new(Mutex::new(false)),
            peer_last_seen: Arc::new(Mutex::new(None)),
            code_hash: Arc::new(Mutex::new(Vec::new())),
            out_folder: Arc::new(Mutex::new(String::new())),
        }
    }

    pub async fn start(&self, code: &str, out_folder: &str) -> Result<(), String> {
        use sha2::{Sha256, Digest};

        let mut running = self.running.lock().await;
        if *running {
            return Ok(());
        }

        let hash = Sha256::digest(code.as_bytes())[..16].to_vec();
        *self.code_hash.lock().await = hash.clone();
        *self.out_folder.lock().await = out_folder.to_string();
        // Clear stale peer info
        *self.peer_last_seen.lock().await = None;
        *running = true;
        drop(running);

        let handle = self.handle.clone();
        let running = self.running.clone();
        let peer_last_seen = self.peer_last_seen.clone();
        let code_hash = self.code_hash.clone();
        let out_folder_arc = self.out_folder.clone();

        // Spawn discovery loop (beacons + TCP listener)
        tokio::spawn(async move {
            discovery::run_discovery(handle, running, peer_last_seen, code_hash, out_folder_arc).await;
        });

        Ok(())
    }

    pub async fn set_out_folder(&self, folder: &str) {
        *self.out_folder.lock().await = folder.to_string();
    }

    pub async fn stop(&self) {
        let mut running = self.running.lock().await;
        *running = false;
        drop(running);

        *self.peer_last_seen.lock().await = None;
    }

    pub async fn send_text(&self, text: &str) -> Result<bool, String> {
        let peer_ip = {
            let guard = self.peer_last_seen.lock().await;
            guard.as_ref().map(|(ip, _)| ip.clone())
        };
        if let Some(ip) = peer_ip {
            let hash = self.code_hash.lock().await.clone();
            transfer::send_text_to_peer(&ip, &hash, text).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn send_files(&self, paths: &[String]) -> Result<bool, String> {
        let peer_ip = {
            let guard = self.peer_last_seen.lock().await;
            guard.as_ref().map(|(ip, _)| ip.clone())
        };
        if let Some(ip) = peer_ip {
            let hash = self.code_hash.lock().await.clone();
            transfer::send_files_to_peer(&ip, &hash, paths, Some(&self.handle)).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    #[allow(dead_code)]
    pub async fn is_connected(&self) -> bool {
        let guard = self.peer_last_seen.lock().await;
        if let Some((_, last_seen)) = guard.as_ref() {
            last_seen.elapsed().as_secs() < 10
        } else {
            false
        }
    }
}

pub struct LanState {
    pub peer: Mutex<LanPeer>,
}

impl LanState {
    pub fn new(handle: AppHandle) -> Self {
        Self {
            peer: Mutex::new(LanPeer::new(handle)),
        }
    }
}
