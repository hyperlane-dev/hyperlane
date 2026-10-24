mod r#const;
mod r#fn;
mod r#struct;
mod r#type;

pub use r#fn::render_report;

pub(crate) use {r#const::*, r#struct::*, r#type::*};

use super::*;
