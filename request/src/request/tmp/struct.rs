use super::*;

#[derive(Clone, Debug)]
pub struct Tmp {
    pub visit_url: HashSet<String>,
    pub root_cert: RootCertStore,
}
