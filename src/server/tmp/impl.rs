use super::r#type::Tmp;
use crate::*;

impl Default for Tmp {
    #[inline]
    fn default() -> Self {
        Self {
            log: Log::default(),
        }
    }
}
