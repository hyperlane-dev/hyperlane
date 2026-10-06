mod compress;

use http_compress::*;

use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};

use {core::hash::BuildHasherDefault, twox_hash::XxHash3_64};
