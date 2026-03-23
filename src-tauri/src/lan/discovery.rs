use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::sync::Mutex;
use tokio::time;
use tauri::{AppHandle, Emitter};

use super::protocol::{BEACON_MAGIC, BEACON_PORT, TCP_PORT};
use super::transfer::Connection;

/// How long without a beacon before we consider the peer gone.
/// 30s with 2s beacon interval = 15 missed beacons — tolerant of UDP loss and transfer bursts.
const STALE_TIMEOUT: Duration = Duration::from_secs(30);

pub async fn run_discovery(
    handle: AppHandle,
    running: Arc<AtomicBool>,
    peer_last_seen: Arc<Mutex<Option<(String, Instant)>>>,
    code_hash: Arc<Mutex<Vec<u8>>>,
    out_folder: Arc<Mutex<String>>,
) {
    // Bind UDP socket for beacon
    let udp = match UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, BEACON_PORT)).await {
        Ok(s) => {
            let _ = s.set_broadcast(true);
            Arc::new(s)
        }
        Err(e) => {
            eprintln!("Failed to bind UDP beacon: {}", e);
            return;
        }
    };

    // Bind TCP listener for incoming transfers
    let tcp_listener = match TcpListener::bind(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, TCP_PORT)).await {
        Ok(l) => Arc::new(l),
        Err(e) => {
            eprintln!("Failed to bind TCP: {}", e);
            return;
        }
    };

    let local_ips = get_local_ips();
    let subnet_broadcasts = get_subnet_broadcasts();

    // ── Task 1: Beacon sender (every 2s) ──
    let udp_send = udp.clone();
    let running_send = running.clone();
    let code_hash_send = code_hash.clone();
    let beacon_sender = tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(2));
        loop {
            interval.tick().await;
            if !running_send.load(Ordering::Relaxed) {
                break;
            }
            let hash = code_hash_send.lock().await.clone();
            if hash.is_empty() {
                continue;
            }
            let mut beacon = Vec::with_capacity(24);
            beacon.extend_from_slice(BEACON_MAGIC);
            beacon.extend_from_slice(&hash);

            // Send to limited broadcast
            let broadcast_addr = SocketAddrV4::new(Ipv4Addr::BROADCAST, BEACON_PORT);
            let _ = udp_send.send_to(&beacon, broadcast_addr).await;

            // Also send to each subnet-directed broadcast (e.g. 192.168.1.255)
            for addr in &subnet_broadcasts {
                let _ = udp_send.send_to(&beacon, SocketAddrV4::new(*addr, BEACON_PORT)).await;
            }
        }
    });

    // ── Task 2: Beacon listener — tracks peer availability ──
    let udp_listen = udp.clone();
    let running_recv = running.clone();
    let code_hash_recv = code_hash.clone();
    let peer_last_seen_recv = peer_last_seen.clone();
    let handle_recv = handle.clone();
    let local_ips_recv = local_ips.clone();
    let beacon_listener = tokio::spawn(async move {
        let mut buf = [0u8; 128];
        loop {
            if !running_recv.load(Ordering::Relaxed) {
                break;
            }
            let recv = time::timeout(Duration::from_secs(1), udp_listen.recv_from(&mut buf)).await;
            if let Ok(Ok((len, addr))) = recv {
                if len < 24 {
                    continue;
                }
                // Ignore our own beacons
                if local_ips_recv.contains(&addr.ip().to_string()) {
                    continue;
                }
                if &buf[..8] != BEACON_MAGIC {
                    continue;
                }
                let peer_hash = &buf[8..24];
                let our_hash = code_hash_recv.lock().await;
                if peer_hash != our_hash.as_slice() {
                    continue;
                }
                drop(our_hash);

                let peer_ip = addr.ip().to_string();
                let mut guard = peer_last_seen_recv.lock().await;
                let was_available = guard.is_some();
                *guard = Some((peer_ip.clone(), Instant::now()));
                drop(guard);

                if !was_available {
                    let _ = handle_recv.emit(
                        "lan_peer_available",
                        serde_json::json!({"peer_ip": peer_ip}),
                    );
                }
            }
        }
    });

    // ── Task 3: Availability checker — emits unavailable when beacons stop ──
    let running_avail = running.clone();
    let peer_last_seen_avail = peer_last_seen.clone();
    let handle_avail = handle.clone();
    let availability_checker = tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(5));
        loop {
            interval.tick().await;
            if !running_avail.load(Ordering::Relaxed) {
                break;
            }
            let mut guard = peer_last_seen_avail.lock().await;
            if let Some((_, last_seen)) = guard.as_ref() {
                if last_seen.elapsed() > STALE_TIMEOUT {
                    *guard = None;
                    drop(guard);
                    let _ = handle_avail.emit("lan_peer_unavailable", serde_json::json!({}));
                }
            }
        }
    });

    // ── Task 4: TCP listener — accepts one-shot incoming transfers ──
    let running_tcp = running.clone();
    let code_hash_tcp = code_hash.clone();
    let handle_tcp = handle.clone();
    let out_folder_tcp = out_folder.clone();
    let peer_last_seen_tcp = peer_last_seen.clone();
    let tcp_acceptor = tokio::spawn(async move {
        loop {
            if !running_tcp.load(Ordering::Relaxed) {
                break;
            }
            let accept = time::timeout(Duration::from_secs(1), tcp_listener.accept()).await;
            if let Ok(Ok((stream, addr))) = accept {
                let hash = code_hash_tcp.lock().await.clone();
                let handle_session = handle_tcp.clone();
                let folder = out_folder_tcp.lock().await.clone();
                let peer_ls = peer_last_seen_tcp.clone();

                // Spawn a task per incoming session — non-blocking
                tokio::spawn(async move {
                    let peer_ip = addr.ip().to_string();
                    // A successful incoming TCP connection proves peer is alive
                    {
                        let mut guard = peer_ls.lock().await;
                        let was_available = guard.is_some();
                        *guard = Some((peer_ip.clone(), Instant::now()));
                        drop(guard);
                        if !was_available {
                            let _ = handle_session.emit(
                                "lan_peer_available",
                                serde_json::json!({"peer_ip": &peer_ip}),
                            );
                        }
                    }
                    if let Err(e) = handle_incoming_session(stream, &hash, &folder, &handle_session).await {
                        eprintln!("Incoming session error: {}", e);
                    }
                    // Refresh after transfer completes too
                    let mut guard = peer_ls.lock().await;
                    *guard = Some((peer_ip, Instant::now()));
                });
            }
        }
    });

    let _ = tokio::join!(beacon_sender, beacon_listener, availability_checker, tcp_acceptor);
}

