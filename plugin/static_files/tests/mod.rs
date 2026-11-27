mod static_files;

use super::*;

use std::{
    collections::VecDeque,
    env::temp_dir,
    fs::{create_dir_all, remove_dir_all, write},
    process,
};

use hyperlane::{Method, tokio};
