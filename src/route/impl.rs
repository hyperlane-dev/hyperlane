use crate::*;

/// Provides a default implementation for RouteMatcher.
impl Default for RouteMatcher {
    /// Creates a new, empty RouteMatcher.
    ///
    /// # Returns
    ///
    /// - `RouteMatcher` - A new RouteMatcher with empty storage for static, dynamic, and regex routes.
    fn default() -> Self {
        Self {
            static_routes: hash_map_xx_hash3_64(),
            dynamic_routes: Vec::new(),
            regex_routes: Vec::new(),
        }
    }
}

/// Implements the `PartialEq` trait for `RoutePattern`.
///
/// This allows for comparing two `RoutePattern` instances for equality.
impl PartialEq for RoutePattern {
    /// Checks if two `RoutePattern` instances are equal.
    ///
    /// # Arguments
    ///
    /// - `&Self`: The other `RoutePattern` instance to compare against.
    ///
    /// # Returns
    ///
    /// - `bool`: `true` if the instances are equal, `false` otherwise.
    fn eq(&self, other: &Self) -> bool {
        self.get_0() == other.get_0()
    }
}

/// Implements the `Eq` trait for `RoutePattern`.
///
/// This indicates that `RoutePattern` has a total equality relation.
impl Eq for RoutePattern {}

/// Implements the `PartialOrd` trait for `RoutePattern`.
///
/// This allows for partial ordering of `RoutePattern` instances.
impl PartialOrd for RoutePattern {
    /// Partially compares two `RoutePattern` instances.
    ///
    /// # Arguments
    ///
    /// - `&Self`: The other `RoutePattern` instance to compare against.
    ///
    /// # Returns
    ///
    /// - `Option<std::cmp::Ordering>`: The ordering of the two instances.
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Implements the `Ord` trait for `RoutePattern`.
///
/// This allows for total ordering of `RoutePattern` instances.
impl Ord for RoutePattern {
    /// Compares two `RoutePattern` instances.
    ///
    /// # Arguments
    ///
    /// - `&Self`: The other `RoutePattern` instance to compare against.
    ///
    /// # Returns
    ///
    /// - `std::cmp::Ordering`: The ordering of the two instances.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_0().cmp(&other.get_0())
    }
}

/// Implements the `PartialEq` trait for `RouteMatcher`.
///
/// This allows for comparing two `RouteMatcher` instances for equality.
impl PartialEq for RouteMatcher {
    /// Checks if two `RouteMatcher` instances are equal.
    ///
    /// # Arguments
    ///
    /// - `&Self`: The other `RouteMatcher` instance to compare against.
    ///
    /// # Returns
    ///
    /// - `bool`: `true` if the instances are equal, `false` otherwise.
    fn eq(&self, other: &Self) -> bool {
        let self_static_keys: BTreeSet<_> = self.static_routes.keys().collect();
        let other_static_keys: BTreeSet<_> = other.static_routes.keys().collect();
        if self_static_keys != other_static_keys {
            return false;
        }
        let self_dynamic_patterns: BTreeSet<_> =
            self.dynamic_routes.iter().map(|(p, _)| p).collect();
        let other_dynamic_patterns: BTreeSet<_> =
            other.dynamic_routes.iter().map(|(p, _)| p).collect();
        if self_dynamic_patterns != other_dynamic_patterns {
            return false;
        }
        let self_regex_patterns: BTreeSet<_> = self.regex_routes.iter().map(|(p, _)| p).collect();
        let other_regex_patterns: BTreeSet<_> = other.regex_routes.iter().map(|(p, _)| p).collect();
        if self_regex_patterns != other_regex_patterns {
            return false;
        }
        true
    }
}

/// Implements the `Eq` trait for `RouteMatcher`.
///
/// This indicates that `RouteMatcher` has a total equality relation.
impl Eq for RouteMatcher {}

/// Implements the `Eq` trait for `RouteSegment`.
///
/// This indicates that `RouteSegment` has a total equality relation.
impl Eq for RouteSegment {}

/// Implements the `PartialOrd` trait for `RouteSegment`.
///
/// This allows for partial ordering of `RouteSegment` instances.
impl PartialOrd for RouteSegment {
    /// Partially compares two `RouteSegment` instances.
    ///
    /// # Arguments
    ///
    /// - `&Self`: The other `RouteSegment` instance to compare against.
    ///
    /// # Returns
    ///
    /// - `Option<Ordering>`: The ordering of the two instances.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Implements the `Ord` trait for `RouteSegment`.
///
/// This allows for total ordering of `RouteSegment` instances.
impl Ord for RouteSegment {
    /// Compares two `RouteSegment` instances.
    ///
    /// # Arguments
    ///
    /// - `&Self`: The other `RouteSegment` instance to compare against.
    ///
    /// # Returns
    ///
    /// - `Ordering`: The ordering of the two instances.
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Static(s1), Self::Static(s2)) => s1.cmp(s2),
            (Self::Dynamic(d1), Self::Dynamic(d2)) => d1.cmp(d2),
            (Self::Regex(n1, r1), Self::Regex(n2, r2)) => {
                n1.cmp(n2).then_with(|| r1.as_str().cmp(r2.as_str()))
            }
            (Self::Static(_), _) => Ordering::Less,
            (_, Self::Static(_)) => Ordering::Greater,
            (Self::Dynamic(_), _) => Ordering::Less,
            (_, Self::Dynamic(_)) => Ordering::Greater,
        }
    }
}

