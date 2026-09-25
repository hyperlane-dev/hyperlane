use super::*;

/// A trait representing common behaviors for HTTP response types.
///
/// This trait provides methods for transforming an HTTP response into
/// different formats (text and binary) and parsing raw HTTP response data.
/// Implementing types should define how to convert the response into text
/// and binary formats, as well as how to parse raw response data into a
/// structured representation.
///
/// # Associated Types
/// - `OutputText`: The type returned by the `text` method, typically a text-based HTTP response.
/// - `OutputBinary`: The type returned by the `binary` method, typically a binary-based HTTP response.
pub trait ResponseTrait: Send + Debug {
    type OutputText: Clone + Sized;
    type OutputBinary: Clone + Sized;

    /// Transforms the HTTP response into a text representation.
    ///
    /// This method converts the body of the HTTP response into a string format.
    ///
    /// # Returns
    /// - `Self::OutputText`: The text representation of the HTTP response, typically a string.
    fn text(&self) -> Self::OutputText;

    /// Transforms the HTTP response into a binary representation.
    ///
    /// This method converts the body of the HTTP response into a byte-based format.
    ///
    /// # Returns
    /// - `Self::OutputBinary`: The binary representation of the HTTP response, typically a byte vector.
    fn binary(&self) -> Self::OutputBinary;

    /// Parses a raw HTTP response into the associated type `Output`.
    ///
    /// This method is responsible for parsing a byte slice representing a raw HTTP response
    /// and transforming it into a structured HTTP response object.
    ///
    /// # Parameters
    /// - `response`: A byte slice representing the raw HTTP response.
    ///
    /// # Returns
    /// - `Self`: An instance of the implementing type, populated with parsed data.
    fn from(response: &[u8]) -> Self
    where
        Self: Sized;

    /// Decodes the data using a specified buffer size.
    ///
    /// This method takes a buffer size as input and performs the decoding process.
    /// It returns the decoded output in the form of `Self::OutputBinary`.
    ///
    /// # Parameters
    /// - `buffer_size`: The buffer size to be used during decoding.
    ///
    /// # Returns
    /// Returns the decoded data as `Self::OutputBinary`. The exact type of `OutputBinary` depends on the implementation of the `Self` type.
    fn decode(&self, buffer_size: usize) -> Self::OutputBinary;
}
