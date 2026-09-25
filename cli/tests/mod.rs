mod bump;
mod config;
mod fmt;
mod new;
mod publish;
mod sync;
mod version;

use hyperlane_cli::*;

use std::{
    env::temp_dir,
    io::{self, Error},
    path::{Path, PathBuf},
};

use tokio::fs::{self, create_dir_all, read_to_string, write};
