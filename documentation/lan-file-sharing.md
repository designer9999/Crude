# LAN File Sharing Architecture

LanDrop uses local-only discovery and on-demand TCP transfer. It must keep
working when multicast is unreliable, when Windows has virtual adapters, and
when VPN software such as Mullvad owns the default route.

## Core Rules

- Only same-LAN IPv4 peers are accepted. With a local IP of `192.168.0.98`,
  valid peers must be on `192.168.0.0/24`; VPN, WSL, Hyper-V, Docker,
  Bluetooth, link-local, loopback, and other virtual addresses are ignored.
- mDNS is treated as a hint, not the source of truth. The app verifies peers
  through the TCP UUID handshake before using a recovered IP.
- Sending is on-demand. The TCP connection opens for one text/file transfer and
  closes after `Done`.
- The app must never get stuck on a stale peer IP. If a stored or UI-provided IP
  fails, the service scans the local `/24` and recovers the peer by UUID.

## Lessons From Mature LAN Apps

- LocalSend combines multicast discovery with an HTTP registration scan across
  local IPs when multicast is unsuccessful. LanDrop follows the same principle:
  mDNS is fast, but active same-LAN probing is the reliability fallback.
- KDE Connect uses UDP discovery plus TCP connections and documents that
  discovery can fail when UDP broadcast is blocked. Its issue tracker also shows
  VPN/default-route bugs where discovery packets go out the wrong interface.
- Warpinator uses zeroconf/mDNS for discovery and a direct TCP/gRPC transfer
  channel. It treats the local network and firewall surface as explicit runtime
  behavior, not incidental OS behavior.

## Runtime Flow

```
start_lan_service
  -> choose local LAN IPv4
     - prefer physical private LAN adapters
     - reject VPN and virtual adapter names
  -> register _landrop._tcp.local. with the chosen LAN IP
  -> browse mDNS for peers
  -> listen on TCP 0.0.0.0:29171
  -> run periodic same-LAN TCP healer scan
```

## Peer Discovery

### mDNS

The app registers `_landrop._tcp.local.` with:

- `id`: stable device UUID
- `alias`: user-visible device name
- `dtype`: device type
- TCP port: `29171`

When a peer resolves, the app selects only an IPv4 address from the same `/24`
as the current local LAN IP. If a peer advertises multiple IPv4 addresses, the
app logs them and stores the same-LAN address.

### Same-LAN Healer Scan

Every 30 seconds, and also after send candidates fail, LanDrop probes the local
`/24` subnet on TCP `29171`.

Each probe performs only the LanDrop UUID handshake:

1. Connect to `peer_ip:29171`.
2. Send this device's 16-byte UUID.
3. Read the remote 16-byte UUID.
4. If the UUID matches the target peer, store that IP and emit discovery.

This makes send-back recover even when mDNS disappears, when the peer changed
IP, or when a VPN adapter polluted discovery.

## Send Path

`send_text` and `send_files` build candidate IPs from backend discovery plus the
UI hint. Before attempting transfer, each candidate must pass
`is_current_lan_peer_ip`.

If all candidates fail:

1. Scan the local `/24`.
2. Recover the peer by UUID.
3. Retry the transfer using the recovered IP.
4. Remember the working IP for future sends.

## Receive Path

Incoming TCP sessions are rejected unless the remote address is same-LAN IPv4.
This prevents VPN, WSL, Hyper-V, and other non-LAN interfaces from rehydrating a
peer with a bad send-back address.

After the incoming UUID handshake succeeds, the sender is registered or updated
in `discovered_peers`, so a device that can send to us can also be sent back to
using the exact same LAN source address.

## Firewall Surface

- TCP `29171`: direct text/file transfer and UUID probes.
- mDNS UDP `5353`: zeroconf discovery used by the `mdns-sd` crate.

Windows installer hooks add inbound TCP/UDP rules for the installed
`landrop.exe`. If running a different executable path during development,
Windows may need a separate firewall allow rule for that path.

## What Not To Do

- Do not trust the first IPv4 address from mDNS.
- Do not accept `10.x`, `172.x`, or `192.168.x` blindly; same subnet matters.
- Do not persist a failed VPN or virtual adapter IP as the peer's working IP.
- Do not rely on mDNS as the only discovery mechanism.
- Do not keep persistent TCP connections or ping/pong keepalives for transfers.
