use super::*;

/// Results keyed by repeat number.
pub(crate) type Repeats = BTreeMap<u32, MeasurementResult>;
/// Repeat maps keyed by variant name.
pub(crate) type Variants = BTreeMap<String, Repeats>;
/// Variant maps keyed by workload cell.
pub(crate) type Cells = BTreeMap<Cell, Variants>;
