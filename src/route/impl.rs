use crate::*;

// Associate a plugin registry with the specified type.
collect!(HookType);

/// Provides a default implementation for RouteMatcher.
impl Default for RouteMatcher {
    /// Creates a new, empty RouteMatcher.
    ///
    /// # Returns
    ///
    /// - `RouteMatcher` - A new RouteMatcher with empty storage for static, dynamic, and regex route.
    #[inline(always)]
    fn default() -> Self {
        Self {
            static_route: hash_map_xx_hash3_64(),
            dynamic_route: hash_map_xx_hash3_64(),
            regex_route: hash_map_xx_hash3_64(),
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
    /// - `&Self` - The other `RoutePattern` instance to compare against.
    ///
    /// # Returns
    ///
    /// - `bool`- `true` if the instances are equal, `false` otherwise.
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.get_0() == other.get_0()
    }
}

/// Implements the `Eq` trait for `RoutePattern`.
///
/// This indicates that `RoutePattern` has a total equality relation.
impl Eq for RoutePattern {}

/// Implements the `Hash` trait for `RoutePattern`.
///
/// This allows `RoutePattern` to be used as a key in hash-based collections.
impl Hash for RoutePattern {
    /// Hashes the `RoutePattern` instance.
    ///
    /// # Arguments
    ///
    /// - `&mut Hasher` - The hasher to use.
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_0().hash(state);
    }
}

/// Implements the `PartialOrd` trait for `RoutePattern`.
///
/// This allows for partial ordering of `RoutePattern` instances.
impl PartialOrd for RoutePattern {
    /// Partially compares two `RoutePattern` instances.
    ///
    /// # Arguments
    ///
    /// - `&Self`- The other `RoutePattern` instance to compare against.
    ///
    /// # Returns
    ///
    /// - `Option<Ordering>`- The ordering of the two instances.
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
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
    /// - `&Self`- The other `RoutePattern` instance to compare against.
    ///
    /// # Returns
    ///
    /// - `Ordering`- The ordering of the two instances.
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_0().cmp(other.get_0())
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
    /// - `&Self`- The other `RouteMatcher` instance to compare against.
    ///
    /// # Returns
    ///
    /// - `bool`- `true` if the instances are equal, `false` otherwise.
    fn eq(&self, other: &Self) -> bool {
        if self.get_static_route().len() != other.get_static_route().len() {
            return false;
        }
        for key in self.get_static_route().keys() {
            if !other.get_static_route().contains_key(key) {
                return false;
            }
        }
        if self.get_dynamic_route().len() != other.get_dynamic_route().len() {
            return false;
        }
        for (segment_count, routes) in self.get_dynamic_route() {
            match other.get_dynamic_route().get(segment_count) {
                Some(other_routes) if routes.len() == other_routes.len() => {
                    for (pattern, _) in routes {
                        if !other_routes.iter().any(|(p, _)| p == pattern) {
                            return false;
                        }
                    }
                }
                _ => return false,
            }
        }
        if self.get_regex_route().len() != other.get_regex_route().len() {
            return false;
        }
        for (segment_count, routes) in self.get_regex_route() {
            match other.get_regex_route().get(segment_count) {
                Some(other_routes) if routes.len() == other_routes.len() => {
                    for (pattern, _) in routes {
                        if !other_routes.iter().any(|(p, _)| p == pattern) {
                            return false;
                        }
                    }
                }
                _ => return false,
            }
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
    /// - `&Self`- The other `RouteSegment` instance to compare against.
    ///
    /// # Returns
    ///
    /// - `Option<Ordering>`- The ordering of the two instances.
    #[inline(always)]
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
    /// - `&Self`- The other `RouteSegment` instance to compare against.
    ///
    /// # Returns
    ///
    /// - `Ordering`- The ordering of the two instances.
    #[inline(always)]
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
    /// - `&Self`- The other `RouteSegment` instance to compare against.
    ///
    /// # Returns
    ///
    /// - `bool`- `true` if the instances are equal, `false` otherwise.
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Static(l0), Self::Static(r0)) => l0 == r0,
            (Self::Dynamic(l0), Self::Dynamic(r0)) => l0 == r0,
            (Self::Regex(l0, l1), Self::Regex(r0, r1)) => l0 == r0 && l1.as_str() == r1.as_str(),
            _ => false,
        }
    }
}

