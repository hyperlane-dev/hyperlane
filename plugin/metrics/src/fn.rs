use super::*;

/// Compares two byte slices in constant time for optional bearer token authentication.
///
/// # Arguments
///
/// - `&[u8]` - First byte slice.
/// - `&[u8]` - Second byte slice.
///
/// # Returns
///
/// - `bool` - `true` iff both slices match identically.
pub(crate) fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff: u8 = 0;
    for i in 0..a.len() {
        diff |= a[i] ^ b[i];
    }
    diff == 0
}

/// Checks optional `Authorization: Bearer <token>` header against `expected_token`.
///
/// # Arguments
///
/// - `&Context` - Incoming request context.
/// - `Option<&str>` - Expected secret bearer token (`None` permits public scrape).
///
/// # Returns
///
/// - `bool` - `true` when access to `/metrics` is authorized.
pub(crate) fn is_scrape_authorized(ctx: &Context, expected_token: Option<&str>) -> bool {
    let Some(expected) = expected_token else {
        return true;
    };
    let Some(header) = ctx.get_request().try_get_header_back(AUTHORIZATION) else {
        return false;
    };
    let trimmed: &str = header.trim();
    if trimmed.len() < PREFIX_BEARER.len() {
        return false;
    }
    let (scheme, token): (&str, &str) = trimmed.split_at(PREFIX_BEARER.len());
    scheme.eq_ignore_ascii_case(PREFIX_BEARER)
        && constant_time_eq(token.trim().as_bytes(), expected.as_bytes())
}

/// Atomically adds `delta` to an `f64` stored in `target` via IEEE-754 bit CAS loop.
///
/// # Arguments
///
/// - `&AtomicU64` - Atomic bit storage holding `f64::to_bits`.
/// - `f64` - Floating-point increment.
pub(crate) fn atomic_f64_add(target: &AtomicU64, delta: f64) {
    let mut observed: u64 = target.load(Ordering::Relaxed);
    loop {
        let next: u64 = (f64::from_bits(observed) + delta).to_bits();
        match target.compare_exchange_weak(observed, next, Ordering::AcqRel, Ordering::Relaxed) {
            Ok(_) => break,
            Err(actual) => observed = actual,
        }
    }
}
