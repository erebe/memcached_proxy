//use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use bytes::buf::BufExt;
use bytes::{Buf, BufMut, BytesMut};
use std::error::Error;
use std::io;
use tokio_util::codec::{Decoder, Encoder, Framed};

pub struct MemcachedBinaryCodec {}

use super::protocol::memcached_binary::PacketHeader;
use crate::protocol::memcached_binary::{Magic, Opcode, HEADER_LEN_BYTES};
use core::fmt;

impl MemcachedBinaryCodec {
    pub fn new() -> MemcachedBinaryCodec {
        MemcachedBinaryCodec {}
    }
}
impl Decoder for MemcachedBinaryCodec {
    type Item = PacketHeader;
    type Error = MemcachedBinaryCodecError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<PacketHeader>, MemcachedBinaryCodecError> {
        if src.len() < HEADER_LEN_BYTES {
            return Ok(None);
        }

        // Extracting total body length field to known if we have enough bytes to parse the packet
        let total_body_length = unsafe { std::mem::transmute::<&u8, &u32>(src.get_unchecked(8)) }.to_be();

        // Not enough bytes to get the full request
        if src.len() < HEADER_LEN_BYTES + total_body_length as usize {
            return Ok(None);
        }

        //Header
        let magic = src.get_u8();
        let opcode = src.get_u8();
        let key_length = src.get_u16();
        let extras_length = src.get_u8();
        let data_type = src.get_u8();
        let vbucket_id_or_status = src.get_u16();
        let total_body_length = src.get_u32();
        let opaque = src.get_u32();
        let cas = src.get_u64();

        // Check header sanity
        if let None = num::FromPrimitive::from_u8(magic) as Option<Magic> {
            return Err(MemcachedBinaryCodecError::InvalidHeader(format!(
                "Invalid magic code {}",
                magic
            )));
        }
        if let None = num::FromPrimitive::from_u8(opcode) as Option<Opcode> {
            return Err(MemcachedBinaryCodecError::InvalidHeader(format!(
                "Invalid opcode {}",
                opcode
            )));
        }

        // Extra body of the request
        let extras = src.split_to(extras_length as usize).freeze();
        let key = src.split_to(key_length as usize).freeze();
        let payload = src
            .split_to(total_body_length as usize - extras_length as usize - key_length as usize)
            .freeze();

        Ok(Some(PacketHeader {
            magic,
            opcode,
            key_length,
            extras_length,
            data_type,
            vbucket_id_or_status,
            total_body_length,
            opaque,
            cas,
            extras,
            key,
            payload,
        }))
    }
}

impl Encoder for MemcachedBinaryCodec {
    type Item = PacketHeader;
    type Error = MemcachedBinaryCodecError;

    fn encode(&mut self, item: PacketHeader, dst: &mut BytesMut) -> Result<(), MemcachedBinaryCodecError> {
        dst.reserve(HEADER_LEN_BYTES + item.total_body_length as usize);
        dst.put_u8(item.magic);
        dst.put_u8(item.opcode);
        dst.put_u16(item.key_length);
        dst.put_u8(item.extras_length);
        dst.put_u8(item.data_type);
        dst.put_u16(item.vbucket_id_or_status);
        dst.put_u32(item.total_body_length);
        dst.put_u32(item.opaque);
        dst.put_u64(item.cas);
        if item.extras_length > 0 {
            dst.put(item.extras);
        }
        if item.key_length > 0 {
            dst.put(item.key);
        }
        if item.total_body_length - item.key_length as u32 - item.extras_length as u32 > 0 {
            dst.put(item.payload);
        }

        Ok(())
    }
}

/// An error occured while encoding or decoding a line.
#[derive(Debug)]
pub enum MemcachedBinaryCodecError {
    /// The maximum line length was exceeded.
    InvalidHeader(String),
    /// An IO error occured.
    Io(io::Error),
}

impl fmt::Display for MemcachedBinaryCodecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemcachedBinaryCodecError::InvalidHeader(msg) => write!(f, "{}", msg),
            MemcachedBinaryCodecError::Io(e) => write!(f, "{}", e),
        }
    }
}

impl From<io::Error> for MemcachedBinaryCodecError {
    fn from(e: io::Error) -> MemcachedBinaryCodecError {
        MemcachedBinaryCodecError::Io(e)
    }
}

impl std::error::Error for MemcachedBinaryCodecError {}
