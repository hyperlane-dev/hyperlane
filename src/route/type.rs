use crate::*;

pub type RouteParams = HashMapXxHash3_64<String, String>;

pub(crate) type VecRouteSegment = Vec<RouteSegment>;
pub(crate) type VecStrRef<'a> = Vec<&'a str>;
pub(crate) type VecRoutePatternArcFnPinBoxSendSync = Vec<(RoutePattern, ArcFnPinBoxSendSync)>;
pub(crate) type HashMapStringArcFnPinBoxSendSyncXxHash3_64 =
    HashMapXxHash3_64<String, ArcFnPinBoxSendSync>;
pub(crate) type ResultAddRoute = Result<(), RouteError>;
pub(crate) type ResultVecRouteSegmentRouteError = Result<VecRouteSegment, RouteError>;
pub(crate) type ResultRoutePatternRouteError = Result<RoutePattern, RouteError>;
pub(crate) type OptionRouteParams = Option<RouteParams>;
