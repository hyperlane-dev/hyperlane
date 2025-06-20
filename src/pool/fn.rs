use crate::*;

fn get_route_params_pool() -> &'static ObjectPool<RouteParams> {
    pool::ROUTE_PARAMS_POOL.get_or_init(|| {
        ObjectPool::new(
            || RouteParams::with_hasher(BuildHasherDefault::<XxHash3_64>::default()),
            32,
        )
    })
}

pub(crate) fn get_route_params() -> RouteParams {
    get_route_params_pool().get()
}
