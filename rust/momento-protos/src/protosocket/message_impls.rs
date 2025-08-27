use crate::protosocket::cache::{CacheCommand, CacheResponse};
use crate::protosocket::common::CommandError;
use protosocket_rpc::ProtosocketControlCode;
use std::fmt::{Display, Formatter};

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

impl protosocket_rpc::Message for CacheResponse {
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

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CommandError {}: {}",
            self.code().as_str_name(),
            self.message
        )
    }
}

impl std::error::Error for CommandError {}
