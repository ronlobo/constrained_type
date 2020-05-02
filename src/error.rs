#![deny(missing_docs)]

//! Error types for the crate

use std::fmt;

use thiserror::Error;

/// An alias for results returned by functions of this crate
pub type ConstrainedTypeResult<T> = ::std::result::Result<T, ConstrainedTypeError>;

/// The concrete error kind
#[derive(Error, Debug, Eq, PartialEq)]
pub enum ConstrainedTypeErrorKind {
    /// Number exceeded the maximum value
    #[error("{field_name:?} must not be less than {expected:?}, {found:?}")]
    InvalidMaxVal {
        /// Field name shown in the error
        field_name: String,
        /// Specified maximum value
        expected: String,
        /// Actual value
        found: String,
    },
    /// Number exceeded the minimum value
    #[error("{field_name:?} must not be greater than {expected:?}, {found:?}")]
    InvalidMinVal {
        /// Field name shown in the error
        field_name: String,
        /// Specified minimum value
        expected: String,
        /// Actual value
        found: String,
    },
    /// String does not match the pattern
    #[error("{field_name:?} does not match pattern {expected:?} for value {found:?}")]
    InvalidPattern {
        /// Field name shown in the error
        field_name: String,
        /// Specified pattern
        expected: String,
        /// Actual value
        found: String,
    },
    /// String is empty
    #[error("{field_name:?} must not be empty")]
    InvalidOption {
        /// Field name shown in the error
        field_name: String
    },
    /// Character data length exceeded the limit
    #[error("{field_name:?} must not be greater than {expected:?} characters, {found:?}")]
    InvalidMaxLen {
        /// Field name shown in the error
        field_name: String,
        /// Specified character limit
        expected: String,
        /// Actual value
        found: String,
    },
}

/// The error type for errors that get returned in the crate
#[derive(Error, Debug, Eq, PartialEq)]
pub struct ConstrainedTypeError {
    kind: ConstrainedTypeErrorKind,
}

impl ConstrainedTypeError {
    /// Get the kind of the error
    pub fn kind(&self) -> &ConstrainedTypeErrorKind {
        &self.kind
    }
}

impl From<ConstrainedTypeErrorKind> for ConstrainedTypeError {
    fn from(kind: ConstrainedTypeErrorKind) -> ConstrainedTypeError {
        ConstrainedTypeError {
            kind
        }
    }
}

impl<T> From<ConstrainedTypeError> for Result<T, ConstrainedTypeError> {
    fn from(e: ConstrainedTypeError) -> Self {
        Err(e)
    }
}

impl fmt::Display for ConstrainedTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.kind, f)
    }
}

