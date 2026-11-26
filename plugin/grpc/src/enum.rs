/// Canonical gRPC status codes (`0..=16`).
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum GrpcStatus {
    /// Not an error; returned on success (`0`).
    #[default]
    Ok = 0,
    /// The operation was cancelled (`1`).
    Cancelled = 1,
    /// Unknown error (`2`).
    Unknown = 2,
    /// Client specified an invalid argument (`3`).
    InvalidArgument = 3,
    /// Deadline expired before operation could complete (`4`).
    DeadlineExceeded = 4,
    /// Requested entity was not found (`5`).
    NotFound = 5,
    /// Entity attempted to be created already exists (`6`).
    AlreadyExists = 6,
    /// Caller does not have permission (`7`).
    PermissionDenied = 7,
    /// Resource exhausted such as quota or message size limit (`8`).
    ResourceExhausted = 8,
    /// System is not in a state required for execution (`9`).
    FailedPrecondition = 9,
    /// Operation was aborted (`10`).
    Aborted = 10,
    /// Operation was attempted past the valid range (`11`).
    OutOfRange = 11,
    /// Operation is not implemented or supported (`12`).
    Unimplemented = 12,
    /// Internal server error (`13`).
    Internal = 13,
    /// Service is currently unavailable (`14`).
    Unavailable = 14,
    /// Unrecoverable data loss or corruption (`15`).
    DataLoss = 15,
    /// Request lacks valid authentication credentials (`16`).
    Unauthenticated = 16,
}
