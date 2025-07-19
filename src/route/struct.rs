use crate::*;

#[derive(Debug, Clone)]
pub(crate) struct RoutePattern(pub(super) VecRouteSegment);

impl fmt::Display for RoutePattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self
            .0
            .iter()
            .map(|seg| match seg {
                RouteSegment::Static(val) => val.clone(),
                RouteSegment::Dynamic(val) => format!(":{}", val),
                RouteSegment::Regex(val, re) => format!(":{}:re({})", val, re),
            })
            .collect::<Vec<_>>()
            .join(ROOT_PATH);
        write!(f, "{}{}", ROOT_PATH, s)
    }
}

#[derive(Clone)]
pub(crate) struct RouteMatcher {
    pub(super) static_routes: HashMapXxHash3_64<String, ArcFnPinBoxSendSync>,
    pub(super) dynamic_routes: Vec<(RoutePattern, ArcFnPinBoxSendSync)>,
    pub(super) regex_routes: Vec<(RoutePattern, ArcFnPinBoxSendSync)>,
}

impl fmt::Debug for RouteMatcher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let static_keys: Vec<&String> = self.static_routes.keys().collect();
        let dynamic_patterns: Vec<String> = self
            .dynamic_routes
            .iter()
            .map(|(pat, _)| format!("{:?}", pat))
            .collect();
        f.debug_struct("RouteMatcher")
            .field("static_routes", &static_keys)
            .field("dynamic_routes", &dynamic_patterns)
            .finish()
    }
}

impl Default for RouteMatcher {
    fn default() -> Self {
        Self {
            static_routes: hash_map_xx_hash3_64(),
            dynamic_routes: Vec::new(),
            regex_routes: Vec::new(),
        }
    }
}
