mod r#const;
mod r#impl;
mod r#static;
mod r#struct;

pub use r#struct::*;
pub use {::log, color_output::*};

pub(crate) use {r#const::*, r#static::*};

pub(crate) use log::SetLoggerError;
