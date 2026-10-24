use super::*;

/// Reports the allocation counters as a line of JSON.
///
/// # Arguments
///
/// - `u64` - The number of requests the server answered.
///
/// # Returns
///
/// - `String` - The JSON line.
pub(crate) fn report(requests: u64) -> String {
    let allocations: u64 = ALLOCATIONS.load(Ordering::Relaxed);
    let bytes: u64 = ALLOCATED_BYTES.load(Ordering::Relaxed);
    let per_request: f64 = if requests == 0 {
        0.0
    } else {
        allocations as f64 / requests as f64
    };
    let bytes_per_request: f64 = if requests == 0 {
        0.0
    } else {
        bytes as f64 / requests as f64
    };
    format!(
        "{{\"requests\":{requests},\"allocations\":{allocations},\"bytes\":{bytes},\"allocations_per_request\":{per_request:.1},\"bytes_per_request\":{bytes_per_request:.1}}}"
    )
}

/// Counts one request.
pub(crate) fn count_request() {
    REQUESTS.fetch_add(1, Ordering::Relaxed);
}
