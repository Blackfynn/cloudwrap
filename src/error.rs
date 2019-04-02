use std::error::Error as StdError;
use std::io::Error as IoError;

use rusoto_core::{request::HttpDispatchError, RusotoError};
use rusoto_secretsmanager::{GetSecretValueError, ListSecretsError};
use rusoto_ssm::{DescribeParametersError, GetParametersByPathError};
use serde_json::Error as JsonError;

#[derive(Debug)]
pub enum Error {
    ExecError,
    GetSecretValueError(String),
    ListSecretsError(String),
    DescribeParametersError(String),
    GetParametersByPathError(String),
    RusotoUnknownServiceError(String),
    RusotoUnknownError(u16, String),
    RusotoCredentialsError(String),
    RusotoValidationError(String),
    RusotoHttpDispatchError(HttpDispatchError),
    RusotoParseError(String),
    InvalidKey(String),
    IoError(IoError),
    ParseError(JsonError),
}

impl<E: StdError + 'static> From<RusotoError<E>> for Error {
    fn from(e: RusotoError<E>) -> Self {
        let description = e.description().to_string();

        match e {
            RusotoError::Service(_) => {
                if let Some(source) = e.source() {
                    if let Some(_) = source.downcast_ref::<GetSecretValueError>() {
                        Error::GetSecretValueError(description)
                    } else if let Some(_) = source.downcast_ref::<ListSecretsError>() {
                        Error::ListSecretsError(description)
                    } else if let Some(_) = source.downcast_ref::<DescribeParametersError>() {
                        Error::DescribeParametersError(description)
                    } else if let Some(_) = source.downcast_ref::<GetParametersByPathError>() {
                        Error::GetParametersByPathError(description)
                    } else {
                        Error::RusotoUnknownServiceError(description)
                    }
                } else {
                    Error::RusotoUnknownServiceError(description)
                }
            }

            // Unknown errors do not show the actual readable response
            // from AWS by default
            RusotoError::Unknown(buffered_response) => Error::RusotoUnknownError(
                buffered_response.status.as_u16(),
                buffered_response.body_as_str().to_string(),
            ),

            RusotoError::Credentials(error) => Error::RusotoCredentialsError(error.message),
            RusotoError::Validation(details) => Error::RusotoValidationError(details),
            RusotoError::ParseError(details) => Error::RusotoParseError(details),
            RusotoError::HttpDispatch(error) => Error::RusotoHttpDispatchError(error),
        }
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Error::IoError(e)
    }
}

impl From<JsonError> for Error {
    fn from(e: JsonError) -> Self {
        Error::ParseError(e)
    }
}
