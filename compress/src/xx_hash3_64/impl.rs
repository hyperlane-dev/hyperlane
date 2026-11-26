use super::*;

/// Default construction for [`XxHash3_64`].
impl Default for XxHash3_64 {
    /// Builds a hasher without initializing unused streaming or prefix storage.
    ///
    /// # Returns
    ///
    /// - `Self` - The allocation-free hasher.
    #[inline(always)]
    fn default() -> Self {
        Self {
            inner: None,
            prefix: [const { MaybeUninit::uninit() }; HASH_PREFIX_CAPACITY],
            prefix_len: 0,
        }
    }
}

/// Clone implementation for [`XxHash3_64`].
impl Clone for XxHash3_64 {
    /// Clones the hasher while copying only the initialized prefix bytes.
    ///
    /// # Returns
    ///
    /// - `Self` - A cloned hasher with identical state.
    #[inline(always)]
    fn clone(&self) -> Self {
        let Self {
            inner,
            prefix: src_prefix,
            prefix_len,
        } = self;
        let mut prefix: [MaybeUninit<u8>; HASH_PREFIX_CAPACITY] =
            [const { MaybeUninit::uninit() }; HASH_PREFIX_CAPACITY];
        let len: usize = *prefix_len;
        if inner.is_none() && len > 0 {
            // SAFETY: `0..len` was initialized in `src_prefix` and `len <= HASH_PREFIX_CAPACITY`.
            unsafe {
                ptr::copy_nonoverlapping(src_prefix.as_ptr(), prefix.as_mut_ptr(), len);
            }
        }
        Self {
            inner: inner.clone(),
            prefix,
            prefix_len: len,
        }
    }
}

/// Implements `impl core::hash::Hasher for XxHash3_64`.
impl core::hash::Hasher for XxHash3_64 {
    /// Feeds bytes into the same XXH3 stream while deferring large-state setup.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The bytes to hash.
    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) {
        let Self {
            inner,
            prefix,
            prefix_len,
        } = self;
        if let Some(hasher) = inner {
            hasher.write(bytes);
            return;
        }
        let current_len: usize = *prefix_len;
        let incoming_len: usize = bytes.len();
        if incoming_len <= HASH_PREFIX_CAPACITY - current_len {
            // SAFETY: `current_len + incoming_len <= HASH_PREFIX_CAPACITY` and
            // `MaybeUninit<u8>` has identical layout to `u8`.
            unsafe {
                ptr::copy_nonoverlapping(
                    bytes.as_ptr(),
                    prefix.as_mut_ptr().add(current_len).cast::<u8>(),
                    incoming_len,
                );
            }
            *prefix_len = current_len + incoming_len;
            return;
        }
        let mut hasher: Box<RawHasher<&'static [u8; DEFAULT_SECRET_LENGTH]>> =
            Box::new(RawHasher::new(SecretBuffer::default()));
        // SAFETY: `0..current_len` bytes in `prefix` were initialized by prior writes.
        let initialized: &[u8] =
            unsafe { slice::from_raw_parts(prefix.as_ptr().cast::<u8>(), current_len) };
        hasher.write(initialized);
        hasher.write(bytes);
        *inner = Some(hasher);
    }

    /// Feeds a single byte into the uninitialized prefix buffer without slice copy overhead.
    ///
    /// # Arguments
    ///
    /// - `u8` - The byte to hash.
    #[inline(always)]
    fn write_u8(&mut self, value: u8) {
        let Self {
            inner,
            prefix,
            prefix_len,
        } = self;
        if let Some(hasher) = inner {
            hasher.write_u8(value);
            return;
        }
        let current_len: usize = *prefix_len;
        if current_len < HASH_PREFIX_CAPACITY {
            prefix[current_len].write(value);
            *prefix_len = current_len + 1;
            return;
        }
        let mut hasher: Box<RawHasher<&'static [u8; DEFAULT_SECRET_LENGTH]>> =
            Box::new(RawHasher::new(SecretBuffer::default()));
        // SAFETY: `0..current_len` bytes in `prefix` were initialized by prior writes.
        let initialized: &[u8] =
            unsafe { slice::from_raw_parts(prefix.as_ptr().cast::<u8>(), current_len) };
        hasher.write(initialized);
        hasher.write_u8(value);
        *inner = Some(hasher);
    }

    /// Reports the same hash irrespective of write boundaries or prefix overflow.
    ///
    /// # Returns
    ///
    /// - `u64` - The hash of all bytes written so far.
    #[inline(always)]
    fn finish(&self) -> u64 {
        let Self {
            inner,
            prefix,
            prefix_len,
        } = self;
        match inner {
            Some(hasher) => hasher.finish(),
            None => {
                // SAFETY: `0..*prefix_len` bytes in `prefix` were initialized by prior writes.
                let initialized: &[u8] =
                    unsafe { slice::from_raw_parts(prefix.as_ptr().cast::<u8>(), *prefix_len) };
                xxhash3_64::Hasher::oneshot(initialized)
            }
        }
    }
}
