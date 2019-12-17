use bytes::{Bytes, BytesMut};

pub const HEADER_LEN_BYTES: usize = 24;

#[allow(dead_code)]
#[repr(u8)]
pub enum Opcode {
    Get = 0x00,
    Set = 0x01,
    Add = 0x02,
    Replace = 0x03,
    Delete = 0x04,
    Increment = 0x05,
    Decrement = 0x06,
    Quit = 0x07,
    Flush = 0x08,
    GetQ = 0x09,
    Noop = 0x0a,
    Version = 0x0b,
    GetK = 0x0c,
    GetKQ = 0x0d,
    Append = 0x0e,
    Prepend = 0x0f,
    Stat = 0x10,
    SetQ = 0x11,
    AddQ = 0x12,
    ReplaceQ = 0x13,
    DeleteQ = 0x14,
    IncrementQ = 0x15,
    DecrementQ = 0x16,
    QuitQ = 0x17,
    FlushQ = 0x18,
    AppendQ = 0x19,
    PrependQ = 0x1a,
    Verbosity = 0x1b,
    Touch = 0x1c,
    GAT = 0x1d,
    GATQ = 0x1e,
    SaslListMechs = 0x20,
    SaslAuth = 0x21,
    SaslStep = 0x22,
    RGet = 0x30,
    RSet = 0x31,
    RSetQ = 0x32,
    RAppend = 0x33,
    RAppendQ = 0x34,
    RPrepend = 0x35,
    RPrependQ = 0x36,
    RDelete = 0x37,
    RDeleteQ = 0x38,
    RIncr = 0x39,
    RIncrQ = 0x3a,
    RDecr = 0x3b,
    RDecrQ = 0x3c,
    SetVbucket = 0x3d,
    GetVbucket = 0x3e,
    DelVbucket = 0x3f,
    TapConnect = 0x40,
    TapMutation = 0x41,
    TapDelete = 0x42,
    TapFlush = 0x43,
    TapOpaque = 0x44,
    TapVbucketSet = 0x45,
    TapCheckpointStart = 0x46,
    TapCheckpointEnd = 0x47,
}

#[repr(u8)]
#[derive(FromPrimitive, Debug)]
pub enum Magic {
    Request = 0x80,
    Response = 0x81,
}

#[allow(dead_code)]
#[repr(u16)]
pub enum ResponseStatus {
    NoError = 0x00,
    KeyNotFound = 0x01,
    KeyExists = 0x02,
    ValueTooLarge = 0x03,
    InvalidArguments = 0x04,
    ItemNotStored = 0x05,
    IncrDecrOnNonNumericValue = 0x06,
    TheVbucketBelongsToAnotherServer = 0x07,
    AuthenticationError = 0x08,
    AuthenticationContinue = 0x09,
    UnknownCommand = 0x81,
    OutOfMemory = 0x82,
    NotSupported = 0x83,
    InternalError = 0x84,
    Busy = 0x85,
    TemporaryFailure = 0x86,
}

#[derive(Debug, Default, Clone)]
pub struct PacketHeader {
    pub magic: u8,
    pub opcode: u8,
    pub key_length: u16,
    pub extras_length: u8,
    pub data_type: u8,
    pub vbucket_id_or_status: u16,
    pub total_body_length: u32,
    pub opaque: u32,
    pub cas: u64,
    pub extras: Bytes,
    pub key: Bytes,
    pub payload: Bytes,
}

#[derive(Debug)]
pub struct StoreExtras {
    pub flags: u32,
    pub expiration: u32,
}

#[derive(Debug)]
pub struct CounterExtras {
    pub amount: u64,
    pub initial_value: u64,
    pub expiration: u32,
}
