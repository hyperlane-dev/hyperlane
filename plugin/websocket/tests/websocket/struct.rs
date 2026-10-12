use super::*;

pub(crate) struct TaskPanicHook {
    pub(super) response_body: String,
    pub(super) content_type: String,
}

pub(crate) struct RequestErrorHook {
    pub(super) response_status_code: ResponseStatusCode,
    pub(super) response_body: String,
}

pub(crate) struct RequestMiddleware {
    pub(crate) socket_addr: String,
}

pub(crate) struct UpgradeHook;

pub(crate) struct ConnectedHook {
    pub(super) receiver_count: ReceiverCount,
    pub(super) data: String,
    pub(super) group_broadcast_type: BroadcastType<String>,
    pub(super) private_broadcast_type: BroadcastType<String>,
}

pub(crate) struct SendedHook {
    pub(super) msg: String,
}

pub(crate) struct GroupChatRequestHook {
    pub(super) body: RequestBody,
    pub(super) receiver_count: ReceiverCount,
}

pub(crate) struct GroupClosedHook {
    pub(super) body: String,
    pub(super) receiver_count: ReceiverCount,
}

pub(crate) struct GroupChat;

pub(crate) struct PrivateChatRequestHook {
    pub(super) body: RequestBody,
    pub(super) receiver_count: ReceiverCount,
}

pub(crate) struct PrivateClosedHook {
    pub(super) body: String,
    pub(super) receiver_count: ReceiverCount,
}

pub(crate) struct PrivateChat;