/// Implements the `PartialEq` trait for `RouteSegment`.
///
/// This allows for comparing two `RouteSegment` instances for equality.
impl PartialEq for RouteSegment {
    /// Checks if two `RouteSegment` instances are equal.
    ///
    /// # Arguments
    ///
    /// - `&Self`: The other `RouteSegment` instance to compare against.
    ///
    /// # Returns
    ///
    /// - `bool`: `true` if the instances are equal, `false` otherwise.
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Static(l0), Self::Static(r0)) => l0 == r0,
            (Self::Dynamic(l0), Self::Dynamic(r0)) => l0 == r0,
            (Self::Regex(l0, l1), Self::Regex(r0, r1)) => l0 == r0 && l1.as_str() == r1.as_str(),
            _ => false,
        }
    }
}

/// Manages route patterns, including parsing and matching.
///
/// This struct is responsible for defining and validating route structures,
/// supporting static, dynamic, and regex-based path matching.
impl RoutePattern {
    /// Creates a new RoutePattern by parsing a route string.
    ///
    /// # Arguments
    ///
    /// - `&str` - The raw route string to parse.
    ///
    /// # Returns
    ///
    /// - `Result<RoutePattern, RouteError>` - The parsed RoutePattern on success, or RouteError on failure.
    pub(crate) fn new(route: &str) -> ResultRoutePatternRouteError {
        Ok(Self(Self::parse_route(route)?))
    }

    /// Parses a raw route string into RouteSegments.
    ///
    /// This is the core logic for interpreting the route syntax.
    ///
    /// # Arguments
    ///
    /// - `&str` - The raw route string.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<RouteSegment>, RouteError>` - Vector of RouteSegments on success, or RouteError on failure.
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

    /// Matches this route pattern against a request path.
    ///
    /// If the pattern matches, extracts any dynamic or regex parameters.
    ///
    /// # Arguments
    ///
    /// - `&str` - The request path to match against.
    ///
    /// # Returns
    ///
    /// - `Option<RouteParams>` - Some with parameters if matched, None otherwise.
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
    /// # Returns
    ///
    /// - `bool` - true if the pattern is static, false otherwise.
    pub(crate) fn is_static(&self) -> bool {
        self.get_0()
            .iter()
            .all(|seg| matches!(seg, RouteSegment::Static(_)))
    }

    /// Checks if the route pattern is dynamic.
    ///
    /// # Returns
    ///
    /// - `bool` - true if the pattern is dynamic, false otherwise.
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

/// Manages a collection of routes, enabling efficient lookup and dispatch.
///
/// This struct stores routes categorized by type (static, dynamic, regex)
/// to quickly find the appropriate handler for incoming requests.
impl RouteMatcher {
    /// Creates a new, empty RouteMatcher.
    ///
    /// # Returns
    ///
    /// - `RouteMatcher` - A new RouteMatcher instance with empty route stores.
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
    /// - `&str` - The route pattern string.
    /// - `ArcFnContextPinBoxSendSync` - The handler function for this route.
    ///
    /// # Returns
    ///
    /// - `Result<(), RouteError>` - Ok on success, or RouteError if pattern is duplicate.
    pub(crate) fn add(
        &mut self,
        pattern: &str,
        handler: ArcFnContextPinBoxSendSync<()>,
    ) -> ResultAddRoute {
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
    /// - `&str` - The pattern of the route to remove.
    ///
    /// # Returns
    ///
    /// - `bool` - true if route was removed, false otherwise.
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

    /// Checks if a path matches any registered routes.
    ///
    /// # Arguments
    ///
    /// - `&str` - The request path to check.
    ///
    /// # Returns
    ///
    /// - `bool` - true if matching route found, false otherwise.
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

    /// Finds the handler for a path by matching against registered routes.
    ///
    /// # Arguments
    ///
    /// - `&Context` - The request context.
    /// - `&str` - The request path to resolve.
    ///
    /// # Returns
    ///
    /// - `Option<ArcFnContextPinBoxSendSync>` - Some handler if match found, None otherwise.
    pub(crate) async fn resolve_route(
        &self,
        ctx: &Context,
        path: &str,
    ) -> OptionArcFnContextPinBoxSendSync<()> {
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