/// Implements the `Hash` trait for `RouteSegment`.
///
/// This allows `RouteSegment` to be used in hash-based collections.
impl Hash for RouteSegment {
    /// Hashes the `RouteSegment` instance.
    ///
    /// # Arguments
    ///
    /// - `&mut HHasher` - The hasher to use.
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Static(s) => {
                0u8.hash(state);
                s.hash(state);
            }
            Self::Dynamic(d) => {
                1u8.hash(state);
                d.hash(state);
            }
            Self::Regex(name, regex) => {
                2u8.hash(state);
                name.hash(state);
                regex.as_str().hash(state);
            }
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
    pub(crate) fn new(route: &str) -> Result<RoutePattern, RouteError> {
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
    /// - `Result<RouteSegmentList, RouteError>` - Vector of RouteSegments on success, or RouteError on failure.
    fn parse_route(route: &str) -> Result<RouteSegmentList, RouteError> {
        if route.is_empty() {
            return Err(RouteError::EmptyPattern);
        }
        let route: &str = route.trim_start_matches(DEFAULT_HTTP_PATH);
        if route.is_empty() {
            return Ok(Vec::new());
        }
        let estimated_segments: usize = route.matches(DEFAULT_HTTP_PATH).count() + 1;
        let mut segments: RouteSegmentList = Vec::with_capacity(estimated_segments);
        for segment in route.split(DEFAULT_HTTP_PATH) {
            if segment.starts_with(LEFT_BRACKET) && segment.ends_with(RIGHT_BRACKET) {
                let content: &str = &segment[1..segment.len() - 1];
                if let Some((name, pattern)) = content.split_once(COLON) {
                    match Regex::new(pattern) {
                        Ok(regex) => {
                            segments.push(RouteSegment::Regex(name.to_owned(), regex));
                        }
                        Err(error) => {
                            return Err(RouteError::InvalidRegexPattern(format!(
                                "Invalid regex pattern '{}{}{}",
                                pattern, COLON, error
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
    pub(crate) fn try_match_path(&self, path: &str) -> Option<RouteParams> {
        let path: &str = path.trim_start_matches(DEFAULT_HTTP_PATH);
        let route_segments_len: usize = self.get_0().len();
        let is_tail_regex: bool = matches!(self.get_0().last(), Some(RouteSegment::Regex(_, _)));
        if path.is_empty() {
            if route_segments_len == 0 {
                return Some(hash_map_xx_hash3_64());
            }
            return None;
        }
        let mut path_segments: PathComponentList = Vec::with_capacity(route_segments_len);
        let path_bytes: &[u8] = path.as_bytes();
        let path_separator_byte: u8 = DEFAULT_HTTP_PATH_BYTES[0];
        let mut segment_start: usize = 0;
        for (i, &byte) in path_bytes.iter().enumerate() {
            if byte == path_separator_byte {
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
                    params.insert(param_name.clone(), path_segments.get(idx)?.to_string());
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
                        return Some(params);
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
    #[inline(always)]
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
    #[inline(always)]
    pub(crate) fn is_dynamic(&self) -> bool {
        self.get_0()
            .iter()
            .any(|seg| matches!(seg, RouteSegment::Dynamic(_)))
            && self
                .get_0()
                .iter()
                .all(|seg| !matches!(seg, RouteSegment::Regex(_, _)))
    }

    /// Gets the number of segments in this route pattern.
    ///
    /// # Returns
    ///
    /// - `usize` - The number of segments.
    #[inline(always)]
    pub(crate) fn segment_count(&self) -> usize {
        self.get_0().len()
    }

    /// Checks if the last segment is a regex pattern.
    ///
    /// # Returns
    ///
    /// - `bool` - true if the last segment is a regex, false otherwise.
    #[inline(always)]
    pub(crate) fn has_tail_regex(&self) -> bool {
        matches!(self.get_0().last(), Some(RouteSegment::Regex(_, _)))
    }
}

/// Manages a collection of route, enabling efficient lookup and dispatch.
///
/// This struct stores route categorized by type (static, dynamic, regex)
/// to quickly find the appropriate hook for incoming requests.
impl RouteMatcher {
    /// Creates a new, empty RouteMatcher.
    ///
    /// # Returns
    ///
    /// - `RouteMatcher` - A new RouteMatcher instance with empty route stores.
    #[inline(always)]
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Counts the number of segments in a path.
    ///
    /// # Arguments
    ///
    /// - `&str` - The path to count segments in.
    ///
    /// # Returns
    ///
    /// - `usize` - The number of segments.
    #[inline(always)]
    fn count_path_segments(path: &str) -> usize {
        let path: &str = path.trim_start_matches(DEFAULT_HTTP_PATH);
        if path.is_empty() {
            return 0;
        }
        path.matches(DEFAULT_HTTP_PATH).count() + 1
    }

    /// Adds a new route and its hook to the matcher.
    ///
    /// Adds a route hook to the matcher.
    ///
    /// This method categorizes the route as static, dynamic, or regex based on its pattern
    /// and stores it in the appropriate collection.
    ///
    /// # Arguments
    ///
    /// - `&str` - The route pattern string.
    /// - `ServerHookHandler` - The boxed route hook.
    ///
    /// # Returns
    ///
    /// - `Result<(), RouteError>` - Ok on success, or RouteError if pattern is duplicate.
    pub(crate) fn add(&mut self, pattern: &str, hook: ServerHookHandler) -> Result<(), RouteError> {
        let route_pattern: RoutePattern = RoutePattern::new(pattern)?;
        if route_pattern.is_static() {
            if self.get_static_route().contains_key(pattern) {
                return Err(RouteError::DuplicatePattern(pattern.to_owned()));
            }
            self.get_mut_static_route()
                .insert(pattern.to_string(), hook);
            return Ok(());
        }
        let target_map: &mut ServerHookPatternRoute = if route_pattern.is_dynamic() {
            self.get_mut_dynamic_route()
        } else {
            self.get_mut_regex_route()
        };
        let segment_count: usize = route_pattern.segment_count();
        let routes_for_count: &mut Vec<(RoutePattern, ServerHookHandler)> =
            target_map.entry(segment_count).or_default();
        match routes_for_count.binary_search_by(|(p, _)| p.cmp(&route_pattern)) {
            Ok(_) => return Err(RouteError::DuplicatePattern(pattern.to_owned())),
            Err(pos) => routes_for_count.insert(pos, (route_pattern, hook)),
        }
        Ok(())
    }

    /// Resolves and executes a route hook.
    ///
    /// This method searches for a matching route and executes it if found.
    /// Finds a matching route hook for the given path.
    ///
    /// # Arguments
    ///
    /// - `&mut Context` - The request context.
    /// - `&str` - The request path to resolve.
    ///
    /// # Returns
    ///
    /// - `Option<ServerHookHandler>` - The matched route hook if found, None otherwise.
    pub(crate) fn try_resolve_route(
        &self,
        ctx: &mut Context,
        path: &str,
    ) -> Option<ServerHookHandler> {
        if let Some(hook) = self.get_static_route().get(path) {
            ctx.set_route_params(RouteParams::default());
            return Some(hook.clone());
        }
        let path_segment_count: usize = Self::count_path_segments(path);
        if let Some(routes) = self.get_dynamic_route().get(&path_segment_count) {
            for (pattern, hook) in routes {
                if let Some(params) = pattern.try_match_path(path) {
                    ctx.set_route_params(params);
                    return Some(hook.clone());
                }
            }
        }
        if let Some(routes) = self.get_regex_route().get(&path_segment_count) {
            for (pattern, hook) in routes {
                if let Some(params) = pattern.try_match_path(path) {
                    ctx.set_route_params(params);
                    return Some(hook.clone());
                }
            }
        }
        for (&segment_count, routes) in self.get_regex_route() {
            if segment_count == path_segment_count {
                continue;
            }
            for (pattern, hook) in routes {
                if pattern.has_tail_regex()
                    && path_segment_count >= segment_count
                    && let Some(params) = pattern.try_match_path(path)
                {
                    ctx.set_route_params(params);
                    return Some(hook.clone());
                }
            }
        }
        None
    }
}
