//use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use bytes::buf::BufExt;
use bytes::{Buf, BytesMut};
use std::error::Error;
use std::io;
use tokio_util::codec::{Decoder, Encoder, Framed};

pub struct MemcachedBinaryCodec {}

use super::protocol::memcached_binary::PacketHeader;
use crate::protocol::memcached_binary::HEADER_LEN_BYTES;

impl MemcachedBinaryCodec {
    pub fn new() -> MemcachedBinaryCodec {
        MemcachedBinaryCodec {}
    }
}
impl Decoder for MemcachedBinaryCodec {
    type Item = PacketHeader;
    type Error = Box<Error>;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<PacketHeader>, Box<Error>> {
        //let request_type: Option<Magic> = num::FromPrimitive::from_u8(src.get_u8());
        if src.len() < HEADER_LEN_BYTES {
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

        // TODO: Do some checking about the protocol integrity

        // Not enough bytes to get the full request
        if src.len() < total_body_length as usize {
            return Ok(None);
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
    type Item = ();
    type Error = Box<Error>;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Box<Error>> {
        unimplemented!()
    }
}
