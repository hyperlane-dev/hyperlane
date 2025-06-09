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
        let route_segments_len: usize = self.0.len();
        let path_segments_len: usize = path_segments.len();
        let is_tail_regex: bool = matches!(self.0.last(), Some(RouteSegment::Regex(_, _)));
        if (!is_tail_regex && path_segments_len != route_segments_len)
            || (is_tail_regex && path_segments_len < route_segments_len - 1)
        {
            return None;
        }
        let mut params: RouteParams = hash_map_xx_hash3_64();
        for (idx, segment) in self.0.iter().enumerate() {
            match segment {
                RouteSegment::Static(path) => {
                    if path_segments.get(idx).copied() != Some(path.as_str()) {
                        return None;
                    }
                }
                RouteSegment::Dynamic(param_name) => {
                    let Some(value) = path_segments.get(idx) else {
                        return None;
                    };
                    params.insert(param_name.clone(), (*value).to_owned());
                }
                RouteSegment::Regex(param_name, regex) => {
                    let segment_value: String = if idx == route_segments_len - 1 {
                        path_segments[idx..].join(DEFAULT_HTTP_PATH)
                    } else {
                        match path_segments.get(idx) {
                            Some(val) => val.to_string(),
                            None => return None,
                        }
                    };
                    if !regex.is_match(&segment_value)
                        || regex
                            .find(&segment_value)
                            .map_or(false, |m| m.start() != 0 || m.end() != segment_value.len())
                    {
                        return None;
                    }
                    params.insert(param_name.clone(), segment_value);
                    break;
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

    pub(crate) fn match_route(&self, path: &str) -> bool {
        for (pattern, _) in &self.0 {
            if pattern.match_path(path).is_some() {
                return true;
            }
        }
        false
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
