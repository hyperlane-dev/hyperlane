use crate::*;

/// Provides a default implementation for `RouteMatcher`.
impl Default for RouteMatcher {
    /// Creates a new, empty `RouteMatcher`.
    ///
    /// # Returns
    ///
    /// Returns a `RouteMatcher` with empty storage for static, dynamic, and regex routes.
    fn default() -> Self {
        Self {
            static_routes: hash_map_xx_hash3_64(),
            dynamic_routes: Vec::new(),
            regex_routes: Vec::new(),
        }
    }
}

/// Implements equality comparison for `RouteSegment`.
///
/// This allows for checking if two route segments are functionally equivalent.
impl PartialEq for RouteSegment {
    /// Compares two `RouteSegment`s for equality.
    ///
    /// - `Static` segments are equal if their string content is identical.
    /// - `Dynamic` segments are considered equal to any other `Dynamic` segment.
    /// - `Regex` segments are equal if their parameter names are the same. The regex pattern itself is not compared.
    ///
    /// # Arguments
    ///
    /// - `other` - The other `RouteSegment` to compare against.
    ///
    /// # Returns
    ///
    /// `true` if the segments are equal, `false` otherwise.
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

/// Implements equality comparison for `RoutePattern`.
///
/// This is used to detect duplicate route patterns.
impl PartialEq for RoutePattern {
    /// Compares two `RoutePattern`s for equality.
    ///
    /// Two patterns are considered equal if they have the same number of segments and
    /// each corresponding segment is equal according to `RouteSegment::eq`.
    ///
    /// # Arguments
    ///
    /// - `other` - The other `RoutePattern` to compare against.
    ///
    /// # Returns
    ///
    /// `true` if the patterns are equal, `false` otherwise.
    fn eq(&self, other: &Self) -> bool {
        if self.get_0().len() != other.get_0().len() {
            return false;
        }
        self.get_0()
            .iter()
            .zip(other.get_0().iter())
            .all(|(segment1, segment2)| segment1 == segment2)
    }
}

impl RoutePattern {
    /// Creates a new `RoutePattern` by parsing a route string.
    ///
    /// # Arguments
    ///
    /// - `route` - The raw route string to parse (e.g., "/users/:id").
    ///
    /// # Returns
    ///
    /// A `Result` containing the new `RoutePattern` on success, or a `RouteError` on failure.
    pub(crate) fn new(route: &str) -> ResultRoutePatternRouteError {
        Ok(Self(Self::parse_route(route)?))
    }

    /// Parses a raw route string into a vector of `RouteSegment`s.
    ///
    /// This is the core logic for interpreting the route syntax.
    ///
    /// # Arguments
    ///
    /// - `route` - The raw route string.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `RouteSegment`s on success, or a `RouteError` if parsing fails.
    fn parse_route(route: &str) -> ResultVecRouteSegmentRouteError {
        if route.is_empty() {
            return Err(RouteError::EmptyPattern);
        }
        let route: &str = route.trim_start_matches(DEFAULT_HTTP_PATH);
        if route.is_empty() {
            return Ok(Vec::new());
        }
        let estimated_segments: usize = route.matches(DEFAULT_HTTP_PATH).count() + 1;
        let mut segments: VecRouteSegment = Vec::with_capacity(estimated_segments);
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

    /// Matches this route pattern against a given request path.
    ///
    /// If the pattern matches, it extracts any dynamic or regex parameters from the path.
    ///
    /// # Arguments
    ///
    /// - `path` - The request path to match against.
    ///
    /// # Returns
    ///
    /// `Some(RouteParams)` if the path matches, containing any extracted parameters.
    /// `None` if the path does not match.
    pub(crate) fn match_path(&self, path: &str) -> OptionRouteParams {
        let path: &str = path.trim_start_matches(DEFAULT_HTTP_PATH);
        let route_segments_len: usize = self.get_0().len();
        let is_tail_regex: bool = matches!(self.get_0().last(), Some(RouteSegment::Regex(_, _)));
        if path.is_empty() {
            if route_segments_len == 0 {
                return Some(hash_map_xx_hash3_64());
            }
            return None;
        }
        let mut path_segments: VecStrRef = Vec::with_capacity(route_segments_len);
        let mut segment_start: usize = 0;
        let path_bytes: &[u8] = path.as_bytes();
        let path_separator_byte: u8 = b'/';
        for i in 0..path_bytes.len() {
            if path_bytes[i] == path_separator_byte {
                if segment_start < i {
                    path_segments.push(&path[segment_start..i]);
                }
                segment_start = i + 1;
            }
        }
        if segment_start < path.len() {
            path_segments.push(&path[segment_start..]);
        }
        let path_segments_len: usize = path_segments.len();
        if (!is_tail_regex && path_segments_len != route_segments_len)
            || (is_tail_regex && path_segments_len < route_segments_len - 1)
        {
            return None;
        }
        let mut params: RouteParams = hash_map_xx_hash3_64();
        for (idx, segment) in self.get_0().iter().enumerate() {
            match segment {
                RouteSegment::Static(expected_path) => {
                    if path_segments.get(idx).copied() != Some(expected_path.as_str()) {
                        return None;
                    }
                }
                RouteSegment::Dynamic(param_name) => {
                    let Some(value) = path_segments.get(idx) else {
                        return None;
                    };
                    params.insert(param_name.clone(), value.to_string());
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
                    if let Some(mat) = regex.find(&segment_value) {
                        if mat.start() != 0 || mat.end() != segment_value.len() {
                            return None;
                        }
                    } else {
                        return None;
                    }
                    params.insert(param_name.clone(), segment_value);
                    if idx == route_segments_len - 1 {
                        break;
                    }
                }
            }
        }
        Some(params)
    }

    /// Checks if the route pattern is static.
    ///
    /// A pattern is static if it contains only `RouteSegment::Static` segments.
    ///
    /// # Returns
    ///
    /// `true` if the pattern is static, `false` otherwise.
    pub(crate) fn is_static(&self) -> bool {
        self.get_0()
            .iter()
            .all(|seg| matches!(seg, RouteSegment::Static(_)))
    }

    /// Checks if the route pattern is dynamic.
    ///
    /// A pattern is dynamic if it contains at least one `RouteSegment::Dynamic` and no
    /// `RouteSegment::Regex` segments.
    ///
    /// # Returns
    ///
    /// `true` if the pattern is dynamic, `false` otherwise.
    pub(crate) fn is_dynamic(&self) -> bool {
        self.get_0()
            .iter()
            .any(|seg| matches!(seg, RouteSegment::Dynamic(_)))
            && self
                .get_0()
                .iter()
                .all(|seg| !matches!(seg, RouteSegment::Regex(_, _)))
    }
}

impl RouteMatcher {
    /// Creates a new, empty `RouteMatcher`.
    ///
    /// # Returns
    ///
    /// Returns a new `RouteMatcher` instance with initialized, empty route stores.
    pub(crate) fn new() -> Self {
        Self {
            static_routes: hash_map_xx_hash3_64(),
            dynamic_routes: Vec::new(),
            regex_routes: Vec::new(),
        }
    }

