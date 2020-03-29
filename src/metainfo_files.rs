use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read};
use std::string::{FromUtf8Error, ParseError};

use bencode::{Bencode, Decoder, FromBencode, NumFromBencodeError, StringFromBencodeError, ToBencode, VecFromBencodeError};
use bencode::Bencode::ByteString;
use bencode::MapFromBencodeError::InvalidType;
use bencode::streaming::{BencodeEvent, StreamingParser};
use rustc_serialize::{Decodable, Encodable};
use serde_derive::Deserialize;

#[derive(Debug)]
pub struct PieceHashes(pub Vec<u8>);

#[derive(PartialEq, Debug)]
pub struct Info {
    pub name: String,
    pub length: isize,
    pub piece_length: isize,
    pub pieces: Vec<u8>,
}

#[derive(PartialEq, Debug)]
pub struct Torrent {
    pub announce: String,
    pub info: Info,
}

#[derive(Debug)]
pub enum TorrentFromBencodeError {
    NotADict,
    KeyDoesNotExist(String),
    NotAString(StringFromBencodeError),
    NotANumber(NumFromBencodeError),
}

impl FromBencode for Info {
    type Err = TorrentFromBencodeError;

    fn from_bencode(bencode: &Bencode) -> Result<Info, TorrentFromBencodeError> {
        use TorrentFromBencodeError::*;
        match bencode {
            &Bencode::Dict(ref m) => {
                let name_bencode = m.get("name".as_bytes()).ok_or(KeyDoesNotExist("name".to_string()))?;
                let name = FromBencode::from_bencode(name_bencode).map_err(NotAString)?;
                let length_bencode = m.get("length".as_bytes()).ok_or(KeyDoesNotExist("length".to_string()))?;
                let length = FromBencode::from_bencode(length_bencode).map_err(NotANumber)?;
                let piece_length_bencode = m.get("piece length".as_bytes()).ok_or(KeyDoesNotExist("piece length".to_string()))?;
                let piece_length = FromBencode::from_bencode(piece_length_bencode).map_err(NotANumber)?;
                let pieces_bencode = m.get("pieces".as_bytes()).ok_or(KeyDoesNotExist("pieces".to_string()))?;
                let pieces_hashes: PieceHashes = FromBencode::from_bencode(pieces_bencode)?;
                let pieces = pieces_hashes.0;
                Ok(Info { name, length, piece_length, pieces })
            }
            _ => Err(NotADict)
        }
    }
}

impl FromBencode for PieceHashes {
    type Err = TorrentFromBencodeError;

    fn from_bencode(bencode: &Bencode) -> Result<PieceHashes, TorrentFromBencodeError> {
        use TorrentFromBencodeError::*;
        match bencode {
            &Bencode::ByteString(ref v) => {
                Ok(PieceHashes(v.clone()))
            }
            _ => Err(NotAString(StringFromBencodeError::InvalidType))
        }
    }
}

impl FromBencode for Torrent {
    type Err = TorrentFromBencodeError;

    fn from_bencode(bencode: &Bencode) -> Result<Torrent, TorrentFromBencodeError> {
        use TorrentFromBencodeError::*;
        match bencode {
            &Bencode::Dict(ref m) => {
                // TODO: maybe use pattern matching instead?
                let announce_bencode = m.get("announce".as_bytes()).ok_or(KeyDoesNotExist("announce".to_string()))?;
                let announce = FromBencode::from_bencode(announce_bencode).map_err(NotAString)?;
                let info_bencode = m.get("info".as_bytes()).ok_or(KeyDoesNotExist("info".to_string()))?;
                let info: Info = FromBencode::from_bencode(info_bencode)?;
                let info_bytes = info_bencode.to_bytes();
                // TODO: hash info_bytes and add to Torrent
                Ok(Torrent { announce, info })
            }
            _ => Err(NotADict)
        }
    }
}

#[derive(Debug)]
pub enum MetaInfoError {
    FileOpenError,
    ReadError(std::io::Error),
    MetadataError(std::io::Error),
}

pub fn open_file(path: &str) -> Result<BufReader<File>, MetaInfoError> {
    use MetaInfoError::*;
    return match File::open(path) {
        Ok(file) => Ok(BufReader::new(file)),
        Err(_) => Err(FileOpenError)
    };
}

#[derive(Debug)]
pub enum PieceHashesError {
    InvalidLength
}

pub fn split_piece_hashes(hashes: Vec<u8>) -> Result<Vec<[u8; 20]>, PieceHashesError> {
    use PieceHashesError::*;
    if hashes.len() % 20 != 0 {
        return Err(InvalidLength);
    }
    let chunks = hashes.chunks_exact(20)
        .map(|x| {
            let mut y = [0u8; 20];
            y.copy_from_slice(x);
            y
        }).collect::<Vec<[u8; 20]>>();
    Ok(chunks)
}