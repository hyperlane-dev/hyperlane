use super::*;

impl Default for Tmp {
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
