mod websocket;

use hyperlane_plugin_websocket::*;

use std::sync::OnceLock;

use {
    hyperlane::*,
    tokio::{spawn, time::sleep},
    tokio_broadcast::*,
};
