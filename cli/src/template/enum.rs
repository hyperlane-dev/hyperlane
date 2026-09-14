use super::*;

/// Types of template components that can be generated
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TemplateType {
    /// Controller component for handling HTTP requests
    Controller,
    /// Domain component for business logic encapsulation
    Domain,
    /// Exception component for error handling
    Exception,
    /// Mapper component for data transformation
    Mapper,
    /// Model component for data structures
    Model,
    /// Repository component for data access
    Repository,
    /// Service component for business services
    Service,
    /// Utils component for utility functions
    Utils,
    /// View component for presentation layer
    View,
}

/// Model subtypes for organizing data structures
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModelSubType {
    /// Application model for internal use
    Application,
    /// Request model for input validation
    Request,
    /// Response model for API responses
    Response,
}

/// Errors that can occur during template generation
#[derive(Debug, thiserror::Error)]
pub enum TemplateError {
    /// IO error occurred
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    /// Invalid template type
    #[error("Invalid template type: {0}")]
    InvalidTemplateType(String),
    /// Invalid model subtype
    #[error("Invalid model subtype: {0}")]
    InvalidModelSubType(String),
    /// Directory already exists
    #[error("Directory '{0}' already exists")]
    DirectoryExists(String),
}
