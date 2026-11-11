mod compress;
mod xx_hash3_64;

use super::*;

use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};

use {core::hash::BuildHasherDefault, core::hash::Hasher};
