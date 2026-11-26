use super::*;

/// Shared HTTP/2 multiplexed client sender type.
pub(crate) type H2SendRequest = SendRequest<Bytes>;

/// Global async HTTP/1.1 idle connection pool map.
pub(crate) type AsyncH1PoolMap = HashMap<PoolKey, VecDeque<PooledAsyncConn>>;

/// Global sync HTTP/1.1 idle connection pool map.
pub(crate) type SyncH1PoolMap = HashMap<PoolKey, VecDeque<PooledSyncConn>>;

/// Global async HTTP/2 multiplexed sender map.
pub(crate) type AsyncH2PoolMap = HashMap<PoolKey, H2SendRequest>;
