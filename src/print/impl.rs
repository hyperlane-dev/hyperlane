use crate::*;

impl<T> ErrorHandle for T where T: Fn(String) {}
