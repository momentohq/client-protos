use crate::protosocket::cache::{CacheCommand, CacheResponse};
use protosocket_rpc::ProtosocketControlCode;

impl protosocket_rpc::Message for CacheCommand {
    fn message_id(&self) -> u64 {
        self.message_id
    }

    fn control_code(&self) -> ProtosocketControlCode {
        ProtosocketControlCode::from_u8(self.control_code as u8)
    }

    fn set_message_id(&mut self, message_id: u64) {
        self.message_id = message_id;
    }

    fn cancelled(message_id: u64) -> Self {
        Self {
            message_id,
            control_code: ProtosocketControlCode::Cancel as u32,
            rpc_kind: None,
        }
    }

    fn ended(message_id: u64) -> Self {
        Self {
            message_id,
            control_code: ProtosocketControlCode::End as u32,
            rpc_kind: None,
        }
    }
}

impl protosocket_rpc::Message for CacheResponse
{
    fn message_id(&self) -> u64 {
        self.message_id
    }

    fn control_code(&self) -> ProtosocketControlCode {
        ProtosocketControlCode::from_u8(self.control_code as u8)
    }

    fn set_message_id(&mut self, message_id: u64) {
        self.message_id = message_id;
    }

    fn cancelled(message_id: u64) -> Self {
        Self {
            message_id,
            control_code: ProtosocketControlCode::Cancel as u32,
            kind: None,
        }
    }

    fn ended(message_id: u64) -> Self {
        Self {
            message_id,
            control_code: ProtosocketControlCode::End as u32,
            kind: None,
        }
    }
}