use crate::*;

impl PartialEq for RouteSegment {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RouteSegment::Static(segment1), RouteSegment::Static(segment2)) => {
                segment1 == segment2
            }
            (RouteSegment::Dynamic(_), RouteSegment::Dynamic(_)) => true,
            (RouteSegment::Regex(name1, _), RouteSegment::Regex(name2, _)) => name1 == name2,
            _ => false,
        }
    }
}

impl PartialEq for RoutePattern {
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        self.0
            .iter()
            .zip(other.0.iter())
            .all(|(segment1, segment2)| segment1 == segment2)
    }
}

impl RoutePattern {
    pub(crate) fn new(route: &str) -> ResultRoutePatternRouteError {
        let segments: VecRouteSegment = Self::parse_route(route)?;
        Ok(Self(segments))
    }

    fn parse_route(route: &str) -> ResultVecRouteSegmentRouteError {
        if route.is_empty() {
            return Err(RouteError::EmptyPattern);
        }
        let mut segments: VecRouteSegment = Vec::new();
        let route: &str = route.trim_start_matches(DEFAULT_HTTP_PATH);
        if route.is_empty() {
            return Ok(segments);
        }
        for segment in route.split(DEFAULT_HTTP_PATH) {
            if segment.starts_with(DYNAMIC_ROUTE_LEFT_BRACKET)
                && segment.ends_with(DYNAMIC_ROUTE_RIGHT_BRACKET)
            {
                let content: &str = &segment[1..segment.len() - 1];
                if let Some((name, pattern)) = content.split_once(':') {
                    match Regex::new(pattern) {
                        Ok(regex) => {
                            segments.push(RouteSegment::Regex(name.to_owned(), regex));
                        }
                        Err(err) => {
                            return Err(RouteError::InvalidRegexPattern(format!(
                                "Invalid regex pattern '{}{}{}",
                                pattern, COLON_SPACE, err
                            )));
                        }
                    }
                } else {
                    segments.push(RouteSegment::Dynamic(content.to_owned()));
                }
            } else {
                segments.push(RouteSegment::Static(segment.to_owned()));
            }
        }
        Ok(segments)
    }

    pub(crate) fn match_path(&self, path: &str) -> OptionRouteParams {
        let path: &str = path.trim_start_matches(DEFAULT_HTTP_PATH);
        let path_segments: Vec<&str> = if path.is_empty() {
            Vec::new()
        } else {
            path.split(DEFAULT_HTTP_PATH).collect()
        };
        if path_segments.len() != self.0.len() {
            return None;
        }
        let mut params: RouteParams = hash_map_xx_hash3_64();
        for (idx, segment) in self.0.iter().enumerate() {
            match segment {
                RouteSegment::Static(path) => {
                    if path != path_segments[idx] {
                        return None;
                    }
                }
                RouteSegment::Dynamic(param_name) => {
                    params.insert(param_name.clone(), path_segments[idx].to_owned());
                }
                RouteSegment::Regex(param_name, regex) => {
                    let segment: &str = path_segments[idx];
                    if !regex.is_match(segment)
                        || regex
                            .find(segment)
                            .map_or(false, |m| m.start() != 0 || m.end() != segment.len())
                    {
                        return None;
                    }
                    params.insert(param_name.clone(), segment.to_owned());
                }
            }
        }
        Some(params)
    }
}

impl RouteMatcher {
    pub(crate) fn new() -> Self {
        Self(Vec::new())
    }

    pub(crate) fn add(&mut self, pattern: &str, handler: ArcFunc) -> ResultAddRoute {
        let route_pattern: RoutePattern = RoutePattern::new(pattern)?;
        let has_same_pattern: bool = self
            .0
            .iter()
            .any(|(tmp_pattern, _)| tmp_pattern == &route_pattern);
        if has_same_pattern {
            return Err(RouteError::DuplicatePattern(pattern.to_owned()));
        }
        self.0.push((route_pattern, handler));
        return Ok(());
    }

    pub(crate) fn match_route(&self, path: &str) -> OptionRouteParams {
        for (pattern, _) in &self.0 {
            if let Some(params) = pattern.match_path(path) {
                return Some(params);
            }
        }
        None
    }

    pub(crate) async fn resolve_route(&self, ctx: &Context, path: &str) -> OptionArcFunc {
        for (pattern, handler) in &self.0 {
            if let Some(params) = pattern.match_path(path) {
                ctx.set_route_params(params).await;
                return Some(handler.clone());
            }
        }
        None
    }
}
