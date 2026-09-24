mod request_builder;

use http_request::*;

use std::{
    sync::{Arc, Mutex},
    thread::{JoinHandle, spawn},
    time::{Duration, Instant},
};
