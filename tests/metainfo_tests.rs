#[cfg(test)]
mod tests {
    extern crate my_torrent_client_project;

    use std::io::Read;

    use bencode::FromBencode;

    pub use my_torrent_client_project::metainfo_files::*;

    const file_path: &str = "debian-10.3.0-amd64-netinst.iso.torrent";

    #[test]
    fn test_open_file() {
        let reader = open_file(file_path);
        assert!(reader.is_ok());
    }

    #[test]
    fn test_info_dict_from_bencode() {
        let test_info = Info {
            name: "debian-10.3.0-amd64-netinst.iso".to_string(),
            length: 351272960,
            piece_length: 262144,
            pieces: "torrent".as_bytes().to_vec(),
        };
        let info = "d6:lengthi351272960e4:name31:debian-10.3.0-amd64-netinst.iso12:piece lengthi262144e6:pieces7:torrente";
        let mut buf = info.as_bytes().to_vec();
        let bencode: bencode::Bencode = match bencode::from_vec(buf) {
            Ok(bc) => bc,
            Err(e) => panic!()
        };
        let result: Info = match FromBencode::from_bencode(&bencode) {
            Ok(i) => i,
            Err(e) => panic!(e)
        };
        assert_eq!(result, test_info);
    }

    #[test]
    fn test_torrent_from_bencode() {
        let test_torrent = Torrent {
            announce: "http://bttracker.debian.org:6969/announce".to_string(),
            info: Info {
                name: "debian-10.3.0-amd64-netinst.iso".to_string(),
                length: 351272960,
                piece_length: 262144,
                pieces: "torrent".as_bytes().to_vec(),
            },
        };
        let torrent = "d8:announce41:http://bttracker.debian.org:6969/announce4:infod6:lengthi351272960e4:name31:debian-10.3.0-amd64-netinst.iso12:piece lengthi262144e6:pieces7:torrentee";
        let mut buf = torrent.as_bytes().to_vec();
        let bencode: bencode::Bencode = match bencode::from_vec(buf) {
            Ok(bc) => bc,
            Err(e) => panic!(e)
        };
        let result: Torrent = match FromBencode::from_bencode(&bencode) {
            Ok(t) => t,
            Err(e) => panic!(e)
        };
        assert_eq!(result, test_torrent);
    }

    #[test]
    fn test_data_from_bencode() {
        let mut reader = open_file(file_path).unwrap();
        let mut buf = Vec::<u8>::new();
        reader.read_to_end(&mut buf);
        let bencode: bencode::Bencode = match bencode::from_vec(buf) {
            Ok(bc) => bc,
            Err(e) => panic!(e)
        };
        let result: Result<Torrent, TorrentFromBencodeError> = FromBencode::from_bencode(&bencode);
        assert!(result.is_ok());
    }

    #[test]
    fn test_split_piece_hashes() {
        let input = vec![3u8; 80];
        match split_piece_hashes(input) {
            Ok(res) => assert_eq!(res.len(), 4);
            Err(_) => panic!()
        }
    }

    #[test]
    fn test_split_piece_hashes_invalid() {
        let input = vec![4u8; 78];
        let res = split_piece_hashes(input);
        assert!(res.is_err());
    }
}