use super::*;

pub(crate) struct TaskPanicHook {
    pub(crate) response_body: String,
    pub(crate) content_type: String,
}

pub(crate) struct RequestErrorHook {
    pub(crate) response_status_code: ResponseStatusCode,
    pub(crate) response_body: String,
}

pub(crate) struct RequestMiddleware {
    pub(crate) socket_addr: String,
}

pub(crate) struct UpgradeHook;

pub(crate) struct ConnectedHook {
    pub(crate) receiver_count: ReceiverCount,
    pub(crate) data: String,
    pub(crate) group_broadcast_type: BroadcastType<String>,
    pub(crate) private_broadcast_type: BroadcastType<String>,
}

pub(crate) struct SendedHook {
    pub(crate) msg: String,
}

pub(crate) struct GroupChatRequestHook {
    pub(crate) body: RequestBody,
    pub(crate) receiver_count: ReceiverCount,
}

pub(crate) struct GroupClosedHook {
    pub(crate) body: String,
    pub(crate) receiver_count: ReceiverCount,
}

pub(crate) struct GroupChat;

pub(crate) struct PrivateChatRequestHook {
    pub(crate) body: RequestBody,
    pub(crate) receiver_count: ReceiverCount,
}

pub(crate) struct PrivateClosedHook {
    pub(crate) body: String,
    pub(crate) receiver_count: ReceiverCount,
}

pub(crate) struct PrivateChat;
