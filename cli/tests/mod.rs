mod command;
mod config;
mod help;
mod logger;
mod new;
mod template;
mod version;

use super::*;

use std::{env::args, io};

use log::{Level, LevelFilter, Log, Metadata, max_level, set_max_level};
