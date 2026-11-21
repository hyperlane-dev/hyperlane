mod r#fn;
#[cfg(target_os = "linux")]
mod r#impl;
#[cfg(target_os = "linux")]
mod r#static;
#[cfg(target_os = "linux")]
mod r#struct;

#[cfg(target_os = "linux")]
use {r#static::*, r#struct::*};

use super::*;
