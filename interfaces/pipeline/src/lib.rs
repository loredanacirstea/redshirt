#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use futures::prelude::*;

pub mod ffi;

pub fn run(node: ffi::PipeNode) -> impl Future<Output = Result<Vec<u8>, ()>> {
    redshirt_log_interface::log(redshirt_log_interface::Level::Info, &"pipeline run!");
    unsafe {
        // let msg = ffi::LoaderMessage::Load(hash);
        match redshirt_syscalls::emit_message_with_response(&ffi::INTERFACE, node) {
            Ok(fut) => fut.map(|rep: ffi::GenerateResponse| rep.result).left_future(),
            Err(_) => future::ready(Err(())).right_future(),
        }
    }
}
