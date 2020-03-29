use futures::{channel::mpsc, lock::Mutex, prelude::*};
use redshirt_core::native::{DummyMessageIdWrite, NativeProgramEvent, NativeProgramRef};
use redshirt_core::{Decode as _, Encode as _, EncodedMessage, InterfaceHash, MessageId, Pid};
use redshirt_pipeline_interface::ffi::{PipelineMessage, PipelineResponse, INTERFACE};
use std::{borrow::Cow, pin::Pin, sync::atomic};

pub struct PipeHandler {
    /// If true, we have sent the interface registration message.
    registered: atomic::AtomicBool,
    /// Message responses waiting to be emitted.
    pending_messages_rx: Mutex<mpsc::UnboundedReceiver<(MessageId, Result<EncodedMessage, ()>)>>,
    /// Sending side of `pending_messages_rx`.
    pending_messages_tx: mpsc::UnboundedSender<(MessageId, Result<EncodedMessage, ()>)>,
}

impl PipeHandler {
    /// Initializes the new state machine for logging.
    pub fn new() -> Self {
        let (pending_messages_tx, pending_messages_rx) = mpsc::unbounded();

        PipeHandler {
            registered: atomic::AtomicBool::new(false),
            pending_messages_tx,
            pending_messages_rx: Mutex::new(pending_messages_rx),
        }
    }
}

impl<'a> NativeProgramRef<'a> for &'a PipeHandler {
    type Future =
        Pin<Box<dyn Future<Output = NativeProgramEvent<Self::MessageIdWrite>> + Send + 'a>>;
    type MessageIdWrite = DummyMessageIdWrite;

    fn next_event(self) -> Self::Future {
        if !self.registered.swap(true, atomic::Ordering::Relaxed) {
            return Box::pin(future::ready(NativeProgramEvent::Emit {
                interface: redshirt_interface_interface::ffi::INTERFACE,
                message_id_write: None,
                message: redshirt_interface_interface::ffi::InterfaceMessage::Register(INTERFACE)
                    .encode(),
            }));
        }

        Box::pin(async move {
            let mut pending_messages_rx = self.pending_messages_rx.lock().await;
            let (message_id, answer) = pending_messages_rx.next().await.unwrap();
            NativeProgramEvent::Answer { message_id, answer }
        })
    }

    fn interface_message(
        self,
        interface: InterfaceHash,
        message_id: Option<MessageId>,
        _emitter_pid: Pid,
        message: EncodedMessage,
    ) {
        debug_assert_eq!(interface, INTERFACE);

        let message_id = match message_id {
            Some(m) => m,
            None => return,
        };

        match PipelineMessage::decode(message) {
            Ok(decoded) => {
                let data: Vec<u8> = vec!(2,4);
                let response = PipelineResponse { result: Ok(data) };
                self.pending_messages_tx
                    .unbounded_send((message_id, Ok(response.encode())))
                    .unwrap();
            }
            Err(_) => self
                .pending_messages_tx
                .unbounded_send((message_id, Err(())))
                .unwrap(),
        }
    }

    fn process_destroyed(self, _: Pid) {}

    fn message_response(self, _: MessageId, _: Result<EncodedMessage, ()>) {
        unreachable!()
    }
}
