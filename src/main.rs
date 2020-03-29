use std::fs::File;
use std::io::{BufReader, Read};

use bencode::FromBencode;
use serde_bencode::de;
use serde_bytes::ByteBuf;
use serde_derive::Deserialize;

fn main() {

    // let s = TorrentFile { announce: "http://bttracker.debian.org:6969/announce".to_string(), info: Info { name: "debian-10.3.0-amd64-netinst.iso".to_string(), piece_length: 262144, length: 351272960, pieces } };
    let mut file = File::open("debian-10.3.0-amd64-netinst.iso.torrent").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut vec = Vec::<u8>::new();
    buf_reader.read_to_end(&mut vec);
    // let vec_clone = vec.clone();

    // stream_parse(vec);

    // let bencode: bencode::Bencode = bencode::from_vec(vec_clone).unwrap();
    // let result: TorrentFile = FromBencode::from_bencode(&bencode).unwrap();
    // assert_eq!(s, result);
}
