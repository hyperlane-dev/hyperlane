use super::*;

impl Default for Tmp {
    /// Builds a default temporary store seeded with the bundled TLS server roots.
    ///
    /// # Returns
    ///
    /// - `Tmp`: a store with an empty visited-URL set and the bundled server roots.
    #[inline(always)]
    fn default() -> Self {
        Self {
            visit_url: HashSet::new(),
            root_cert: RootCertStore {
                roots: TLS_SERVER_ROOTS.to_vec(),
            },
        }
    }
}