    /// Adds a new route and its handler to the matcher.
    ///
    /// The route is categorized as static, dynamic, or regex based on its pattern.
    ///
    /// # Arguments
    ///
    /// - `pattern` - The route pattern string.
    /// - `handler` - The handler function for this route.
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or a `RouteError` if the pattern is a duplicate.
    pub(crate) fn add(&mut self, pattern: &str, handler: ArcFnPinBoxSendSync) -> ResultAddRoute {
        let route_pattern: RoutePattern = RoutePattern::new(pattern)?;
        if route_pattern.is_static() {
            if self.get_static_routes().contains_key(pattern) {
                return Err(RouteError::DuplicatePattern(pattern.to_owned()));
            }
            self.get_mut_static_routes()
                .insert(pattern.to_string(), handler);
            return Ok(());
        }
        let target_vec: &mut VecRoutePatternArcFnPinBoxSendSync = if route_pattern.is_dynamic() {
            self.get_mut_dynamic_routes()
        } else {
            self.get_mut_regex_routes()
        };
        let has_same_pattern: bool = target_vec
            .iter()
            .any(|(tmp_pattern, _)| tmp_pattern == &route_pattern);
        if has_same_pattern {
            return Err(RouteError::DuplicatePattern(pattern.to_owned()));
        }
        target_vec.push((route_pattern, handler));
        Ok(())
    }

    /// Removes a route from the matcher based on its pattern.
    ///
    /// # Arguments
    ///
    /// - `pattern` - The pattern of the route to remove.
    ///
    /// # Returns
    ///
    /// `true` if a route was successfully removed, `false` otherwise.
    pub(crate) fn remove(&mut self, pattern: &str) -> bool {
        if let Ok(route_pattern) = RoutePattern::new(pattern) {
            if route_pattern.is_static() {
                return self.get_mut_static_routes().remove(pattern).is_some();
            }
            let target_vec: &mut VecRoutePatternArcFnPinBoxSendSync = if route_pattern.is_dynamic()
            {
                self.get_mut_dynamic_routes()
            } else {
                self.get_mut_regex_routes()
            };
            if let Some(pos) = target_vec
                .iter()
                .position(|(tmp_pattern, _)| tmp_pattern == &route_pattern)
            {
                target_vec.remove(pos);
                return true;
            }
        }
        false
    }

    /// Checks if a given path matches any of the registered routes.
    ///
    /// This is useful for checking if a route exists without needing the handler.
    ///
    /// # Arguments
    ///
    /// - `path` - The request path to check.
    ///
    /// # Returns
    ///
    /// `true` if a matching route is found, `false` otherwise.
    pub(crate) fn match_route(&self, path: &str) -> bool {
        if self.get_static_routes().contains_key(path) {
            return true;
        }
        for (pattern, _) in self.get_dynamic_routes().iter() {
            if pattern.match_path(path).is_some() {
                return true;
            }
        }
        for (pattern, _) in self.get_regex_routes().iter() {
            if pattern.match_path(path).is_some() {
                return true;
            }
        }
        false
    }

    /// Finds the handler for a given path by matching against all registered routes.
    ///
    /// It checks static, dynamic, and regex routes in order. If a match is found,
    /// it populates the request context with any captured parameters.
    ///
    /// # Arguments
    ///
    /// - `ctx` - The request context, which will be updated with route parameters on a match.
    /// - `path` - The request path to resolve.
    ///
    /// # Returns
    ///
    /// `Some(handler)` if a match is found, `None` otherwise.
    pub(crate) async fn resolve_route(
        &self,
        ctx: &Context,
        path: &str,
    ) -> OptionArcFnPinBoxSendSync {
        if let Some(handler) = self.get_static_routes().get(path) {
            ctx.set_route_params(RouteParams::default()).await;
            return Some(handler.clone());
        }
        for (pattern, handler) in self.get_dynamic_routes().iter() {
            if let Some(params) = pattern.match_path(path) {
                ctx.set_route_params(params).await;
                return Some(handler.clone());
            }
        }
        for (pattern, handler) in self.get_regex_routes().iter() {
            if let Some(params) = pattern.match_path(path) {
                ctx.set_route_params(params).await;
                return Some(handler.clone());
            }
        }
        None
    }
}
