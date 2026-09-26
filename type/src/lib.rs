//! http-type
//!
//! A comprehensive Rust type library for HTTP operations and concurrent programming.

mod any;
mod arc_mutex;
mod arc_rwlock;
mod attribute;
mod box_leak;
mod box_rwlock;
mod content_type;
mod cookie;
mod file_extension;
mod hash_map_xx_hash3_64;
mod hash_set_xx_hash3_64;
mod http_status;
mod http_url;
mod http_version;
mod lifetime;
mod methods;
mod panic;
mod protocol;
mod rc_rwlock;
mod request;
mod response;
mod status;
mod stream;
mod task;
mod upgrade_type;
mod websocket_frame;

pub use {
    any::*, arc_mutex::*, arc_rwlock::*, attribute::*, box_leak::*, box_rwlock::*, content_type::*,
    cookie::*, file_extension::*, hash_map_xx_hash3_64::*, hash_set_xx_hash3_64::*, http_status::*,
    http_url::*, http_version::*, lifetime::*, methods::*, panic::*, protocol::*, rc_rwlock::*,
    request::*, response::*, status::*, stream::*, task::*, upgrade_type::*, websocket_frame::*,
};

pub use {http_compress::*, http_constant::*, serde_json, tokio};

use std::{
    any::Any,
    collections::{HashMap, HashSet, VecDeque},
    fmt::{self, Debug, Display, Formatter},
    hash::Hash,
    io::ErrorKind,
    net::IpAddr,
    num::ParseIntError,
    pin::Pin,
    rc::Rc,
    result::Result,
    str::{FromStr, SplitWhitespace},
    sync::{
        Arc,
        atomic::{self, AtomicBool, AtomicUsize},
    },
    time::Duration,
};

use {
    core::hash::BuildHasherDefault,
    lombok_macros::*,
    serde::{Deserialize, Serialize, de::DeserializeOwned},
    tokio::{
        io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
        net::TcpStream,
        runtime::Handle,
        sync::{
            Mutex, Notify, RwLock, RwLockReadGuard, RwLockWriteGuard,
            mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel},
        },
        task::{JoinError, LocalSet, spawn_blocking, spawn_local},
        time::{error::Elapsed, timeout},
    },
    url::{ParseError, Url},
};
