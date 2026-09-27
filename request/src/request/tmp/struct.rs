use super::*;

/// Per-request scratch space.
///
/// Holds redirect-loop detection (`visit_url`) and the TLS root cert store.
/// Lives inside [`HttpRequest`] but is **not** serialized — purely an
/// implementation detail of the client state machine.
///
/// Fields are private; access through the methods on [`HttpRequest`].
#[derive(Clone, Debug)]
pub struct Tmp {
    pub(crate) visit_url: HashSet<String>,
    pub(crate) root_cert: RootCertStore,
}

impl Tmp {
    /// Borrow `visit_url` set.
    ///
    /// # Returns
    ///
    /// - `&HashSet<String>` - The visited URL set.
    pub(crate) fn get_visit_url_ref(&self) -> &HashSet<String> {
        &self.visit_url
    }

    /// Mutable borrow of `visit_url` set.
    ///
    /// # Returns
    ///
    /// - `&mut HashSet<String>` - The mutable visited URL set.
    pub(crate) fn get_visit_url_mut(&mut self) -> &mut HashSet<String> {
        &mut self.visit_url
    }

    /// Clone the `root_cert` store.
    ///
    /// # Returns
    ///
    /// - `RootCertStore` - A cloned root certificate store.
    pub(crate) fn get_root_cert(&self) -> RootCertStore {
        self.root_cert.clone()
    }
}

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
