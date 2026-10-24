/// The allocator that counts what the process allocates.
///
/// The counters are read by the watchdog and reported as allocations and bytes
/// **per request**, which is the unit that says whether the framework allocates
/// on the hot path. A total says nothing: a server that answered ten thousand
/// requests allocated ten thousand times as much as one that answered ten.
///
/// The allocator is a diagnostic instrument, not a measurement instrument: it
/// adds two atomic increments to every allocation, so a window measured with
/// it is perturbed and must not be used as an acceptance sample. It is
/// therefore behind a cargo feature, and the acceptance image is built without
/// it.
pub(crate) struct Counting;
