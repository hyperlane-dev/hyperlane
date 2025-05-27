#!/bin/bash
RUST_BACKTRACE=1 cargo test --release --verbose -- --nocapture
