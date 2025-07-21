use crate::*;

pub type OptionPanic = Option<Panic>;
pub type OptionLocationRef<'a, 'b> = Option<&'a Location<'b>>;
