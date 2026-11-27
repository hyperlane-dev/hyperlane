use super::*;

/// Computes the cache shard index for a canonical file path using `XxHash3_64`.
///
/// # Arguments
///
/// - `&Path` - Canonical file path.
///
/// # Returns
///
/// - `usize` - Shard index in `0..STATIC_CACHE_SHARDS`.
pub(crate) fn static_cache_shard_index(path: &Path) -> usize {
    let mut hasher: XxHash3_64 = XxHash3_64::default();
    hasher.write(path.as_os_str().as_encoded_bytes());
    (hasher.finish() as usize) & (STATIC_CACHE_SHARDS - 1)
}

/// Percent-decodes a URL path segment while rejecting NUL bytes (`%00`) and invalid UTF-8.
///
/// # Arguments
///
/// - `&str` - Raw URL path slice.
///
/// # Returns
///
/// - `Option<String>` - Decoded UTF-8 relative path, or `None` if malformed or contains NUL/backslash.
pub(crate) fn percent_decode_path(raw: &str) -> Option<String> {
    let bytes: &[u8] = raw.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i: usize = 0;
    while i < bytes.len() {
        let b: u8 = bytes[i];
        if b == 0 || b == b'\\' {
            return None;
        }
        if b == b'%' {
            if i + 2 >= bytes.len() {
                return None;
            }
            let hi: u8 = hex_nibble(bytes[i + 1])?;
            let lo: u8 = hex_nibble(bytes[i + 2])?;
            let decoded: u8 = (hi << 4) | lo;
            if decoded == 0 || decoded == b'\\' {
                return None;
            }
            out.push(decoded);
            i += 3;
        } else {
            out.push(b);
            i += 1;
        }
    }
    String::from_utf8(out).ok()
}

/// Converts an ASCII hex digit byte into its 4-bit value.
///
/// # Arguments
///
/// - `u8` - ASCII byte.
///
/// # Returns
///
/// - `Option<u8>` - 4-bit value `0..=15`, or `None` when not hex.
pub(crate) fn hex_nibble(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

/// Safely joins an untrusted relative URL path onto a canonicalized `root_dir`, rejecting traversal and dotfiles.
///
/// # Arguments
///
/// - `&Path` - Canonicalized root directory.
/// - `&str` - Raw URL relative path (after stripping mount prefix).
/// - `bool` - Whether hidden dotfiles/dotdirs (segments beginning with `.`) are permitted.
///
/// # Returns
///
/// - `Option<PathBuf>` - Validated filesystem path inside `root_dir`, or `None` on traversal/escape.
pub fn resolve_safe_path(
    root_dir: &Path,
    relative_url: &str,
    allow_dotfiles: bool,
) -> Option<PathBuf> {
    let decoded: String = percent_decode_path(relative_url)?;
    let trimmed: &str = decoded.trim_start_matches('/');
    let mut candidate: PathBuf = root_dir.to_path_buf();
    for component in Path::new(trimmed).components() {
        match component {
            Component::Normal(seg) => {
                let seg_str: &str = seg.to_str()?;
                if seg_str.is_empty() || (!allow_dotfiles && seg_str.starts_with('.')) {
                    return None;
                }
                candidate.push(seg);
            }
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => return None,
        }
    }
    Some(candidate)
}

/// Resolves the MIME `Content-Type` string for `path` using Hyperlane's [`FileExtension`] table.
///
/// # Arguments
///
/// - `&Path` - Target file path.
///
/// # Returns
///
/// - `&'static str` - Canonical MIME type (defaults to `application/octet-stream`).
pub fn mime_for_path(path: &Path) -> &'static str {
    let path_str: &str = path.to_str().unwrap_or_default();
    let ext: String = FileExtension::get_extension_name(path_str);
    if ext.is_empty() {
        return APPLICATION_OCTET_STREAM;
    }
    let parsed: FileExtension = FileExtension::parse(ext.to_ascii_lowercase());
    let mime: &'static str = parsed.get_content_type();
    if mime.is_empty() {
        APPLICATION_OCTET_STREAM
    } else {
        mime
    }
}

