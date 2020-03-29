use std::net::IpAddr;

pub const ports: [u16; 9] = [
    6881, 6882, 6883, 884, 6885, 6886, 6887, 6888, 6889
];

enum TrackerEvent {
    Started,
    Completed,
    Stopped,
    Empty,
}

pub struct TrackerRequst {
    pub info_hash: [u8; 20],
    pub peer_id: [u8; 20],
    pub ip: IpAddr,
    pub port: u16,
    pub uploaded: usize,
    pub downloaded: usize,
    pub left: usize,
    pub event: TrackerEvent,
}

pub struct Peer {
    pub peer_id: [u8; 20],
    pub  ip: IpAddr,
    pub port: u16,
}

pub struct TrackerResponse {
    pub failure_reason: String,
    pub interval: usize,
    pub peers: Vec<Peer>,
}