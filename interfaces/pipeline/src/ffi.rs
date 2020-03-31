extern crate alloc;

use alloc::vec::Vec;
use parity_scale_codec::{Decode, Encode};
use redshirt_syscalls::InterfaceHash;


// TODO: this has been randomly generated; instead should be a hash or something
pub const INTERFACE: InterfaceHash = InterfaceHash::from_raw_hash([
    0xa6, 0xbc, 0x8d, 0xc3, 0x43, 0xbd, 0xdd, 0x3b, 0x44, 0x2f, 0x06, 0x40, 0xa8, 0x40, 0xad, 0x4f,
    0x25, 0x57, 0x23, 0x91, 0x79, 0xc8, 0x16, 0x07, 0x6f, 0xab, 0xa9, 0xd6, 0x38, 0xca, 0x01, 0x8c,
]);

#[derive(Debug, Encode, Decode)]
pub struct PipelineMessage {
    /// blake3 hash passed as parameter.
    // pub module: [u8; 32],
    pub module: Vec<u8>,
    pub funcname: Vec<u8>,
    pub inputs: Vec<i32>,
}

#[derive(Debug, Encode, Decode)]
pub struct PipelineResponse {
    pub result: Result<Vec<u8>, ()>,
}