/// Computes a deterministic strong `ETag` from file byte length and modification timestamp.
///
/// # Arguments
///
/// - `u64` - File size in bytes.
/// - `u64` - Modification timestamp in milliseconds since Unix epoch.
///
/// # Returns
///
/// - `String` - Quoted strong ETag (for example `"\"400-18f123ab\""`).
pub(crate) fn compute_file_etag(size: u64, modified_ms: u64) -> String {
    format!("\"{size:x}-{modified_ms:x}\"")
}

/// Extracts modification time in milliseconds since Unix epoch from filesystem [`Metadata`].
///
/// # Arguments
///
/// - `&Metadata` - File metadata.
///
/// # Returns
///
/// - `u64` - Modification timestamp in milliseconds, or `0` if unavailable.
pub(crate) fn metadata_modified_ms(meta: &Metadata) -> u64 {
    meta.modified()
        .ok()
        .and_then(|ts: SystemTime| ts.duration_since(UNIX_EPOCH).ok())
        .map(|dur: Duration| dur.as_millis() as u64)
        .unwrap_or(0)
}

/// Checks whether an `If-None-Match` header matches `etag` (handling `*`, comma lists, and `W/` weak prefixes).
///
/// # Arguments
///
/// - `&str` - Incoming `If-None-Match` header value.
/// - `&str` - Resource strong `ETag`.
///
/// # Returns
///
/// - `bool` - `true` when the client validator matches (`304 Not Modified`).
pub fn if_none_match_matches(header: &str, etag: &str) -> bool {
    let trimmed: &str = header.trim();
    if trimmed == "*" {
        return true;
    }
    trimmed.split(',').any(|token: &str| {
        let candidate: &str = token.trim();
        let normalized: &str = candidate.strip_prefix("W/").unwrap_or(candidate);
        normalized == etag
    })
}

/// Parses a single RFC 9110 byte `Range` header (`bytes=start-end`, `bytes=start-`, `bytes=-suffix`).
///
/// # Arguments
///
/// - `&str` - Raw `Range` header value.
/// - `usize` - Total resource length in bytes.
///
/// # Returns
///
/// - `Option<(usize, usize)>` - Inclusive `(start, end)` byte offsets, or `None` when unsatisfiable/invalid.
pub fn parse_byte_range(header: &str, total_len: usize) -> Option<(usize, usize)> {
    if total_len == 0 {
        return None;
    }
    let trimmed: &str = header.trim();
    let spec: &str = trimmed.strip_prefix(RANGE_BYTES_PREFIX)?;
    if spec.contains(',') {
        return None;
    }
    let (start_str, end_str): (&str, &str) = spec.split_once('-')?;
    let start_str: &str = start_str.trim();
    let end_str: &str = end_str.trim();
    if start_str.is_empty() {
        let suffix: usize = end_str.parse::<usize>().ok()?;
        if suffix == 0 {
            return None;
        }
        let start: usize = total_len.saturating_sub(suffix);
        return Some((start, total_len - 1));
    }
    let start: usize = start_str.parse::<usize>().ok()?;
    if start >= total_len {
        return None;
    }
    let end: usize = if end_str.is_empty() {
        total_len - 1
    } else {
        end_str.parse::<usize>().ok()?.min(total_len - 1)
    };
    if start > end {
        return None;
    }
    Some((start, end))
}

/// Reports whether `accept_encoding` permits `token` without `q=0`.
///
/// # Arguments
///
/// - `&str` - Incoming `Accept-Encoding` header value.
/// - `&str` - Target encoding token (`"br"` or `"gzip"`).
///
/// # Returns
///
/// - `bool` - `true` when accepted.
pub(crate) fn accepts_encoding(accept_encoding: &str, token: &str) -> bool {
    for part in accept_encoding.split(',') {
        let mut segments: Split<'_, char> = part.trim().split(';');
        let coding: &str = segments.next().unwrap_or_default().trim();
        if !coding.eq_ignore_ascii_case(token) {
            continue;
        }
        let mut rejected: bool = false;
        for param in segments {
            let trimmed: &str = param.trim();
            if let Some(qval) = trimmed
                .strip_prefix("q=")
                .or_else(|| trimmed.strip_prefix("Q="))
                && qval.trim().parse::<f32>().ok() == Some(0.0)
            {
                rejected = true;
            }
        }
        if !rejected {
            return true;
        }
    }
    false
}
