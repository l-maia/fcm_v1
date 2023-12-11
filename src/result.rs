/// Custom result type for FCM API calls.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FCMError {
    UnspecifiedError(String),
    InvalidArgument(String),
    Unregistered(String),
    SenderIdMismatch(String),
    QuotaExceeded(String),
    Unavailable(String),
    Internal(String),
    ThirdPartyAuthError(String),
}

impl From<(u16, String)> for FCMError {
    fn from((code, details): (u16, String)) -> Self {
        match code {
            400 => FCMError::InvalidArgument(details),
            404 => FCMError::Unregistered(details),
            403 => FCMError::SenderIdMismatch(details),
            429 => FCMError::QuotaExceeded(details),
            503 => FCMError::Unavailable(details),
            500 => FCMError::Internal(details),
            401 => FCMError::ThirdPartyAuthError(details),
            _ => FCMError::UnspecifiedError(details),
        }
    }
}

impl std::fmt::Display for FCMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FCMError::UnspecifiedError(details) => write!(f, "UnspecifiedError: {}", details),
            FCMError::InvalidArgument(details) => write!(f, "Invalid Argument: {}", details),
            FCMError::Unregistered(details) => write!(f, "Unregistered: {}", details),
            FCMError::SenderIdMismatch(details) => write!(f, "Sender Id Mismatch: {}", details),
            FCMError::QuotaExceeded(details) => write!(f, "Quota Exceeded: {}", details),
            FCMError::Unavailable(details) => write!(f, "Unavailable: {}", details),
            FCMError::Internal(details) => write!(f, "Internal : {}", details),
            FCMError::ThirdPartyAuthError(details) => {
                write!(f, "Third Party Auth Error: {}", details)
            }
        }
    }
}

/// Custom error type for FCM API calls.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// OAuth2 authentication error.
    Auth,
    /// Arbitrary configuration error (e.g. unable to initialize TLS backend).
    Config,
    /// Deserialization error (i.e. unexpected result format received from server).
    Deserialization,
    /// FCM server error (returned directly to caller).
    FCM(FCMError),
    /// Timed out while waiting for server. According to Google, [the server should use exponential back-off to
    /// deal with timeout errors](https://firebase.google.com/docs/cloud-messaging/server).
    Timeout,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Auth => write!(f, "authentication error"),
            Error::Config => write!(f, "configuration error"),
            Error::Deserialization => write!(f, "deserialization error"),
            Error::FCM(msg) => write!(f, "firebase error: {}", msg),
            Error::Timeout => write!(f, "timeout"),
        }
    }
}

impl std::error::Error for Error {}