/// Handle a single incoming TCP session: authenticate, receive messages, close.
async fn handle_incoming_session(
    stream: TcpStream,
    expected_hash: &[u8],
    out_folder: &str,
    handle: &AppHandle,
) -> Result<(), String> {
    let conn = Connection::from_incoming(stream, expected_hash).await?;

    // Read messages until the connection closes
    loop {
        let msg = match conn.recv_message().await {
            Ok(msg) => msg,
            Err(_) => break, // Connection closed — session done
        };

        match msg {
            super::protocol::Message::Text { text } => {
                let _ = handle.emit(
                    "lan_text_received",
                    serde_json::json!({"text": text}),
                );
            }
            super::protocol::Message::File { name, size } => {
                match super::transfer::receive_file(&conn, &name, size, out_folder, Some(handle)).await {
                    Ok(path) => {
                        let _ = handle.emit(
                            "lan_files_received",
                            serde_json::json!({
                                "files": [&name],
                                "file_details": [{
                                    "name": &name,
                                    "path": &path,
                                    "size": size,
                                }]
                            }),
                        );
                    }
                    Err(e) => eprintln!("File receive error: {}", e),
                }
            }
            super::protocol::Message::Batch { count } => {
                match super::transfer::receive_batch(&conn, count, out_folder, Some(handle)).await {
                    Ok(files) => {
                        let names: Vec<&str> = files.iter().map(|(n, _, _)| n.as_str()).collect();
                        let details: Vec<serde_json::Value> = files
                            .iter()
                            .map(|(name, path, size)| {
                                serde_json::json!({
                                    "name": name,
                                    "path": path,
                                    "size": size,
                                })
                            })
                            .collect();
                        let _ = handle.emit(
                            "lan_files_received",
                            serde_json::json!({
                                "files": names,
                                "file_details": details,
                            }),
                        );
                    }
                    Err(e) => eprintln!("Batch receive error: {}", e),
                }
            }
            super::protocol::Message::Done => break,
            _ => {}
        }
    }

    Ok(())
}

fn get_local_ips() -> Vec<String> {
    local_ip_address::list_afinet_netifas()
        .unwrap_or_default()
        .into_iter()
        .filter(|(_, ip)| ip.is_ipv4())
        .map(|(_, ip)| ip.to_string())
        .collect()
}

/// Compute subnet-directed broadcast addresses (e.g. 192.168.1.255 for /24)
fn get_subnet_broadcasts() -> Vec<Ipv4Addr> {
    let mut addrs = Vec::new();
    let interfaces = local_ip_address::list_afinet_netifas().unwrap_or_default();
    for (_, ip) in &interfaces {
        if let std::net::IpAddr::V4(v4) = ip {
            if v4.is_loopback() { continue; }
            let octets = v4.octets();
            let broadcast = Ipv4Addr::new(octets[0], octets[1], octets[2], 255);
            if !addrs.contains(&broadcast) {
                addrs.push(broadcast);
            }
        }
    }
    addrs
}
