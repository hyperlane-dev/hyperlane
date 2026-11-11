use super::*;

impl Default for XxHash3_64 {
    /// Builds a hasher that allocates nothing.
    ///
    /// # Returns
    ///
    /// - `Self` - The hasher.
    fn default() -> Self {
        Self {
            inner: RawHasher::new(SecretBuffer::default()),
        }
    }
}

impl XxHash3_64 {
    /// Returns the hasher that holds the secret, the seed and the buffer.
    ///
    /// # Returns
    ///
    /// - `&RawHasher<&'static [u8; DEFAULT_SECRET_LENGTH]>` - The inner hasher.
    pub(crate) fn get_inner(&self) -> &RawHasher<&'static [u8; DEFAULT_SECRET_LENGTH]> {
        &self.inner
    }

    /// Returns the inner hasher for mutation.
    ///
    /// # Returns
    ///
    /// - `&mut RawHasher<&'static [u8; DEFAULT_SECRET_LENGTH]>` - The inner hasher.
    pub(crate) fn get_mut_inner(&mut self) -> &mut RawHasher<&'static [u8; DEFAULT_SECRET_LENGTH]> {
        &mut self.inner
    }
}

impl core::hash::Hasher for XxHash3_64 {
    /// Feeds bytes into the hasher.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The bytes to hash.
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.get_mut_inner().write(bytes);
    }

    /// Reports the hash of everything written so far.
    ///
    /// # Returns
    ///
    /// - `u64` - The hash.
    #[inline]
    fn finish(&self) -> u64 {
        self.get_inner().finish()
    }
}
