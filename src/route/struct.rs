use crate::*;
use std::collections::HashMap;
use std::fmt;

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
            .join("/");
        write!(f, "/{}", s)
    }
}

#[derive(Clone)]
pub(crate) struct RouteMatcher {
    pub(super) static_routes: HashMap<String, ArcFunc>,
    pub(super) dynamic_and_regex_routes: Vec<(RoutePattern, ArcFunc)>,
}

impl fmt::Debug for RouteMatcher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let static_keys: Vec<&String> = self.static_routes.keys().collect();
        let dynamic_patterns: Vec<String> = self
            .dynamic_and_regex_routes
            .iter()
            .map(|(pat, _)| format!("{:?}", pat))
            .collect();
        f.debug_struct("RouteMatcher")
            .field("static_routes", &static_keys)
            .field("dynamic_and_regex_routes", &dynamic_patterns)
            .finish()
    }
}

impl Default for RouteMatcher {
    fn default() -> Self {
        Self {
            static_routes: HashMap::new(),
            dynamic_and_regex_routes: Vec::new(),
        }
    }
}
